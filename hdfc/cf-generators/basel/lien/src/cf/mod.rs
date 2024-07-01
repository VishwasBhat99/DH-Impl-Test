mod account_as_cashflows;
mod account_writer;
mod currency;
mod data_appender;

use self::data_appender::append_data;
use calamine::{open_workbook_auto, Reader};
use cf::account_writer::AccountWriter;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;

pub fn derive(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let currency_converter = currency::create_currency_converter(
        config_params.base_currency(),
        config_params.currency_conversion_file_path(),
    );

    // read cust master file
    let cust_master_file = match new_buf_rdr(config_params.cust_type_ref_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.cust_type_ref_path(),
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
        let fields: Vec<&str> = line.split(',').collect();
        cust_master.insert(fields[0].to_string(), fields[1].to_string());
    }
    // init writer
    let mut writer = AccountWriter::new(config_params.output_file_path(), logger);

    let mut tot_rec = 0;
    let mut skp_rec = 0;
    let mut tot_amt = 0.0;
    let mut lien_excel =
        open_workbook_auto(config_params.input_file_path()).expect("Cannot open input file.");
    if let Some(Ok(reader)) = lien_excel.worksheet_range(&config_params.input_sheet_name()) {
        for row in reader.rows() {
            tot_rec += 1;
            log_debug!(diag_logger, "{:#?}", row);
            let acc_id = row[*config_params.acc_id_col_id()].to_string();
            let line_exp_date = match rbdate::NaiveDate::parse_from_str(
                &row[*config_params.exp_date_col_id()].to_string(),
                "%d-%b-%Y %T",
            ) {
                Ok(val) => val,
                Err(_) => {
                    match rbdate::datevalue_to_naive_date(
                        &row[*config_params.exp_date_col_id()].to_string(),
                    ) {
                        Ok(val) => val,
                        Err(err) => {
                            match rbdate::NaiveDate::parse_from_str(
                                &row[*config_params.exp_date_col_id()].to_string(),
                                "%d-%b-%Y",
                            ) {
                                Ok(val) => val,
                                Err(err) => {
                                    log_error!(
                                    logger,
                                    "Cannot read expiry date --{}-- from input file: {}, Hence skipping account: {}",
                                    row[*config_params.exp_date_col_id()],
                                    err,
                                    acc_id
                                );
                                    skp_rec += 1;
                                    continue;
                                }
                            }
                        }
                    }
                }
            };
            let os_amt = row[*config_params.os_col_id()]
                .to_string()
                .parse::<f64>()
                .expect("Cannot read OS amount field from input file.");
            let fd_amt = row[*config_params.fd_amt_col_id()]
                .to_string()
                .parse::<f64>()
                .expect("Cannot read FD amount field from input file.");
            let os_amt_lcy;
            let fd_amt_lcy;
            if config_params.input_currency() != config_params.base_currency() {
                os_amt_lcy =
                    currency_converter.get_consol_data(config_params.input_currency(), &os_amt);
                fd_amt_lcy =
                    currency_converter.get_consol_data(config_params.input_currency(), &fd_amt);
            } else {
                os_amt_lcy = os_amt;
                fd_amt_lcy = fd_amt;
            }
            let tenor = if &line_exp_date > config_params.as_on_date() {
                rbdate::num_days_start_to_end(*config_params.as_on_date(), line_exp_date)
            } else {
                0
            };
            let amt_to_be_considered = if tenor > *config_params.residual_maturity_days() && os_amt_lcy > 0.0 {
                if fd_amt_lcy > os_amt_lcy {
                    os_amt_lcy
                } else {
                    fd_amt_lcy
                }
            } else {
                0.0
            };
            tot_amt += os_amt_lcy;

            let cust_typ_key = row[*config_params.cust_typ_col_id()].to_string();
            let cust_type = match cust_master.get(&cust_typ_key) {
                Some(val) => val.trim().to_string(),
                None => "NA".to_string(),
            };
            let ccy = if *config_params.act_ccy_col_id() == 99 {
                "INR".to_string()
            } else {
                row[*config_params.act_ccy_col_id()].to_string()
            };
            let account_data = append_data(
                acc_id,
                ccy,
                amt_to_be_considered,
                tenor,
                cust_type,
                &config_params,
            );
            writer.write(account_data);
        }
        writer.close();
    } else {
        log_info!(
            logger,
            "Sheet name mismatched! No sheet found with name - `{}`",
            config_params.input_sheet_name()
        );
        panic!("Unable to match configured sheet - `{}`.", config_params.input_sheet_name());
    }
    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(logger, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file_path());
}
