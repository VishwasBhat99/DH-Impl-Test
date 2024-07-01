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
    let master_file = File::open(&config_params.input_master_file()).expect("Could Not Read File");
    let master_reader = BufReader::new(master_file);
    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;
    let mut tot_amt = 0.0;
    let cf_file = File::open(&config_params.input_cf_file()).expect("Could Not Read File");
    let cf_reader = BufReader::new(cf_file);

    let mut cf_file_date: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    for (index, line) in cf_reader.lines().enumerate().skip(1) {
        let mut cf_record: Vec<String> = Vec::new();
        for component in line.expect("Could Not Read Line").split('|') {
            cf_record.push(component.to_string());
        }
        let mut key = &cf_record[0].to_owned();
        if cf_file_date.contains_key(&cf_record[0]) {
            cf_file_date
                .get_mut(&cf_record[0])
                .as_mut()
                .unwrap()
                .push(cf_record);
        } else {
            let mut cf_val: Vec<Vec<String>> = Vec::new();
            cf_val.push(cf_record);
            cf_file_date.insert(key.to_string(), cf_val);
        }
    }

    let date_parser = DateParser::new("%d-%b-%Y".to_string(), false);
    for (index, line) in master_reader.lines().enumerate().skip(1) {
        tot_acc_encntrd += 1;
        let line = line.expect("Could Not Read Line").to_string();
        let master_fields: Vec<&str> = line.split('|').collect();
        let mut principal_record_present = false;
        let mut balance = 0.0;
        let mut reference = "";
        let mut record_present = false;
        let mut cash_flow_Date;
        acc_pro_suc += 1;
        if cf_file_date.contains_key(master_fields[0]) {
            record_present = true;
            for val in cf_file_date.get(master_fields[0]) {
                for cf_fields in val {
                    if cf_fields[2] == "P" {
                        principal_record_present = true;
                        reference = &cf_fields[1];
                        balance += cf_fields[3].parse::<f64>().unwrap_or(0.0);
                    }
                    if master_fields[22] == "" && master_fields[10] == "" && cf_fields[5] == "" {
                        cash_flow_Date = config_params
                            .as_on_date()
                            .succ()
                            .format("%d-%b-%Y")
                            .to_string();
                    } else if master_fields[22] == ""
                        && master_fields[10] == ""
                        && cf_fields[5] != ""
                    {
                        cash_flow_Date = cf_fields[5].to_string();
                    } else if master_fields[22] == ""
                        && master_fields[10] != ""
                        && cf_fields[5] == ""
                    {
                        cash_flow_Date = master_fields[10].to_string();
                    } else if master_fields[22] != ""
                        && master_fields[10] == ""
                        && cf_fields[5] == ""
                    {
                        cash_flow_Date = master_fields[22].to_string();
                    } else if master_fields[22] != ""
                        && master_fields[10] != ""
                        && cf_fields[5] == ""
                    {
                        if date_parser.parse(master_fields[22])
                            < date_parser.parse(master_fields[10])
                        {
                            cash_flow_Date = master_fields[22].to_string();
                        } else {
                            cash_flow_Date = master_fields[10].to_string();
                        }
                    } else if master_fields[22] == ""
                        && master_fields[10] != ""
                        && cf_fields[5] != ""
                    {
                        if date_parser.parse(&cf_fields[5]) < date_parser.parse(master_fields[10]) {
                            cash_flow_Date = cf_fields[5].to_string();
                        } else {
                            cash_flow_Date = master_fields[10].to_string();
                        }
                    } else if date_parser.parse(master_fields[22])
                        > date_parser.parse(&cf_fields[5])
                    {
                        cash_flow_Date = cf_fields[5].to_string();
                    } else {
                        cash_flow_Date = master_fields[22].to_string();
                    }
                    write!(
                    op_writer,
                    "{}|{}|{}||{}||{}|{}|{}|{}||{}|{}|{}|{}|{}|{}||{}|{}|{}|||||{}|{}|{}|{}||{}||||{}|{}|{}|{}|||||{}||||||||\n",
                    cf_fields[1],
                    master_fields[0],
                    master_fields[6],
                    master_fields[11],
                    date_parser.parse_opt(&master_fields[9]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
                    date_parser.parse_opt(&master_fields[9]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
                    date_parser.parse_opt(&master_fields[10]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
                    date_parser.parse(&cash_flow_Date).format("%d-%m-%Y"),
                    master_fields[1],
                    master_fields[24],
                    master_fields[8],
                    master_fields[7],
                    if cf_fields[2] == "P" {
                        "PRINCIPAL"
                    }
                    else{
                        "MAIN_INT"
                    },
                    master_fields[14],
                    cf_fields[3],
                    master_fields[13],
                    master_fields[26],
                    master_fields[15],
                    date_parser.parse_opt(&master_fields[22]).unwrap_or(DateParser::new("%d-%b-%Y".to_string(), false).parse("31-Dec-2099")).format("%d-%m-%Y"),
                    if master_fields[21] == ""{
                        date_parser.parse_opt(&master_fields[9]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y")
                    }
                    else{
                        date_parser.parse(&master_fields[21]).format("%d-%m-%Y")
                    },
                    master_fields[12],
                    master_fields[2],
                    master_fields[16],
                    date_parser.parse_opt(&master_fields[20]).unwrap_or(DateParser::new("%d-%b-%Y".to_string(), false).parse("31-Dec-2099")).format("%d-%m-%Y"),
                    if master_fields[19] == ""{
                        date_parser.parse_opt(&master_fields[9]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y")
                    }
                     else{
                        date_parser.parse(&master_fields[19]).format("%d-%m-%Y")
                    },
                    config_params.as_on_date().format("%d-%m-%Y"),
                    master_fields[15],
                );
                }
            }
        }
        if record_present == false
            || principal_record_present == false
            || balance != master_fields[7].parse::<f64>().unwrap_or(0.0)
        {
            if master_fields[22] == "" && master_fields[10] == "" {
                cash_flow_Date = config_params.as_on_date().to_string();
            } else if master_fields[10] == "" || master_fields[22] == "" {
                if master_fields[10] == "" {
                    cash_flow_Date = master_fields[22].to_string()
                } else {
                    cash_flow_Date = master_fields[10].to_string()
                }
            } else if date_parser.parse(master_fields[10]) > date_parser.parse(master_fields[22]) {
                cash_flow_Date = master_fields[22].to_string()
            } else {
                cash_flow_Date = master_fields[10].to_string()
            }
            write!(
                op_writer,
                "{}|{}|{}||{}||{}|{}|{}|{}||{}|{}|{}|{}|{}|{}||{}|{}|{}|||||{}|{}|{}|{}||{}||||{}|{}|{}|{}|||||{}||||||||\n",
                if reference !=""{
                    reference
                }else{
                    master_fields[0]
                },
                master_fields[0],
                master_fields[6],
                master_fields[11],
                date_parser.parse_opt(&master_fields[9]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
                date_parser.parse_opt(&master_fields[9]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
                date_parser.parse_opt(&master_fields[10]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
                date_parser.parse(&cash_flow_Date).format("%d-%m-%Y"),
                master_fields[1],
                master_fields[24],
                master_fields[8],
                master_fields[7],
                "PRINCIPAL",
                master_fields[14],
                master_fields[7].to_string()
                .parse::<f64>()
                .unwrap_or(0.0)-balance,
                master_fields[13],
                master_fields[26],
                master_fields[15],
                date_parser.parse_opt(&master_fields[22]).unwrap_or(DateParser::new("%d-%b-%Y".to_string(), false).parse("31-Dec-2099")).format("%d-%m-%Y"),
                if master_fields[21] == ""{
                    date_parser.parse_opt(&master_fields[9]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y")
                }
                else{
                    date_parser.parse(&master_fields[21]).format("%d-%m-%Y")
                },
                master_fields[12],
                master_fields[2],
                master_fields[16],
                date_parser.parse_opt(&master_fields[20]).unwrap_or(DateParser::new("%d-%b-%Y".to_string(), false).parse("31-Dec-2099")).format("%d-%m-%Y"),
                if master_fields[19] == ""{
                    date_parser.parse_opt(&master_fields[9]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y")
                }
                else{
                    date_parser.parse(&master_fields[19]).format("%d-%m-%Y")
                },
                config_params.as_on_date().format("%d-%m-%Y"),
                master_fields[15],
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
