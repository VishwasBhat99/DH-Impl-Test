use rbdate::DateParser;
use rbdate::NaiveDate;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub acc_no: String,
    pub sub_gl_code: String,
    pub sanction_amt: f64,
    pub currency: String,
    pub disbursement_amt: f64,
    pub outstanding_amt: f64,
    pub interest_rate: f64,
    pub interest_type: String,
    pub maturity_dt: Option<NaiveDate>,
    pub intr_comp_freq: String,
    pub department: String,
    pub ind_corp_flag: String,
    pub bra_code: String,
    pub cus_num: String,
    pub acc_open_dt: Option<NaiveDate>,
    pub emi_flag: String,
    pub flow_amt: f64,
    pub princ_pay_freq: String,
    pub intr_pay_freq: String,
    pub repricing_dt: Option<NaiveDate>,
    pub cus_name: String,
    pub loan_type: String,
    pub princ_sch_srt_dt: Option<NaiveDate>,
    pub intr_sch_srt_dt: Option<NaiveDate>,
    pub princ_flow_num: i64,
    pub intr_flow_num: i64,
    pub int_calc_basis: String,
    pub currency_conversion_rate: f64,
    pub cust_ctry_code: String,
    pub cust_crdt_rtng: String,
    pub cust_sect_code: String,
    pub cust_indt_code: String,
    pub custom1: String,
    pub custom2: String,
    pub floating_type: String,
    pub npa_classification: String,
    pub frequency_type: String,
    pub final_npa_class: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            acc_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acc_no`.");
                }
            },
            sub_gl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sub_gl_code`.");
                }
            },
            sanction_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `sanction_amt`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            disbursement_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `disbursement_amt`.");
                }
            },
            outstanding_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `outstanding_amt`.");
                }
            },
            interest_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `interest_rate`.");
                }
            },
            interest_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `interest_type`.");
                }
            },
            maturity_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_dt`.");
                }
            },
            intr_comp_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `intr_comp_freq`.");
                }
            },
            department: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `department`.");
                }
            },
            ind_corp_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ind_corp_flag`.");
                }
            },
            bra_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bra_code`.");
                }
            },
            cus_num: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cus_num`.");
                }
            },
            acc_open_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `acc_open_dt`.");
                }
            },
            emi_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `emi_flag`.");
                }
            },
            flow_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `flow_amt`.");
                }
            },
            princ_pay_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `princ_pay_freq`.");
                }
            },
            intr_pay_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `intr_pay_freq`.");
                }
            },
            repricing_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `repricing_dt`.");
                }
            },
            cus_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cus_name`.");
                }
            },
            loan_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `loan_type`.");
                }
            },
            princ_sch_srt_dt: match value_iterator.next() {
                Some(val) => {
                    let st_dt = dmy_date_parser.parse_opt(val);
                    if st_dt.is_none() {
                        return Err("Could not parse property `account_start_date`.Value empty.");
                    }
                    st_dt
                }
                None => {
                    return Err("Could not read property `account_start_date`.");
                }
            },
            intr_sch_srt_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `intr_sch_srt_dt`.");
                }
            },
            princ_flow_num: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `princ_flow_num`.");
                }
            },
            intr_flow_num: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `intr_flow_num`.");
                }
            },
            int_calc_basis: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_calc_basis`.");
                }
            },
            currency_conversion_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `currency_conversion_rate`.");
                }
            },
            cust_ctry_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_ctry_code`.");
                }
            },
            cust_crdt_rtng: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_crdt_rtng`.");
                }
            },
            cust_sect_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_sect_code`.");
                }
            },
            cust_indt_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_indt_code`.");
                }
            },
            custom1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `custom1`.");
                }
            },
            custom2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `custom2`.");
                }
            },
            floating_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `floating_type`.");
                }
            },
            npa_classification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa_classification`.");
                }
            },
            frequency_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `frequency_type`.");
                }
            },
            final_npa_class: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `final_npa_class`.");
                }
            },
        };
        Ok(input_account)
    }
}
