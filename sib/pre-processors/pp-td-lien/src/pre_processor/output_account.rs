use std::fs::File;
use std::io::BufWriter;

#[derive(Debug)]
pub struct OutputData {
    pub acid: String,
    pub b2k_type: String,
    pub b2k_id: String,
    pub entity_cre_flag: String,
    pub del_flag: String,
    pub lien_amt: String,
    pub lien_start_date: String,
    pub lien_expiry_date: String,
    pub lien_reason_code: String,
    pub sol_id: String,
    pub currency: String,
    pub clr_bal_amt: String,
    pub const_code: String,
    pub maturity_date: String,
    pub tenor: String,
    pub gl_sub_head_code: String,
    pub final_lien_amt: String,
}

impl OutputData {
    pub fn format_with_separator(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.acid,
            self.b2k_type,
            self.b2k_id,
            self.entity_cre_flag,
            self.del_flag,
            self.lien_amt,
            self.lien_start_date,
            self.lien_expiry_date,
            self.lien_reason_code,
            self.sol_id,
            self.currency,
            self.clr_bal_amt,
            self.const_code,
            self.maturity_date,
            self.tenor,
            self.gl_sub_head_code,
            self.final_lien_amt
        )
    }
}

pub fn get_writer(file_path: &str) -> BufWriter<File> {
    match sdb_io::buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!("Unable to create file `{}` due to: {}", file_path, error),
    }
}
