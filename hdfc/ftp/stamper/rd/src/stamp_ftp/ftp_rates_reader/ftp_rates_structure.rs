use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug)]
pub struct ftp_rates {
   pub account_number: i64,
   pub start_date: i64,
   pub end_date: i64,
   pub int_rate: f64,
   pub base_rate: f64,
   pub adj1_rate: f64,
   pub adj2_rate: f64,
   pub adj3_rate: f64,
   pub adj4_rate: f64,
   pub adj5_rate: f64,
   pub adj6_rate: f64,
   pub spread: f64
}

impl ftp_rates{
   pub fn new(line: String) -> Result<ftp_rates, String>
    {
        let mut value_iterator = line.split('|');

        let rates = ftp_rates
         {
            account_number: match value_iterator.next() {
                Some(val) => { val.parse().unwrap_or(DEFAULT_INT) }
                None => { return Err(format!("Could not parse property `account_number'. --ftp_rates reader")); }
            },
            start_date: match value_iterator.next() {
                Some(val) => { val.parse().unwrap_or(DEFAULT_INT) }
                None => { return Err(format!("Could not parse property `start_date`. --ftp_rates reader")); }
            },
            end_date: match value_iterator.next() {
                Some(val) => { val.parse().unwrap_or(DEFAULT_INT) }
                None => { return Err(format!("Could not parse property `end_date`. --ftp_rates reader")); }
            },
            int_rate: match value_iterator.next() {
                Some(val) => { val.parse().unwrap_or(DEFAULT_FLOAT) }
                None => { return Err(format!("Could not parse property `int_rate`. --ftp_rates reader")); }
            },
            base_rate: match value_iterator.next() {
                Some(val) => { val.parse().unwrap_or(DEFAULT_FLOAT) }
                None => { return Err(format!("Could not parse property `base_rate`. --ftp_rates reader")); }
            },
            adj1_rate: match value_iterator.next() {
                Some(val) => { val.parse().unwrap_or(DEFAULT_FLOAT) }
                None => { return Err(format!("Could not parse property `adj1_rate`. --ftp_rates reader")); }
            },
            adj2_rate: match value_iterator.next() {
                Some(val) => { val.parse().unwrap_or(DEFAULT_FLOAT) }
                None => { return Err(format!("Could not parse property `adj2_rate`. --ftp_rates reader")); }
            },
            adj3_rate: match value_iterator.next() {
                Some(val) => { val.parse().unwrap_or(DEFAULT_FLOAT) }
                None => { return Err(format!("Could not parse property `adj3_rate`. --ftp_rates reader")); }
            },
            adj4_rate: match value_iterator.next() {
                Some(val) => { val.parse().unwrap_or(DEFAULT_FLOAT) }
                None => { return Err(format!("Could not parse property `adj4_rate`. --ftp_rates reader")); }
            },
            adj5_rate: match value_iterator.next() {
                Some(val) => { val.parse().unwrap_or(DEFAULT_FLOAT) }
                None => { return Err(format!("Could not parse property `adj5_rate`. --ftp_rates reader")); }
            },
            adj6_rate: match value_iterator.next() {
                Some(val) => { val.parse().unwrap_or(DEFAULT_FLOAT) }
                None => { return Err(format!("Could not parse property `adj6_rate`. --ftp_rates reader")); }
            },
            spread: match value_iterator.next() {
                Some(val) => { val.parse().unwrap_or(DEFAULT_FLOAT) }
                None => { return Err(format!("Could not parse property `spread`. --ftp_rates reader")); }
            },
         };

         Ok(rates)
        
    }
}