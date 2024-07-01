use self::structs::*;
use super::derive_cashflow::get_cashflows;
use super::structs;
use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_with_cashflows::Account;
use chrono::NaiveDate;
use rbdate::timestamp;
use slog::Logger;
use statics::*;

pub fn create_account_with_cashflows(
    cashflow_vec: &Vec<CashflowData>,
    inp_acc: InputAccount,
    as_on_date:&NaiveDate,
    log: &Logger,
) -> Account {
    let mut out_acc = Account::new();
    out_acc.customer_id = inp_acc.customer_id.to_string();
    out_acc.cod_acct_no = inp_acc.cod_acct_no.to_string();
    out_acc.ccy_code = inp_acc.ccy_code.to_string();
    out_acc.gl_account_principal = inp_acc.gl_account_principal;
    out_acc.acct_open_date = if let Some(dt) = inp_acc.acct_open_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.maturity_date = if let Some(dt) = inp_acc.maturity_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.eop_balance = inp_acc.eop_balance;
    out_acc.index_rate = inp_acc.index_rate;
    out_acc.net_rate = inp_acc.net_rate;
    out_acc.benchmark_name = inp_acc.benchmark_name.to_string();
    out_acc.foreclosure = inp_acc.foreclosure;
    out_acc.foreclosure_rate_1 = inp_acc.foreclosure_rate_1;
    out_acc.foreclosure_rate_2 = inp_acc.foreclosure_rate_2;
    out_acc.index_code = inp_acc.index_code.to_string();
    out_acc.rate_type = inp_acc.rate_type.to_string();
    out_acc.reset_frequency = inp_acc.reset_frequency.to_string();
    out_acc.npa_status_final = inp_acc.npa_status_final.to_string();
    out_acc.derived_reset_date = if let Some(dt) = inp_acc.derived_reset_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.old_prin_os_bal = inp_acc.eop_balance;
    out_acc.add_string_1 = inp_acc.add_string_1.to_string();
    out_acc.add_string_2 = inp_acc.add_string_2.to_string();
    out_acc.add_int_1 = inp_acc.add_int_1 as i64;
    out_acc.add_int_2 = inp_acc.add_int_2 as i64;
    out_acc.add_double_1 = inp_acc.add_float_1;
    out_acc.add_double_2 = inp_acc.add_float_2;
    out_acc.add_date_1 = if let Some(dt) = inp_acc.add_date_1 {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.add_date_2 = if let Some(dt) = inp_acc.add_date_2 {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };

    let cashflows = get_cashflows(cashflow_vec, &inp_acc, as_on_date,log);
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
