extern crate serde;
extern crate serde_derive;

use self::serde_derive::Deserialize;
use self::serde_derive::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InputAccount {
    pub customer_id: String,
    pub account_id: String,
    pub branch: String,
    pub customer_name: String,
    pub product_type: String,
    pub scheme_type: String,
    pub product_code: String,
    pub currency: String,
    pub customer_type: String,
    pub gl_account_principal: String,
    pub gl_account_interest: String,
    pub open_date: String,
    pub value_date: String,
    pub maturity_date: String,
    pub limit_amount: String,
    pub current_bal_amount: String,
    pub flg_moratarium: String,
    pub date_till_moratarium: String,
    pub flg_int_svc: String,
    pub credit_indx: String,
    pub credit_spread: String,
    pub credit_bench_mark: String,
    pub debit_indx: String,
    pub debit_spread: String,
    pub debit_bench_mark: String,
    pub tempod_indx: String,
    pub tempod_spread: String,
    pub tempod_bench_mark: String,
    pub overline_indx: String,
    pub overline_spread: String,
    pub overline_bench_mark: String,
    pub interest_accrued_debit: String,
    pub interest_accrued_credit: String,
    pub compounded_interest_credit: String,
    pub compounded_interest_debit: String,
    pub interest_paid: String,
    pub interest_received: String,
    pub next_interest_cap_date_credit: String,
    pub next_interest_cap_date_debit: String,
    pub flg_fixed_floating: String,
    pub overdue_interest_amount: String,
    pub overdue_principal_amount: String,
    pub any_excess_paid_amount: String,
    pub fixed_tenor_period: String,
    pub flg_performing_npa: String,
    pub flg_securitisation: String,
    pub pool_id: String,
    pub compounding_frequency_credit: String,
    pub compounding_frequency_debit: String,
    pub day_count_int_accrual: String,
    pub day_count_int_cap: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Ref1 {
    pub asset_type: String,
    pub cod_acc_no: String,
    pub cod_limit_no: i64,
    pub loan_limit_amount: f64,
    pub index_code: String,
    pub index_name: String,
    pub index_rate: f64,
    pub effective_roi: f64,
    pub reset_frequency: String,
    pub next_reset_date: String,
    pub tenure: f64,
}

impl Ref1 {
    pub fn get_max(&mut self, fields: Vec<&str>) {
        if self.cod_limit_no < fields[4].parse().unwrap_or(0) {
            self.asset_type = fields[0].to_string().replace("'", "").replace("\"", "");
            self.cod_acc_no = fields[1].to_string().replace("'", "").replace("\"", "");
            self.cod_limit_no = fields[4].parse().unwrap_or(0);
            self.loan_limit_amount = fields[5].parse().unwrap_or(0.0);
            self.index_code = fields[10].to_string().replace("'", "").replace("\"", "");
            self.index_name = fields[11].to_string().replace("'", "").replace("\"", "");
            self.index_rate = fields[12].parse().unwrap_or(0.0);
            self.effective_roi = fields[14].parse().unwrap_or(0.0);
            self.reset_frequency = fields[15].to_string().replace("'", "").replace("\"", "");
            self.next_reset_date = fields[16].to_string().replace("'", "").replace("\"", "");
            self.tenure = fields[17].parse().unwrap_or(0.0);
        }
    }
    pub fn get_default(account_id: &String) -> Ref1 {
        Ref1 {
            asset_type: "NA".to_string(),
            cod_acc_no: account_id.to_owned(),
            cod_limit_no: 0,
            loan_limit_amount: 0.0,
            index_code: "NA".to_string(),
            index_name: "NA".to_string(),
            index_rate: 0.0,
            effective_roi: 0.0,
            reset_frequency: "NA".to_string(),
            next_reset_date: "01-JAN-1900".to_string(),
            tenure: 0.0,
        }
    }
}
