use self::io::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
use health_report::HealthReport;
use rbdate::DateParser;
use sdb_io::new_buf_rdr;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut op_writer = get_writer(&config_params.output_file_path());
    let date_parser = DateParser::new("%d-%b-%Y".to_string(), false);

    let input = File::open(&config_params.input_file()).expect("Could Not Read File");
    let input_reader = BufReader::new(input);
    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;
    let mut tot_amt = 0.0;

    for (index, line) in input_reader.lines().enumerate().skip(1) {
        let line = line.expect("Could Not Read Line").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        tot_acc_encntrd += 1;
        acc_pro_suc += 1;
        let mut leg_type = input_fields[9].to_string().to_lowercase();
        write!(
            op_writer,
            "{}{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}||{}|{}|{}|{}|{}|{}||{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}||||||{}|{}|||\n",
            input_fields[0],
            input_fields[3],
            input_fields[1],
            input_fields[2],
            input_fields[3],
            input_fields[4],
            input_fields[5],
            date_parser.parse_opt(&input_fields[6]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            date_parser.parse_opt(&input_fields[7]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            date_parser.parse_opt(&input_fields[8]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            input_fields[11],
            if leg_type == "receipt" {
                input_fields[12]
            }else{
                ""
            },
            if leg_type == "receipt" {
                input_fields[13]
            }else{
                ""
            },
            if leg_type == "receipt" {
                input_fields[14]
            }else{
                ""
            },
            if leg_type == "receipt" {
                input_fields[15]
            }else{
                ""
            },
            input_fields[11],
            if leg_type == "payment" {
                input_fields[12]
            }else{
                ""
            },
            if leg_type == "payment" {
                input_fields[13]
            }else{
                ""
            },
            if leg_type == "payment" {
                input_fields[14]
            }else{
                ""
            },
            if leg_type == "payment" {
                input_fields[15]
            }else{
                ""
            },
            if leg_type == "payment" {
                input_fields[16]
            }else{
                ""
            },
            if leg_type == "payment" {
                input_fields[17]
            }else{
                ""
            },
            if leg_type == "payment" {
                input_fields[18]
            }else{
                ""
            },
            if leg_type == "receipt" {
                input_fields[16]
            }else{
                ""
            },
            if leg_type == "receipt" {
                input_fields[17]
            }else{
                ""
            },
            if leg_type == "receipt" {
                input_fields[18]
            }else{
                ""
            },
            input_fields[19],
            "",
            if leg_type == "payment" {
                (date_parser.parse_opt(&input_fields[20]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y")).to_string()
            }else{
                "".to_string()
            },
            if leg_type == "receipt" {
                date_parser.parse_opt(&input_fields[20]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y").to_string()
            }else{
                "".to_string()
            },
            date_parser.parse_opt(&input_fields[21]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y").to_string(),
            date_parser.parse_opt(&input_fields[21]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y").to_string(),
            if leg_type == "payment" {
                input_fields[22]
            }else{
                ""
            },
            if leg_type == "receipt" {
                input_fields[22]
            }else{
                ""
            },
            if leg_type == "payment" {
                input_fields[23]
            }else{
                ""
            },
            if leg_type == "receipt" {
                input_fields[23]
            }else{
                ""
            },
            if leg_type == "payment" {
                input_fields[24]
            }else{
                ""
            },
            if leg_type == "receipt" {
                input_fields[24]
            }else{
                ""
            },
            input_fields[9],
            input_fields[29],
            input_fields[28]
        );
    }
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
        tot_amt,
        tot_amt,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}
