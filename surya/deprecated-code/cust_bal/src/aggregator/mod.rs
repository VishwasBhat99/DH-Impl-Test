use self::account_reader::InputAccountReader;
use self::reader::BiuData;
use self::structs::Schema;
use self::structs::{AccData, AccKey, AggrData};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::Write;

mod account_reader;
mod reader;
mod structs;

pub fn aggregate(config_params: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let reader = InputAccountReader::new(config_params.input_file_path(), log);
    // init ret and non-ret writers
    let lcy_op = format!("{}-lcy.txt", config_params.output_file_path());
    let mut lcy_writer = match buf_file_wrtr(&lcy_op, None) {
        Ok(val) => val,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                lcy_op,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let mut reader_iterator = reader.into_iter();
    // init bucket schema file
    let bkt_file = match new_buf_rdr(config_params.bkt_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.bkt_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    // read biu master file
    let biu_master_file = match new_buf_rdr(config_params.biu_master_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.biu_master_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut biu_master: HashMap<String, BiuData> = HashMap::new();
    for (line_num, lines) in biu_master_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.output_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        let biu_line = BiuData {
            t1: fields[1].to_string(),
            t2: fields[2].to_string(),
            t3: fields[3].to_string(),
            nob: fields[4].to_string(),
            desc: fields[5].to_string(),
        };
        biu_master.insert(fields[0].to_string(), biu_line);
    }
    log_info!(log, "Reading BIU data completed.");
    // read casa ret file
    let mut tot_rec = 0;
    let skp_rec = 0;
    let mut tot_amt = 0.0;
    let casa_ret_file = match new_buf_rdr(config_params.casa_ret_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.casa_ret_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut casa_master: HashMap<AccKey, Vec<AccData>> = HashMap::new();
    for (line_num, lines) in casa_ret_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.casa_ret_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        tot_rec += 1;
        tot_amt += fields[7].parse::<f64>().unwrap_or(0.0);
        let casa_acc_line = AccData {
            amount: fields[7].parse::<f64>().unwrap_or(0.0),
            lcy_amount: fields[8].parse::<f64>().unwrap_or(0.0),
            is_nwd_final: fields[12].to_string(),
            bucket_id: fields[13].parse::<usize>().unwrap_or(0),
        };
        let key = AccKey {
            class_id: fields[0].to_string(),
            cust_id: fields[2].to_string(),
        };
        if let Some(data) = casa_master.get_mut(&key) {
            data.push(casa_acc_line);
        } else {
            casa_master.insert(key, vec![casa_acc_line]);
        }
    }
    log_info!(log, "Reading CASA data completed.");
    // read td ret file
    let td_ret_file = match new_buf_rdr(config_params.td_ret_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.bkt_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut td_master: HashMap<AccKey, Vec<AccData>> = HashMap::new();
    for (line_num, lines) in td_ret_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.output_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        tot_rec += 1;
        tot_amt += fields[7].parse::<f64>().unwrap_or(0.0);
        let acc_line = AccData {
            amount: fields[7].parse::<f64>().unwrap_or(0.0),
            lcy_amount: fields[8].parse::<f64>().unwrap_or(0.0),
            is_nwd_final: fields[12].to_string(),
            bucket_id: fields[13].parse::<usize>().unwrap_or(0),
        };
        let key = AccKey {
            class_id: fields[0].to_string(),
            cust_id: fields[2].to_string(),
        };
        if let Some(data) = td_master.get_mut(&key) {
            data.push(acc_line);
        } else {
            td_master.insert(key, vec![acc_line]);
        }
    }
    log_info!(log, "Reading TD data completed.");
    // read rd ret file
    let rd_ret_file = match new_buf_rdr(config_params.rd_ret_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.bkt_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut rd_master: HashMap<AccKey, Vec<AccData>> = HashMap::new();
    for (line_num, lines) in rd_ret_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.output_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        tot_rec += 1;
        tot_amt += fields[7].parse::<f64>().unwrap_or(0.0);
        let acc_line = AccData {
            amount: fields[7].parse::<f64>().unwrap_or(0.0),
            lcy_amount: fields[8].parse::<f64>().unwrap_or(0.0),
            is_nwd_final: fields[12].to_string(),
            bucket_id: fields[13].parse::<usize>().unwrap_or(0),
        };
        let key = AccKey {
            class_id: fields[0].to_string(),
            cust_id: fields[2].to_string(),
        };
        if let Some(data) = rd_master.get_mut(&key) {
            data.push(acc_line);
        } else {
            rd_master.insert(key, vec![acc_line]);
        }
    }
    log_info!(log, "Reading RD data completed.");
    let mut bkt_def: Vec<Schema> = Vec::new();
    let mut num_of_bkts = 0;
    for (line_num, lines) in bkt_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.bkt_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        let bkt_id = fields[2]
            .parse::<i64>()
            .expect("Cannot parse bkt id as integer.");
        let bkt_schema = Schema {
            from_bkt: fields[0]
                .parse::<i64>()
                .expect("Cannot parse from_bkt as integer."),
            to_bkt: fields[1]
                .parse::<i64>()
                .expect("Cannot parse to_bkt as integer."),
            id: bkt_id,
        };
        if bkt_id > num_of_bkts {
            num_of_bkts = bkt_id;
        }
        bkt_def.push(bkt_schema);
    }
    loop {
        let account_opt = reader_iterator.next();
        if account_opt.is_none() {
            break;
        }
        let input_account = account_opt.expect("Unable to parse record into input struct");
        let no_data = AccData {
            amount: 0.0,
            lcy_amount: 0.0,
            is_nwd_final: "FALSE".to_string(),
            bucket_id: 0,
        };
        let max_stable_amt: f64 = config_params.max_stable_amount();
        let biu_no_data = BiuData {
            t1: "N".to_string(),
            t2: "N".to_string(),
            t3: "N".to_string(),
            nob: "NA".to_string(),
            desc: "NA".to_string(),
        };
        // tuple fields: T1, T2, T3, NOB
        let biu_data = match biu_master.get(&input_account.customer_id) {
            Some(val) => val,
            None => &biu_no_data,
        };
        let acc_key = AccKey {
            class_id: input_account.class_id,
            cust_id: input_account.customer_id,
        };
        let mut acc_data = AggrData::new(num_of_bkts as usize);
        acc_data.init_biu_data(&biu_data.t1, &biu_data.t2, &biu_data.t3);
        // tuple fields: amount, lcy_amount, is_nwd, bkt_id
        match casa_master.get(&acc_key) {
            Some(val) => {
                // aggr rd data bu cust id
                for casa_data in val {
                    acc_data.add_data(casa_data);
                }
            }
            None => {}
        };
        match td_master.get(&acc_key) {
            Some(val) => {
                // aggr td data bu cust id
                for td_data in val {
                    acc_data.aggr_acc_data(&td_data, &no_data);
                }
            }
            None => {}
        };
        match rd_master.get(&acc_key) {
            Some(val) => {
                // aggr rd data bu cust id
                for rd_data in val {
                    acc_data.aggr_acc_data(&no_data, &rd_data);
                }
            }
            None => {}
        };
        calculate_write_stable_data(acc_key, acc_data, max_stable_amt, &mut lcy_writer);
    }
    log_info!(log, "Completed level 1 processing all the records.");
    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file_path());
}

