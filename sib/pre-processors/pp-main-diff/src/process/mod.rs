use crate::configuration_parameters::ConfigurationParameters;
use crate::process::input_account::*;
use crate::process::output_account::{format_output, get_writer, OutputField};
use health_report::HealthReport;
use slog::Logger;
use std::collections::HashMap;
use std::{fs, io::Write};
mod input_account;
mod output_account;

pub fn process(config_params: &ConfigurationParameters, _logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_proc = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;

    //Reading Mapping master File
    let mut master_map: HashMap<String, MappingMaster> = HashMap::new();
    let mapping_master_reader = fs::read_to_string(config_params.mapping_master_file_path())
        .expect("Could Not Read Mapping master file");
    for (line_no, line) in mapping_master_reader.lines().enumerate().skip(0) {
        acc_enc += 1;
        let mapping_master_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let mapping_master_data: MappingMaster = MappingMaster::new(
            config_params.mapping_master_file_path(),
            &mapping_master_vec,
            line_no + 1,
        );
        let gl_acct_number = mapping_master_data.gl_account_number.to_string();
        master_map.insert(gl_acct_number, mapping_master_data);
    }
    //Reading Recon file
    let mut recon_map: HashMap<String, ReconData> = HashMap::new();
    let recon_file_reader =
        fs::read_to_string(config_params.recon_file_path()).expect("Could not read Recon file");
    for (line_no, line) in recon_file_reader.lines().enumerate().skip(0) {
        let recon_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let recon_data: ReconData = ReconData::new(
            config_params,
            config_params.recon_file_path(),
            &recon_vec,
            line_no + 1,
        );
        let gl_code = recon_data.gl_code.to_string();
        let ccy = recon_data.ccy.to_string();
        let agg_key = format!("{}{}", gl_code, ccy);
        let value_to_added = recon_data.lcy_aggr_amt;
        recon_map
            .entry(agg_key.clone())
            .and_modify(|recon_data| {
                recon_data.lcy_aggr_amt += value_to_added;
            })
            .or_insert(recon_data);
    }

    // Reading gstt file
    let mut gstt_map: HashMap<String, GsttData> = HashMap::new();
    let gstt_file_reader = fs::read_to_string(config_params.gstt_extraction_file_path())
        .expect("Could not read Gstt file");

    for (line_no, line) in gstt_file_reader.lines().enumerate().skip(0) {
        let gstt_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let gstt_data: GsttData = GsttData::new(
            config_params.gstt_extraction_file_path(),
            &gstt_vec,
            line_no + 1,
        );
        let gl_code = gstt_data.gl_sub_head.to_string();
        let ccy = gstt_data.crncy_code.to_string();
        let agg_key = format!("{}{}", gl_code, ccy);
        let cr_amount = gstt_data.tot_cr_bal;
        let dr_amount = gstt_data.tot_dr_bal;
        gstt_map
            .entry(agg_key.clone())
            .and_modify(|gstt_data| {
                gstt_data.tot_cr_bal += cr_amount;
                gstt_data.tot_dr_bal += dr_amount;
            })
            .or_insert(gstt_data);
    }
    let mut op_writer = get_writer(config_params.output_file());

    for (index, val) in recon_map {
        acc_proc += 1;
        let gl_code = val.gl_code;
        let ccy = val.ccy;
        let recon_amt = val.lcy_aggr_amt;
        let maping_val = master_map
            .get(&gl_code)
            .cloned()
            .unwrap_or(Default::default());
        let description = &maping_val.description;
        let classificaton = &maping_val.classification;
        let group = &maping_val.group;
        let llg = &maping_val.llg;
        let gstt_data = gstt_map.get(&index).cloned().unwrap_or(Default::default());
        let gstt_amt = gstt_data.tot_cr_bal - gstt_data.tot_dr_bal;
        let diff_amt = gstt_amt.abs() - recon_amt.abs();
        let output_data: OutputField = OutputField {
            gl_code,
            ccy,
            description: description.to_string(),
            classification: classificaton.to_string(),
            group: group.to_string(),
            llg: llg.to_string(),
            recon_amt,
            gstt_amt,
            diff_amt,
        };
        ip_amt += gstt_amt;
        op_amt += diff_amt;

        writeln!(op_writer, "{}", format_output(output_data)).expect("Error in Writing Output");
    }

    let health_report = HealthReport::new(acc_enc, acc_proc, acc_enc - acc_proc, ip_amt, op_amt, 0);
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file());
}
