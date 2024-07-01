pub mod alm_master;
pub mod pass_throughs;
pub mod cust_master;
pub mod input_account;

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

pub fn get_pass_throughs(data: &str) -> String {
    if data.is_empty() {
        String::from("N")
    } else {
        String::from(data)
    }
}
