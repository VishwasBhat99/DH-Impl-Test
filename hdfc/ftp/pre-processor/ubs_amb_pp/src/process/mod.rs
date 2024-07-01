use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::io::prelude::*;

pub fn process_data(config_params: &ConfigurationParameters, log: &Logger) {
    let mut input_data: HashMap<String, Vec<f64>> = HashMap::new();
    let mut npa_data: HashMap<String, f64> = HashMap::new();
    let mut lst_ex_gl: Vec<String> = Vec::new();

    let mut tot_rec: i64 = 0;
    let mut succ_rec: i64 = 0;
    //npa file reading started
    let npa_file = match new_buf_rdr(config_params.npa_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` : {}.",
            config_params.npa_file_path(),
            error
        ),
    };

    for (line_num, lines) in npa_file.lines().enumerate().skip(1) {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.npa_file_path(),
                line_num,
                error
            ),
        };
        let fields: Vec<&str> = line.split(',').collect();
        //checking the if current line has expected no. of fields
        if fields.len() >= 2 {
            npa_data.insert(
                fields[0].to_string().trim().to_string(),
                fields[1].parse::<f64>().unwrap_or(0.0),
            );
        } else {
            log_debug!(
                log,
                "skipped for account : {} at line number :{} in file :{}",
                fields[0],
                line_num,
                config_params.npa_file_path()
            );
        }
    }
    //npa file reading completed
    //Exclude GL file reading started
    let gl_file = match new_buf_rdr(config_params.ex_gl_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` : {}.",
            config_params.ex_gl_file_path(),
            error
        ),
    };

    for (line_num, lines) in gl_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.ex_gl_file_path(),
                line_num,
                error
            ),
        };
        lst_ex_gl.push(line.trim().to_string());
    }
    //Exclude GL file reading completed
    let mut output_file = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file `{}`: {}",
            config_params.output_file_path(),
            error
        ),
    };
    //Base input file reading started
    let input_file = match new_buf_rdr(config_params.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}`: {}.",
            config_params.input_file_path(),
            error
        ),
    };
    // Add the amount fields if account no is repeating and store in a hashmap.
    for (line_num, lines) in input_file.lines().enumerate().skip(1) {
        tot_rec += 1;
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_file_path(),
                line_num,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        //checking the if current line has expected no. of fields
        if fields.len() >= 6 {
            succ_rec += 1;
            let gl_acc_no: String = fields[9].to_string().trim().to_string();
            let ref_no: String = fields[0].to_string().trim().to_string();
            let mut accured_interest = fields[5].parse::<f64>().unwrap_or(0.0);
            if !lst_ex_gl.contains(&gl_acc_no) {
                let npa_int_amount = match npa_data.get(&ref_no) {
                    Some(x) => x,
                    None => &0.0,
                };
                accured_interest -= npa_int_amount;
            } else {
                accured_interest = 0.0;
            }
            input_data
                .entry(fields[0].to_string().trim().to_string())
                .and_modify(|new_vec| {
                    new_vec[0] += fields[2].parse::<f64>().unwrap_or(0.0);
                    new_vec[1] += accured_interest;
                })
                .or_insert(vec![
                    fields[2].parse::<f64>().unwrap_or(0.0),
                    accured_interest,
                ]);
        } else {
            log_debug!(
                log,
                "skipped for account : {} at line number :{} in file {}",
                fields[0],
                line_num,
                config_params.input_file_path()
            );
        }
    }
    for (key, value) in input_data {
        writeln!(
            output_file,
            "{}",
            format!("{}|{}|{}", key, value[0], value[1])
        )
        .expect("Output Line can not be written");
    }

    let health_report = HealthReport::new(tot_rec, succ_rec, tot_rec - succ_rec, 0.0, 0.0, 0);
    health_report.gen_health_rpt(&config_params.output_file_path());
}
