use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use crate::process::input_struct::{get_str, InputData};
use health_report::HealthReport;
use rbdate::DateParser;
use slog::Logger;
use hashbrown::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::time::SystemTime;
mod input_struct;

pub fn process(config_params: &ConfigurationParameters, _log: &Logger, diag_log: &Logger) {
    let start_process_timer = SystemTime::now();

    let mut tot_acc_encntrd = 0;
    let mut skip_rec_count = 0;
    let cust_master_reader: String = fs::read_to_string(config_params.cust_master_file())
        .expect("Could Not Read Customer master file");
    let mut customer_map: HashMap<String, f64> = HashMap::new();
    for (line_no, line) in cust_master_reader.lines().enumerate().skip(0) {
        let cust_master_vec: Vec<&str> = line
            .split(config_params.cust_field_delimiter())
            .collect::<Vec<&str>>();
        let cust_id = get_str(
            config_params.cust_master_file(),
            &cust_master_vec,
            0,
            line_no,
        );
        let cust_bal = get_str(
            config_params.cust_master_file(),
            &cust_master_vec,
            1,
            line_no,
        )
        .parse::<f64>()
        .unwrap_or(0.0);
        customer_map.insert(cust_id, cust_bal);
    }

    // Read input file
    let input_file = match File::open(config_params.ucic_master_file()) {
        Ok(io_config_file) => io_config_file,
        Err(_err) => {
            panic!(
                "Could not open UCIC master file {}",
                config_params.ucic_master_file()
            );
        }
    };
    let master_row = fs::read_to_string(config_params.ucic_master_file())
        .expect("Unable to read Ucic Master File");
    let last_master_row_num = master_row.lines().count() - 1;
    drop(master_row);
    let reader = BufReader::new(input_file);
    let mut ucic_data_map: HashMap<String, InputData> = HashMap::new();
    let mut footer = String::new();
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
        if line_no == last_master_row_num {
            footer = data.to_string();
            break;
        }
        let fields: Vec<&str> = data.split(config_params.ucic_field_delimiter()).collect();
        let input_data: InputData = InputData::new(
            config_params,
            &config_params.ucic_master_file,
            &fields,
            line_no,
        );
        let blank_flg = "BLANK".to_string().to_ascii_lowercase();
        let zero_flag = "0".to_string().to_ascii_lowercase();
        let null_flg = "NULL".to_string().to_ascii_lowercase();
        let key = if !input_data.ucic.is_empty()
            && input_data.ucic.to_ascii_lowercase() != blank_flg
            && input_data.ucic.to_ascii_lowercase() != zero_flag
            && input_data.ucic.to_ascii_lowercase() != null_flg
        {
            input_data.ucic.to_string()
        } else {
            input_data.cod_cust_id.to_string()
        };
        ucic_data_map
            .entry(key)
            .and_modify(|prev_data| {
                let prev_cust_id = &prev_data.cod_cust_id;
                let prev_cust_bal = customer_map.get(&prev_cust_id.to_string()).unwrap_or(&0.0);
                let curr_cust_id = &input_data.cod_cust_id;
                let curr_cust_bal = customer_map.get(&curr_cust_id.to_string()).unwrap_or(&0.0);
                if prev_cust_bal == curr_cust_bal {
                    let prev_dt = prev_data.dat_last_mnt;
                    let mut curr_dt = input_data.dat_last_mnt;
                    let mut flag = false;
                    if prev_dt < curr_dt {
                        *prev_data = input_data.clone()
                    }
                } else if prev_cust_bal < curr_cust_bal {
                    *prev_data = input_data.clone()
                }
            })
            .or_insert(input_data);
    }

    // Write output to a new file
    let output_file_path = Path::new(config_params.output_file_path());
    let mut output_file = File::create(output_file_path).expect("Cannot create output file path");

    let out_error = format!(
        "Could not write output in file {}",
        config_params.output_file_path
    );
    let delimiter = config_params.ucic_field_delimiter();

    let header_str = "UCIC~#~NATUREOFBUS~#~TXTBUSDESC~#~FLGCUSTTYPE~#~TXTCUSTTYP~#~PAN~#~CODCUSTID~#~NAMCUST~#~TXTCUSTTYP~#DATCUSTOPEN~#MOD_DATE".to_string();
    writeln!(output_file, "{}", header_str).expect(&out_error);
    for (key, ucic_data) in ucic_data_map.iter() {
        let ouput_vec: Vec<&str> = ucic_data.required_data.split("~#~").collect();
        let line = format!(
            "{}~#~{}~#~{}~#~{}~#~{}~#~{}~#~{}~#~{}~#~{}~#~{}~#~{}",
            key,
            ouput_vec[0],
            ouput_vec[1],
            ouput_vec[2],
            ouput_vec[3],
            ouput_vec[4],
            ucic_data.cod_cust_id,
            ouput_vec[5],
            ouput_vec[6],
            ucic_data.dat_cust_open.format("%d-%m-%Y"),
            ucic_data.dat_last_mnt.format("%d-%m-%Y")
        );
        writeln!(output_file, "{}", line).expect(&out_error);
    }
    writeln!(output_file, "{}", footer.to_string()).expect(&out_error);
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
        "Total Duration for Reading and Writing UcicDatas: {:?}.", duration
    );
}
