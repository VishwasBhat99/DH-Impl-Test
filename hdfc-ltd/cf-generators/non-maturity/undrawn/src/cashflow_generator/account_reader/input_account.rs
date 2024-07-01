use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub br_no: String,
    pub origin_branch: String,
    pub lac_no: String,
    pub laf_no: String,
    pub borr_name: String,
    pub disb_amt: String,
    pub sanc_amt: String,
    pub emi: String,
    pub roi: String,
    pub clps_loan_type: String,
    pub first_disb_date: Option<NaiveDate>,
    pub last_disb_date: Option<NaiveDate>,
    pub commitment_amt: Option<f64>,
    pub amt_of_disb: String,
    pub approval_date: Option<NaiveDate>,
    pub currency: String,
    pub treasury_glcode: String,
    pub app1: String,
    pub app2: String,
    pub app3: String,
    pub app4: String,
    pub app5: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            br_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `br_no`.");
                }
            },
            origin_branch: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `origin_branch`.");
                }
            },
            lac_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `lac_no`.");
                }
            },
            laf_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `laf_no`.");
                }
            },
            borr_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `borr_name`.");
                }
            },
            disb_amt: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `disb_amt`.");
                }
            },
            sanc_amt: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sanc_amt`.");
                }
            },
            emi: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `emi`.");
                }
            },
            roi: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `roi`.");
                }
            },
            clps_loan_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `clps_loan_type`.");
                }
            },
            first_disb_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `first_disb_date`.");
                }
            },
            last_disb_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_disb_date`.");
                }
            },
            commitment_amt: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `commitment_amt`.");
                }
            },
            amt_of_disb: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `amt_of_disb`.");
                }
            },
            approval_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `approval_date`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            treasury_glcode: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `treasury_glcode`.");
                }
            },
            app1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app1`.");
                }
            },
            app2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app2`.");
                }
            },
            app3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app3`.");
                }
            },
            app4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app4`.");
                }
            },
            app5: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app5`.");
                }
            },
        };
        Ok(input_account)
    }
}
