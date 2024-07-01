extern crate serde;
extern crate serde_derive;
use self::serde_derive::Deserialize;
use self::serde_derive::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InputAccount {
    pub customer_id: String,
    pub account_id: String,
    pub branch_name: String,
    pub customer_name: String,
    pub product_type: String,
    pub scheme_type: String,
    pub product_code: String,
    pub currency: String,
    pub business_segment: String,
    pub cust_type: String,
    pub regular_deposit_gl: String,
    pub accrued_gl: String,
    pub compounded_gl: String,
    pub payout_gl: String,
    pub open_date: String,
    pub value_date: String,
    pub maturity_date: String,
    pub initial_principal_balance: String,
    pub current_principal_balance: String,
    pub maturity_amount: String,
    pub interest_rate: String,
    pub interest_accrued: String,
    pub compounded_interest: String,
    pub interest_paid: String,
    pub next_interest_payement_date: String,
    pub flg_fixed_float: String,
    pub float_benchmark: String,
    pub spread: String,
    pub lien_amount: String,
    pub flg_auto_rollover: String,
    pub flg_si_compounding: String,
    pub compounding_frequency: String,
    pub payout_frequency: String,
    pub day_count_convention: String,
    pub tenure: String,
    pub entity_type: String,
    pub outstanding_ason: String,
}

impl InputAccount {
    pub fn new() -> InputAccount {
        InputAccount {
            ..Default::default()
        }
    }
}
