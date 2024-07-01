use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut ref_excel3: Xlsx<_> = open_workbook(config_param.ref_file_path_3())
        .expect("Unable to open `ALM_Line_Master.xlsx`.");
    let mut isin_to_n1: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel3.worksheet_range(config_param.ref3_sheet_name()) {
        for row in reader.rows().skip(1) {
            isin_to_n1.insert(row[58].to_string(), row[15].to_string());
        }
    }

    let mut ref_excel4: Xlsx<_> =
        open_workbook(config_param.ref_file_path_4()).expect("Unable to open `Ora_PROD.xlsx`.");
    let mut n1_to_cat: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel4.worksheet_range(config_param.ref4_sheet_name()) {
        for row in reader.rows().skip(1) {
            n1_to_cat.insert(row[1].to_string(), row[2].to_string());
        }
    }

    let ref2_file = match new_buf_rdr(config_param.ref_file_path_2()) {
        Ok(ref2_file) => ref2_file,
        Err(error) => panic!(
            "Could not found ref-2 file: `{}` on location `{}` : {}.",
            config_param.ref_file_path_2(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut ref_excel5: Xlsx<_> = open_workbook(config_param.ref_file_path_5())
        .expect("Unable to open `Term Loans UpdateType Master`.");
    let mut term_update_type_master: HashMap<String, Vec<String>> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel5.worksheet_range(config_param.ref5_sheet_name()) {
        for row in reader.rows().skip(1) {
            let term_update_type_vec = vec![
                row[1].to_string().trim().to_string(),
                row[2].to_string().trim().to_string(),
            ];
            term_update_type_master
                .insert(row[0].to_string().trim().to_string(), term_update_type_vec);
        }
    }

    let mut is_cf: HashMap<String, Vec<String>> = HashMap::new();
    let mut tot_amt_ip: f64 = 0.0;
    let mut tot_no_cf: i64 = 0;
    for (line_num, lines) in ref2_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();

        let mut s = String::new();
        s.push_str(fields[2]);
        s.push('|');
        s.push_str(fields[3]);
        s.push('|');
        s.push_str(fields[4]);

        let amt = fields[3].to_string().parse::<f64>();

        if !amt.is_err() {
            tot_amt_ip += fields[3].to_string().parse::<f64>().unwrap();
        } else {
            error!(log, "Amt could not be parsed in f64");
        }

        tot_no_cf += 1;

        let val_0 = fields[0].parse::<u64>();
        let val_1 = fields[1].parse::<u64>();

        if term_update_type_master.contains_key(&fields[2].to_string()) {
            if !val_0.is_err() {
                if is_cf.contains_key(fields[0]) {
                    let v = is_cf.get_mut(&fields[0].to_string()).unwrap();
                    v.push(s);
                } else {
                    let mut v = Vec::new();
                    v.push(s);
                    is_cf.insert(fields[0].to_string(), v);
                }
            } else if !val_1.is_err() {
                if is_cf.contains_key(fields[1]) {
                    let v = is_cf.get_mut(fields[1]).unwrap();
                    v.push(s);
                } else {
                    let mut v = Vec::new();
                    v.push(s);
                    is_cf.insert(fields[1].to_string(), v);
                }
            }
        }
    }

    let end_read_timer = SystemTime::now();
    let duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration read timer.");
    debug!(
        diag_log,
        "Reading Reference Files, Total Duration: {:?}.", duration
    );

    let start_derive_timer = SystemTime::now();
    let mut output_line: String = String::new();
    let mut tot_acc_encntrd: i64 = 0;
    let mut tot_amt_op: f64 = 0.0;
    for (line_num, lines) in file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_file_path(),
                line_num + 1,
                error
            ),
        };

        tot_acc_encntrd += 1;

        let fields: Vec<&str> = line.split('|').collect();

        let mut due_dt = "".to_string();
        let mut component = "".to_string();
        let mut cf_amt = "".to_string();

        if is_cf.contains_key(fields[0]) {
            let values: Vec<String> = is_cf.get(fields[0]).unwrap().to_vec();
            for val in values {
                let vals: Vec<&str> = val.split('|').collect();
                let default_term_data = Vec::default();
                let term_data_vec = term_update_type_master
                    .get(&vals[0].to_string())
                    .unwrap_or(&default_term_data);
                if term_data_vec[0] == "Principal".to_string()
                    && term_data_vec[1] == "NEG".to_string()
                {
                    let cf_amt_val: f64 = vals[1].parse().unwrap();
                    cf_amt = (0.0 - cf_amt_val).to_string();
                    component = "PRINCIPAL".to_string();
                    due_dt = vals[2].to_string();
                } else if term_data_vec[0] == "Principal".to_string()
                    && term_data_vec[1] == "POS".to_string()
                {
                    cf_amt = vals[1].to_string();
                    component = "PRINCIPAL".to_string();
                    due_dt = vals[2].to_string();
                } else if term_data_vec[0] == "Interest".to_string()
                    && term_data_vec[1] == "POS".to_string()
                {
                    cf_amt = vals[1].to_string();
                    component = "INTEREST".to_string();
                    due_dt = vals[2].to_string();
                } else if term_data_vec[0] == "Interest".to_string()
                    && term_data_vec[1] == "NEG".to_string()
                {
                    let cf_amt_val: f64 = vals[1].parse().unwrap();
                    cf_amt = (0.0 - cf_amt_val).to_string();
                    component = "INTEREST".to_string();
                    due_dt = vals[2].to_string();
                } else if term_data_vec[0] == "".to_string() || term_data_vec[1] == "".to_string() {
                    error!(
                        log,
                        "Cannot get {} from Term Loans Update Type Master file", term_data_vec[0]
                    );
                }
                let amt = cf_amt.parse::<f64>();

                if !amt.is_err() {
                    tot_amt_op += cf_amt.parse::<f64>().unwrap();
                } else {
                    error!(log, "Amt could not be parsed in f64");
                }

                let mut cust_name = fields[6];
                if n1_to_cat.contains_key(fields[2]) {
                    cust_name = n1_to_cat.get(fields[2]).unwrap();
                }

                let mut prod_cd = "";
                if fields[1].eq("Term Loan") {
                    prod_cd = "10C";
                } else if fields[1].eq("WCDL") {
                    prod_cd = "10B";
                }

                let mut bucket_category = "";
                if fields[1] == "10G" {
                    bucket_category = "NHB"
                } else {
                    bucket_category = "Bank"
                }

                let mut nxt_repricing_dt = fields[18];
                if fields[18].chars().count() == 0 {
                    nxt_repricing_dt = fields[10];
                }

                output_line.push_str(&format!(
                        "{}|{}|{}||{}||||{}|{}||{}||{}|{}|{}|||{}|{}|{}|{}|{}|||{}|||||||||{}|{}|{}|{}|{}|{}|||{}|{}\n",
                        fields[0],
                        fields[0],
                        cust_name,
                        fields[11],
                        fields[10],
                        &due_dt,
                        prod_cd,
                        fields[8],
                        fields[7],
                        &component,
                        &cf_amt,
                        fields[13],
                        bucket_category,
                        "SECURED",
                        fields[1],
                        fields[15],
                        fields[19],
                        nxt_repricing_dt,
                        fields[17],
                        &config_param.as_on_date().format("%d-%m-%Y").to_string(),
                        fields[23],
                        fields[12],
                        fields[1],
                        fields[2]
                    ));
            }
        } else {
            cf_amt = fields[7].to_string();
            component = "PRINCIPAL".to_string();
            due_dt = fields[10].to_string();

            let amt = cf_amt.parse::<f64>();

            if !amt.is_err() {
                tot_amt_op += cf_amt.parse::<f64>().unwrap();
            } else {
                error!(log, "Amt could not be parsed in f64");
            }

            let mut cust_name = fields[6];
            if n1_to_cat.contains_key(fields[2]) {
                cust_name = n1_to_cat.get(fields[2]).unwrap();
            }

            let mut prod_cd = "";
            if fields[1].eq("Term Loan") {
                prod_cd = "10C";
            } else if fields[1].eq("WCDL") {
                prod_cd = "10B";
            }

            let mut bucket_category = "";
            if fields[1] == "10G" {
                bucket_category = "NHB"
            } else {
                bucket_category = "Bank"
            }

            let mut nxt_repricing_dt = fields[18];
            if fields[18].chars().count() == 0 {
                nxt_repricing_dt = fields[10];
            }

            output_line.push_str(&format!(
                    "{}|{}|{}||{}||||{}|{}||{}||{}|{}|{}|||{}|{}|{}|{}|{}|||{}|||||||||{}|{}|{}|{}|{}|{}|||{}|{}\n",
                    fields[0],
                    fields[0],
                    cust_name,
                    fields[11],
                    fields[10],
                    &due_dt,
                    prod_cd,
                    fields[8],
                    fields[7],
                    &component,
                    &cf_amt,
                    fields[13],
                    bucket_category,
                    "SECURED",
                    fields[1],
                    fields[15],
                    fields[19],
                    nxt_repricing_dt,
                    fields[17],
                    &config_param.as_on_date().format("%d-%m-%Y").to_string(),
                    fields[23],
                    fields[12],
                    fields[1],
                    fields[2]
                ));
        }
    }
    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);

    let start_write_timer = SystemTime::now();
    let mut writer = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            config_param.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };

    match writer.write_all(output_line.as_bytes()) {
        Ok(_) => println!("Successfully processed all accounts."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`: {}.",
            config_param.output_file_path(),
            error,
        ),
    }

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd,
        0,
        tot_amt_ip,
        tot_amt_op,
        tot_no_cf,
    );

    health_report.gen_health_rpt(&config_param.output_file_path());

    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing pre-processed output.");
    debug!(
        diag_log,
        "Writing Records and Reconcilation File, Total Duration: {:?}.", duration
    );
}
