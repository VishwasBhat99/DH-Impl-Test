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
        if date_parser
            .parse_opt(&input_fields[17])
            .unwrap_or(*config_params.as_on_date())
            > *config_params.as_on_date()
        {
            acc_pro_suc += 1;
            write!(
            op_writer,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            input_fields[0],
            input_fields[1],
            input_fields[2],
            input_fields[3],
            input_fields[4],
            input_fields[5],
            date_parser.parse_opt(&input_fields[6]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            input_fields[7],
            input_fields[8],
            input_fields[9],
            date_parser.parse_opt(&input_fields[10]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            input_fields[11],
            input_fields[12],
            input_fields[13],
            input_fields[14],
            input_fields[15],
            input_fields[16],
            date_parser.parse_opt(&input_fields[17]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            input_fields[18],
            input_fields[19],
            input_fields[20],
            input_fields[21],
            input_fields[22],
            input_fields[23],
            date_parser.parse_opt(&input_fields[24]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            input_fields[25],
            date_parser.parse_opt(&input_fields[26]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            input_fields[27],
            input_fields[28],
            date_parser.parse_opt(&input_fields[29]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            input_fields[30],
            input_fields[31],
            input_fields[32],
            date_parser.parse_opt(&input_fields[33]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            input_fields[34],
            input_fields[35],
            input_fields[36],
            input_fields[40],
            input_fields[41],
            date_parser.parse_opt(&input_fields[42]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            date_parser.parse_opt(&input_fields[43]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            input_fields[44],
            input_fields[45],
            input_fields[13].parse().unwrap_or(0.0) + input_fields[15].parse().unwrap_or(0.0)
        );
        }
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
