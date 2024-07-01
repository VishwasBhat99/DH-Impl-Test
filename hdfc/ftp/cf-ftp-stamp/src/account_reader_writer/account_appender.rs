use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;

use rbdate::timestamp;
use statics::*;

pub fn create_account_without_cashflows(account: InputAccount) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.account_number = account.account_number;
    out_acc.cust_name = account.cust_name;
    out_acc.average_balance = account.average_balance;
    out_acc.accr_int = account.accr_int;
    out_acc.yld_to_call = account.yld_to_call;
    out_acc.interest_rate = account.interest_rate;
    out_acc.base_rate_1 = account.base_rate_1;
    out_acc.final_ftp_rate = account.final_ftp_rate;
    out_acc.value_date = if let Some(date) = account.value_date {
        timestamp(date)
    } else {
        DEFAULT_INT
    };
    out_acc.maturity_date = if let Some(date) = account.maturity_date {
        timestamp(date)
    } else {
        DEFAULT_INT
    };
    out_acc.next_reprice_date = if let Some(date) = account.next_reprice_date {
        timestamp(date)
    } else {
        DEFAULT_INT
    };
    out_acc.last_reprice_date = if let Some(date) = account.last_reprice_date {
        timestamp(date)
    } else {
        DEFAULT_INT
    };
    out_acc.mis1 = account.mis1;
    out_acc.mis2 = account.mis2;
    out_acc.psl_code = account.psl_code;
    out_acc.prod_code_type = account.prod_code_type;
    out_acc.rate_flag = account.rate_flag;
    out_acc.blank_1 = account.blank_1;
    out_acc.source_file_name = account.source_file_name;
    out_acc.currency = account.currency;
    out_acc.gl = account.gl;
    out_acc.cust_id = account.cust_id;
    out_acc.final_ftp_amount = account.final_ftp_amount;
    out_acc.alm_line = account.alm_line;
    out_acc.blank_2 = account.blank_2;
    out_acc.initial_dep_amt_td = account.initial_dep_amt_td;
    out_acc.current_outstanding_td = account.current_outstanding_td;
    out_acc.base_rate_2 = account.base_rate_2;
    out_acc.adj1 = account.adj1;
    out_acc.adj2 = account.adj2;
    out_acc.adj3 = account.adj3;
    out_acc.adj4 = account.adj4;
    out_acc.adj5 = account.adj5;
    out_acc.adj6 = account.adj6;
    out_acc.input_benchmark = account.input_benchmark;
    out_acc.pdo = account.pdo;
    out_acc.npa = account.npa;
    out_acc.ftp_method = account.ftp_method;
    out_acc.ftp_rate_curve = account.ftp_rate_curve;
    out_acc.org_tenor = account.org_tenor;
    out_acc.repricing_tenor = account.repricing_tenor;
    out_acc.fixed_spread = account.fixed_spread;
    out_acc.variable_spread = account.variable_spread;
    out_acc.first_month_ftp = account.first_month_ftp;
    out_acc.bc_as_on_rule = if let Some(date) = account.bc_as_on_rule {
        timestamp(date)
    } else {
        DEFAULT_INT
    };
    out_acc.tenor_start_date_rule = if let Some(date) = account.tenor_start_date_rule {
        timestamp(date)
    } else {
        DEFAULT_INT
    };
    out_acc.tenor_end_date_rule = if let Some(date) = account.tenor_end_date_rule {
        timestamp(date)
    } else {
        DEFAULT_INT
    };
    out_acc.bc_as_on_applied = if let Some(date) = account.bc_as_on_applied {
        timestamp(date)
    } else {
        DEFAULT_INT
    };
    out_acc.tenor_start_date_applied = if let Some(date) = account.tenor_start_date_applied {
        timestamp(date)
    } else {
        DEFAULT_INT
    };
    out_acc.tenor_end_date_applied = if let Some(date) = account.tenor_end_date_applied {
        timestamp(date)
    } else {
        DEFAULT_INT
    };

    out_acc
}
