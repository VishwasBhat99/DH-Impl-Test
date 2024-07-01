use configuration_parameters::ConfigurationParameters;
use macros;
use process::account_field_names::AccFieldNames;
use process::account_reader::input_account::MasterAccount;
use process::account_with_cashflows_writer::AccountWithCashflowsWriter;
use process::create_account::{
    create_account_from_master, create_account_with_cashflows, get_data_from_cf, get_data_from_txt,
};
use process::structs::*;
use rbdate::NaiveDate;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::{get_field_value, AccountWithCFs};
use sdb_dyn_proto_rdr::reader::Reader;
use sdb_io::buf_file_wrtr;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::io::{BufRead, Write};
mod account_field_names;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod create_account;
mod structs;

mod account_reader;

pub fn classify(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let as_on_date = config_params.as_on_date();
    let as_on_format = as_on_date.format("%d%m%Y");
    let mut input_reader: Reader = reader::Reader::new_at_path(
        config_params.metadata_file_path(),
        config_params.input_file_path(),
    );
    let field_reader: Reader = reader::Reader::new_at_path(
        config_params.metadata_file_path(),
        config_params.input_file_path(),
    );

    let keys = AccFieldNames::new_from_path(config_params.req_fields_file());
    let mut master_map: HashMap<String, MasterAccount> = HashMap::new();

    let master_reader = match new_buf_rdr(config_params.master_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}`.",
            config_params.master_file_path(),
            error
        ),
    };
    for (line_num, lines) in master_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    logger,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_params.master_file_path(),
                    line_num + 1,
                    error
                );
                "".to_string()
            }
        };

        let fields: Vec<&str> = line.split('|').collect();
        master_map.insert(
            fields[0].to_string(),
            MasterAccount {
                account_number: fields[0].to_string(),
                account_id: fields[1].to_string(),
                as_on_date: NaiveDate::parse_from_str(fields[2], "%d-%m-%Y")
                    .unwrap_or(config_params.as_on_date()),
                acc_open_date: NaiveDate::parse_from_str(fields[3], "%d-%m-%Y")
                    .unwrap_or(config_params.as_on_date()),
                acc_crncy_code: fields[4].to_string(),
                out_bal: fields[5].to_string().parse::<f64>().unwrap_or(0.0),
                out_bal_lcy: fields[6].to_string().parse::<f64>().unwrap_or(0.0),
                maturity_date: NaiveDate::parse_from_str(fields[7], "%d-%m-%Y")
                    .unwrap_or(config_params.as_on_date()),
                interest_rate: fields[8].to_string().parse::<f64>().unwrap_or(0.0),
                next_reprise_date: NaiveDate::parse_from_str(fields[9], "%d-%m-%Y")
                    .unwrap_or(config_params.as_on_date()),
                last_reprise_date: NaiveDate::parse_from_str(fields[10], "%d-%m-%Y")
                    .unwrap_or(config_params.as_on_date()),
                gl_code: fields[11].to_string(),
                scheme_code: fields[12].to_string(),
                customer_id: fields[13].to_string(),
                customer_type: fields[14].to_string(),
                cust_const_code: fields[15].to_string(),
                customer_name: fields[16].to_string(),
                tot_int_amt: fields[17].to_string().parse::<f64>().unwrap_or(0.0),
                total_prin_amt: fields[18].to_string().parse::<f64>().unwrap_or(0.0),
                acct_type: fields[19].to_string(),
                pt_f64_1: fields[20].to_string().parse::<f64>().unwrap_or(0.0),
                pt_f64_2: fields[21].to_string().parse::<f64>().unwrap_or(0.0),
                pt_f64_3: fields[22].to_string().parse::<f64>().unwrap_or(0.0),
                pt_f64_4: fields[23].to_string().parse::<f64>().unwrap_or(0.0),
                pt_i64_1: fields[24].to_string().parse::<i64>().unwrap_or(0),
                pt_i64_2: fields[25].to_string().parse::<i64>().unwrap_or(0),
                pt_i64_3: fields[26].to_string().parse::<i64>().unwrap_or(0),
                pt_i64_4: fields[27].to_string().parse::<i64>().unwrap_or(0),
                pt_str_1: fields[28].to_string(),
                pt_str_2: fields[29].to_string(),
                pt_str_3: fields[30].to_string(),
                pt_str_4: fields[31].to_string(),
                cashflows: fields[32].to_string(),
            },
        );
    }

    let mut acc_enc = 0;
    let mut acc_succ = 0;
    //Initialize writers:
    let mut incremental_writer = AccountWithCashflowsWriter::new(
        config_params.output_file_path(),
        "incremental",
        as_on_format.to_string(),
        logger,
    );

    let mut renewal_writer = AccountWithCashflowsWriter::new(
        config_params.output_file_path(),
        "renewal",
        as_on_format.to_string(),
        logger,
    );
    let mut pre_matured_writer = AccountWithCashflowsWriter::new(
        config_params.output_file_path(),
        "pre-matured",
        as_on_format.to_string(),
        logger,
    );
    let mut matured_writer = AccountWithCashflowsWriter::new(
        config_params.output_file_path(),
        "matured",
        as_on_format.to_string(),
        logger,
    );
    let mut master_writer = match buf_file_wrtr(config_params.master_file_path(), None) {
        Ok(wrtr) => wrtr,
        Err(_) => {
            panic!(
                "Could not create file: `{}`.",
                config_params.master_file_path()
            );
        }
    };
    for mut account in input_reader.iter() {
        acc_enc += 1;
        // Get account no
        let account_number =
            get_field_value(&account, &field_reader, keys.account_number.to_owned())
                .unwrap_or("NA".to_string());
        // Get maturity date.
        let maturity_date = naivedate_from_timestamp(
            account
                .get_i64_for_key(&keys.maturity_date.to_owned())
                .unwrap_or(0),
        );
        let (account, cashflows) = get_data(
            &field_reader,
            &mut account,
            &keys,
            account_number.to_owned(),
            maturity_date.to_owned(),
            as_on_date,
        );
        let mut account_with_cashflows =
            create_account_with_cashflows(account.to_owned(), cashflows.to_owned(), as_on_date);
        //Classify accounts.
        match master_map.get_mut(&account_number) {
            Some(master_data) => {
                //Compare the maturity dates.
                if maturity_date > master_data.maturity_date {
                    //Write to renewal accounts.
                    account_with_cashflows.acct_type = "TDA_ren".to_string();
                    renewal_writer.write(account_with_cashflows);
                    log_debug!(
                        logger,
                        "Account:{} written to RENEWAL file.",
                        account_number
                    );
                    acc_succ += 1;
                    //Update the maturity date and write to master file.
                    master_data.maturity_date = maturity_date;
                    master_data.acct_type = "TDA_ren".to_string();
                } else if maturity_date == as_on_date {
                    //Write to the matured accounts file.
                    account_with_cashflows.acct_type = "TDA_mat".to_string();
                    matured_writer.write(account_with_cashflows);
                    log_debug!(
                        logger,
                        "Account:{} written to MATURED file.",
                        account_number
                    );
                    acc_succ += 1;
                    master_data.maturity_date = maturity_date;
                    master_data.acct_type = "TDA_mat".to_string();
                } else {
                    log_info!(logger, "Account :{} in input file skipped.", account_number);
                }
                let master_file_data = get_data_from_txt(master_data);
                master_writer
                    .write_all(master_file_data.as_bytes())
                    .expect("Cannot write to master file.");
                log_debug!(
                    logger,
                    "Account:{} written to master file(Renewal/Maturity).",
                    account_number
                );
                //Remove the record from hashmap as it will no longer be looked up.
                master_map.remove(&account_number);
            }
            None => {
                //Write to incremental file.
                account_with_cashflows.acct_type = "TDA_new".to_string();
                incremental_writer.write(account_with_cashflows);
                log_debug!(
                    logger,
                    "Account:{} written to INCREMENTAL file.",
                    account_number
                );
                acc_succ += 1;
                //Write to master file.
                let master_file_data = get_data_from_cf(account, cashflows);
                master_writer
                    .write_all(master_file_data.as_bytes())
                    .expect("Cannot write to master file.");
                log_debug!(
                    logger,
                    "Account:{} written to master file(Incremental).",
                    account_number
                );
            }
        };
    }
    //Write the remaining record in master map as pre-matured.
    for (acc_no, master_data) in master_map.iter_mut() {
        //Write to pre-matured file.
        master_data.acct_type = "TDA_pre".to_string();
        let (account, cashflows) = get_master_data(master_data, as_on_date);
        let account_with_cashflows = create_account_from_master(account, cashflows, as_on_date);
        pre_matured_writer.write(account_with_cashflows);
        log_debug!(logger, "Account:{} written to PRE-MATURED file.", acc_no);
        //Write to master file.
        let master_file_data = get_data_from_txt(&mut master_data.to_owned());
        master_writer
            .write_all(master_file_data.as_bytes())
            .expect("Cannot write to master file.");
        log_debug!(
            logger,
            "Account:{} written to master file(Pre-matured).",
            acc_no
        );
    }
    log_info!(logger, "Total account read from input file: {}", acc_enc);
    log_info!(
        logger,
        "Total account written to output files: {}",
        acc_succ
    );
}
fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = rbdate::NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}
