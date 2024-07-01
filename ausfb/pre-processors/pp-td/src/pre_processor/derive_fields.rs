use super::input_account::InputAccount;
use rbdate::{num_days_start_to_end, NaiveDate};

pub fn get_op_line(out_acc: &InputAccount, as_on_date: NaiveDate) -> String {
    let asondate = as_on_date.format("%d-%m-%Y").to_string();

    let open_dt = NaiveDate::parse_from_str(&out_acc.open_date.as_str(), "%d-%b-%Y")
        .unwrap_or(as_on_date)
        .format("%d-%m-%Y")
        .to_string();
    let val_dt = NaiveDate::parse_from_str(out_acc.value_date.as_str(), "%d-%b-%Y")
        .unwrap_or(as_on_date)
        .format("%d-%m-%Y")
        .to_string();
    let mat_dt = NaiveDate::parse_from_str(out_acc.maturity_date.as_str(), "%d-%b-%Y")
        .unwrap_or(as_on_date)
        .format("%d-%m-%Y")
        .to_string();
    let next_int_pay_dt =
        NaiveDate::parse_from_str(out_acc.next_interest_payement_date.as_str(), "%d-%b-%Y")
            .unwrap_or(as_on_date)
            .format("%d-%m-%Y")
            .to_string();
    let residual_days = num_days_start_to_end(
        as_on_date,
        NaiveDate::parse_from_str(out_acc.maturity_date.as_str(), "%d-%b-%Y").unwrap_or(as_on_date),
    );
    let contractual_days = num_days_start_to_end(
        NaiveDate::parse_from_str(out_acc.open_date.as_str(), "%d-%b-%Y").unwrap_or(as_on_date),
        NaiveDate::parse_from_str(out_acc.maturity_date.as_str(), "%d-%b-%Y").unwrap_or(as_on_date),
    );
    let current_book_balance = out_acc
        .current_principal_balance
        .parse::<f64>()
        .unwrap_or(0.0)
        + out_acc.compounded_interest.parse::<f64>().unwrap_or(0.0);

    format!("{}|{}|{}|{}|{}||{}|{}|{}|{}|{}|{}|{}|||{}||{}||{}|{}|{}||{}|{}|{}||||{}||{}||{}|{}|||{}||{}|{}|{}|||{}|||||||{}|{}||||||||",
    out_acc.account_id,
    out_acc.interest_accrued,
    out_acc.product_type,
    mat_dt,
    out_acc.interest_rate,
    next_int_pay_dt,
    val_dt,
    open_dt,
    out_acc.currency,
    out_acc.cust_type,
    out_acc.initial_principal_balance,
    mat_dt,
    out_acc.customer_name,
    asondate,
    out_acc.branch_name,
    out_acc.flg_fixed_float,
    out_acc.payout_frequency,
    out_acc.currency,
    out_acc.regular_deposit_gl,
    out_acc.interest_rate,
    current_book_balance,
    out_acc.compounding_frequency,
    out_acc.scheme_type,
    out_acc.product_code,
    val_dt,
    out_acc.initial_principal_balance,
    out_acc.current_principal_balance,
    out_acc.compounded_interest,
    open_dt,
    residual_days,
    contractual_days
    )
}
