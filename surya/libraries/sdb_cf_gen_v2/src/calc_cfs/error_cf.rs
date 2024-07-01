use super::process::*;
use crate::structs::*;

pub fn check_error_cases(acc_data: &AccData) -> ErrorMsg {
    let mut cashflows = Vec::new();
    let mut error_code = 0;
    let mut error_msg = "No error cases.";
    let int_pay_freq = get_pay_freq(acc_data.int_payout_freq.as_ref().unwrap());
    if acc_data.int_rate < 0.0 && acc_data.int_rate > 100.0 {
        error_code = -1;
        error_msg = "Interest rate out of range."
    } else if acc_data.ost_bal <= 0.0 {
        error_code = -2;
        error_msg = "Outstanding balance is less than or equal to zero."
    } else if acc_data.maturity_date < acc_data.as_on_date {
        error_code = -3;
        error_msg = "Maturity date is less than as on date."
    } else if acc_data.acc_start_date > acc_data.maturity_date {
        error_code = -4;
        error_msg = "Account start date is greater than maturity date.";
    } else if int_pay_freq != 1 && int_pay_freq != 12 && int_pay_freq != 6 && int_pay_freq != 3 {
        error_code = int_pay_freq as i16;
        error_msg = "Unexpected account frequency.";
    }
    let cf = new_cashflow(acc_data.ost_bal, 0.0, &acc_data.maturity_date);
    cashflows.push(cf);
    let error_cf = ErrorMsg {
        error_code: error_code,
        error_msg: error_msg.to_string(),
        default_cfs: cashflows,
    };

    return error_cf;
}
