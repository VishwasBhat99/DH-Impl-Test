use crate::configuration_parameters::ConfigurationParameters;
use crate::process::input_account::*;
use crate::process::output_account::{format_output, get_writer, OutputField};
use calamine::{open_workbook_auto, Reader};
use health_report::HealthReport;
use slog::Logger;
use std::collections::{HashMap, HashSet};
use std::{fs, io::Write};
mod input_account;
mod output_account;

pub fn process(config_params: &ConfigurationParameters, _logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_proc = 0;
    //Reading NPA file

    let mut concat_map: HashMap<String,OutputField> = HashMap::new();
    let mut asset_class_map: HashMap<String, OutputField> = HashMap::new();
    let npa_file_reader =
        fs::read_to_string(config_params.npa_file_path()).expect("Could Not Read NPA file");
    for (line_no, line) in npa_file_reader.lines().enumerate() {
        acc_enc += 1;
        let npa_vec: Vec<&str> = line.split(',').collect::<Vec<&str>>();
        let npa_data: NpaData = NpaData::new(
            config_params,
            config_params.npa_file_path(),
            &npa_vec,
            line_no + 1,
        );
        let src_system=npa_data.src_system.to_string();
        let asst_class=npa_data.asst_class.to_string();
        let net_npa=npa_data.net_npa.clone();
        let asst_class_temp=npa_data.asst_class.to_string().to_ascii_uppercase();
        let spec_prov=npa_data.spec_prov.clone();
        let tot_sum= npa_data.tot_prov + net_npa;
        let concat = format!("{}{}",src_system,asst_class);
        let concat_map_data = OutputField {
            src_code: src_system.clone(),
            tot_bal: spec_prov.clone(),
            asst_class:asst_class.clone(),
        };
        let asst_map_data =OutputField{
            src_code: "LOAN".to_string(),
            tot_bal: tot_sum,
            asst_class,
        };
        concat_map.entry(concat).and_modify(|prev_data|{
             prev_data.tot_bal += spec_prov
        }).or_insert(concat_map_data);
        asset_class_map.entry(asst_class_temp).and_modify(|prev_data|{
           prev_data.tot_bal += tot_sum
        }).or_insert(asst_map_data);
        }


    let mut op_writer = get_writer(config_params.output_file());
    for (_key,val) in concat_map {
        acc_proc += 1;
        writeln!(op_writer, "{}", format_output(val)).expect("Error in Writing Output");
    }
    for (_key,val) in asset_class_map {
        acc_proc += 1;
        writeln!(op_writer, "{}", format_output(val)).expect("Error in Writing Output");
    }

    let health_report = HealthReport::new(acc_enc, acc_proc, 0, 0.0, 0.0, 0);
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file());
}
