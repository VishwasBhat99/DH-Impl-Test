use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use health_report::HealthReport;
use rbdate::DateParser;
use slog::Logger;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::time::SystemTime;

pub fn process(config_params: &ConfigurationParameters, _log: &Logger, diag_log: &Logger) {
    let start_process_timer = SystemTime::now();

    let mut tot_acc_encntrd = 0;
    let mut skip_rec_count = 0;

    // Read input file
    let input_file = match File::open(config_params.biu_ucic_file()) {
        Ok(io_config_file) => io_config_file,
        Err(_err) => {
            panic!(
                "Could not open BIU UCIC Master file {}",
                config_params.biu_ucic_file()
            );
        }
    };
    let reader = BufReader::new(input_file);
    let mut ucic_data_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut ucic_freq_map: HashMap<String, u32> = HashMap::new();
    let mut ucic_t3_set: HashSet<String> = HashSet::new();
    let date_parser = DateParser::new("%d-%m-%Y %H:%M:%S".to_string(), false);
    // Read each line from the input file and create UcicData records
    for (line_no, line) in reader.lines().enumerate().skip(1) {
        tot_acc_encntrd += 1;
        let data = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                skip_rec_count += 1;
                log_error!(
                    _log,
                    "Cannot read line {} from Input file: {:?}",
                    line_no + 1,
                    error
                );
                continue;
            }
        };
        let fields: Vec<&str> = data
            .split(config_params.biu_ucic_field_delimiter())
            .collect();

        let key = fields[0].to_string();
        let curr_vec: Vec<String> = vec![
            fields[1].to_string(),
            fields[2].to_string(),
            fields[3].to_string(),
            fields[4].to_string(),
            fields[5].to_string(),
            fields[6].to_string(),
            fields[7].to_string(),
            fields[8].to_string(),
            fields[9].to_string(),
        ];

        ucic_freq_map
            .entry(key.clone())
            .and_modify(|prev_cnt| {
                *prev_cnt += 1;
            })
            .or_insert(1);

        //if any of the t1,t2,t3 is "Y", we'll store "Y":
        if fields[1].to_string() == "Y"
            || fields[2].to_string() == "Y"
            || fields[3].to_string() == "Y"
        {
            ucic_t3_set.insert(key.clone());
        }

        // Update the HashMap with latest UcicData record for each ucic_id
        ucic_data_map
            .entry(key)
            .and_modify(|prev_vec| {
                let prev_bal = prev_vec[8].to_string().parse::<f64>().unwrap_or(0.0);
                let curr_bal = curr_vec[8].to_string().parse::<f64>().unwrap_or(0.0);
                let prev_close_dt = date_parser.parse(&prev_vec[6]);
                let curr_close_dt = date_parser.parse(&curr_vec[6]);
                let prev_open_dt = date_parser.parse(&prev_vec[5]);
                let curr_open_dt = date_parser.parse(&curr_vec[5]);

                if prev_bal < curr_bal {
                    *prev_vec = curr_vec.to_owned();
                } else if prev_bal == curr_bal && prev_close_dt < curr_close_dt {
                    *prev_vec = curr_vec.to_owned();
                } else if prev_bal == curr_bal
                    && prev_close_dt == curr_close_dt
                    && prev_open_dt < curr_open_dt
                {
                    *prev_vec = curr_vec.to_owned();
                }
            })
            .or_insert(curr_vec);
    }
    // Write output to a new file
    let output_file_path = Path::new(config_params.output_file_path());
    let mut output_file = File::create(output_file_path).expect("Cannot create Output File Path");

    let out_error = format!(
        "Could not write output in file {}",
        config_params.output_file_path
    );
    let delimiter = config_params.biu_ucic_field_delimiter();
    let header_str = "UCIC|T1|T2|T3|T4|DIVISION|Customer_ID".to_string();
    writeln!(output_file, "{}", header_str).expect(&out_error);
    for (key, ucic_data) in ucic_data_map.iter() {
        let freq = match ucic_freq_map.get(&key.clone()) {
            Some(val) => *val,
            None => 0 as u32,
        };
        let mut curr_t3 = ucic_data[2].clone();
        if freq > 1 && ucic_t3_set.contains(key) {
            curr_t3 = "Y".to_string();
        }
        let line = format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}",
            key,
            delimiter,
            ucic_data[0],
            delimiter,
            ucic_data[1],
            delimiter,
            curr_t3,
            delimiter,
            ucic_data[3],
            delimiter,
            ucic_data[4],
            delimiter,
            ucic_data[7]
        );
        writeln!(output_file, "{}", line).expect(&out_error);
    }

    // generate health check
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skip_rec_count,
        skip_rec_count,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path);
    let end_process_timer = SystemTime::now();
    let duration = end_process_timer
        .duration_since(start_process_timer)
        .expect("Could not calculate total duration for the process.");
    debug!(
        diag_log,
        "Total Duration for Reading and Writing BIU UCIC Sorting: {:?}.", duration
    );
}
