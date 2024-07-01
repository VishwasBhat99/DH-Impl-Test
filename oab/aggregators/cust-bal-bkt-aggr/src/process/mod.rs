use crate::configuration_parameters::ConfigurationParameters;
use crate::process::account::*;
use crate::process::writer::write_data;
use bucket::{get_amts_in_buckets, get_buckets_data};
use io::get_writer;
use slog::Logger;
use std::collections::HashMap;
use std::{fs, io::Write};

mod account;
mod bucket;
mod io;
mod writer;

pub fn process(config_params: &ConfigurationParameters, _logger: &Logger, _diag_logger: &Logger) {
    let mut tot_acc_encntrd = 0;
    let mut tot_amt_in_ip = 0.0;
    let mut tot_amt_in_op = 0.0;
    let mut tot_acc_proc = 0;
    let month_bucketing_start_day = rbdate::num_days_start_to_end(
        *config_params.as_on_date(),
        rbdate::incr_dt_by_mon_presrv_eom_checked(*config_params.as_on_date(), 6)
            .expect("Error in adding 6 months to AsOnDate"),
    );
    let buckets_data: HashMap<u32, u32> =
        get_buckets_data(config_params, month_bucketing_start_day);
    //Init Output Writer
    let op_path = format!(
        "{}.txt",
        &config_params.output_file_path().replace(".txt", "")
    );
    let mut op_writer = get_writer(&op_path);
    let mut writer_map: HashMap<(String, String), Vec<f64>> = HashMap::new();
    //Read LLG Mapper File
    let llg_mapper_reader = fs::read_to_string(config_params.llg_mapper_file_path())
        .expect("Could Not Read LLG Mapper File");
    let mut fileid_llg_mapper: HashMap<String, LLGMapper> = HashMap::new();
    for line in llg_mapper_reader.lines().skip(1) {
        let llg_mapper_vec = line.split('|').collect::<Vec<&str>>();
        let llg_mapper_data = LLGMapper::new(&llg_mapper_vec);
        fileid_llg_mapper.insert(llg_mapper_data.file_id.to_string(), llg_mapper_data);
    }

    //Read Final Ret/Non-Ret File
    let final_input_reader = fs::read_to_string(config_params.final_input_file_path())
        .expect("Could Not Read Final Ret/Non-Ret File");
    let mut is_cust_nwd: HashMap<(String, i64), Vec<String>> = HashMap::new();
    let mut final_input_map: HashMap<(String, i64, u32, bool), f64> = HashMap::new();
    for line in final_input_reader.lines() {
        let final_input_data = line.split('|').collect::<Vec<&str>>();
        let final_inp_acc = FinalAccount::new(&final_input_data);
        is_cust_nwd
            .entry((final_inp_acc.file_id.to_string(), final_inp_acc.cust_id))
            .and_modify(|data| {
                if data[0] != final_inp_acc.is_nwd_final.to_uppercase() {
                    data.push(final_inp_acc.is_nwd_final.to_uppercase())
                }
            })
            .or_insert(vec![final_inp_acc.is_nwd_final.to_uppercase()]);
        let res_days_above_two_yrs =
            rbdate::get_num_days_from_add(*config_params.as_on_date(), 2, 0, 0);
        let bucket_id = if final_inp_acc.residual_days > res_days_above_two_yrs as u32 {
            203_u32
        } else {
            *buckets_data.get(&final_inp_acc.residual_days).unwrap_or(&1)
        };
        final_input_map
            .entry((
                final_inp_acc.file_id.to_string(),
                final_inp_acc.cust_id,
                bucket_id,
                final_inp_acc.is_nwd_final == "TRUE",
            ))
            .and_modify(|amt| *amt += final_inp_acc.lcy_amount)
            .or_insert(final_inp_acc.lcy_amount);
    }

    //Read Ret/Non-Ret Cust-Bal-Aggr Output File
    let input_reader = fs::read_to_string(config_params.input_file_path())
        .expect("Could Not Read Ret/Non-Ret Cust-Bal-Aggr Output File");
    for line in input_reader.lines() {
        let input_data = line.split('|').collect::<Vec<&str>>();
        let inp_acc = Account::new(&input_data);
        tot_acc_encntrd += 1;
        let mut stable_vec: Vec<f64> = Vec::new();
        let mut less_stable_vec: Vec<f64> = Vec::new();
        let mut wd_stable_amt = inp_acc.stable_b1 + inp_acc.stable_b2 + inp_acc.stable_b3;
        let wd_less_stable_amt =
            inp_acc.less_stable_b1 + inp_acc.less_stable_b2 + inp_acc.less_stable_b3;
        let mut nwd_stable_amt =
            inp_acc.nwd_stable_b1 + inp_acc.nwd_stable_b2 + inp_acc.nwd_stable_b3;
        let nwd_less_stable_amt =
            inp_acc.nwd_less_stable_b1 + inp_acc.nwd_less_stable_b2 + inp_acc.nwd_less_stable_b3;
        let mut _is_nwd = false;
        if !is_cust_nwd.contains_key(&(inp_acc.file_id.to_string(), inp_acc.cust_id)) {
            continue;
        }
        let final_data = is_cust_nwd
            .get(&(inp_acc.file_id.to_string(), inp_acc.cust_id))
            .unwrap();

        if final_data.len() == 1 && final_data[0] == "TRUE" {
            _is_nwd = true;
        } else {
            get_amts_in_buckets(
                config_params,
                &mut final_input_map,
                inp_acc.clone(),
                &mut stable_vec,
                &mut less_stable_vec,
                &mut wd_stable_amt,
                false,
            );
            write_data(
                wd_less_stable_amt + wd_stable_amt,
                &mut fileid_llg_mapper,
                less_stable_vec.clone(),
                stable_vec.clone(),
                nwd_less_stable_amt,
                nwd_stable_amt,
                inp_acc.clone(),
                false,
                &mut writer_map,
            );
            stable_vec.clear();
            less_stable_vec.clear();
            get_amts_in_buckets(
                config_params,
                &mut final_input_map,
                inp_acc.clone(),
                &mut stable_vec,
                &mut less_stable_vec,
                &mut nwd_stable_amt,
                true,
            );
            write_data(
                nwd_less_stable_amt + nwd_stable_amt,
                &mut fileid_llg_mapper,
                less_stable_vec.clone(),
                stable_vec.clone(),
                nwd_less_stable_amt,
                nwd_stable_amt,
                inp_acc.clone(),
                true,
                &mut writer_map,
            );
            continue;
        }
        tot_acc_proc += 1;
        tot_amt_in_ip += wd_less_stable_amt + wd_stable_amt + nwd_less_stable_amt + nwd_stable_amt;
        let (mut stable_amt, less_stable_amt) = if _is_nwd {
            (nwd_stable_amt, nwd_less_stable_amt)
        } else {
            (wd_stable_amt, wd_less_stable_amt)
        };
        let write_stable_amt = stable_amt;
        get_amts_in_buckets(
            config_params,
            &mut final_input_map,
            inp_acc.clone(),
            &mut stable_vec,
            &mut less_stable_vec,
            &mut stable_amt,
            _is_nwd,
        );
        write_data(
            write_stable_amt + less_stable_amt,
            &mut fileid_llg_mapper,
            less_stable_vec.clone(),
            stable_vec.clone(),
            nwd_less_stable_amt,
            nwd_stable_amt,
            inp_acc.clone(),
            _is_nwd,
            &mut writer_map,
        );
        tot_amt_in_op += write_stable_amt + less_stable_amt;
    }
    for ((curr, llg), amts) in writer_map.iter() {
        writeln!(
            op_writer,
            "{}|{}|{}|{}|{}",
            config_params.as_on_date().format("%Y-%m-%d"),
            config_params.country_code(),
            curr,
            llg,
            amts.iter()
                .map(|amt| amt.to_string())
                .collect::<Vec<_>>()
                .join("|"),
        )
        .expect("Unable to write output file.");
    }
    // Flush Output Writers
    op_writer
        .flush()
        .expect("Error while flushing data from Writer!!");

    // Generate Health Check Report
    let health_report = health_report::HealthReport::new(
        tot_acc_encntrd,
        tot_acc_proc,
        tot_acc_encntrd - tot_acc_proc,
        tot_amt_in_ip,
        tot_amt_in_op,
        0,
    );
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file_path());
}
