use self::account_reader::InputAccountReader;
use self::reader::BiuData;
use self::structs::Schema;
use self::structs::{AccData, AccKey, AggrData};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::Write;

mod account_reader;
mod reader;
mod structs;

pub fn aggregate(config_params: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let reader = InputAccountReader::new(config_params.input_file_path(), log);
    let mut lien_map: HashMap<String, f64> = HashMap::new();
    // init ret and non-ret writers
    let op = format!(
        "{}-{}.txt",
        config_params.output_file_path().replace(".txt", ""),
        config_params.is_lcy_or_ccy()
    );
    let mut op_writer = match buf_file_wrtr(&op, None) {
        Ok(val) => val,
        Err(error) => {
            panic!("Could not create file: `{}` due to : {:?}.", op, error);
        }
    };
    let mut reader_iterator = reader.into_iter();
    // init bucket schema file
    let bkt_file = match new_buf_rdr(config_params.bkt_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.bkt_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    // read biu master file
    let biu_master_file = match new_buf_rdr(config_params.biu_master_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.biu_master_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut biu_master: HashMap<i32, BiuData> = HashMap::new();
    for (line_num, lines) in biu_master_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.biu_master_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        let biu_line = BiuData {
            t1: fields[1].to_string(),
            t2: fields[2].to_string(),
            t3: fields[3].to_string(),
        };
        biu_master.insert(fields[0].parse().unwrap_or(0), biu_line);
    }
    log_info!(log, "Reading BIU data completed.");

    let mut tot_rec = 0;
    let skp_rec = 0;
    let mut tot_amt = 0.0;

    // read ca ret file
    let ca_ret_file = match new_buf_rdr(config_params.ca_ret_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.ca_ret_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut ca_master: HashMap<AccKey, Vec<AccData>> = HashMap::new();
    for (line_num, lines) in ca_ret_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.ca_ret_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        tot_rec += 1;
        tot_amt += fields[7].parse::<f64>().unwrap_or(0.0);
        let ca_acc_line = AccData {
            amount: fields[7].parse::<f64>().unwrap_or(0.0),
            lcy_amount: fields[8].parse::<f64>().unwrap_or(0.0),
            is_nwd_final: fields[12].to_string(),
            bucket_id: fields[13].parse::<usize>().unwrap_or(0),
            int_rate: fields
                .get(14)
                .unwrap_or(&"0.0")
                .parse::<f64>()
                .unwrap_or(0.0),
        };
        let currency = if config_params.is_lcy_or_ccy() == "CCY" {
            fields[4]
        } else {
            ""
        }
        .to_string();
        let key = AccKey {
            class_id: fields[0].parse().unwrap_or(0),
            cust_id: fields[2].parse().unwrap_or(0),
            currency,
        };
        if let Some(data) = ca_master.get_mut(&key) {
            data.push(ca_acc_line);
        } else {
            ca_master.insert(key, vec![ca_acc_line]);
        }
    }
    log_info!(log, "Reading CA data completed.");

    // read sa ret file
    let sa_ret_file = match new_buf_rdr(config_params.sa_ret_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.sa_ret_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut sa_master: HashMap<AccKey, Vec<AccData>> = HashMap::new();
    for (line_num, lines) in sa_ret_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.sa_ret_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        tot_rec += 1;
        tot_amt += fields[7].parse::<f64>().unwrap_or(0.0);
        let sa_acc_line = AccData {
            amount: fields[7].parse::<f64>().unwrap_or(0.0),
            lcy_amount: fields[8].parse::<f64>().unwrap_or(0.0),
            is_nwd_final: fields[12].to_string(),
            bucket_id: fields[13].parse::<usize>().unwrap_or(0),
            int_rate: fields
                .get(14)
                .unwrap_or(&"0.0")
                .parse::<f64>()
                .unwrap_or(0.0),
        };
        let currency = if config_params.is_lcy_or_ccy() == "CCY" {
            fields[4]
        } else {
            ""
        }
        .to_string();
        let key = AccKey {
            class_id: fields[0].parse().unwrap_or(0),
            cust_id: fields[2].parse().unwrap_or(0),
            currency,
        };
        if let Some(data) = sa_master.get_mut(&key) {
            data.push(sa_acc_line);
        } else {
            sa_master.insert(key, vec![sa_acc_line]);
        }
    }
    log_info!(log, "Reading SA data completed.");

    // read td ret file
    let td_ret_file = match new_buf_rdr(config_params.td_ret_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.td_ret_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut td_master: HashMap<AccKey, Vec<AccData>> = HashMap::new();
    for (line_num, lines) in td_ret_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.td_ret_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        tot_rec += 1;
        tot_amt += fields[7].parse::<f64>().unwrap_or(0.0);
        let acc_line = AccData {
            amount: fields[7].parse::<f64>().unwrap_or(0.0),
            lcy_amount: fields[8].parse::<f64>().unwrap_or(0.0),
            is_nwd_final: fields[12].to_string(),
            bucket_id: fields[13].parse::<usize>().unwrap_or(0),
            int_rate: fields
                .get(14)
                .unwrap_or(&"0.0")
                .parse::<f64>()
                .unwrap_or(0.0),
        };
        let currency = if config_params.is_lcy_or_ccy() == "CCY" {
            fields[4]
        } else {
            ""
        }
        .to_string();
        let key = AccKey {
            class_id: fields[0].parse().unwrap_or(0),
            cust_id: fields[2].parse().unwrap_or(0),
            currency,
        };
        if let Some(data) = td_master.get_mut(&key) {
            data.push(acc_line);
        } else {
            td_master.insert(key, vec![acc_line]);
        }
    }
    log_info!(log, "Reading TD data completed.");

    // read rd ret file
    let rd_ret_file = match new_buf_rdr(config_params.rd_ret_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.rd_ret_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut rd_master: HashMap<AccKey, Vec<AccData>> = HashMap::new();
    for (line_num, lines) in rd_ret_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.rd_ret_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        tot_rec += 1;
        tot_amt += fields[7].parse::<f64>().unwrap_or(0.0);
        let acc_line = AccData {
            amount: fields[7].parse::<f64>().unwrap_or(0.0),
            lcy_amount: fields[8].parse::<f64>().unwrap_or(0.0),
            is_nwd_final: fields[12].to_string(),
            bucket_id: fields[13].parse::<usize>().unwrap_or(0),
            int_rate: fields
                .get(14)
                .unwrap_or(&"0.0")
                .parse::<f64>()
                .unwrap_or(0.0),
        };
        let currency = if config_params.is_lcy_or_ccy() == "CCY" {
            fields[4]
        } else {
            ""
        }
        .to_string();
        let key = AccKey {
            class_id: fields[0].parse().unwrap_or(0),
            cust_id: fields[2].parse().unwrap_or(0),
            currency,
        };
        if let Some(data) = rd_master.get_mut(&key) {
            data.push(acc_line);
        } else {
            rd_master.insert(key, vec![acc_line]);
        }
    }
    log_info!(log, "Reading RD data completed.");

    let mut bkt_def: Vec<Schema> = Vec::new();
    let mut num_of_bkts = 0;
    for (line_num, lines) in bkt_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.bkt_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        let bkt_id = fields[2]
            .parse::<i64>()
            .expect("Cannot parse bkt id as integer.");
        let bkt_schema = Schema {
            from_bkt: fields[0]
                .parse::<i64>()
                .expect("Cannot parse from_bkt as integer."),
            to_bkt: fields[1]
                .parse::<i64>()
                .expect("Cannot parse to_bkt as integer."),
            id: bkt_id,
        };
        if bkt_id > num_of_bkts {
            num_of_bkts = bkt_id;
        }
        bkt_def.push(bkt_schema);
    }

    let lien_file = match new_buf_rdr(config_params.lien_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` due to: {}.",
            config_params.lien_file_path(),
            error
        ),
    };

    for (line_num, lines) in lien_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.lien_file_path(),
                line_num + 1,
                error
            ),
        };
        let lien_fields: Vec<&str> = line.split('|').collect();
        let cust_id = lien_fields[0].to_string();
        let lien_amt = lien_fields[1].to_string().parse().unwrap_or(0.0);
        lien_map.insert(cust_id, lien_amt);
    }
    let max_stable_amt: f64 = config_params.max_stable_amount();
    loop {
        let account_opt = reader_iterator.next();
        if account_opt.is_none() {
            break;
        }
        let input_account = account_opt.expect("Unable to parse record into input struct");
        let no_data = AccData {
            amount: 0.0,
            lcy_amount: 0.0,
            is_nwd_final: "FALSE".to_string(),
            bucket_id: 0,
            int_rate: 0.0,
        };
        let biu_no_data = BiuData {
            t1: "N".to_string(),
            t2: "N".to_string(),
            t3: "N".to_string(),
        };
        // tuple fields: T1, T2, T3
        let biu_data = match biu_master.get(&input_account.customer_id) {
            Some(val) => val,
            None => {
                log_warn!(
                    log,
                    "BIU data not found for Customer ID: {}",
                    &input_account.customer_id
                );
                &biu_no_data
            }
        };
        let currency = if config_params.is_lcy_or_ccy() == "CCY" {
            input_account.currency
        } else {
            "".to_string()
        };
        let acc_key = AccKey {
            class_id: input_account.class_id,
            cust_id: input_account.customer_id,
            currency: currency.to_string(),
        };
        let mut acc_data = AggrData::new(num_of_bkts as usize);

        acc_data.init_biu_data(&biu_data.t1, &biu_data.t2, &biu_data.t3);

        // tuple fields: amount, lcy_amount, is_nwd, bkt_id
        if let Some(val) = ca_master.get(&acc_key) {
            // aggr CA data bu cust id
            for ca_data in val {
                acc_data.aggr_ca_data(ca_data, &config_params);
            }
        };
        if let Some(val) = sa_master.get(&acc_key) {
            // aggr SA data bu cust id
            for sa_data in val {
                acc_data.aggr_sa_data(sa_data, &config_params);
            }
        };
        if let Some(val) = td_master.get(&acc_key) {
            // aggr TD data bu cust id
            for td_data in val {
                acc_data.aggr_acc_data(td_data, &no_data, &config_params);
            }
        };
        if let Some(val) = rd_master.get(&acc_key) {
            // aggr RD data bu cust id
            for rd_data in val {
                acc_data.aggr_acc_data(&no_data, &rd_data, &config_params);
            }
        };

        // Write LCY data
        calculate_write_stable_data(
            acc_key,
            acc_data,
            max_stable_amt,
            if config_params.is_lcy_or_ccy() == "LCY" {
                config_params.base_currency().to_string()
            } else {
                currency.clone()
            },
            &mut op_writer,
            config_params.is_casa_stable_amount(),
            &mut lien_map,
        );
    }
    log_info!(log, "Completed aggregation of all the records.");
    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file_path());
}

fn calculate_write_stable_data(
    cust_key: AccKey,
    data: AggrData,
    max_stable_amt: f64,
    currency: String,
    writer: &mut BufWriter<File>,
    is_casa_stable_amt: bool,
    lien_map: &mut HashMap<String, f64>,
) {
    let logic_type = if data.t1 == "N" && data.t2 == "N" && data.t3 == "N" {
        "N"
    } else {
        "Y"
    };
    let mut lien_amt = 0.0;
    if lien_map.contains_key(&cust_key.cust_id.to_string()) {
        let fd_lien_amt = *lien_map
            .get(&cust_key.cust_id.to_string())
            .unwrap_or_else(|| panic!("Could Not get Lien-Data for Cust-ID: {}", cust_key.cust_id));
        lien_amt = if fd_lien_amt > (data.td_wd_amt + data.rd_amt) {
            data.td_wd_amt + data.rd_amt
        } else {
            fd_lien_amt
        };
    }
    let bal_before_lien = data.ca_amt + data.sa_amt + data.td_wd_amt + data.rd_amt;
    let bal_after_lien = bal_before_lien - lien_amt;
    let total_wd = bal_after_lien;
    let total_stable;
    let total_unstable;

    if logic_type == "Y" {
        if total_wd > max_stable_amt {
            total_stable = max_stable_amt;
            total_unstable = total_wd - max_stable_amt;
        } else {
            total_stable = total_wd;
            total_unstable = 0.0;
        }
    } else if is_casa_stable_amt {
        let casa_amt = data.ca_amt + data.sa_amt;
        if casa_amt <= max_stable_amt {
            total_stable = casa_amt;
        } else {
            total_stable = max_stable_amt;
        }
        total_unstable = casa_amt - total_stable + data.td_wd_amt + data.rd_amt;
    } else {
        total_stable = 0.0;
        total_unstable = total_wd;
    }
    let ca_stable = if data.ca_amt < total_stable {
        data.ca_amt
    } else {
        total_stable
    };
    let ca_unstable = data.ca_amt - ca_stable;

    let remaining_stable_for_sa = total_stable - ca_stable;
    let sa_stable = if data.sa_amt < remaining_stable_for_sa {
        data.sa_amt
    } else {
        remaining_stable_for_sa
    };
    let sa_unstable = data.sa_amt - sa_stable;

    let casa_stable = ca_stable + sa_stable;
    let casa_unstable = ca_unstable + sa_unstable;
    let mut remaining_stable;
    if casa_stable != 0.0 {
        remaining_stable = max_stable_amt - casa_stable;
    } else {
        remaining_stable = max_stable_amt;
    }
    let mut stable_bkts: Vec<f64> = Vec::new();
    let mut unstable_bkts: Vec<f64> = Vec::new();
    let mut nwd_stable_bkts: Vec<f64> = Vec::with_capacity(3);
    let mut nwd_unstable_bkts: Vec<f64> = Vec::with_capacity(3);

    for val in &data.td_rd_wd_bkts {
        if logic_type == "Y" {
            if remaining_stable == 0.0 {
                stable_bkts.push(0.0);
                unstable_bkts.push(*val);
            } else {
                if val < &remaining_stable {
                    stable_bkts.push(*val);
                    unstable_bkts.push(0.0);
                    remaining_stable -= val;
                } else {
                    stable_bkts.push(remaining_stable);
                    unstable_bkts.push(*val - remaining_stable);
                    remaining_stable = 0.0;
                }
            }
        } else {
            stable_bkts.push(0.0);
            unstable_bkts.push(*val);
            remaining_stable -= val;
        }
    }
    for val in &data.td_nwd_bkts {
        if logic_type == "Y" {
            if val < &remaining_stable {
                nwd_stable_bkts.push(*val);
                nwd_unstable_bkts.push(0.0);
                remaining_stable -= val
            } else {
                nwd_stable_bkts.push(remaining_stable);
                nwd_unstable_bkts.push(val - remaining_stable);
                remaining_stable -= remaining_stable
            }
        } else {
            if val < &remaining_stable {
                nwd_stable_bkts.push(0.0);
                nwd_unstable_bkts.push(*val);
                remaining_stable -= val
            } else {
                nwd_stable_bkts.push(0.0);
                nwd_unstable_bkts.push(*val);
                remaining_stable -= val
            }
        }
    }

    writeln!(
        writer,
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        cust_key.class_id,
        cust_key.cust_id,
        currency,
        data.ca_amt,
        data.sa_amt,
        data.td_wd_amt,
        data.td_nwd_amt,
        data.rd_amt,
        vec_as_str(data.td_wd_bkts),
        vec_as_str(data.td_nwd_bkts),
        vec_as_str(data.rd_wd_bkts),
        vec_as_str(data.td_rd_wd_bkts),
        data.t1,
        data.t2,
        data.t3,
        total_wd,
        data.td_nwd_amt,
        logic_type,
        total_stable,
        total_unstable,
        ca_stable,
        ca_unstable,
        sa_stable,
        sa_unstable,
        casa_stable,
        casa_unstable,
        vec_as_str(stable_bkts),
        vec_as_str(unstable_bkts),
        vec_as_str(nwd_stable_bkts),
        vec_as_str(nwd_unstable_bkts),
        get_weighted_avg_int_rate(data.ca_wt_int_rate,data.ca_amt),
        get_weighted_avg_int_rate(data.sa_wt_int_rate,data.sa_amt),
        get_weighted_avg_int_rate(data.td_wd_wt_int_rate,data.td_wd_amt),
        get_weighted_avg_int_rate(data.td_nwd_wt_int_rate,data.td_nwd_amt),
        get_weighted_avg_int_rate(data.rd_wt_int_rate,data.rd_amt),
        lien_amt,
        bal_before_lien,
        bal_after_lien,
    )
    .expect("Error while writing aggregation summary data to file.");
}

fn get_weighted_avg_int_rate(wt_int_rt: f64, tot_amt: f64) -> f64 {
    if tot_amt != 0.0 {
        wt_int_rt / tot_amt
    } else {
        0.0
    }
}

fn vec_as_str(bkts: Vec<f64>) -> String {
    let mut vec_as_str: String = String::new();
    let mut is_skip = true;
    for val in bkts {
        if is_skip {
            is_skip = false;
            continue;
        }
        vec_as_str.push_str(&val.to_string());
        vec_as_str.push('|');
    }
    vec_as_str.pop();
    vec_as_str
}
