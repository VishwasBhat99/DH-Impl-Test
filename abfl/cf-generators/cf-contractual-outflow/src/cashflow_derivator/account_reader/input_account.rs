use crate::statics::DEFAULT_FLOAT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub serial_no: String,
    pub incr_dt: Option<rbdate::NaiveDate>,
    pub incr_amt: f64,
    pub applicable_dt: Option<rbdate::NaiveDate>,
    pub projected_outflow: f64,
    pub lob: String,
    pub currency: String,
    pub add_field1: String,
    pub add_field2: String,
    pub add_field3: String,
    pub add_field4: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &rbdate::DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            serial_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `serial no`.");
                }
            },
            incr_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `increment date`.");
                }
            },
            incr_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `increment amount`.");
                }
            },
            applicable_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `applicable date`.");
                }
            },
            projected_outflow: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `projected outflow`.");
                }
            },
            lob: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `lob`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            add_field1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `add field 1`.");
                }
            },
            add_field2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `add field 1`.");
                }
            },
            add_field3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `add field 1`.");
                }
            },
            add_field4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `add field 1`.");
                }
            },
        };
        Ok(input_account)
    }
}
