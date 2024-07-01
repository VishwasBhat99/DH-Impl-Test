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

    let repayment_file =
        File::open(&config_params.input_repayment_file()).expect("Could Not Read File");
    let repayment_reader = BufReader::new(repayment_file);

    let mut repayment_file_data: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    for (index, line) in repayment_reader.lines().enumerate().skip(1) {
        let mut repayment_record: Vec<String> = Vec::new();
        for component in line.expect("Could Not Read Line").split('|') {
            repayment_record.push(component.to_string());
        }
        let mut key = &repayment_record[2].to_owned();
        if repayment_file_data.contains_key(&repayment_record[2]) {
            repayment_file_data
                .get_mut(&repayment_record[2])
                .as_mut()
                .unwrap()
                .push(repayment_record);
        } else {
            let mut repayment_val: Vec<Vec<String>> = Vec::new();
            repayment_val.push(repayment_record);
            repayment_file_data.insert(key.to_string(), repayment_val);
        }
    }

    for (index, line) in master_reader.lines().enumerate().skip(1) {
        let line = line.expect("Could Not Read Line").to_string();
        let master_fields: Vec<&str> = line.split('|').collect();
        tot_acc_encntrd += 1;
        if repayment_file_data.contains_key(master_fields[0]) {
            acc_pro_suc += 1;
            for val in repayment_file_data.get(master_fields[0]) {
                for repayment_fields in val {
                    if repayment_fields[3].replace(" ", "").to_lowercase() != "leg1"
                        && repayment_fields[3].to_lowercase() != "interest"
                    {
                        continue;
                    }
                    let leg_type;
                    let amt_due;
                    if repayment_fields[3].replace(" ", "").to_lowercase() == "leg1" {
                        leg_type = "PRINCIPAL";
                        amt_due = &repayment_fields[4];
                    } else {
                        leg_type = "MAIN_INT";
                        amt_due = &repayment_fields[9];
                    }
                    write!(
                        op_writer,
                        "{}_{}_{}|{}|{}||{}|{}|{}|{}|{}|{}|{}|{}||{}|{}|{}|{}||{}||{}|{}||{}|{}||{}|{}|||{}|||||{}|{}|{}|||{}||\n",
                        master_fields[0],
                        master_fields[1],
                        master_fields[2],
                        repayment_fields[2],
                        master_fields[7],
                        master_fields[9],
                        repayment_fields[6],
                        if master_fields[10] != ""{
                            master_fields[10].to_string()
                        }else{
                            config_params.as_on_date().format("%d-%m-%Y").to_string()
                        },
                        if master_fields[19] != ""{
                            master_fields[19].to_string()
                        }else{
                            config_params.as_on_date().format("%d-%m-%Y").to_string()
                        },
                        if master_fields[21] != ""{
                            master_fields[21].to_string()
                        }else{
                            config_params.as_on_date().format("%d-%m-%Y").to_string()
                        },
                        if master_fields[21] != ""{
                            master_fields[21].to_string()
                        }else{
                            config_params.as_on_date().format("%d-%m-%Y").to_string()
                        },
                        master_fields[5],
                        master_fields[6],
                        config_params.currency(),
                        master_fields[22],
                        leg_type,
                        amt_due,
                        amt_due,
                        master_fields[16],
                        master_fields[13],
                        master_fields[23],
                        master_fields[25],
                        config_params.as_on_date().format("%d-%m-%Y"),
                        config_params.as_on_date().format("%d-%m-%Y"),
                        master_fields[3],
                        if master_fields[17] != ""{
                            master_fields[17].to_string()
                        }else{
                            config_params.as_on_date().format("%d-%m-%Y").to_string()
                        },
                        if master_fields[18] != ""{
                            master_fields[18].to_string()
                        }else{
                            config_params.as_on_date().format("%d-%m-%Y").to_string()
                        },
                        config_params.as_on_date().format("%d-%m-%Y"),
                        master_fields[6],
                    );
                }
            }
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
