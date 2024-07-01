use rbdate::NaiveDate;
use sdb_day_convention::conventions::Conventions;

//Account Data Struct
pub struct AccData<'a> {
    pub as_on_date: NaiveDate,
    pub acc_start_date: NaiveDate,
    pub maturity_date: NaiveDate,
    pub ost_bal: f64,
    pub int_rate: f32,
    pub int_payout_freq: Option<String>,
    pub comp_freq: Option<i8>,
    pub installment_amt: Option<f64>,
    pub pre_payment_rates: &'a Vec<f64>,
    pub convention: Conventions,
}

//Cashflow Struct
#[derive(Debug)]
pub struct Cashflow {
    pub int_amt: f64,
    pub prin_amt: f64,
    pub cf_date: i64,
}

//Error Message Struct
pub struct ErrorMsg {
    pub error_code: i16,
    pub error_msg: String,
    pub default_cfs: Vec<Cashflow>,
}

impl AccData<'static> {
    pub fn new(self) -> AccData<'static> {
        AccData {
            as_on_date: self.as_on_date,
            acc_start_date: self.acc_start_date,
            maturity_date: self.maturity_date,
            ost_bal: self.ost_bal,
            int_rate: self.int_rate,
            int_payout_freq: match self.int_payout_freq {
                Some(val) => Some(val),
                None => Some("B".to_string()),
            },
            comp_freq: match self.comp_freq {
                Some(val) => Some(val),
                None => Some(1),
            },
            installment_amt: match self.installment_amt {
                Some(val) => Some(val),
                None => Some(0.0),
            },
            pre_payment_rates: self.pre_payment_rates,
            convention: self.convention,
        }
    }
}
