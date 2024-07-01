use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub type_id: String,
    pub subtype_id: String,
    pub dis_amt: f64,
    pub rephasement_principal: f64,
    pub rep_shdl_num: i64,
    pub rep_shdl_date: Option<NaiveDate>,
    pub acid: String,
    pub bacid: String,
    pub clr_bal_amt: f64,
    pub sanct_lim: f64,
    pub gl_sub_head_code: String,
    pub schm_code: String,
    pub schm_type: String,
    pub acct_crncy_code: String,
    pub acct_opn_date: NaiveDate,
    pub dis_shdl_num: i64,
    pub dis_shdl_date: Option<NaiveDate>,
    pub sol_id: String,
    pub cust_name: String,
    pub nrml_int_pcnt: f64,
    pub end_date: NaiveDate,
    pub fixedornot: String,
    pub cust_id: String,
    pub foracid: String,
    pub int_rate: f64,
    pub ei_schm_flag: String,
    pub ei_formula_flg: String,
    pub ei_int_calc_freq: String,
    pub ei_method: String,
    pub interest_rate_available: String,
    pub repricing_freq: String,
    pub ir_freq: String,
    pub float_rate_benchmark: String,
    pub is_renewed: String,
    pub npa: String,
    pub no_of_flows: i64,
    pub flow_start_date: NaiveDate,
    pub flow_amt: f64,
    pub lr_freq_type: String,
    pub num_of_dmds: i64,
    pub cashflow_code: String,
    pub cashflow_type: String,
    pub cust_country_cd: String,
    pub cust_credit_rating: String,
    pub cust_sector_cd: String,
    pub cust_industry_cd: String,
    pub exchange_rt: f64,
    pub custom1: String,
    pub custom2: String,
    pub npa_classification: String,
    pub floating_type: String,
    pub out_bal_amount: f64,
    pub cust_hlth_code: String,
    pub cust_npa_class: String,
    pub final_npa_class: String,
    pub repricing_plan: String,
    pub next_repricing_date: Option<NaiveDate>,
    pub der_pegged_flg: String,
    pub int_tbl_code: String,
}

impl InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, String> {
        let mut value_iterator = line.split('|');

        let input_account = InputAccount {
            type_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `typeid`.".to_string());
                }
            },
            subtype_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `subtype_id`.".to_string());
                }
            },
            dis_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `dis_amt`.".to_string());
                }
            },
            rephasement_principal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `rephasement_principal`.".to_string());
                }
            },
            rep_shdl_num: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `rep_shdl_num`.".to_string());
                }
            },
            rep_shdl_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `rep_shdl_date`.".to_string());
                }
            },
            acid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `acid`.".to_string());
                }
            },
            bacid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `bacid`.".to_string());
                }
            },
            clr_bal_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `clr_bal_amt`.".to_string());
                }
            },
            sanct_lim: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `sanct_lim`.".to_string());
                }
            },
            gl_sub_head_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `gl_sub_head_code`.".to_string());
                }
            },
            schm_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `schm_code`.".to_string());
                }
            },
            schm_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `schm_type`.".to_string());
                }
            },
            acct_crncy_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `acct_crncy_code`.".to_string());
                }
            },
            acct_opn_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse(val),
                None => {
                    return Err("Could not read property `acct_opn_date`.".to_string());
                }
            },
            dis_shdl_num: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `dis_shdl_num`.".to_string());
                }
            },
            dis_shdl_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `dis_shdl_date`.".to_string());
                }
            },
            sol_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `t_name`.".to_string());
                }
            },
            cust_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `cust_name`.".to_string());
                }
            },
            nrml_int_pcnt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `nrml_int_pcnt`.".to_string());
                }
            },
            end_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse(val),
                None => {
                    return Err("Could not read property `end_date`.".to_string());
                }
            },
            fixedornot: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `fixedornot`.".to_string());
                }
            },
            cust_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `cust_id`.".to_string());
                }
            },
            foracid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `foracid`.".to_string());
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `int_rate`.".to_string());
                }
            },
            ei_schm_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `ei_schm_flag`.".to_string());
                }
            },
            ei_formula_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `ei_formula_flg`.".to_string());
                }
            },
            ei_int_calc_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `ei_int_calcfreq`.".to_string());
                }
            },
            ei_method: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `ei_method`.".to_string());
                }
            },
            interest_rate_available: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `interest_rate_available`.".to_string());
                }
            },
            repricing_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `repricing_freq`.".to_string());
                }
            },
            ir_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `ir_freq`.".to_string());
                }
            },
            float_rate_benchmark: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `float_rate_benchmark_int`.".to_string());
                }
            },
            is_renewed: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `is_renewed`.".to_string());
                }
            },
            npa: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `npa`.".to_string());
                }
            },
            no_of_flows: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `no_of_flows`.".to_string());
                }
            },
            flow_start_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse(val),
                None => {
                    return Err("Could not read property `flow_start_date`.".to_string());
                }
            },
            flow_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `flow_amt`.".to_string());
                }
            },
            lr_freq_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `lr_freq_type`.".to_string());
                }
            },
            num_of_dmds: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `num_of_dmds`.".to_string());
                }
            },
            cashflow_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `cashflow_code`.".to_string());
                }
            },
            cashflow_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `cashflow_type`.".to_string());
                }
            },
            cust_country_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `cust_country_cd`.".to_string());
                }
            },
            cust_credit_rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `cust_credit_rating`.".to_string());
                }
            },
            cust_sector_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `cust_sector_cd`.".to_string());
                }
            },
            cust_industry_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `cust_industry_cd`.".to_string());
                }
            },
            exchange_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `exchange_rt`.".to_string());
                }
            },
            custom1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `cutom1`.".to_string());
                }
            },
            custom2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `custom2`.".to_string());
                }
            },
            npa_classification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `npa_classification`.".to_string());
                }
            },
            floating_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `floating_type`.".to_string());
                }
            },
            out_bal_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `out_bal_amount`.".to_string());
                }
            },
            cust_hlth_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `cust_hlth_code`.".to_string());
                }
            },
            cust_npa_class: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `cust_npa_class`.".to_string());
                }
            },
            final_npa_class: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `final_npa_class`.".to_string());
                }
            },
            repricing_plan: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `repricing_plan`.".to_string());
                }
            },
            next_repricing_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `next_repricing_date`.".to_string());
                }
            },
            der_pegged_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `der_pegged_flg`.".to_string());
                }
            },
            int_tbl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `int_tbl_code`.".to_string());
                }
            },
        };

        Ok(input_account)
    }
}
