use config_params::ConfigurationParameters;
use macros;
use processing::structs::DepositsDet;
use processing::structs::TopNDepDet;
use processing::structs::UcicDet;
use sdb_io;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::Write;

pub fn get_ucic_cust_map(
    config_params: &ConfigurationParameters,
    log: &Logger,
    diag_log: &Logger,
) -> HashMap<String, UcicDet> {
    info!(diag_log, "Start of reading ucic mapping file.");
    let ucic_rdr = match new_buf_rdr(config_params.ucic_map_file()) {
        Ok(rdr) => rdr,
        Err(err) => {
            log_error!(
                log,
                "Cannot read file at path: '{}', Error: '{}'",
                config_params.ucic_map_file,
                err
            );
            panic!(format!(
                "Cannot read file at path: '{}', Error: '{}'",
                config_params.ucic_map_file, err
            ));
        }
    };
    info!(diag_log, "End of reading ucic mapping file.");
    info!(
        diag_log,
        "Start of creating hashtable for custid and ucic details mapping."
    );

    let mut ucic_cust_map: HashMap<String, UcicDet> = HashMap::new();
    for (idx, line) in ucic_rdr.lines().enumerate().skip(1) {
        let ucic_det_line = line.expect(&format!("Cannot read line at {}", idx));
        let fields: Vec<&str> = ucic_det_line.split("~#~").collect();
        let ucic_data = UcicDet {
            ucic_id: fields[3].to_string(),
            ucic_name: fields[2].to_string(),
        };
        if !ucic_cust_map.contains_key(fields[0]) {
            ucic_cust_map.insert(fields[0].to_string(), ucic_data);
        }
    }
    info!(
        diag_log,
        "End of creating hashtable for custid and ucic details mapping."
    );

    ucic_cust_map
}

pub fn get_dep_map(
    config_params: &ConfigurationParameters,
    ttl_accnts: &mut usize,
    ttl_suc_accnts: &mut usize,
    ttl_bal_ip: &mut f64,
    log: &Logger,
    diag_log: &Logger,
) -> HashMap<String, DepositsDet> {
    info!(diag_log, "Start of reading deposits input file.");
    let dep_file_rdr = match new_buf_rdr(config_params.dep_file()) {
        Ok(rdr) => rdr,
        Err(err) => {
            log_error!(
                log,
                "Cannot read file at path: '{}', Error: '{}'",
                config_params.dep_file(),
                err
            );
            panic!(format!(
                "Cannot read file at path: '{}', Error: '{}'",
                config_params.dep_file, err
            ));
        }
    };
    info!(diag_log, "End of reading deposits input file.");
    info!(
        diag_log,
        "Start of creating hashtable for custid and deposits details."
    );

    let mut deposits_det: HashMap<String, DepositsDet> = HashMap::new();
    for (idx, line) in dep_file_rdr.lines().enumerate() {
        let dep_det_line = line.expect(&format!("Cannot read line at {}", idx));
        let fields: Vec<&str> = dep_det_line.split('|').collect();

        *ttl_accnts += 1;

        if fields.len() == 7 {
            let ca_bal = fields[2].parse::<f64>().unwrap_or(0.0);
            let sa_bal = fields[3].parse::<f64>().unwrap_or(0.0);
            let tdwd_bal = fields[4].parse::<f64>().unwrap_or(0.0);
            let tdnwd_bal = fields[5].parse::<f64>().unwrap_or(0.0);
            let rd_bal = fields[6].parse::<f64>().unwrap_or(0.0);
            let mut dep_data = DepositsDet::new();
            dep_data.ca_bal = ca_bal;
            dep_data.sa_bal = sa_bal;
            dep_data.td_bal = tdwd_bal + tdnwd_bal + rd_bal;
            dep_data.tot_bal = ca_bal + sa_bal + tdwd_bal + tdnwd_bal + rd_bal;

            *ttl_suc_accnts += 1;
            *ttl_bal_ip += dep_data.tot_bal;

            deposits_det
                .entry(fields[1].to_string())
                .and_modify(|val| val.update_deposistsdet(dep_data))
                .or_insert(dep_data);
        } else {
            continue;
        }
    }
    info!(
        diag_log,
        "End of creating hashtable for custid and deposits details."
    );

    deposits_det
}

