use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;
use std::io::Write;

pub fn process(config_params: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let mut tot_acc_encntrd = 0;
    let mut tot_amt_in_ip = 0.0;
    let mut tot_amt_in_op = 0.0;
    let mut skip_rec_count = 0;

    let mut writer = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!(
            "Unable to create output file: `{}` due to : `{}`",
            config_params.output_file_path(),
            error,
        ),
    };

    //Reading PSL Category File
    let psl =
        File::open(&config_params.psl_category_file()).expect("Could Not Read PSL Category File");
    let psl_reader = BufReader::new(psl);
    let mut psl_map: HashMap<String, f64> = HashMap::new();
    for (line_no, line) in psl_reader.lines().enumerate() {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                log_error!(
                    log,
                    "Cannot read line {} from PSL Category file: {:?}",
                    line_no,
                    error
                );
                continue;
            }
        };
        let psl_fields: Vec<&str> = acc_info.split('|').collect();
        if psl_fields.len() != 2 {
            log_error!(
                log,
                "Cannot read line {} from PSL Category file due to incorrect column count {:?}",
                line_no,
                psl_fields.len()
            );
            continue;
        }
        let mut key_1 = psl_fields[0].to_string();
        key_1.pop();
        let psl_bal = psl_fields[1].to_string().parse::<f64>().unwrap_or(0.0);
        psl_map.insert(key_1, psl_bal);
    }

    //Reading Daily Balance File
    let daily_bal =
        File::open(&config_params.daily_bal_file()).expect("Could Not Read Daily Balance File");
    let daily_bal_reader = BufReader::new(daily_bal);
    let mut out_map: HashMap<String, f64> = HashMap::new();
    for (line_no, line) in daily_bal_reader.lines().enumerate() {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                skip_rec_count += 1;
                log_error!(
                    log,
                    "Cannot read line {} from Daily Balance file: {:?}",
                    line_no,
                    error
                );
                continue;
            }
        };
        tot_acc_encntrd += 1;

        //key_1|out_bal|int_rate|int_inc_exp|.....|prod_code|daily_update_count
        let daily_bal_fields: Vec<&str> = acc_info.split('|').collect();
        let key_1 = daily_bal_fields[0].to_string();

        //daily_update_count|prod_code|int_inc_exp|int_rate|out_bal|.....|key_1
        let daily_bal_fields_rev: Vec<&str> = daily_bal_fields.iter().copied().rev().collect();

        //Out-Bal found in 4,7,10,13... in daily_bal_fields_rev
        let mut iter = 4;
        let days = daily_bal_fields.len().to_owned() / 3 - 1;
        while iter < daily_bal_fields_rev.len() {
            //Get minimum out of PSL-Bal and Out-Bal for each day
            let out_bal = if daily_bal_fields_rev[iter]
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0)
                < *psl_map.get(&key_1.to_owned()).unwrap_or(&0.0)
            {
                daily_bal_fields_rev[iter]
                    .to_string()
                    .parse::<f64>()
                    .unwrap_or(0.0)
            } else {
                *psl_map.get(&key_1).unwrap_or(&0.0)
            };
            tot_amt_in_ip += out_bal;
            out_map
                .entry(key_1.to_owned())
                .and_modify(|data| *data += out_bal)
                .or_insert(out_bal);
            iter += 3;
        }
        tot_amt_in_op += out_map.get(&key_1).unwrap_or(&0.0);
        writeln!(
            writer,
            "{}|{:.12}|{:.12}",
            key_1.to_owned(),
            out_map.get(&key_1).unwrap_or(&0.0) / days as f64,
            config_params.incentive_rate()
        )
        .expect("Output Line can not be written");
    }

    // Generate Health Check Report
    let health_report = health_report::HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skip_rec_count,
        skip_rec_count,
        tot_amt_in_ip,
        tot_amt_in_op,
        0,
    );
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file_path());
}
