use self::io::*;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::date_from_timestamp;
use rbdate::DateParser;
use slog::Logger;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::time::SystemTime;

mod io;

pub fn aggregate(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut tot_amt = 0.0;
    let mut tot_acc = 0;
    let gl_op_path = format!("{}gl_report.txt", config_params.output_file_path());
    let gn_op_path = format!("{}gn_report.txt", config_params.output_file_path());
    let ebn_op_path = format!("{}ebn_report.txt", config_params.output_file_path());
    let mut gl_op_writer = get_writer(&gl_op_path);
    let mut gn_op_writer = get_writer(&gn_op_path);
    let mut ebn_op_writer = get_writer(&ebn_op_path);
    let od_input = File::open(&config_params.input_file_path()).expect("Could Not Read File");
    let od_input_reader = BufReader::new(od_input);
    let mut gl_agg_map: HashMap<Vec<String>, f64> = HashMap::new();
    let mut gn_agg_map: HashMap<Vec<String>, f64> = HashMap::new();
    let mut ebn_agg_map: HashMap<Vec<String>, f64> = HashMap::new();
    for (_index, line) in od_input_reader.lines().enumerate() {
        tot_acc += 1;
        let line = line.expect("Could Not Read Line in GAM file.").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        if !input_fields[5].to_owned().to_string().contains("NFS") && input_fields[71].to_owned().to_uppercase() != "S" && input_fields[71].to_owned().to_uppercase() != "D" && input_fields[71].to_owned().to_uppercase() != "L" {
            let bacid = input_fields[2].to_owned();
            let ccy_cd = input_fields[17].to_owned();
            let seg_cd = input_fields[55].to_owned();
            let gl_sub_cd = input_fields[13].to_owned();
            let npa_class = input_fields[71].to_owned();
            let foracid_prefix = input_fields[75].to_owned();
            let foracid_suffix = input_fields[76].to_owned();
            let gl_gn_key = vec![
                ccy_cd.to_owned(),
                seg_cd.to_owned(),
                gl_sub_cd.to_owned(),
                npa_class.to_owned(),
            ];
            let ebn_key = vec![bacid, ccy_cd, seg_cd, gl_sub_cd,npa_class,foracid_prefix,foracid_suffix];
            let npa_amt = input_fields[77].parse::<f64>().unwrap_or(0.0);

            tot_amt += npa_amt;
            gn_agg_map
                .entry(gl_gn_key.clone())
                .and_modify(|amt| *amt += npa_amt)
                .or_insert(npa_amt);
            gl_agg_map
                .entry(gl_gn_key.clone())
                .and_modify(|amt| *amt += npa_amt)
                .or_insert(npa_amt);
            ebn_agg_map
                .entry(ebn_key)
                .and_modify(|amt| *amt += npa_amt)
                .or_insert(npa_amt);
        } else {
            log_debug!(
                logger,
                "This account with account number:{} contains solID: {} NFS",
                input_fields[0],
                input_fields[5]
            );
        }
    }
    for (key, val) in gn_agg_map {
        let vec = key;
        let gn_amt = val.abs();
        writeln!(gn_op_writer, "|{}|1|{}||||||||{}|{}", vec[0], gn_amt,vec[2],vec[3])
            .expect("Could not write to output file.");
    }
    for (key, val) in gl_agg_map {
        let vec = key;
        let gl_amt = -1.0 * val;
        writeln!(gl_op_writer, "|{}|1|{}||||||||{}|{}", vec[0], gl_amt,vec[2],vec[3])
            .expect("Could not write to output file.");
    }
    for (key, val) in ebn_agg_map {
        let vec = key;
        let ebn_amt = -1.0 * val.abs();
        writeln!(
            ebn_op_writer,
            "{}|{}|1|{}|{}|{}|||||||{}|{}",
            vec[0], vec[1], ebn_amt, vec[3], vec[4],vec[5],vec[6]
        )
        .expect("Could not write to output file.");
    }
    let health_report = HealthReport::new(0, 0, 0, tot_amt, tot_amt, 0);
    health_report.gen_health_rpt(config_params.output_file_path());
}
