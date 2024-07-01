use self::account::Account;
use self::account::Cashflow;
use self::account_writer::AccountWithoutCashflows;
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::{date_from_timestamp, NaiveDate};
use slog::Logger;

mod account;
mod account_writer;
mod io;

pub fn process(config_param: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let mut writer = AccountWithoutCashflows::new(config_param.output_file_path(), log);
    let mut input_file: Xlsx<_> =
        open_workbook(config_param.input_file()).expect("Error while opening input file.");
    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;
    let mut tot_amt: f64 = 0.0;
    let mut is_header: bool = true;
    if let Some(Ok(reader)) = input_file.worksheet_range(config_param.sheet_name()) {
        for row in reader.rows() {
            tot_acc_encntrd += 1;
            if is_header && row[2].to_string().parse::<f64>().is_err() {
                is_header = false;
                continue;
            }
            acc_pro_suc += 1;
            let mut cf_vec = Vec::with_capacity(1);
            let mut account = Account::new();
            let amount = match row[2].to_string().parse::<f64>() {
                Ok(val) => val,
                Err(error) => {
                    log_error!(
                        log,
                        "Invalid amount field: `{:?}` for line number: `{}`: `{}`.",
                        row[2],
                        tot_acc_encntrd,
                        error
                    );
                    0.0
                }
            };
            tot_amt += amount;
            let int_rate = match row[3].to_string().parse::<f64>() {
                Ok(val) => val,
                Err(error) => {
                    log_error!(
                        log,
                        "Invalid interest field: `{:?}` for line number: `{}`: `{}`.",
                        row[3],
                        tot_acc_encntrd,
                        error
                    );
                    0.0
                }
            };
            let maturity_date;
            if row[4].to_string().is_empty() {
                maturity_date = NaiveDate::parse_from_str(config_param.as_on_date(), "%d-%m-%Y")
                    .expect("Cannot parse as on date as NaiveDate");
            } else {
                maturity_date =
                    if let Ok(dt) = NaiveDate::parse_from_str(&row[4].to_string(), "%d-%m-%Y") {
                        dt
                    } else if let Some(val_dt) = datevalue_to_naive_date(row[4].to_string()) {
                        val_dt
                    } else {
                        panic!("Cannot parse maturity date as NaiveDate.")
                    }
            }
            account.set_llg_id(row[0].to_string());
            account.set_currency(row[1].to_string());
            account.set_amount(amount);
            account.set_int_rate(int_rate);
            account.set_maturity_date(maturity_date.to_string());
            let cf = new_cashflow(maturity_date, amount, 0.0);
            cf_vec.push(cf);
            let cashflows = protobuf::RepeatedField::from_vec(cf_vec);
            account.set_cashflows(cashflows);
            writer.write(account);
            let health_report = HealthReport::new(
                tot_acc_encntrd,
                acc_pro_suc,
                tot_acc_encntrd - acc_pro_suc,
                tot_amt,
                tot_amt,
                0,
            );
            health_report.gen_health_rpt(&config_param.output_file_path());
        }
    }
    writer.close();
}

fn new_cashflow(d: NaiveDate, p: f64, i: f64) -> Cashflow {
    let mut cf = Cashflow::new();
    let dt_timestamp = rbdate::timestamp(d);
    cf.set_date(dt_timestamp);
    cf.set_principal_amount(p);
    cf.set_interest_amount(i);

    cf
}

fn datevalue_to_naive_date(date: String) -> Option<NaiveDate> {
    if let Ok(timestamp) = date.parse::<f64>() {
        Some(date_from_timestamp(((timestamp as i64) - 25569) * 86400))
    } else {
        None
    }
}
