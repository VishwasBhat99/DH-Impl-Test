use rbdate::DateParser;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub cf_type: String,
    pub llg_id: String,
    pub item_type: String,
    pub level1: String,
    pub level2: String,
    pub level3: String,
    pub level4: String,
    pub currency: String,
    pub values: Vec<String>,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        _dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let amt_bkts = line.to_string();
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            cf_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cf_type`.");
                }
            },
            llg_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `llg_id`.");
                }
            },
            item_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `item_type`.");
                }
            },
            level1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `level1`.");
                }
            },
            level2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `level2`.");
                }
            },
            level3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `level3`.");
                }
            },
            level4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `level4`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            values: match value_iterator.next() {
                Some(_val) => {
                    let values: Vec<String> =
                        amt_bkts.split('|').map(|val| val.to_string()).collect();
                    values
                }
                None => {
                    return Err("Could not parse property `amt`.");
                }
            },
        };
        Ok(input_account)
    }
}
