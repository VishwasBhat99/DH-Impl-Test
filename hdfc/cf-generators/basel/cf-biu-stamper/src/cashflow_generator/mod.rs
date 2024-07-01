mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_appender;
mod input_structs;

use self::cashflow_appender::append_data;
use self::input_structs::{BiuData, CustData, DescMaster};
use ahash::AHashMap;
use cashflow_generator::account_reader::InputAccountReader;
use cashflow_generator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::new_buf_rdr;
use slog::Logger;
use statics::*;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::SystemTime;

pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut total_accounts_encountered: i64 = DEFAULT_INT;

    let start_generate_timer = SystemTime::now();
    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut input_map: AHashMap<String, _> = AHashMap::new();

    let input_file = match new_buf_rdr(config_params.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!("Unable to open input file : {:?}", error),
    };
    for line in BufReader::new(input_file).lines().skip(0) {
        let record = match line {
            Ok(ln) => ln,
            Err(error) => {
                panic!("Error while reading input file : {:?}", error);
            }
        };
        let fields: Vec<&str> = record.split("|").collect();
        input_map.insert(fields[1].to_string(), 0 as u8);
    }
    let custid_timer = SystemTime::now();
    let tot_dur_custid = custid_timer
        .duration_since(start_generate_timer)
        .expect("Could not calculate total duration.");
    println!(
        "Time to read and store cust-ids from Input File: {:?}",
        tot_dur_custid
    );
    info!(
        log,
        "Time to read and store cust-ids from Input File: {:?}", tot_dur_custid
    );

    let mut reader_iterator = reader;
    // read biu master file
    let biu_master_file = match new_buf_rdr(config_params.biu_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.biu_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut biu_master: AHashMap<String, BiuData> = AHashMap::new();
    for (line_num, lines) in biu_master_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.output_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        let biu_line = BiuData {
            t1: fields[1].to_string(),
            t2: fields[2].to_string(),
            t3: fields[3].to_string(),
            t4: fields[4].to_string(),
            div: fields[5].to_string(),
        };
        if input_map.contains_key(&fields[0].to_string()) {
            biu_master.insert(fields[0].to_string(), biu_line);
        }
    }
    let biu_time = SystemTime::now();
    let biu_dur = biu_time
        .duration_since(custid_timer)
        .expect("Could not calculate total duration.");
    println!("Time to read BIU Master File: {:?}", biu_dur);
    info!(log, "Time to read BIU Master File: {:?}", biu_dur);
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
    let mut cust_master: AHashMap<String, CustData> = AHashMap::new();
    for (line_num, lines) in cust_master_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.cust_master_file_path(),
                line_num + 1,
                error
            ),
        };
        if line.contains("~#~") {
            let fields: Vec<&str> = line.split("~#~").collect();
            let cust_info_line = CustData {
                nob: fields[1].to_string(),
                text_desc: fields[2].to_string(),
            };
            if input_map.contains_key(&fields[0].to_string()) {
                cust_master.insert(fields[0].to_string(), cust_info_line);
            }
        }
    }
    let cust_master_time = SystemTime::now();
    let cust_master_dur = cust_master_time
        .duration_since(biu_time)
        .expect("Could not calculate total duration.");
    println!("Time to read Cust Master File: {:?}", cust_master_dur);
    info!(log, "Time to read Cust Master File: {:?}", cust_master_dur);
    //Reading of text_desc_master file
    let text_desc_master_file = match new_buf_rdr(config_params.text_desc_master_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}`,{}",
            config_params.text_desc_master_file_path(),
            error
        ),
    };
    let mut text_desc_vec: Vec<DescMaster> = Vec::new();
    let mut cust_matchcase_vec: Vec<String> = Vec::new();
    for (line_num, lines) in text_desc_master_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.text_desc_master_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split("|").collect();
        let text_desc_master_line = DescMaster {
            flag_value: fields[0].to_string(),
            condition: fields[1].to_string(),
            txt_desc_flag: fields[2].to_string(),
        };
        let condition = fields[1].to_string();
        text_desc_vec.push(text_desc_master_line);
        if condition.to_uppercase() == "MATCHCASE"{
            cust_matchcase_vec.push(fields[0].to_string());
        }
    }
    let text_desc_master_time = SystemTime::now();
    let text_desc_master_dur = text_desc_master_time
        .duration_since(cust_master_time)
        .expect("Could not calculate total duration.");
    println!(
        "Time to read Text Desc Master File: {:?}",
        text_desc_master_dur
    );
    info!(
        log,
        "Time to read Text Desc Master File: {:?}", text_desc_master_dur
    );
    // read cust total balance file
    let ttl_bal_file = match new_buf_rdr(config_params.ttl_bal_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.ttl_bal_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut ttl_bal_master: AHashMap<String, f64> = AHashMap::new();
    for (line_num, lines) in ttl_bal_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.output_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        if input_map.contains_key(&fields[0].to_string()) {
            ttl_bal_master.insert(
                fields[0].to_string(),
                fields[1].parse().unwrap_or(DEFAULT_FLOAT),
            );
        }
    }
    let ttl_time = SystemTime::now();
    let tot_dur_ttl = ttl_time
        .duration_since(text_desc_master_time)
        .expect("Could not calculate total duration.");
    println!("Time to read TTL Bal File: {:?}", tot_dur_ttl);
    info!(log, "Time to read TTL Bal File: {:?}", tot_dur_ttl);
    let mut tot_rec = 0;
    let skp_rec = 0;
    let mut tot_amt = 0.0;

    let biu_no_data = BiuData {
        t1: "NONE".to_string(),
        t2: "NONE".to_string(),
        t3: "NONE".to_string(),
        t4: "NONE".to_string(),
        div: "NONE".to_string(),
    };
    let cust_no_data = CustData {
        nob: "NONE".to_string(),
        text_desc: "NONE".to_string(),
    };

    loop {
        let account_opt = log_measurements!(
            diag_log,
            [format!(
                "Type: ReadParseInputAccount, Identifier: `{}`",
                total_accounts_encountered
            )],
            reader_iterator.next()
        );

        if account_opt.is_none() {
            break;
        }
        tot_rec += 1;
        let input_account = account_opt.expect("Unable to parse `input records`.");
        total_accounts_encountered += 1;

        let biu_data = match biu_master.get(&input_account.cust_id) {
            Some(val) => val,
            None => &biu_no_data,
        };
        let cust_data: &CustData = match cust_master.get(&input_account.cust_id) {
            Some(val) => val,
            None => &cust_no_data,
        };
        let t1 = &biu_data.t1;
        let t2 = &biu_data.t2;
        let t3 = &biu_data.t3;
        let t4 = &biu_data.t4;
        let nob = &cust_data.nob;
        let text_desc = &cust_data.text_desc;
        let div = &biu_data.div;
        // TODO: Parameterize this set of values for text desc flag

        let text_desc_flag = get_text_desc_flag(cust_data.to_owned(), text_desc_vec.to_owned(),cust_matchcase_vec.clone());
        let total_deposits = match ttl_bal_master.get(&input_account.cust_id) {
            Some(val) => *val,
            None => DEFAULT_FLOAT,
        };
        tot_amt += total_deposits;

        let account_data = append_data(
            input_account,
            t1.to_string(),
            t2.to_string(),
            t3.to_string(),
            t4.to_string(),
            nob.to_string(),
            text_desc.to_string(),
            text_desc_flag.to_string(),
            total_deposits,
            div.to_string(),
        );
        writer.write(account_data);
    }
    writer.close();
    let end_time = SystemTime::now();
    let tot_dur_proc = end_time
        .duration_since(ttl_time)
        .expect("Could not calculate total duration.");
    println!("Time to process and write Output: {:?}", tot_dur_proc);
    info!(log, "Time to process and write Output: {:?}", tot_dur_proc);

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file_path());
}

fn create_io_workers(
    input_path: &str,
    output_path: &str,
    log: &Logger,
) -> (InputAccountReader, AccountWithCashflowsWriter) {
    let reader = InputAccountReader::new(input_path, log);
    let writer = AccountWithCashflowsWriter::new(output_path, log);

    (reader, writer)
}

fn get_text_desc_flag(cust_data: CustData, text_desc_data1: Vec<DescMaster>, cust_matchcase_vec: Vec<String>) -> String {
    let mut text_desc_flag = "OTHERS".to_string();
    for text_desc_data in text_desc_data1 {
        text_desc_flag = match text_desc_data.condition.to_uppercase().as_str() {
            "MATCHCASE" => {
                if cust_data.text_desc
                    .to_uppercase()
                    .to_string()
                    .trim_end()
                    .trim_start()
                    .eq(&text_desc_data.flag_value.to_uppercase())
                {
                    text_desc_data.txt_desc_flag
                } else {
                    "OTHERS".to_string()
                }
            }
         
            "START" => {
                if cust_data.text_desc
                    .to_uppercase()
                    .starts_with(&text_desc_data.flag_value.to_uppercase()) && !cust_matchcase_vec.contains(&cust_data.text_desc)
                {
                    text_desc_data.txt_desc_flag
                } else {
                    "OTHERS".to_string()
                }
            }
            "END" => {
                if cust_data.text_desc
                    .to_uppercase()
                    .ends_with(&text_desc_data.flag_value.to_uppercase()) && !cust_matchcase_vec.contains(&cust_data.text_desc)
                {
                    text_desc_data.txt_desc_flag
                } else {
                    "OTHERS".to_string()
                }
            }
            "BETWEEN" => {
                if cust_data.text_desc
                    .to_uppercase()
                    .contains(&text_desc_data.flag_value.to_uppercase()) && !cust_matchcase_vec.contains(&cust_data.text_desc)
                {
                    text_desc_data.txt_desc_flag
                } else {
                    "OTHERS".to_string()
                }
            }
           
            _ => "OTHERS".to_string(),
        };
        if text_desc_flag != "OTHERS" {
            break;
        }
    }
    text_desc_flag
}
