use self::io::*;
use self::structs::*;
use crate::configuration_parameters::ConfigurationParameters;
use convert::*;
use get_data_ei::get_processed_data_ei;
use get_data_nonei::get_processed_data_nonei;
use health_report::HealthReport;
use multimap::MultiMap;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use writer_ei::write_ei_data;
use writer_nonei::write_nonei_data;

mod convert;
mod get_data_ei;
mod get_data_nonei;
mod io;
mod structs;
mod writer_ei;
mod writer_nonei;

pub fn process(config_params: &ConfigurationParameters, _logger: &Logger, _diag_logger: &Logger) {
    let mut op_writer = get_writer(config_params.output_file_path());
    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;
    let mut tot_amt_in_ip = 0;
    let mut tot_amt_in_op = 0;
    let input = File::open(&config_params.input_file()).expect("Could Not Read File");
    let input_reader = BufReader::new(input);
    let mut writer_map_ei: HashMap<OPKeyEI, OPDataEI> = HashMap::new();
    let mut writer_map_nonei: HashMap<OPKeyNonEI, OPDataNonEI> = HashMap::new();
    let mut reader_map_ei: MultiMap<InputKeyEI, InputDataEI> = MultiMap::new();
    let mut reader_map_nonei: MultiMap<InputKeyNonEI, InputDataNonEI> = MultiMap::new();

    if config_params.loan_type().to_uppercase().trim() == "EI" {
        for (_index, line) in input_reader.lines().enumerate() {
            tot_acc_encntrd += 1;
            let line = line.expect("Could Not Read Line").to_string();
            let input_fields: Vec<&str> = line.split('|').collect();
            let key = InputKeyEI {
                acid: input_fields[0].to_string(),
            };
            let data = InputDataEI {
                shdl_num: input_fields[1].to_string(),
                num_of_dmds: to_i64(input_fields[2]),
                flow_start_date: to_date(input_fields[3]),
                flow_amt: to_i64(input_fields[4]),
                lr_freq_type: input_fields[5].to_string(),
                cf_code: input_fields[8].to_string(),
                num_of_flows: to_i64(input_fields[10]),
            };
            reader_map_ei.insert(key, data);
            acc_pro_suc += 1;
            tot_amt_in_ip += to_i64(input_fields[4]);
        }

        get_processed_data_ei(&mut reader_map_ei, &mut writer_map_ei, config_params);

        write_ei_data(
            &mut writer_map_ei,
            config_params,
            &mut op_writer,
            &mut tot_amt_in_op,
        );
    } else if config_params.loan_type().to_uppercase() == *"NON-EI" {
        for (_index, line) in input_reader.lines().enumerate() {
            tot_acc_encntrd += 1;
            let line = line.expect("Could Not Read Line").to_string();
            let input_fields: Vec<&str> = line.split('|').collect();
            let key = InputKeyNonEI {
                acid: input_fields[0].to_string(),
                cf_code: input_fields[8].to_string(),
            };
            let data = InputDataNonEI {
                shdl_num: input_fields[1].to_string(),
                num_of_dmds: to_i64(input_fields[2]),
                flow_start_date: to_date(input_fields[3]),
                flow_amt: to_i64(input_fields[4]),
                lr_freq_type: input_fields[5].to_string(),
                cf_code: input_fields[8].to_string(),
                num_of_flows: to_i64(input_fields[10]),
            };
            reader_map_nonei.insert(key, data);
            acc_pro_suc += 1;
            tot_amt_in_ip += to_i64(input_fields[4]);
        }

        get_processed_data_nonei(&mut reader_map_nonei, &mut writer_map_nonei, config_params);

        write_nonei_data(
            &mut writer_map_nonei,
            config_params,
            &mut op_writer,
            &mut tot_amt_in_op,
        );
    }

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
        tot_amt_in_ip as f64,
        tot_amt_in_op as f64,
        0,
    );
    health_report.gen_health_rpt(config_params.output_file_path());
}
