use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use cashflow_generator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use cashflow_generator::payment::PaymentDetails;
use macros;
use rbdate;
use rbdate::NaiveDate;
use sdb_day_convention::conventions::Conventions;
use sdb_day_convention::days_with_convn;
use slog::Logger;

pub fn generate_cashflows(
    as_on_date: &NaiveDate,
    prev_amt: f64,
    prev_desc: String,
    interest_rate_due_date: &Vec<PaymentDetails>,
    count: i64,
    writer: &mut AccountWithCashflowsWriter,
    log: &Logger,
    day_convention: &Conventions,
) {
    log_debug!(
        log,
        "Account description: `{}` and amount is '{}'.",
        prev_desc,
        prev_amt
    );

    let mut cashflows: Vec<Cashflow> = Vec::new();

    let mut prev_due_date =
        NaiveDate::parse_from_str("01-01-1900", "%d-%m-%Y").unwrap_or(*as_on_date);
    let mut curr_due_date =
        NaiveDate::parse_from_str("01-01-1900", "%d-%m-%Y").unwrap_or(*as_on_date);
    let mut pre_due_date_flag = true;
    let mut rows_count = 0;
    let mut account_os_amt: f64 = 0.0;
    let mut acc_due_date: NaiveDate =
        NaiveDate::parse_from_str("01-01-1900", "%d-%m-%Y").unwrap_or(*as_on_date);
    for record in interest_rate_due_date {
        let due_date = record.due_date;
        let out_amt = calculate_out_rate(prev_amt, rows_count, count);

        if pre_due_date_flag == true {
            curr_due_date = due_date;
            prev_due_date = *as_on_date;
            account_os_amt = out_amt;
            pre_due_date_flag = false;
        } else {
            prev_due_date = curr_due_date;
            curr_due_date = due_date;
            acc_due_date = due_date;
        }

        let rate = record.interest_rate;
        let int_amt = calculate_int_rate(
            out_amt,
            rate,
            curr_due_date,
            prev_due_date,
            as_on_date,
            day_convention,
        );
        rows_count += 1;
        let mut cf = Cashflow::new();
        cf.interest_amount = int_amt;
        cf.principal_amount = prev_amt;
        cf.date = rbdate::timestamp(due_date);

        cashflows.push(cf);
    }

    let mut account_with_cashflows = AccountWithCashflows::new();

    account_with_cashflows.description = prev_desc;
    account_with_cashflows.due_date = rbdate::timestamp(acc_due_date);
    account_with_cashflows.current_balnace_amount = prev_amt;
    account_with_cashflows.ccy = "INR".to_string();
    account_with_cashflows.outstanding_amt = account_os_amt;
    account_with_cashflows.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    writer.write(account_with_cashflows);
}
fn calculate_out_rate(amt: f64, record: i64, count: i64) -> f64 {
    amt * ((count - record) as f64)
}
fn calculate_int_rate(
    out_amt: f64,
    rate: f64,
    curr_due_date: NaiveDate,
    prev_due_date: NaiveDate,
    as_on_date: &NaiveDate,
    day_convention: &Conventions,
) -> f64 {
    if curr_due_date > *as_on_date {
        let days = days_with_convn(prev_due_date, curr_due_date, day_convention)
            .expect("Failed to calculate days with convention")
            .days_btw_dts;
        (out_amt * rate * days as f64) / (36500 as f64)
    } else {
        0.0
    }
}
