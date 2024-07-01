use std::fs::File;
use std::io::BufWriter;

#[derive(Debug)]
pub struct OutputData {
    pub account_id: String,
    pub customer_id: String,
    pub group_id: String,
    pub outstanding_amount: String,
    pub outstanding_amount_lcy: String,
    pub ccy: String,
    pub maturity_date: String,
    pub gl_code: String,
    pub pan_number: String,
    pub customer_classification_code: String,
    pub npa: String,
    pub provision_amount: String,
    pub provision_percentage: String,
    pub restructured_flag: String,
    pub sanction_date: String,
    pub product_code: String,
    pub product_description: String,
    pub ltv: String,
    pub residential_mortgage_flag: String,
    pub sub_sector: String,
    pub group_level_total_exposure: String,
    pub rating_agency: String,
    pub rating: String,
    pub bank_category: String,
    pub cet_ratio: String,
    pub guaranteed_by: String,
    pub collateral: String,
    pub as_on_date: String,
    pub residual_tenor: String,
    pub maturity_tenor: String,
}

impl OutputData {
    pub fn format_with_separator(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{:.4}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            self.account_id,
            self.customer_id,
            self.group_id,
            self.outstanding_amount,
            self.outstanding_amount_lcy,
            self.ccy,
            self.maturity_date,
            self.gl_code,
            self.pan_number,
            self.customer_classification_code,
            self.npa,
            self.provision_amount,
            self.provision_percentage,
            self.restructured_flag,
            self.sanction_date,
            self.product_code,
            self.product_description,
            self.ltv,
            self.residential_mortgage_flag,
            self.sub_sector,
            self.group_level_total_exposure,
            self.rating_agency,
            self.rating,
            self.bank_category,
            self.cet_ratio,
            self.guaranteed_by,
            self.collateral,
            self.as_on_date,
            self.residual_tenor,
            self.maturity_tenor
        )
    }
}

pub fn get_writer(file_path: &str) -> BufWriter<File> {
    match sdb_io::buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!("Unable to create file `{}` due to: {}", file_path, error),
    }
}
