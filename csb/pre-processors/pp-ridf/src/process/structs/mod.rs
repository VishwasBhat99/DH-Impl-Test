pub mod alm_master;
pub mod ridf;

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
