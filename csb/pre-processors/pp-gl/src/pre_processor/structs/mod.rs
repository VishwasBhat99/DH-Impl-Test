pub mod alm_master;
pub mod input_account;

pub fn get_master_data(data: &str) -> String {
    if data.is_empty() {
        String::from("NONE")
    } else {
        String::from(data)
    }
}
