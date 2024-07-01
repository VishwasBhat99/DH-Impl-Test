#[derive(Debug, Clone)]
pub struct InputAccount {
    pub class_id: i8,
    pub customer_id: i32,
    pub currency: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(line: String) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            class_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `class_id`.");
                }
            },
            customer_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `customer_id`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
        };
        Ok(input_account)
    }
}

pub fn get_str_from_txt(input_file: &str, data: &[&str], index: usize, row: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` in row-no: `{:?}` from File: {}",
                index + 1,
                row,
                input_file,
            )
        })
        .trim()
        .trim_matches(|pat| pat == ' ' || pat == '"')
        .to_string()
}

pub fn get_date(
    config_params: &crate::configuration_parameters::ConfigurationParameters,
    input_file: &str,
    data: &[&str],
    index: usize,
    row: usize,
) -> rbdate::NaiveDate {
    let date_parser =
        rbdate::DateParser::new(config_params.lien_input_date_format().to_string(), false);
    date_parser
        .parse_opt(
            &data
                .get(index)
                .unwrap_or_else(|| {
                    panic!(
                        "Could not get data at column-no: `{}` in row-no: `{:?}` from File: {}",
                        index + 1,
                        row,
                        input_file,
                    )
                })
                .replace('.', ""),
        )
        .unwrap_or(*config_params.as_on_date())
}
