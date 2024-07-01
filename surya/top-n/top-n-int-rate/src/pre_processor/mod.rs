extern crate serde;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::DateParser;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::default;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;
pub struct CustDataValue {
    pub amt: f64,
    pub int_rate: f64,
}
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct CustDataKey {
    pub cust_id: String,
    pub prod_code: String,
}
pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };

    let mut tot_input_acc_encntrd: i64 = 0;
    let mut tot_acc_skippd: i64 = 0;
    let mut tot_succ_rec: i64 = 0;
    let mut writer = BufWriter::new(output_file);
    let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
    let top_n_cust_acc = match new_buf_rdr(config_param.top_n_cust_acc()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found top n cust acc file: `{}` on location `{}` : {}.",
            config_param.top_n_cust_acc(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut top_n_cust_acc_map: HashMap<CustDataKey, CustDataValue> = HashMap::new();
    for (line_num, lines) in top_n_cust_acc.lines().enumerate() {
        let top_n_cust_acc_line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.top_n_cust_acc(),
                line_num + 1,
                error
            ),
        };

        let top_n_cust_acc_fields = top_n_cust_acc_line.split('|').collect::<Vec<&str>>();
        if top_n_cust_acc_fields.len() == 13 {
            //Store the LoanAccNo and NPA_Classification as key-value pairs.

            let key = CustDataKey {
                cust_id: top_n_cust_acc_fields[2].to_string(),
                prod_code: top_n_cust_acc_fields[7].to_string(),
            };

            let cust_data = top_n_cust_acc_map.entry(key).or_insert(CustDataValue {
                amt: top_n_cust_acc_fields[9].parse::<f64>().unwrap_or(0.0),
                int_rate: (top_n_cust_acc_fields[9].parse::<f64>().unwrap_or(0.0)
                    * top_n_cust_acc_fields[10].parse::<f64>().unwrap_or(0.0)),
            });

            cust_data.amt += top_n_cust_acc_fields[9].parse::<f64>().unwrap_or(0.0);
            cust_data.int_rate += (top_n_cust_acc_fields[9].parse::<f64>().unwrap_or(0.0)
                * top_n_cust_acc_fields[10].parse::<f64>().unwrap_or(0.0));
        } else {
            log_error!(
                log,
                "Found line :{} at line number {} defective in NPA data file.",
                line_num,
                top_n_cust_acc_line
            );
        }
    }

    let top_n_cust_prod_file = match new_buf_rdr(config_param.top_n_cust_prod()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found npa live file: `{}` on location `{}` : {}.",
            config_param.top_n_cust_prod(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in top_n_cust_prod_file.lines().enumerate() {
        let top_n_cust_prod_line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.top_n_cust_prod(),
                line_num + 1,
                error
            ),
        };

        let top_n_cust_prod_fields = top_n_cust_prod_line.split('|').collect::<Vec<&str>>();
        let default_cust_data = CustDataValue {
            amt: 1.0,
            int_rate: 0.0,
        };
        let key_sa = CustDataKey {
            cust_id: top_n_cust_prod_fields[2].to_string(),
            prod_code: "SA".to_string(),
        };
        let cust_data_sa = top_n_cust_acc_map
            .get(&key_sa)
            .unwrap_or(&default_cust_data);

        let sa_int_rate = cust_data_sa.int_rate / cust_data_sa.amt;

        let key_sa = CustDataKey {
            cust_id: top_n_cust_prod_fields[2].to_string(),
            prod_code: "CA".to_string(),
        };
        let cust_data_ca = top_n_cust_acc_map
            .get(&key_sa)
            .unwrap_or(&default_cust_data);

        let ca_int_rate = cust_data_ca.int_rate / cust_data_ca.amt;

        let key_sa = CustDataKey {
            cust_id: top_n_cust_prod_fields[2].to_string(),
            prod_code: "TDWD".to_string(),
        };
        let cust_data_tdwd = top_n_cust_acc_map
            .get(&key_sa)
            .unwrap_or(&default_cust_data);

        let tdwd_int_rate = cust_data_tdwd.int_rate / cust_data_tdwd.amt;

        let key_sa = CustDataKey {
            cust_id: top_n_cust_prod_fields[2].to_string(),
            prod_code: "TDNWD".to_string(),
        };
        let cust_data_tdnwd = top_n_cust_acc_map
            .get(&key_sa)
            .unwrap_or(&default_cust_data);

        let tdnwd_int_rate = cust_data_tdnwd.int_rate / cust_data_tdnwd.amt;

        let op_line = format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            top_n_cust_prod_fields[0],
            top_n_cust_prod_fields[1],
            top_n_cust_prod_fields[2],
            top_n_cust_prod_fields[3],
            top_n_cust_prod_fields[4],
            top_n_cust_prod_fields[5],
            top_n_cust_prod_fields[6],
            top_n_cust_prod_fields[7],
            top_n_cust_prod_fields[8],
            top_n_cust_prod_fields[9],
            top_n_cust_prod_fields[10],
            top_n_cust_prod_fields[11],
            sa_int_rate,
            ca_int_rate,
            tdwd_int_rate,
            tdnwd_int_rate
        );

        tot_succ_rec += 1;
        match writer.write_all(op_line.as_bytes()) {
            Ok(val) => val,
            Err(error) => {
                panic!("Error writing processed data: {:?}", error);
            }
        }
    }

    let health_report = HealthReport::new(
        tot_input_acc_encntrd,
        tot_succ_rec,
        tot_acc_skippd,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(config_param.output_file_path());
}
