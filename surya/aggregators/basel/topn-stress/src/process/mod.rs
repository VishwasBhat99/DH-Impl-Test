use self::output::*;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::BufRead, BufReader, BufWriter, Write};

mod output;

pub fn process(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut succ_acc = 0;
    let mut ip_amt: f64 = 0.0;
    let mut op_amt: f64 = 0.0;

    //Init Writer
    let mut op_writer = get_writer(config_params.output_file_path());

    //Reading Class LLG Mapper File
    let mut class_llg_mapper: HashMap<String, String> = HashMap::new();
    let class_llg_file = File::open(config_params.class_llg_mapper_file_path())
        .expect("Could Not Read Class LLG Mapper File");
    let class_llg_reader = BufReader::new(class_llg_file);
    for (line_no, line) in class_llg_reader.lines().enumerate() {
        let data: String = match line {
            Ok(data) => data,
            Err(error) => {
                log_error!(
                    logger,
                    "Cannot read line {} from class_llg_mapper: {:?}",
                    line_no + 1,
                    error
                );
                continue;
            }
        };
        let class_llg_fields: Vec<&str> = data.split('|').collect();
        let llgid = get_str(
            config_params.class_llg_mapper_file_path(),
            &class_llg_fields,
            0,
            line_no,
        );
        let classid = get_str(
            config_params.class_llg_mapper_file_path(),
            &class_llg_fields,
            1,
            line_no,
        );
        let stable_or_less_stable = get_str(
            config_params.class_llg_mapper_file_path(),
            &class_llg_fields,
            2,
            line_no,
        );
        if stable_or_less_stable.trim().to_uppercase() == "STABLE" {
            class_llg_mapper.insert(classid + "S", llgid);
        } else {
            class_llg_mapper.insert(classid + "LS", llgid);
        }
    }

    //Reading Exchange Rate File
    let mut exrt_map: HashMap<String, f64> = HashMap::new();
    let exrt_file =
        File::open(config_params.exrt_rate_file_path()).expect("Could Not Read Exchange Rate File");
    let exrt_reader = BufReader::new(exrt_file);
    for (line_no, line) in exrt_reader.lines().enumerate() {
        let data: String = match line {
            Ok(data) => data,
            Err(error) => {
                log_error!(
                    logger,
                    "Cannot read line {} from Exchange Rate File: {:?}",
                    line_no + 1,
                    error
                );
                continue;
            }
        };
        let exrt_fields: Vec<&str> = data.split('|').collect();
        //Store From-CCY -> Exrt-Rate
        exrt_map.insert(
            get_str(
                config_params.exrt_rate_file_path(),
                &exrt_fields,
                0,
                line_no,
            ),
            get_float(get_str(
                config_params.exrt_rate_file_path(),
                &exrt_fields,
                2,
                line_no,
            )),
        );
    }

    //Reading Retail and Non-Retail Deposits File
    let mut ret_nonret_map: HashMap<String, (String, f64, f64)> = HashMap::new();
    let ret_file =
        File::open(config_params.retail_input_file_path()).expect("Could Not Read Retail File");
    let ret_reader = BufReader::new(ret_file);
    for (line_no, line) in ret_reader.lines().enumerate() {
        let data: String = match line {
            Ok(data) => data,
            Err(error) => {
                log_error!(
                    logger,
                    "Cannot read line {} from Retail File: {:?}",
                    line_no + 1,
                    error
                );
                continue;
            }
        };
        let ret_fields: Vec<&str> = data.split('|').collect();
        let class_id: String = get_str(
            config_params.retail_input_file_path(),
            &ret_fields,
            0,
            line_no,
        );
        let cust_id: String = get_str(
            config_params.retail_input_file_path(),
            &ret_fields,
            1,
            line_no,
        );
        let ccy = get_str(
            config_params.retail_input_file_path(),
            &ret_fields,
            2,
            line_no,
        );
        let stable_amt = get_float(get_str(
            config_params.retail_input_file_path(),
            &ret_fields,
            26,
            line_no,
        )) * exrt_map.get(&ccy).unwrap_or_else(|| {
            panic!(
                "Could not get exch-rate for {}|{}",
                ccy,
                config_params.ccy_id()
            )
        });
        let less_stable_amt = get_float(get_str(
            config_params.retail_input_file_path(),
            &ret_fields,
            27,
            line_no,
        )) * exrt_map.get(&ccy).unwrap_or_else(|| {
            panic!(
                "Could not get exch-rate for {}|{}",
                ccy,
                config_params.ccy_id()
            )
        });
        if ret_nonret_map.contains_key(&cust_id) {
            let (clsid, st_amt, lsst_amt) = ret_nonret_map
                .get(&cust_id)
                .expect("Unable to get Retail Data");
            if *clsid != class_id {
                log_error!(
                    logger,
                    "Different Class-IDs: {} and {} found for Same Customer: {}",
                    class_id,
                    clsid,
                    cust_id
                );
            }
            ret_nonret_map.insert(
                cust_id,
                (
                    clsid.to_string(),
                    stable_amt + st_amt,
                    less_stable_amt + lsst_amt,
                ),
            );
        } else {
            ret_nonret_map.insert(cust_id, (class_id, stable_amt, less_stable_amt));
        }
    }
    //Reading Non-Retail File
    let non_ret_file = File::open(config_params.non_retail_input_file_path())
        .expect("Could Not Read Non Retail File");
    let non_ret_reader = BufReader::new(non_ret_file);
    for (line_no, line) in non_ret_reader.lines().enumerate() {
        let data: String = match line {
            Ok(data) => data,
            Err(error) => {
                log_error!(
                    logger,
                    "Cannot read line {} from Retail File: {:?}",
                    line_no + 1,
                    error
                );
                continue;
            }
        };
        let non_ret_fields: Vec<&str> = data.split('|').collect();
        let class_id: String = get_str(
            config_params.non_retail_input_file_path(),
            &non_ret_fields,
            0,
            line_no,
        );
        let cust_id: String = get_str(
            config_params.non_retail_input_file_path(),
            &non_ret_fields,
            1,
            line_no,
        );
        let ccy = get_str(
            config_params.non_retail_input_file_path(),
            &non_ret_fields,
            2,
            line_no,
        );
        let stable_amt = get_float(get_str(
            config_params.non_retail_input_file_path(),
            &non_ret_fields,
            26,
            line_no,
        )) * exrt_map.get(&ccy).unwrap_or_else(|| {
            panic!(
                "Could not get exch-rate for {}|{}",
                ccy,
                config_params.ccy_id()
            )
        });
        let less_stable_amt = get_float(get_str(
            config_params.non_retail_input_file_path(),
            &non_ret_fields,
            27,
            line_no,
        )) * exrt_map.get(&ccy).unwrap_or_else(|| {
            panic!(
                "Could not get exch-rate for {}|{}",
                ccy,
                config_params.ccy_id()
            )
        });
        if ret_nonret_map.contains_key(&cust_id) {
            let (clsid, st_amt, lsst_amt) = ret_nonret_map
                .get(&cust_id)
                .expect("Unable to get Retail Data");
            if *clsid != class_id {
                log_error!(
                    logger,
                    "Different Class-IDs: {} and {} found for Same Customer: {}",
                    class_id,
                    clsid,
                    cust_id
                );
            }
            ret_nonret_map.insert(
                cust_id,
                (
                    clsid.to_string(),
                    stable_amt + st_amt,
                    less_stable_amt + lsst_amt,
                ),
            );
        } else {
            ret_nonret_map.insert(cust_id, (class_id, stable_amt, less_stable_amt));
        }
    }

    //Reading TopN Deposits File
    let topn_dep_file =
        File::open(config_params.topn_dep_file_path()).expect("Could Not Read TopN Deposits File");
    let topn_reader = BufReader::new(topn_dep_file);
    for (line_no, line) in topn_reader.lines().enumerate() {
        let data: String = match line {
            Ok(data) => data,
            Err(error) => {
                log_error!(
                    logger,
                    "Cannot read line {} from TopN Deposits File: {:?}",
                    line_no + 1,
                    error
                );
                continue;
            }
        };
        let topn_fields: Vec<&str> = data.split('|').collect();
        acc_enc += 1;
        ip_amt += get_float(get_str(
            config_params.topn_dep_file_path(),
            &topn_fields,
            5,
            line_no,
        ));
        let (cust_id, cust_name) = (
            get_str(config_params.topn_dep_file_path(), &topn_fields, 3, line_no),
            get_str(config_params.topn_dep_file_path(), &topn_fields, 4, line_no),
        );
        let stable_op = OutputAccount::new(
            true,
            (cust_id.to_string(), cust_name.to_string()),
            &config_params,
            line_no,
            &mut ret_nonret_map,
            &mut class_llg_mapper,
        );
        succ_acc += 1;
        op_amt += stable_op.hcy_amt;
        let less_stable_op = OutputAccount::new(
            false,
            (cust_id.to_string(), cust_name.to_string()),
            &config_params,
            line_no,
            &mut ret_nonret_map,
            &mut class_llg_mapper,
        );
        succ_acc += 1;
        op_amt += less_stable_op.hcy_amt;
        writeln!(op_writer, "{}", format_output(stable_op))
            .expect("Error while Writing Stable Data");
        writeln!(op_writer, "{}", format_output(less_stable_op))
            .expect("Error while Writing Less-Stable Data");
    }

    // Generate Health Check Report
    let health_report =
        HealthReport::new(acc_enc, succ_acc, acc_enc - succ_acc / 2, ip_amt, op_amt, 0);
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file_path());
}

pub fn get_writer(file_path: &str) -> BufWriter<File> {
    match sdb_io::buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!("Unable to create file `{}`: {}", file_path, error),
    }
}

pub fn get_str(input_file: &str, data: &[&str], index: usize, row: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` in row-no: `{:?}` from File: {}",
                index + 1,
                row + 1,
                input_file,
            )
        })
        .trim()
        .to_string()
}

pub fn get_float(data: String) -> f64 {
    data.parse::<f64>().unwrap_or(0.0)
}
