use self::exrt::*;
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;
mod exrt;
mod writers;

pub fn process(config_param: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    // Initialize a pool of writers.
    let mut writers_pool: HashMap<String, BufWriter<File>> = HashMap::new();
    //Src map
    let source_map_reader = fs::read_to_string(&config_param.srcmap_file_path())
        .expect("Failed to read source map file!");
    for line in source_map_reader.lines() {
        let new_writer = writers::get_new_writer(line.to_string(), config_param.output_file_path());
        writers_pool.insert(line.to_string(), new_writer);
    }

    let start_gac_read_timer = SystemTime::now();
    let mut gac_map: HashMap<String, String> = HashMap::new();
    let gac_file = match new_buf_rdr(config_param.gac_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.gac_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in gac_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.gac_file_path(),
                line_num + 1,
                error
            ),
        };
        match line.split_once('|') {
            Some((key, value)) => gac_map.insert(key.to_string(), value.to_string()),
            None => None,
        };
    }
    let end_gac_read_timer = SystemTime::now();
    let duration = end_gac_read_timer
        .duration_since(start_gac_read_timer)
        .expect("Could not calculate total duration for gac read timer.");
    debug!(log, "Readings GAC File, Total Duration: {:?}.", duration);

    let start_cmg_read_timer = SystemTime::now();
    let mut cmg_map: HashMap<String, String> = HashMap::new();
    let cmg_file = match new_buf_rdr(config_param.cmg_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.cmg_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in cmg_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line.replace("\"", ""),
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.cmg_file_path(),
                line_num + 1,
                error
            ),
        };
        match line.split_once('|') {
            Some((key, value)) => cmg_map.insert(key.to_string(), value.to_string()),
            None => None,
        };
    }
    let end_cmg_read_timer = SystemTime::now();
    let duration = end_cmg_read_timer
        .duration_since(start_cmg_read_timer)
        .expect("Could not calculate total duration for read timer.");
    debug!(log, "Reading CMG File, Total Duration: {:?}.", duration);

    let start_ucif_read_timer = SystemTime::now();
    let mut ucif_map: HashMap<String, String> = HashMap::new();
    let ucif_file = match new_buf_rdr(config_param.ucif_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.ucif_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in ucif_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line.replace("\"", ""),
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.ucif_file_path(),
                line_num + 1,
                error
            ),
        };
        match line.split_once('|') {
            Some((key, value)) => ucif_map.insert(key.to_string(), value.to_string()),
            None => None,
        };
    }
    let end_ucif_read_timer = SystemTime::now();
    let duration = end_ucif_read_timer
        .duration_since(start_ucif_read_timer)
        .expect("Could not calculate total duration for read timer.");
    debug!(log, "Reading UCIF File, Total Duration: {:?}.", duration);

    let start_rct_read_timer = SystemTime::now();
    let mut rct_map: HashMap<String, String> = HashMap::new();
    let rct_file = match new_buf_rdr(config_param.rct_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.rct_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in rct_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.rct_file_path(),
                line_num + 1,
                error
            ),
        };
        match line.split_once('|') {
            Some((key, value)) => rct_map.insert(key.trim().to_string(), value.to_string()),
            None => None,
        };
    }
    let end_rct_read_timer = SystemTime::now();
    let duration = end_rct_read_timer
        .duration_since(start_rct_read_timer)
        .expect("Could not calculate total duration for read timer.");
    debug!(log, "Reading RCT File, Total Duration: {:?}.", duration);

    let start_ex_rt_read_timer = SystemTime::now();
    let mut ex_rt_map: HashMap<ExrtKey, String> = HashMap::new();
    let ex_rt_file = match new_buf_rdr(config_param.ex_rt_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.ex_rt_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in ex_rt_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.ex_rt_file_path(),
                line_num + 1,
                error
            ),
        };
        let derived_fields: Vec<&str> = line.split("|").collect();
        let ex_rt_key = ExrtKey::new(derived_fields[0].to_string(), derived_fields[1].to_string());
        ex_rt_map.insert(ex_rt_key, derived_fields[2].to_string());
    }
    let end_ex_rt_read_timer = SystemTime::now();
    let duration = end_ex_rt_read_timer
        .duration_since(start_ex_rt_read_timer)
        .expect("Could not calculate total duration for read timer.");
    log_debug!(
        log,
        "Reading EXCHANGE RATE File, Total Duration: {:?}.",
        duration
    );

    let start_gam_read_timer = SystemTime::now();
    let gam_file = match new_buf_rdr(config_param.gam_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.gac_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let default_gac_value = "||||||".to_string();
    let default_cmg_value = "|||||".to_string();
    let default_ucif_value = "||".to_string();
    let default_rct_value = "".to_string();
    let mut cust_grp_id = String::new();
    let mut cust_const_id = String::new();
    let mut ex_rt = String::new();
    let mut seg_cd = String::new();
    let mut nfs = String::new();
    for (line_num, lines) in gam_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.gam_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        //ACID is lookup key for gac_map.
        let gac_line = match gac_map.get(fields[0]) {
            Some(val) => val.to_string(),
            None => {
                log_debug!(log,"The lookup key:{} in could not be found in GAC file. Empty fields passed for account with id: {}",fields[0],fields[0]);
                default_gac_value.to_string()
            }
        };
        //Remove fields read from hashmap.
        gac_map.remove(fields[0]);
        let cmg_line = match cmg_map.get(fields[6]) {
            Some(val) => val.to_string(),
            None => {
                log_debug!(log,"The lookup key:{} in could not be found in CMG file.Empty fields passed for account with id: {} is skipped.",fields[6],fields[0]);
                default_cmg_value.to_string()
            }
        };
        //Calculated field
        let out_bal_amt =
            fields[3].parse::<f64>().unwrap_or(0.0) + fields[4].parse::<f64>().unwrap_or(0.0);
        if ucif_map.contains_key(&fields[6].to_string()) {
            let ucif_line = ucif_map
                .get(&fields[6].to_string())
                .unwrap_or(&default_ucif_value);
            let ucif_fields: Vec<&str> = ucif_line.split('|').collect();
            cust_grp_id = ucif_fields[0].to_string();
            cust_const_id = ucif_fields[0].to_string();
        } else {
            cust_const_id = "".to_string();
            cust_grp_id = fields[6].to_string();
        }
        if fields[17] == "INR" {
            ex_rt = "1".to_string();
        } else {
            let default_exrt_val = "1".to_string();
            let get_exrt_key = ExrtKey::new(fields[17].to_string(), "INR".to_string());
            ex_rt = ex_rt_map
                .get(&get_exrt_key)
                .unwrap_or(&default_exrt_val)
                .to_string();
        }

        let out_bal_amt_con = out_bal_amt * ex_rt.parse::<f64>().unwrap_or(0.0);

        let gac_vec: Vec<&str> = gac_line.split("|").collect();
        let cd_1 = gac_vec[0].to_string();
        seg_cd = if rct_map.contains_key(&cd_1) {
            cd_1
        } else {
            log_debug!(log, "The lookup key:{} in could not be found in RCT file. Empty fields passed for account with id: {}", cd_1, fields[0]);
            "".to_string()
        };
        let nfs_key = &fields[5][0..3];
        if nfs_key == "NFS".to_string() {
            nfs = "NFS".to_string();
        } else {
            nfs = "".to_string();
        }
        let final_line = format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            line,
            gac_line,
            cmg_line,
            out_bal_amt,
            cust_grp_id,
            cust_const_id,
            ex_rt,
            out_bal_amt_con,
            seg_cd,
            nfs
        );
        let writer = match writers_pool.get_mut(&fields[15].to_string()) {
            Some(writer) => writer,
            None => {
                //If the source id could not be found the output is written to a default file: "NA.txt".
                let new_writer =
                    writers::get_new_writer("GAM_NA".to_string(), config_param.output_file_path());
                writers_pool.insert(fields[15].to_string(), new_writer);
                // Cannot return new writer as ownership of that writer is assigned to writers_pool in prev step
                writers_pool
                    .get_mut(&fields[15].to_string())
                    .expect("Cannot get field value to writer.")
            }
        };
        writers::write_data(writer, final_line, log);
    }
    let end_gam_read_timer = SystemTime::now();
    let duration = end_gam_read_timer
        .duration_since(start_gam_read_timer)
        .expect("Could not calculate total duration for read timer.");
    debug!(
        log,
        "Reading and processing GAM File, Total Duration: {:?}.", duration
    );
}
