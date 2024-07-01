extern crate serde;
extern crate serde_derive;
use self::serde_derive::Deserialize;
use self::serde_derive::Serialize;
use rbdate::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InputAccount {
    pub account_id: String,
    pub customername: String,
    pub branch_name: String,
    pub customer_no: String,
    pub commonclientcode: String,
    pub productcode: String,
    pub loan_start_date: String,
    pub maturity_date: String,
    pub balance_term: String,
    pub sanctionamount: String,
    pub disbursed_amount: String,
    pub currency: String,
    pub principal_ouststanding_amount: String,
    pub overdue_interest: String,
    pub overdue_principal: String,
    pub pre_emi_outstanding_amount: String,
    pub pre_emi_remaining: String,
    pub interest_type: String,
    pub interestrate: String,
    pub interest_calulation_method: String,
    pub number_of_total_emi: String,
    pub emi_frequency: String,
    pub installment_type: String,
    pub revised_lob: String,
    pub revised_vertical: String,
    pub accountstatus: String,
}

impl InputAccount {
    pub fn new() -> InputAccount {
        InputAccount {
            ..Default::default()
        }
    }
    pub fn acc_to_string(acc: InputAccount) -> String {
        let default_date = NaiveDate::parse_from_str("1900-01-01", "%Y-%m-%d").unwrap();
        let mat_dt = NaiveDate::parse_from_str(acc.maturity_date.as_str(), "%Y-%m-%d")
            .unwrap_or(default_date)
            .format("%d-%m-%Y");
        let loan_str_dt = NaiveDate::parse_from_str(acc.loan_start_date.as_str(), "%Y-%m-%d")
            .unwrap_or(default_date)
            .format("%d-%m-%Y");
        let str = format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            acc.account_id,
            acc.customername,
            acc.branch_name,
            acc.customer_no,
            acc.commonclientcode,
            acc.productcode,
            loan_str_dt,
            mat_dt,
            acc.balance_term,
            acc.sanctionamount,
            acc.disbursed_amount,
            acc.currency,
            acc.principal_ouststanding_amount,
            acc.overdue_interest,
            acc.overdue_principal,
            acc.pre_emi_outstanding_amount,
            acc.pre_emi_remaining,
            acc.interest_type,
            acc.interestrate,
            acc.interest_calulation_method,
            acc.number_of_total_emi,
            acc.emi_frequency,
            acc.installment_type,
            acc.revised_lob,
            acc.revised_vertical,
            acc.accountstatus
        );
        return str;
    }
}
