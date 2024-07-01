use super::account_reader::input_account::MasterAccount;
use super::naivedate_from_timestamp;
use process::account_field_names::AccFieldNames;
use process::account_reader::input_account::InputAccount;
use process::account_with_cashflows::Cashflow;
use process::AccountWithCFs;
use rbdate::{timestamp, NaiveDate};
use sdb_dyn_proto_rdr::reader::account_with_cfs::get_field_value;
use sdb_dyn_proto_rdr::reader::Reader;

pub fn get_data(
    method_reader: &Reader,
    account: &mut AccountWithCFs,
    keys: &AccFieldNames,
    acc_num: String,
    mat_dt: NaiveDate,
    as_on_date: NaiveDate,
) -> (InputAccount, Vec<Cashflow>) {
    let cashflows = get_cashflows(account, keys);
    let account_id = get_field_value(account, method_reader, keys.account_id.to_owned())
        .unwrap_or("NA".to_string());
    let acc_open_date = match account.get_i64_for_key(&keys.acc_open_date).unwrap_or(0) {
        0 => naivedate_from_timestamp(0),
        dt => naivedate_from_timestamp(dt),
    };
    let next_reprise_date = match account
        .get_i64_for_key(&keys.next_reprise_date)
        .unwrap_or(0)
    {
        0 => naivedate_from_timestamp(0),
        dt => naivedate_from_timestamp(dt),
    };
    let last_reprise_date = match account
        .get_i64_for_key(&keys.last_reprise_date)
        .unwrap_or(0)
    {
        0 => naivedate_from_timestamp(0),
        dt => naivedate_from_timestamp(dt),
    };
    let acc_crncy_code = get_field_value(account, method_reader, keys.acc_crncy_code.to_owned())
        .unwrap_or("NA".to_string());
    let out_bal = get_field_value(account, method_reader, keys.out_bal.to_owned())
        .unwrap_or("0.0".to_string())
        .parse::<f64>()
        .unwrap_or(0.0);
    let out_bal_lcy = get_field_value(account, method_reader, keys.out_bal_lcy.to_owned())
        .unwrap_or("0.0".to_string())
        .parse::<f64>()
        .unwrap_or(0.0);
    let interest_rate = get_field_value(account, method_reader, keys.interest_rate.to_owned())
        .unwrap_or("0.0".to_string())
        .parse::<f64>()
        .unwrap_or(0.0);
    let gl_code = get_field_value(account, method_reader, keys.gl_code.to_owned())
        .unwrap_or("NA".to_string());
    let scheme_code = get_field_value(account, method_reader, keys.scheme_code.to_owned())
        .unwrap_or("NA".to_string());
    let customer_id = get_field_value(account, method_reader, keys.customer_id.to_owned())
        .unwrap_or("NA".to_string());

    let customer_type = get_field_value(account, method_reader, keys.customer_type.to_owned())
        .unwrap_or("NA".to_string());
    let cust_const_code = get_field_value(account, method_reader, keys.cust_const_code.to_owned())
        .unwrap_or("NA".to_string());
    let customer_name = get_field_value(account, method_reader, keys.customer_name.to_owned())
        .unwrap_or("NA".to_string());
    let tot_int_amt = get_field_value(account, method_reader, keys.tot_int_amt.to_owned())
        .unwrap_or("0.0".to_string())
        .parse::<f64>()
        .unwrap_or(0.0);
    let tot_prin_amt = get_field_value(account, method_reader, keys.tot_prin_amt.to_owned())
        .unwrap_or("0.0".to_string())
        .parse::<f64>()
        .unwrap_or(0.0);
    let pt_f64_1 = get_field_value(account, method_reader, keys.pt_f64_1.to_owned())
        .unwrap_or("0.0".to_string())
        .parse::<f64>()
        .unwrap_or(0.0);
    let pt_f64_2 = get_field_value(account, method_reader, keys.pt_f64_2.to_owned())
        .unwrap_or("0.0".to_string())
        .parse::<f64>()
        .unwrap_or(0.0);
    let pt_f64_3 = get_field_value(account, method_reader, keys.pt_f64_3.to_owned())
        .unwrap_or("0.0".to_string())
        .parse::<f64>()
        .unwrap_or(0.0);
    let pt_f64_4 = get_field_value(account, method_reader, keys.pt_f64_4.to_owned())
        .unwrap_or("0.0".to_string())
        .parse::<f64>()
        .unwrap_or(0.0);
    let pt_i64_1 = get_field_value(account, method_reader, keys.pt_i64_1.to_owned())
        .unwrap_or("0".to_string())
        .parse::<i64>()
        .unwrap_or(0);
    let pt_i64_2 = get_field_value(account, method_reader, keys.pt_i64_2.to_owned())
        .unwrap_or("0".to_string())
        .parse::<i64>()
        .unwrap_or(0);
    let pt_i64_3 = get_field_value(account, method_reader, keys.pt_i64_3.to_owned())
        .unwrap_or("0".to_string())
        .parse::<i64>()
        .unwrap_or(0);
    let pt_i64_4 = get_field_value(account, method_reader, keys.pt_i64_4.to_owned())
        .unwrap_or("0".to_string())
        .parse::<i64>()
        .unwrap_or(0);
    let pt_str_1 = get_field_value(account, method_reader, keys.pt_str_1.to_owned())
        .unwrap_or("NA".to_string());
    let pt_str_2 = get_field_value(account, method_reader, keys.pt_str_2.to_owned())
        .unwrap_or("NA".to_string());
    let pt_str_3 = get_field_value(account, method_reader, keys.pt_str_3.to_owned())
        .unwrap_or("NA".to_string());
    let pt_str_4 = get_field_value(account, method_reader, keys.pt_str_4.to_owned())
        .unwrap_or("NA".to_string());

    (
        InputAccount {
            account_number: acc_num,
            account_id,
            as_on_date,
            acc_open_date,
            acc_crncy_code,
            out_bal,
            out_bal_lcy,
            maturity_date: mat_dt,
            interest_rate,
            next_reprise_date,
            last_reprise_date,
            gl_code,
            scheme_code,
            customer_id,
            customer_type,
            cust_const_code,
            customer_name,
            total_int_amt: tot_int_amt,
            total_prin_amt: tot_prin_amt,
            pt_f64_1,
            pt_f64_2,
            pt_f64_3,
            pt_f64_4,
            pt_i64_1,
            pt_i64_2,
            pt_i64_3,
            pt_i64_4,
            pt_str_1,
            pt_str_2,
            pt_str_3,
            pt_str_4,
            cashflows: "".to_string(),
        },
        cashflows,
    )
}

