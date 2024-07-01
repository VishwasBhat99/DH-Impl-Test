use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub casa_acc_no: String,
    pub casa_prod_cd: String,
    pub acc_stats: i64,
    pub acc_br_cd: i64,
    pub book_bal: f64,
    pub avail_bal: f64,
    pub flex_cube_cust_id: i64,
    pub tot_od_lmt: f64,
    pub acc_open_dt: Option<NaiveDate>,
    pub cust_shrt_name: String,
    pub asset_bal_gl: i64,
    pub liability_bal_gl: i64,
    pub int_acrd_base_cd: String,
    pub cbr_num_1: i64,
    pub cbr_num_2: i64,
    pub cbr_num_3: i64,
    pub cr_rt: Option<f64>,
    pub dr_rt: Option<f64>,
    pub act_typ: String,
    pub prod_name: String,
    pub int_rt: f64,
    pub component: String,
    pub rt_flg: String,
    pub inst: String,
    pub crnt_book_bal: Option<f64>,
    pub acrl_basis: String,
    pub div: String,
    pub alm_line: String,
    pub ia_llg: String,
    pub balm_llg: String,
    pub int_index_cd: i64,
    pub int_index_name: String,
    pub od_variance: f64,
    pub npa_flg: String,
    pub gl: i64,
    pub cust_cat: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            casa_acc_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `casa_account_no`.");
                }
            },
            casa_prod_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `casa_product_code`.");
                }
            },
            acc_stats: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `account_status`.");
                }
            },
            acc_br_cd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `account_branch_code`.");
                }
            },
            book_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `book_balance`.");
                }
            },
            avail_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `available_balance`.");
                }
            },
            flex_cube_cust_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `flex_cube_cust_id`.");
                }
            },
            tot_od_lmt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `total_od_limit`.");
                }
            },
            acc_open_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `account_open_date`.");
                }
            },
            cust_shrt_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customer_short_name`.");
                }
            },
            asset_bal_gl: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `asset_balance_gl`.");
                }
            },
            liability_bal_gl: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `liability_balance_gl`.");
                }
            },
            int_acrd_base_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `interest_accured_base_code`.");
                }
            },
            cbr_num_1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `cbr_number_1`.");
                }
            },
            cbr_num_2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `cbr_number_2`.");
                }
            },
            cbr_num_3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `cbr_number_3`.");
                }
            },
            cr_rt: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `cr_date`.");
                }
            },
            dr_rt: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `dr_rate`.");
                }
            },
            act_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `act_type`.");
                }
            },
            prod_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `product_name`.");
                }
            },
            int_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `interest_rate`.");
                }
            },
            component: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `component`.");
                }
            },
            rt_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rate_flag`.");
                }
            },
            inst: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `institution`.");
                }
            },
            crnt_book_bal: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `current_book_balance`.");
                }
            },
            acrl_basis: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `accural_basis`.");
                }
            },
            div: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `division`.");
                }
            },
            alm_line: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `alm_line`.");
                }
            },
            ia_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ia_llg`.");
                }
            },
            balm_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `balm_llg`.");
                }
            },
            int_index_cd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `interest_index_code`.");
                }
            },
            int_index_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `interest_index_name`.");
                }
            },
            od_variance: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `od_variance`.");
                }
            },
            npa_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa_flag`.");
                }
            },
            gl: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `gl`.");
                }
            },
            cust_cat: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa_flag`.");
                }
            },
        };
        Ok(input_account)
    }
}
