use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub id: String,
    pub desc: String,
    pub amt: f64,
    pub flow_date: Option<NaiveDate>,
    pub gl_code: String,
    pub inflow_outflow: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `id`.");
                }
            },
            desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `desc`.");
                }
            },
            amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `amt`.");
                }
            },
            flow_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `flow_date`.");
                }
            },
            gl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_code`.");
                }
            },
            inflow_outflow: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `inflow_outflow`.");
                }
            },
        };
        Ok(input_account)
    }
}