fn calculate_write_stable_data(
    cust_key: AccKey,
    data: AggrData,
    max_stable_amt: f64,
    lcy_writer: &mut BufWriter<File>,
) {
    let logic_type = if data.t1 == "N" && data.t2 == "N" && data.t3 == "N" {
        "N"
    } else {
        "Y"
    };
    // TODO: Get rid of this duplication of code
    let total_lcy_wd = data.casa_lcy_amt + data.td_wd_lcy_amt + data.rd_lcy_amt;
    let total_lcy_stable;
    let total_lcy_unstable;
    let casa_lcy_stable;
    let casa_lcy_unstable;

    if logic_type == "Y" {
        if total_lcy_wd > max_stable_amt {
            total_lcy_stable = max_stable_amt;
            total_lcy_unstable = total_lcy_wd - max_stable_amt;
        } else {
            total_lcy_stable = total_lcy_wd;
            total_lcy_unstable = 0.0;
        }
    } else {
        total_lcy_stable = 0.0;
        total_lcy_unstable = total_lcy_wd;
    }
    if data.casa_lcy_amt < total_lcy_stable {
        casa_lcy_stable = data.casa_lcy_amt;
    } else {
        casa_lcy_stable = total_lcy_stable;
    }
    casa_lcy_unstable = data.casa_lcy_amt - casa_lcy_stable;
    let mut remaining_stable;
    if casa_lcy_stable != 0.0 {
        remaining_stable = max_stable_amt - casa_lcy_stable;
    } else {
        remaining_stable = 0.0;
    }
    let mut stable_lcy_bkts: Vec<f64> = Vec::new();
    let mut unstable_lcy_bkts: Vec<f64> = Vec::new();
    for val in &data.td_rd_wd_lcy_bkts {
        if remaining_stable == 0.0 {
            stable_lcy_bkts.push(0.0);
            unstable_lcy_bkts.push(*val);
        } else {
            if val < &remaining_stable {
                stable_lcy_bkts.push(*val);
                unstable_lcy_bkts.push(0.0);
                remaining_stable = remaining_stable - val;
            } else {
                stable_lcy_bkts.push(remaining_stable);
                unstable_lcy_bkts.push(*val - remaining_stable);
                remaining_stable = 0.0;
            }
        }
    }
    write!(
        lcy_writer,
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        cust_key.class_id,
        cust_key.cust_id,
        data.casa_lcy_amt,
        data.td_wd_lcy_amt,
        data.td_nwd_lcy_amt,
        data.rd_lcy_amt,
        vec_as_str(data.td_wd_lcy_bkts),
        vec_as_str(data.td_nwd_lcy_bkts),
        vec_as_str(data.rd_wd_lcy_bkts),
        vec_as_str(data.td_rd_wd_lcy_bkts),
        data.t1,
        data.t2,
        data.t3,
        total_lcy_wd,
        data.td_nwd_lcy_amt,
        logic_type,
        total_lcy_stable,
        total_lcy_unstable,
        casa_lcy_stable,
        casa_lcy_unstable,
        vec_as_str(stable_lcy_bkts),
        vec_as_str(unstable_lcy_bkts),
    )
    .expect("Unable to generate LCY summary file.");
}

fn vec_as_str(bkts: Vec<f64>) -> String {
    let mut vec_as_str: String = String::new();
    let mut is_skip = true;
    for val in bkts {
        if is_skip {
            is_skip = false;
            continue;
        }
        vec_as_str.push_str(&val.to_string());
        vec_as_str.push('|');
    }
    vec_as_str.pop();
    vec_as_str
}
