use self::account_reader::InputAccountReader;
use self::aggr_data::AggrData;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::env::current_dir;
use std::io::prelude::*;

mod account_reader;
mod aggr_data;
mod statics;
use self::statics::DEFAULT_INT;
pub fn process_name(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    let mut depth_value = DEFAULT_INT;
    let mut order_value = DEFAULT_INT;
    #[derive(Hash, Eq, PartialEq, Debug)]
    struct depth_order {
        depth: i64,
        disp_order: i64,
    }
    impl depth_order {
        fn new(depth: &i64, disp_order: &i64) -> depth_order {
            depth_order {
                depth: *depth,
                disp_order: *disp_order,
            }
        }
    }
    // read cust master file
    let cust_master_file = match new_buf_rdr(config_params.cust_master_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.cust_master_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut cust_master: HashMap<String, String> = HashMap::new();
    for (line_num, lines) in cust_master_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.output_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split(config_params.cust_master_delimiter()).collect();
        let mut id: String = "999".to_string();
        if fields[1].to_string() != "" {
            id = fields[1].to_string();
        }
        cust_master.insert(fields[0].to_string(), id);
    }
    let rbi_cat_def = match new_buf_rdr(config_params.rbi_cat_def_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.rbi_cat_def_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut rbi_cat_depth_order = HashMap::new();
    let mut rbi_cat_parent_map: HashMap<String, Vec<String>> = HashMap::new();
    for (line_num, lines) in rbi_cat_def.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.rbi_cat_def_file_path(),
                line_num + 1,
                error
            ),
        };
        let mut cat_id_vec: Vec<String> = Vec::new();
        let fields: Vec<&str> = line.split("|").collect();
        rbi_cat_depth_order.insert(
            fields[0].to_string(),
            depth_order::new(
                &fields[3].to_string().parse::<i64>().unwrap(),
                &fields[5].parse::<i64>().unwrap(),
            ),
        );
        if rbi_cat_parent_map.contains_key(fields[6].trim()) {
            cat_id_vec = rbi_cat_parent_map.get(fields[6].trim()).unwrap().clone();
        }
        if fields[6].trim() != fields[0] {
            cat_id_vec.push(fields[0].trim().to_string());
        }
        rbi_cat_parent_map.insert(fields[6].trim().to_string(), cat_id_vec);
    }
    // read RBI CBS file
    let rbi_cbs = match new_buf_rdr(config_params.rbi_cat_map_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.rbi_cat_def_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut rbi_cbs_map: HashMap<String, String> = HashMap::new();
    let mut rbi_cbs_map_reverse: HashMap<String, String> = HashMap::new();
    for (line_num, lines) in rbi_cbs.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.output_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split("|").collect();
        rbi_cbs_map.insert(fields[1].trim().to_string(), fields[0].trim().to_string());
        rbi_cbs_map_reverse.insert(fields[0].trim().to_string(), fields[1].trim().to_string());
    }
    let input_reader = create_io_workers(config_params.input_file_path(), logger);
    let mut input_reader_iterator = input_reader.into_iter();
    let mut aggr_data: HashMap<String, AggrData> = HashMap::new();
    let mut skipp_acc = 0;
    let mut tot_acc_encntrd = 0;
    let mut total_amt_ip: f64 = 0.0;
    let mut total_amt_op: f64 = 0.0;
    loop {
        let account_opt = input_reader_iterator.next();
        if account_opt.is_none() {
            break;
        }
        tot_acc_encntrd += 1;
        let input_account =
            account_opt.expect("Unexpected error occured while reading input file.");
        let aggr_key = cust_master
            .get(&input_account.cust_id)
            .unwrap_or(&"NA".to_string())
            .trim()
            .to_string();
        let rbi_cat_id = rbi_cbs_map
            .get(&aggr_key)
            .unwrap_or(&"NONE".to_string())
            .to_string();
        if rbi_cat_depth_order.contains_key(&rbi_cat_id) {
            let depth_order_value = rbi_cat_depth_order.get(&rbi_cat_id);
            depth_value = depth_order_value.unwrap().depth;
            order_value = depth_order_value.unwrap().disp_order;
        } else {
            depth_value = DEFAULT_INT;
            order_value = DEFAULT_INT;
        }
        let acc_data = AggrData {
            ca_amt: input_account.ca,
            sa_amt: input_account.sa,
            td_amt: (input_account.td_wd + input_account.td_nwd + input_account.rd),
            depth: depth_value,
            order: order_value,
        };
        total_amt_ip = total_amt_ip
            + input_account.ca
            + input_account.sa
            + input_account.td_wd
            + input_account.td_nwd
            + input_account.rd;
        aggr_data
            .entry(rbi_cat_id)
            .and_modify(|data| data.add_data(acc_data))
            .or_insert(acc_data);
    }
    for (key, value) in aggr_data.clone() {
        let mut cat_id_vec: Vec<String> = Vec::new();
        if rbi_cat_parent_map.contains_key(&key) {
            cat_id_vec = rbi_cat_parent_map.get(&key).unwrap().clone();
        }
        for items in cat_id_vec.iter() {
            if !aggr_data.contains_key(items) {
                continue;
            }
            if rbi_cat_parent_map.contains_key(items.trim()) && cat_id_vec.len() > 1 {
                continue;
            }
            let child_amt = &aggr_data.get(items.trim()).unwrap().clone();
            aggr_data
                .entry(key.to_string())
                .and_modify(|data| data.add_data(child_amt.clone()))
                .or_insert(value);
        }
        total_amt_op = total_amt_op + value.ca_amt + value.sa_amt + value.td_amt;
    }
    // Write output
    write_aggr_smry(aggr_data, config_params, rbi_cbs_map_reverse);
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skipp_acc,
        skipp_acc,
        total_amt_ip,
        total_amt_op,
        0,
    );
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file_path());
}

fn create_io_workers(input_path: &str, logger: &Logger) -> InputAccountReader {
    let reader = InputAccountReader::new(input_path, logger);
    reader
}

pub fn write_aggr_smry(
    mut aggr_data: HashMap<String, AggrData>,
    config_params: &ConfigurationParameters,
    rbi_cbs_map_reverse: HashMap<String, String>,
) {
    let mut output_file = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(create) => create,
        Err(error) => {
            panic!(
                "Could not create output file: `{}` on location `{}`: {}.",
                config_params.output_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };

    for (aggr_key, data) in aggr_data.drain() {
        let rbi_cat_id = rbi_cbs_map_reverse
            .get(&aggr_key)
            .unwrap_or(&"999".to_string())
            .to_string();
        write!(
            output_file,
            "{}|{}|{}|{}|{}\n",
            config_params.as_on_date().format("%d-%m-%Y"),
            config_params.display_ccy(),
            rbi_cat_id,
            aggr_key,
            data
        )
        .expect("Unable to generate summary file.");
    }
}
