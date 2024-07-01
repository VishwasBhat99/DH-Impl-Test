use std::fs::File;
use std::io::BufWriter;

#[derive(Debug)]
pub struct OutputData {
    pub position_date: String,
    pub scrip_code: String,
    pub scrip_name: String,
    pub portfolio: String,
    pub instrument_name: String,
    pub maturity_date: String,
    pub currency: String,
    pub isin: String,
    pub outstanding_q_ty: f64,
    pub face_value_lcy: f64,
    pub book_value_lcy: f64,
    pub weighte_avg_price: f64,
    pub market_price: f64,
    pub market_value_lcy: f64,
    pub appr_depr: f64,
    pub coupon: String,
    pub frequency: f64,
    pub res_tenor: String,
    pub yields: String,
    pub mduration: String,
    pub timeband: String,
    pub period: String,
    pub zone: String,
    pub position: String,
}

impl OutputData {
    pub fn format_with_separator(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|{}|{:.2}|{}|{}|{}|{}|{}|{}|{}",
            self.position_date,
            self.scrip_code,
            self.scrip_name,
            self.portfolio,
            self.instrument_name,
            self.maturity_date,
            self.currency,
            self.isin,
            self.outstanding_q_ty,
            self.face_value_lcy,
            self.book_value_lcy,
            self.weighte_avg_price,
            self.market_price,
            self.market_value_lcy,
            self.appr_depr,
            self.coupon,
            self.frequency,
            self.res_tenor,
            self.yields,
            self.mduration,
            self.timeband,
            self.period,
            self.zone,
            self.position,
        )
    }
}

pub fn get_writer(file_path: &str) -> BufWriter<File> {
    match sdb_io::buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!("Unable to create file `{}` due to: {}", file_path, error),
    }
}
