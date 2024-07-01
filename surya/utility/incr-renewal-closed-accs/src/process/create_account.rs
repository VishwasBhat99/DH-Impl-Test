use super::{account_reader::input_account::MasterAccount, naivedate_from_timestamp};
use crate::process::account_reader::input_account::InputAccount;
use process::account_with_cashflows::{AccountWithCashflows, Cashflow};
use rbdate::{timestamp, NaiveDate};

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
    as_on_date: NaiveDate,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    out_acc.account_number = account.account_number;
    out_acc.account_id = account.account_id;
    out_acc.as_on_date = timestamp(as_on_date);
    out_acc.acc_open_date = timestamp(account.acc_open_date);
    out_acc.acc_crncy_code = account.acc_crncy_code;
    out_acc.out_bal = account.out_bal;
    out_acc.out_bal_lcy = account.out_bal_lcy;
    out_acc.maturity_date = timestamp(account.maturity_date);
    out_acc.interest_rate = account.interest_rate;
    out_acc.next_reprise_date = timestamp(account.next_reprise_date);
    out_acc.last_reprise_date = timestamp(account.last_reprise_date);
    out_acc.gl_code = account.gl_code;
    out_acc.scheme_code = account.scheme_code;
    out_acc.customer_id = account.customer_id;
    out_acc.customer_type = account.customer_type;
    out_acc.cust_const_code = account.cust_const_code;
    out_acc.customer_name = account.customer_name;
    out_acc.tot_int_amt = account.total_int_amt;
    out_acc.total_prin_amt = account.total_prin_amt;
    out_acc.pt_f64_1 = account.pt_f64_1;
    out_acc.pt_f64_2 = account.pt_f64_2;
    out_acc.pt_f64_3 = account.pt_f64_3;
    out_acc.pt_f64_4 = account.pt_f64_4;
    out_acc.pt_i64_1 = account.pt_i64_1;
    out_acc.pt_i64_2 = account.pt_i64_2;
    out_acc.pt_i64_3 = account.pt_i64_3;
    out_acc.pt_i64_4 = account.pt_i64_4;
    out_acc.pt_str_1 = account.pt_str_1;
    out_acc.pt_str_2 = account.pt_str_2;
    out_acc.pt_str_3 = account.pt_str_3;
    out_acc.pt_str_4 = account.pt_str_4;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
pub fn create_account_from_master(
    account: MasterAccount,
    cashflows: Vec<Cashflow>,
    as_on_date: NaiveDate,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    out_acc.account_number = account.account_number;
    out_acc.account_id = account.account_id;
    out_acc.as_on_date = timestamp(as_on_date);
    out_acc.acc_open_date = timestamp(account.acc_open_date);
    out_acc.acc_crncy_code = account.acc_crncy_code;
    out_acc.out_bal = account.out_bal;
    out_acc.out_bal_lcy = account.out_bal_lcy;
    out_acc.maturity_date = timestamp(account.maturity_date);
    out_acc.interest_rate = account.interest_rate;
    out_acc.next_reprise_date = timestamp(account.next_reprise_date);
    out_acc.last_reprise_date = timestamp(account.last_reprise_date);
    out_acc.gl_code = account.gl_code;
    out_acc.scheme_code = account.scheme_code;
    out_acc.customer_id = account.customer_id;
    out_acc.customer_type = account.customer_type;
    out_acc.cust_const_code = account.cust_const_code;
    out_acc.customer_name = account.customer_name;
    out_acc.tot_int_amt = account.tot_int_amt;
    out_acc.acct_type = account.acct_type;
    out_acc.total_prin_amt = account.total_prin_amt;
    out_acc.pt_f64_1 = account.pt_f64_1;
    out_acc.pt_f64_2 = account.pt_f64_2;
    out_acc.pt_f64_3 = account.pt_f64_3;
    out_acc.pt_f64_4 = account.pt_f64_4;
    out_acc.pt_i64_1 = account.pt_i64_1;
    out_acc.pt_i64_2 = account.pt_i64_2;
    out_acc.pt_i64_3 = account.pt_i64_3;
    out_acc.pt_i64_4 = account.pt_i64_4;
    out_acc.pt_str_1 = account.pt_str_1;
    out_acc.pt_str_2 = account.pt_str_2;
    out_acc.pt_str_3 = account.pt_str_3;
    out_acc.pt_str_4 = account.pt_str_4;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}

pub fn get_data_from_txt(account: &mut MasterAccount) -> String {
    //Format cashflows:
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        account.account_number,
        account.account_id,
        account.as_on_date.format("%d-%m-%Y"),
        account.acc_open_date.format("%d-%m-%Y"),
        account.acc_crncy_code,
        account.out_bal,
        account.out_bal_lcy,
        account.maturity_date.format("%d-%m-%Y"),
        account.interest_rate,
        account.next_reprise_date.format("%d-%m-%Y"),
        account.last_reprise_date.format("%d-%m-%Y"),
        account.gl_code,
        account.scheme_code,
        account.customer_id,
        account.customer_type,
        account.cust_const_code,
        account.customer_name,
        account.tot_int_amt,
        account.total_prin_amt,
        account.acct_type,
        account.pt_f64_1,
        account.pt_f64_2,
        account.pt_f64_3,
        account.pt_i64_4,
        account.pt_i64_1,
        account.pt_i64_2,
        account.pt_i64_3,
        account.pt_i64_4,
        account.pt_str_1,
        account.pt_str_2,
        account.pt_str_3,
        account.pt_str_4,
        account.cashflows
    )
}

pub fn get_data_from_cf(account: InputAccount, cashflows: Vec<Cashflow>) -> String {
    //Format cashflows:
    let mut cf_string = String::new();
    for cf in cashflows {
        let data = format!(
            "{},{},{},",
            cf.interest_amount,
            cf.principal_amount,
            naivedate_from_timestamp(cf.date).format("%d-%m-%Y")
        );
        cf_string.push_str(&data);
    }
    cf_string.pop();
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|TDA_new|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        account.account_number,
        account.account_id,
        account.as_on_date.format("%d-%m-%Y"),
        account.acc_open_date.format("%d-%m-%Y"),
        account.acc_crncy_code,
        account.out_bal,
        account.out_bal_lcy,
        account.maturity_date.format("%d-%m-%Y"),
        account.interest_rate,
        account.next_reprise_date.format("%d-%m-%Y"),
        account.last_reprise_date.format("%d-%m-%Y"),
        account.gl_code,
        account.scheme_code,
        account.customer_id,
        account.customer_type,
        account.cust_const_code,
        account.customer_name,
        account.total_int_amt,
        account.total_prin_amt,
        account.pt_f64_1,
        account.pt_f64_2,
        account.pt_f64_3,
        account.pt_i64_4,
        account.pt_i64_1,
        account.pt_i64_2,
        account.pt_i64_3,
        account.pt_i64_4,
        account.pt_str_1,
        account.pt_str_2,
        account.pt_str_3,
        account.pt_str_4,
        cf_string
    )
}
