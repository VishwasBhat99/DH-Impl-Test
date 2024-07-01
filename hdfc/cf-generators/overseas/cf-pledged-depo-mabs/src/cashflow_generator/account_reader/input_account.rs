use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug)]
pub struct InputAccount {
    pub account_number: String,
    pub accrued_interest: f64,
    pub deposit_type: String,
    pub maturity_date: NaiveDate,
    pub rat_acct_int: f64,
    pub rat_acct_int_var: f64,
    pub next_compound_date: Option<NaiveDate>,
    pub next_payment_date: Option<NaiveDate>,
    pub account_start_date: NaiveDate,
    pub currency_code: i64,
    pub customer_id: i64,
    pub original_balance: f64,
    pub origination_date: Option<NaiveDate>,
    pub previous_roll_over_date: Option<NaiveDate>,
    pub description: String,
    pub client_name: String,
    pub tname: String,
    pub as_on_date: String,
    pub bank_num: String,
    pub branch: String,
    pub rate_flag: String,
    pub int_pay_freq: i64,
    pub cost_centre_ftp: String,
    pub institution: String,
    pub new_gl: String,
    pub int_rate: f64,
    pub concat: String,
    pub ia_llg: String,
    pub balm_llg: String,
    pub current_book_balance: f64,
    pub cost_center: String,
    pub comp_freq: i64,
    pub fin_cost_ftp: String,
    pub cust_cat: String,
    pub lien_amount: f64,
}

impl InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, String> {
        let mut value_iterator = line.split('|');

        let input_account = InputAccount {
            account_number: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `account_number`.".to_string());
                }
            },
            accrued_interest: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `accrued_interest`.".to_string());
                }
            },
            deposit_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `deposit_type`.".to_string());
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => {
                    let mat_dt = dmy_date_parser.parse_opt(val);
                    if mat_dt.is_none() {
                        return Err("Could not parse property `maturity_date`.".to_string());
                    }
                    mat_dt.unwrap()
                }
                None => {
                    return Err("Could not read property `maturity_date`.".to_string());
                }
            },
            rat_acct_int: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `rat_acct_int`.".to_string());
                }
            },
            rat_acct_int_var: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `rat_acct_int_var`.".to_string());
                }
            },
            next_compound_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `next_compund_date`.".to_string());
                }
            },
            next_payment_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `next_payment_date`.".to_string());
                }
            },
            account_start_date: match value_iterator.next() {
                Some(val) => {
                    let st_dt = dmy_date_parser.parse_opt(val);
                    if st_dt.is_none() {
                        return Err("Could not parse property `account_start_date`.".to_string());
                    }
                    st_dt.unwrap()
                }
                None => {
                    return Err("Could not read property `account_start_date`.".to_string());
                }
            },
            currency_code: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `currency_code`.".to_string());
                }
            },
            customer_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `customer_id`.".to_string());
                }
            },
            original_balance: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `original_balance`.".to_string());
                }
            },
            origination_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `origination_date`.".to_string());
                }
            },
            previous_roll_over_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `previous_roll_over_date`.".to_string());
                }
            },
            description: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `description`.".to_string());
                }
            },
            client_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `client_name`.".to_string());
                }
            },
            tname: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `t_name`.".to_string());
                }
            },
            as_on_date: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `as_on_date`.".to_string());
                }
            },
            bank_num: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `bank_num`.".to_string());
                }
            },
            branch: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `branch`.".to_string());
                }
            },
            rate_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `rate_flag`.".to_string());
                }
            },
            int_pay_freq: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `int_pay_freq`.".to_string());
                }
            },
            cost_centre_ftp: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `cost_centre_ftp`.".to_string());
                }
            },
            institution: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `institution`.".to_string());
                }
            },
            new_gl: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `new_gl`.".to_string());
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `int_rate`.".to_string());
                }
            },
            concat: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `concat`.".to_string());
                }
            },
            ia_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `ia_llg`.".to_string());
                }
            },
            balm_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `balm_llg`.".to_string());
                }
            },
            current_book_balance: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `current_book_balance`.".to_string());
                }
            },
            cost_center: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `cost_center`.".to_string());
                }
            },
            comp_freq: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `comp_freq_int`.".to_string());
                }
            },
            fin_cost_ftp: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `fin_cost_ftp`.".to_string());
                }
            },
            cust_cat: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust-cat`.".to_string());
                }
            },
            lien_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `lien_amount`.".to_string());
                }
            },
        };

        Ok(input_account)
    }
}