pub fn get_master_data(
    data: &mut MasterAccount,
    as_on_date: NaiveDate,
) -> (MasterAccount, Vec<Cashflow>) {
    let cashflows = get_master_cashflows(data.cashflows.to_owned());
    (
        MasterAccount {
            account_number: data.account_number.to_owned(),
            account_id: data.account_id.to_owned(),
            as_on_date,
            acc_open_date: data.acc_open_date,
            acc_crncy_code: data.acc_crncy_code.to_owned(),
            out_bal: data.out_bal,
            out_bal_lcy: data.out_bal_lcy,
            maturity_date: data.maturity_date,
            interest_rate: data.interest_rate,
            next_reprise_date: data.next_reprise_date,
            last_reprise_date: data.last_reprise_date,
            gl_code: data.gl_code.to_owned(),
            scheme_code: data.scheme_code.to_owned(),
            customer_id: data.customer_id.to_owned(),
            customer_type: data.customer_type.to_owned(),
            cust_const_code: data.cust_const_code.to_owned(),
            customer_name: data.customer_name.to_owned(),
            tot_int_amt: data.tot_int_amt,
            total_prin_amt: data.total_prin_amt,
            acct_type: data.acct_type.to_owned(),
            pt_f64_1: data.pt_f64_1,
            pt_f64_2: data.pt_f64_2,
            pt_f64_3: data.pt_f64_3,
            pt_f64_4: data.pt_f64_4,
            pt_i64_1: data.pt_i64_1,
            pt_i64_2: data.pt_i64_2,
            pt_i64_3: data.pt_i64_3,
            pt_i64_4: data.pt_i64_4,
            pt_str_1: data.pt_str_1.to_owned(),
            pt_str_2: data.pt_str_2.to_owned(),
            pt_str_3: data.pt_str_3.to_owned(),
            pt_str_4: data.pt_str_4.to_owned(),
            cashflows: "".to_string(),
        },
        cashflows,
    )
}

pub fn get_cashflows(account: &mut AccountWithCFs, keys: &AccFieldNames) -> Vec<Cashflow> {
    let cashflows = account
        .remove_cfs_for_key(&keys.cashflows)
        .expect("Error while removing cashflow from the pool of cashflows.");
    let mut cfs: Vec<Cashflow> = Vec::new();
    for cf in cashflows.iter() {
        let prin_amount = cf.get_principal_amount();
        let int_amount = cf.get_interest_amount();
        let cf_date = cf.get_date();
        let cashflow = new_cashflow(int_amount, prin_amount, cf_date);
        cfs.push(cashflow.to_owned());
    }
    cfs
}

pub fn get_master_cashflows(cashflows: String) -> Vec<Cashflow> {
    let cashflow_fields: Vec<&str> = cashflows.split(',').collect();
    let mut index = 0;
    let mut cfs: Vec<Cashflow> = Vec::new();
    while index < cashflow_fields.len() {
        let int_amt = cashflow_fields[index].parse::<f64>().unwrap_or(0.0);
        let prin_amt = cashflow_fields[index + 1].parse::<f64>().unwrap_or(0.0);
        let date = NaiveDate::parse_from_str(cashflow_fields[index + 2], "%d-%m-%Y")
            .expect("Could not parse cashflow date from master file.");
        let cashflow = new_cashflow(int_amt, prin_amt, timestamp(date));
        cfs.push(cashflow.to_owned());
        index += 3;
    }
    cfs
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;

    cf
}
