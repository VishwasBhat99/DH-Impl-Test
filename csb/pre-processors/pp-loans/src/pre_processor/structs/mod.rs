pub mod additional_passthroughs;
pub mod alm_master;
pub mod cust_master;
pub mod input_account;
pub mod loan_additional;
pub mod npa;
pub mod schedule;
pub mod td_cr_cust_master;

pub fn get_data(data: &str) -> String {
    if data.is_empty() {
        String::from("NA")
    } else {
        String::from(data)
    }
}

pub fn get_master_data(data: &str) -> String {
    if data.is_empty() {
        String::from("NONE")
    } else {
        String::from(data)
    }
}

pub fn get_def_float_data(data: &str) -> String {
    if data.is_empty() {
        String::from("0.0")
    } else {
        String::from(data)
    }
}
