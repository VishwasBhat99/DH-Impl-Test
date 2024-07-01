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
    let td_file = File::open(&config_params.input_td_file()).expect("Could Not Read File");
    let td_reader = BufReader::new(td_file);
    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;
    let mut tot_amt = 0.0;
    let date_parser = DateParser::new("%d-%b-%Y".to_string(), false);

    for (index, line) in td_reader.lines().enumerate().skip(1) {
        let line = line.expect("Could Not Read Line").to_string();
        let td_fields: Vec<&str> = line.split('|').collect();
        tot_acc_encntrd += 1;
        if date_parser
            .parse_opt(&td_fields[17])
            .unwrap_or(*config_params.as_on_date())
            > *config_params.as_on_date()
        {
            acc_pro_suc += 1;
            write!(op_writer,
                "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}||||||||{}\n",
                td_fields[0],
                td_fields[1],
                td_fields[2],
                td_fields[3],
                td_fields[4],
                td_fields[5],
                date_parser.parse_opt(&td_fields[6]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
                td_fields[7],
                td_fields[8],
                td_fields[9],
                date_parser.parse_opt(&td_fields[10]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
                td_fields[11],
                td_fields[12],
                td_fields[13],
                td_fields[14],
                td_fields[15],
                td_fields[16],
                date_parser.parse_opt(&td_fields[17]).unwrap_or(config_params.as_on_date().succ()).format("%d-%m-%Y"),
                td_fields[18],
                td_fields[19],
                td_fields[20],
                td_fields[21],
                td_fields[22],
                td_fields[23],
                date_parser.parse_opt(&td_fields[24]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
                td_fields[25],
                date_parser.parse_opt(&td_fields[26]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
                td_fields[27],
                td_fields[28],
                date_parser.parse_opt(&td_fields[29]).unwrap_or(date_parser.parse_opt(&td_fields[17]).unwrap_or(*config_params.as_on_date())).format("%d-%m-%Y"),
                td_fields[30],
                td_fields[31],
                td_fields[32],
                date_parser.parse_opt(&td_fields[33]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
                td_fields[34],
                td_fields[35],
                td_fields[36],
                td_fields[37],
                td_fields[38],
                td_fields[39],
                td_fields[40],
                td_fields[41],
                date_parser.parse_opt(&td_fields[42]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
                date_parser.parse_opt(&td_fields[43]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
                td_fields[44],
                td_fields[45],
                td_fields[13].parse().unwrap_or(0.0) + td_fields[15].parse().unwrap_or(0.0)
            ).expect("the output line can not be written");
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
