use std::collections::HashMap;
use aggregate::aggr_key::Customer;
use ftp_parameters::FtpParameters;
use rbdate::{timestamp, NaiveDateTime};
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;
use health_report::HealthReport;

pub fn aggr_bal_with_no_lock(ftp_parameters: &mut FtpParameters) -> (String, String) {
    let mut total_account_with_cf = DEFAULT_INT;
    let mut total_accounts_currmnth = DEFAULT_INT;
    let mut total_num_cust_multiple = DEFAULT_INT;
    let mut total_bal_in = DEFAULT_FLOAT;
    let mut aggr: HashMap<Customer, f64> = HashMap::new();
    let mut aggr_count: HashMap<Customer, i64> = HashMap::new();
    let inputfields = &ftp_parameters.input_field_names;

    for account_input in ftp_parameters.input_data.iter() {
        total_account_with_cf += 1;

        let custid = match account_input.get_i64_for_key(&inputfields.customer_id) {
            Ok(result) => result,
            Err(e) => DEFAULT_INT,
        };

        let ccy = match account_input.get_string_for_key(&inputfields.institution) {
            Ok(result) => result,
            Err(e) => "",
        }
        .to_string();

        let start_dt = match account_input.get_i64_for_key(&inputfields.origination_date) {
            Ok(result) => result,
            Err(e) => DEFAULT_INT,
        };

        let mat_dt = match account_input.get_i64_for_key(&inputfields.maturity_date) {
            Ok(result) => result,
            Err(e) => DEFAULT_INT,
        };

        let amount = match account_input.get_f64_for_key(&inputfields.original_balance) {
            Ok(result) => result,
            Err(e) => DEFAULT_FLOAT,
        };

        total_bal_in += amount;

        total_accounts_currmnth += 1;
        let cust_key = Customer::new(custid, start_dt, mat_dt, ccy.to_string());
        let cust_key_copy = cust_key.clone();

        aggr.entry(cust_key)
            .and_modify(|val| *val += amount)
            .or_insert(amount);

        aggr_count
            .entry(cust_key_copy)
            .and_modify(|cnt| *cnt += 1)
            .or_insert(1);
    }

    let total_num_cust = aggr.len();

    let mut op_line = String::new();
    let comparevalue = 1;
    for (key, value) in &aggr_count {
        if value > &comparevalue {
            total_num_cust_multiple += 1;

            let amount = match aggr.get(&key) {
                Some(x) => x,
                None => &0.0,
            };
            let st_dt = NaiveDateTime::from_timestamp(key.start_dt, 0)
                .date()
                .format("%d-%m-%Y");
            let mat_dt = NaiveDateTime::from_timestamp(key.mat_dt, 0)
                .date()
                .format("%d-%m-%Y");
            let op = format!(
                "{}|{}|{}|{}|{:?}",
                key.cust_id, st_dt, mat_dt, key.ccy, amount
            );
            op_line.push_str(&op[..]);
            op_line.push_str("\n");
        }
    }

    let report_string = format!(
        "Accounts Encountered: {}\n\
         Accounts opened in current month: {}\n\
         Total number of Customers opened account in this month: {}\n\
         Total number of Customers with multiple accounts opened in this month: {:?} \n\
         Total outstanding amount in output: {:?}",
        total_account_with_cf,
        total_accounts_currmnth,
        total_num_cust,
        total_num_cust_multiple,
        total_bal_in,
    );

    let health_report = HealthReport::new(
        total_account_with_cf,
        total_account_with_cf,
        0,
        total_bal_in,
        total_bal_in,
        0,
    );
    health_report.gen_health_rpt(&ftp_parameters.cp.output_file_path());

    (op_line, report_string)
}
