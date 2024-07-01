use rbdate::NaiveDate;
use statics::*;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InputAccount {
    pub acc_no: String,
    pub date: String,
    pub amt: String,
    pub int_rt: String,
}

#[derive(Debug)]
pub struct InputParsedAccount {
    pub acc_no: String,
    pub date: NaiveDate,
    pub amt: f64,
    pub int_rt: f64,
}

impl InputAccount {
    pub fn parse(&self) -> InputParsedAccount {
        InputParsedAccount {
            acc_no: self.acc_no.to_string(),
            date: NaiveDate::parse_from_str(&self.date, "%d-%m-%Y")
                .expect("Error getting `as_on_date` while parsing input record."),
            amt: self.amt.parse().unwrap_or(DEFAULT_FLOAT),
            int_rt: self.int_rt.parse().unwrap_or(DEFAULT_FLOAT),
        }
    }
}
