use rbdate::{DateParser, NaiveDate};
use statics::DEFAULT_FLOAT;

#[derive(Debug)]
pub struct InputValues {
    pub prin_amt: f64,
    pub int_amt: f64,
    pub emi_amt: f64,
    pub emi_date: Option<NaiveDate>,
}

impl<'a> InputValues {
    pub fn set_of_values(line: String, total: usize) -> Vec<InputValues> {
        let val_iter: Vec<&str> = line.split("~#~").collect();
        let dmy_date_parser = DateParser::new("%d%m%Y".to_string(), false);
        // exclude last 13 columns which are not InputValues
        // last 3 columns of InputValues will be accessed in for loop
        let total = total - 13 - 3;
        let mut index = 5;
        let mut all_values: Vec<InputValues> = Vec::new();

        while index <= total {
            let input_account_values = InputValues {
                prin_amt: val_iter[index].parse().unwrap_or(DEFAULT_FLOAT),
                int_amt: val_iter[index + 1].parse().unwrap_or(DEFAULT_FLOAT),
                emi_amt: val_iter[index + 2].parse().unwrap_or(DEFAULT_FLOAT),
                emi_date: dmy_date_parser.parse_opt(val_iter[index + 3]),
            };
            // increment by 4 for next InputValues set
            index = index + 4;
            all_values.push(input_account_values);
        }
        all_values
    }
}
