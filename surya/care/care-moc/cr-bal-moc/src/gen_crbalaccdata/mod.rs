use calamine::{open_workbook, Reader, Xlsx};
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use gen_crbalaccdata::structs::Account;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use statics::{DEFAULT_DATE, DEFAULT_FLOAT, DEFAULT_INT};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufWriter, Write};
mod structs;

pub fn gen_crbaldata(
    config_params: ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
) {
    let exchange_rates = get_exchange_rates(
        config_params.currency_conversion_file_path(),
        config_params.base_currency(),
    );
    let mut input_reader: Xlsx<_> = open_workbook(config_params.input_file_path())
        .expect("Error while opening `product report file`.");
    let mut output_wtr = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(r) => r,
        Err(e) => panic!(
            "Cannot write to file at path: '{}', Error: '{}'",
            config_params.output_file_path(),
            e
        ),
    };
    if let Some(Ok(reader)) = input_reader.worksheet_range(config_params.sheet_name()) {
        for row in reader.rows().skip(1) {
            let mut account = Account::get_from_line(row);
            account.exchnage_rt = *exchange_rates.get(&account.ccyid).unwrap_or(&1.0);
            let ost_bal_ccy = convert_ccy(account.out_stand_bal_hcy, account.exchnage_rt);
            let rw_out_bal_ccy = convert_ccy(account.credit_equi_hcy, account.exchnage_rt);
            let output_line = get_output_line(
                &account,
                ost_bal_ccy,
                rw_out_bal_ccy,
                config_params.as_on_date(),
                config_params.sheet_name(),
            );
            write_output(output_line.as_str(), &mut output_wtr, logger);
        }
    }
}

fn get_output_line(
    account: &Account,
    ost_bal_ccy: f64,
    rw_out_bal_ccy: f64,
    as_on_date: &NaiveDate,
    sheet_name: &str,
) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|||{}\
    ||||{}|{}|{}||||||||9999|9999|9999|||{}|\
    {}|{}||||||9999|9999|{}|{}|{}|{}|0.0|{}|{}|0.0|0.0|0.0|0.0|0.0|\
    {}|{}|{}|{}|{}|||||\n",
        as_on_date.format("%d-%m-%Y"),
        account.moc_id,
        sheet_name,
        account.claim_id,
        ost_bal_ccy,
        account.out_stand_bal_hcy,
        account.ccyid,
        account.exchnage_rt,
        account.glcd1,
        account.glcd1,
        account.glcd1,
        account.mod_desc,
        DEFAULT_DATE,
        DEFAULT_DATE,
        DEFAULT_DATE,
        account.claim_id,
        account.claim_id,
        account.rw_perc,
        account.rw_perc,
        account.credit_equi_hcy,
        rw_out_bal_ccy,
        account.prov_amt_hcy,
        account.crm_amt_hcy,
        account.final_rw_amt_hcy,
        DEFAULT_DATE,
        DEFAULT_DATE,
        DEFAULT_DATE,
        DEFAULT_DATE,
        DEFAULT_DATE
    )
}

pub fn str_to_flt(num: &str) -> f64 {
    num.parse().unwrap_or(DEFAULT_FLOAT)
}
pub fn str_to_int(num: &str) -> i64 {
    num.parse().unwrap_or(DEFAULT_INT)
}
pub fn get_exchange_rates(exchange_rate_file: &str, base_currency: &str) -> HashMap<String, f64> {
    let mut exchanges_rates: HashMap<String, f64> = HashMap::new();
    let rdr = match new_buf_rdr(exchange_rate_file) {
        Ok(r) => r,
        Err(e) => panic!(
            "Cannot read file at path: '{}', Error: '{}'",
            exchange_rate_file, e
        ),
    };
    for line in rdr.lines() {
        if let Ok(each_line) = line {
            let line_contents: Vec<&str> = each_line.split("|").collect();
            if line_contents.len() < 3 {
                continue;
            }
            if line_contents[0].eq(base_currency) {
                exchanges_rates.insert(line_contents[1].to_string(), str_to_flt(line_contents[2]));
            }
        }
    }
    exchanges_rates
}
fn convert_ccy(amount: f64, ex_rt: f64) -> f64 {
    amount / ex_rt
}

fn write_output(data: &str, writer: &mut BufWriter<File>, logger: &Logger) {
    match writer.write_all(data.as_bytes()) {
        Ok(_) => {}
        Err(e) => log_error!(logger, "unable write output to output file: `{}`", e),
    }
}
