#[derive(Debug, Clone)]
pub struct InputAccount {
    pub class_id: String,
    pub customer_id: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(line: String) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            class_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `class_id`.");
                }
            },
            customer_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customer_id`.");
                }
            },
        };
        Ok(input_account)
    }
}
