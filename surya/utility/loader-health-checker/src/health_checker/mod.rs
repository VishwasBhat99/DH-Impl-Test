use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use sdb_io::new_buf_rdr;
use std::io::BufRead;
use std::time::SystemTime;

pub fn process(config_param: ConfigurationParameters) {
    let start_write_timer = SystemTime::now();
    let mut tot_acc = 0;
    let mut tot_succ = 0;
    let mut tot_fail = 0;
    let tot_amt_inp = 0.0;
    let tot_amt_op = 0.0;
    let tot_cfs = 0;
    let loader_log_file = match new_buf_rdr(config_param.loader_log_file_path()) {
        Ok(file) => file,
        Err(_error) => panic!(
            "Could not found loader_log_file: `{}`",
            &config_param.loader_log_file_path(),
        ),
    };
    if config_param.loader_flag().trim().to_uppercase() == "ORACLE" {
        //Extraction flag to decide the log file is a extracted file
        let mut extracion_flag = true;
        let mut total_num_lines = 0;
        for (line_num, lines) in loader_log_file.lines().enumerate().skip(0) {
            let loader_log_line = match lines {
                Ok(loader_log_line) => loader_log_line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    &config_param.loader_log_file_path(),
                    line_num + 1,
                    error
                ),
            };
            // All the follwing conditions for Oracle loader
            total_num_lines += 1;
            if loader_log_line.contains("Rows successfully loaded.")
                || loader_log_line.contains("Row successfully loaded.")
            {
                let loader_fields: Vec<String> =
                    loader_log_line.split(' ').map(|s| s.to_string()).collect();
                tot_succ = loader_fields[2].parse().unwrap_or(0);
                extracion_flag = false;
                continue;
            }
            if loader_log_line.contains("Total logical records read:") {
                let loader_fields: Vec<String> = loader_log_line
                    .split(':')
                    .map(|s| s.trim().to_string())
                    .collect();
                tot_acc = loader_fields[1].parse().unwrap_or(0);
                continue;
            }
            if loader_log_line.contains("Total logical records rejected:") {
                let loader_fields: Vec<String> = loader_log_line
                    .split(':')
                    .map(|s| s.trim().to_string())
                    .collect();
                tot_fail += loader_fields[1].parse().unwrap_or(0);
                continue;
            }
            if loader_log_line.contains("Total logical records discarded:") {
                let loader_fields: Vec<String> = loader_log_line
                    .split(':')
                    .map(|s| s.trim().to_string())
                    .collect();
                tot_fail += loader_fields[1].parse().unwrap_or(0);
                continue;
            }
        }
        if extracion_flag {
            tot_acc = total_num_lines;
            tot_succ = total_num_lines;
        }
    } else if config_param.loader_flag().trim().to_uppercase() == "MSSQL" {
        for (line_num, lines) in loader_log_file.lines().enumerate().skip(0) {
            let loader_log_line = match lines {
                Ok(loader_log_line) => loader_log_line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    &config_param.loader_log_file_path(),
                    line_num + 1,
                    error
                ),
            };
            //For MS-SQL Loader Report
            if loader_log_line.contains("rows affected)") {
                let loader_fields: Vec<String> =
                    loader_log_line.split(' ').map(|s| s.to_string()).collect();
                //remove starting small bracket '(' to get the success record
                tot_succ = loader_fields[0][1..].parse().unwrap_or(0);
                tot_acc = loader_fields[0][1..].parse().unwrap_or(0);
            }
            //For MS-SQL Extrator Report
            else if loader_log_line.contains("rows copied.") {
                let loader_fields: Vec<String> =
                    loader_log_line.split(' ').map(|s| s.to_string()).collect();
                tot_succ = loader_fields[0].parse().unwrap_or(0);
                tot_acc = loader_fields[0].parse().unwrap_or(0);
            }
            //In case any Error occured in Loader or Extractor
            else if loader_log_line.contains("The statement has been terminated.") {
                panic!(
                    "Error in loader,Please check {} for detailed Error ",
                    config_param.loader_log_file_path()
                );
            }
        }
    } else {
        panic!("Loader Flag doesn't match, Possible Values are ORACLE or MSSQL")
    }

    let health_stat = HealthReport::new(
        tot_acc,
        tot_succ,
        tot_fail,
        tot_amt_inp,
        tot_amt_op,
        tot_cfs,
    );
    health_stat.gen_health_rpt(config_param.output_file_path());
    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing health check report file.");
    println!("Total duration for health check report: `{:?}`", duration);
}
