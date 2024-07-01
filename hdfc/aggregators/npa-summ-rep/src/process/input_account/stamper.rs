use super::statics::DEFAULT_FLOAT;

#[derive(Debug, Clone, Default)]
///Stamper-Data-Fields used for Lookups
pub struct StamperData {
    pub account_number: String,
    pub average_balance: f64,
    pub final_ftp_rate: f64,
    pub mis1: String,
    pub prod_type: String,
}

impl<'a> StamperData {
    pub fn new_from_line(line: String) -> Result<StamperData, &'a str> {
        let stamper_req_fields: Vec<&str> = line.split('|').collect();
        let stamper_data = StamperData {
            account_number: stamper_req_fields[0].to_string(),
            average_balance: stamper_req_fields[2]
                .to_string()
                .parse::<f64>()
                .unwrap_or(DEFAULT_FLOAT),
            final_ftp_rate: stamper_req_fields[7]
                .to_string()
                .parse::<f64>()
                .unwrap_or(DEFAULT_FLOAT),
            mis1: stamper_req_fields[12].to_string(),
            prod_type: stamper_req_fields[15].to_string(),
        };
        Ok(stamper_data)
    }
    pub fn def() -> StamperData {
        ::std::default::Default::default()
    }
}