pub fn get_summarised_data(
    ucic_cust_map: HashMap<String, UcicDet>,
    deposits_det: HashMap<String, DepositsDet>,
    diag_log: &Logger,
) -> HashMap<String, TopNDepDet> {
    info!(
        diag_log,
        "Start of creating hashtable for final data with ucic id."
    );
    let mut top_n_data: HashMap<String, TopNDepDet> = HashMap::new();
    for (key, value) in ucic_cust_map {
        let ucic_det = value;
        if let Some(dep_det) = deposits_det.get(&key) {
            let mut new_data = TopNDepDet::new();
            new_data.ucic_id = ucic_det.ucic_id;
            new_data.ucic_name = ucic_det.ucic_name;
            new_data.ca_bal = dep_det.ca_bal;
            new_data.sa_bal = dep_det.sa_bal;
            new_data.td_bal = dep_det.td_bal;
            new_data.tot_bal = dep_det.tot_bal;
            top_n_data
                .entry(new_data.ucic_id.to_string())
                .and_modify(|val| val.update_topndepdet(&mut new_data))
                .or_insert(new_data);
        }
    }
    info!(
        diag_log,
        "End of creating hashtable for final data with ucic id."
    );

    top_n_data
}

pub fn get_sorted_data(
    summarised_data: HashMap<String, TopNDepDet>,
    diag_log: &Logger,
) -> Vec<TopNDepDet> {
    info!(diag_log, "Start of creating vector for sorting.");
    let mut top_n_vec: Vec<TopNDepDet> = Vec::new();
    for (_, value) in summarised_data {
        top_n_vec.push(value);
    }
    info!(diag_log, "End of creating vector for sorting.");
    info!(diag_log, "Start of ordering vector.");
    top_n_vec.sort_by(|item2, item1| order_float(item1.tot_bal, item2.tot_bal));
    info!(diag_log, "End of ordering vector.");

    top_n_vec
}

pub fn create_output(
    summarised_data: &Vec<TopNDepDet>,
    ttl_bal_op: &mut f64,
    config_params: &ConfigurationParameters,
    diag_log: &Logger,
) -> String {
    info!(diag_log, "Start of creating topnd output data.");
    let mut op_params: String = String::new();
    op_params.push_str(config_params.country_code());
    op_params.push_str("|");
    op_params.push_str(&config_params.ason_date().format("%d-%m-%Y").to_string());
    op_params.push_str("|");
    op_params.push_str(config_params.currency_id());
    op_params.push_str("|");
    let mut op_string: String = String::new();
    let mut rec_count = 0;
    loop {
        let s = summarised_data[rec_count].to_string() + "\n";
        *ttl_bal_op += summarised_data[rec_count].tot_bal;
        op_string.push_str(&op_params.to_string());
        op_string.push_str(&s.to_string());
        if (rec_count == summarised_data.len() - 1)
            || rec_count == (config_params.top_cust_count() - 1)
        {
            op_string.pop();
            break;
        }
        rec_count += 1;
    }
    info!(diag_log, "End of creating topnd output data.");

    op_string
}

pub fn write_output(
    config_params: &ConfigurationParameters,
    op_string: String,
    log: &Logger,
    diag_log: &Logger,
) {
    info!(diag_log, "Start of creating topnd output file.");
    let full_path = config_params.output_file();
    let mut wrtr = match sdb_io::buf_file_wrtr(full_path, None) {
        Ok(write) => write,
        Err(error) => {
            log_error!(
                log,
                "Cannot write to file at path: '{}'. Error: {}",
                full_path,
                error
            );
            panic!(format!(
                "Cannot write to file at path: '{}'. Error: {}",
                full_path, error
            ));
        }
    };

    info!(diag_log, "End of creating topnd output file.");
    info!(diag_log, "Start of writing topnd data into output file.");

    wrtr.write(op_string.as_bytes()).expect(&format!(
        "Couldn't write to output file. String: {}",
        op_string
    ));
    wrtr.flush()
        .expect("Unable to flush topnd ouput writer contents");
    info!(diag_log, "End of writing topnd data into output file.");
}

fn order_float(bal1: f64, bal2: f64) -> Ordering {
    if bal1 < bal2 {
        Ordering::Less
    } else if bal1 > bal2 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
