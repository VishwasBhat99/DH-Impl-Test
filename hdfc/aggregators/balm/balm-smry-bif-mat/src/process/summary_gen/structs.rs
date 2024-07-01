use statics::{DEFAULT_FLOAT, DEFAULT_INT};

#[derive(Debug, Clone)]
pub struct Account {
    pub llg_code: i64,
    date: String,
    pub currency: String,
    field_1: String,
    field_2: String,
    field_3: String,
    pub amount: f64,
    field_5: String,
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
