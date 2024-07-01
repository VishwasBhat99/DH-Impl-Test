use statics::{DEFAULT_FLOAT, DEFAULT_INT};

#[derive(Debug, Clone)]
pub struct GrpData {
    pub llg_vec: Vec<String>,
    pub limit: f64,
    pub limit_llg: i64,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct OPKey {
    pub llg: String,
    pub as_on: String,
    pub ccy: String,
    pub field4: String,
    pub field5: String,
    pub flow_type: String,
}

#[derive(Debug, Clone)]
pub struct OPVal {
    pub amt: f64,
    pub int: String,
}

#[derive(Debug, Clone)]
pub struct Account {
    pub llg_code: i64,
    pub date: String,
    pub currency: String,
    pub field_1: String,
    pub field_2: String,
    pub field_3: String,
    pub amount: f64,
    pub field_5: String,
}
impl<'a> OPVal {
    pub fn default() -> OPVal {
        OPVal {
            amt: 0.0,
            int: "0.0".to_string(),
        }
    }
}

impl<'a> Account {
    pub fn new_from_line(line: String) -> Result<Account, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = Account {
            llg_code: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `llg_code`.");
                }
            },
            date: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `date`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            field_1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `field 1`.");
                }
            },
            field_2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `field 2`.");
                }
            },
            field_3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `field 3`.");
                }
            },
            amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `llg_code`.");
                }
            },
            field_5: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `field 5`.");
                }
            },
        };
        Ok(input_account)
    }

    pub fn output_data(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}\n",
            self.llg_code,
            self.date,
            self.currency,
            self.field_1,
            self.field_2,
            self.field_3,
            self.amount,
            self.field_5
        )
    }
}
