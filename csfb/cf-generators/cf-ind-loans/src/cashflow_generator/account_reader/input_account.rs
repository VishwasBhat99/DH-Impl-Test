use rbdate::*;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub acc_no: String,
    pub disbursed_amt: f64,
    pub os_loan_bal_lcy: f64,
    pub int_rate: f64,
    pub ei_amt_crnt: f64,
    pub int_type: String,
    pub os_p_bal_due_local_ccy: f64,
    pub os_i_bal_due_local_ccy: f64,
    pub ei_amt_paid_adv_lcy: f64,
    pub pre_ei_bal_lcy: f64,
    pub acc_open_value_date: Option<NaiveDate>,
    pub maturity_date: Option<NaiveDate>,
    pub ei_start_date_crnt: Option<NaiveDate>,
    pub ei_end_date_crnt: Option<NaiveDate>,
    pub ei_pay_freq_crnt: String,
    pub emi_last_paid_date_crnt: Option<NaiveDate>,
    pub ei_pay_day: i64,
    pub ei_orginal_term: i64,
    pub ei_bal_term: i64,
    pub rep_bm: String,
    pub spread: String,
    pub last_rep_date: Option<NaiveDate>,
    pub next_rep_date: Option<NaiveDate>,
    pub rep_freq: String,
    pub no_ei_structures: i64,
    pub npa_class: String,
    pub remark: String,
    pub months_os_comb: String,
    pub mor_type: String,
    pub from_mor_date: Option<NaiveDate>,
    pub to_mor_date: Option<NaiveDate>,
    pub recalc_ei_amt_flag: String,
    pub mor_int_calc: String,
    pub bullet_pay_flag: String,
    pub restrct_flag: String,
    pub residential_mortgage: String,
    pub risk_weight: String,
    pub internal_rating: String,
    pub external_rating: String,
    pub contractual_tenor: i64,
    pub residual_tenor: i64,
    pub cust_constitution_code: String,
    pub prod_code: String,
    pub p_gl_code: String,
    pub m_npaclass: String,
    pub acrd_int: f64,
    pub cust_id: String,
    pub cust_name: String,
    pub group_id: String,
    pub group_name: String,
    pub branch_code: String,
    pub sector: String,
    pub industry: String,
    pub ltv: String,
    pub overdue_acc: String,
    pub excess_acc: String,
    pub loan_type: String,
    pub resid_int: f64,
    pub ccy: String,
    pub hdfc_ltd_percent: f64,
    pub sec_percent: f64,
    pub overdue_type: String,
    pub alm_line: String,
    pub emi_overdue_gl_cd: i64,
    pub pre_emi_overdue_gl_cd: i64,
    pub excess_emi_gl_cd: i64,
    pub excess_pre_emi_gl_cd: i64,
    pub lcr_fin_non_fin_flag: String,
    pub undrawn_loans: f64,
    pub undrawn_ccod: f64,
    pub purpose: String,
    pub drawing_power: f64,
    pub tenor: String,
    pub turn_over: String,
    pub line_of_activity: String,
    pub rating: String,
    pub net_wt: String,
    pub curr_os_bal: f64,
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
            disbursed_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `disbursed_amt`.");
                }
            },
            os_loan_bal_lcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `os_loan_bal_lcy`.");
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_rate`.");
                }
            },
            ei_amt_crnt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ei_amt_crnt`.");
                }
            },
            int_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_type`.");
                }
            },
            os_p_bal_due_local_ccy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `os_p_bal_due_local_ccy`.");
                }
            },
            os_i_bal_due_local_ccy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `os_i_bal_due_local_ccy`.");
                }
            },
            ei_amt_paid_adv_lcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ei_amt_paid_adv_lcy`.");
                }
            },
            pre_ei_bal_lcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pre_ei_bal_lcy`.");
                }
            },
            acc_open_value_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `acc_open_value_date`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_date`.");
                }
            },
            ei_start_date_crnt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `ei_start_date_crnt`.");
                }
            },
            ei_end_date_crnt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `ei_end_date_crnt`.");
                }
            },
            ei_pay_freq_crnt: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ei_pay_freq_crnt`.");
                }
            },
            emi_last_paid_date_crnt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_last_paid_date_crnt`.");
                }
            },
            ei_pay_day: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `ei_pay_day`.");
                }
            },
            ei_orginal_term: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `ei_orginal_term`.");
                }
            },
            ei_bal_term: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `ei_bal_term`.");
                }
            },
            rep_bm: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rep_bm`.");
                }
            },
            spread: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `spread`.");
                }
            },
            last_rep_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_rep_date`.");
                }
            },
            next_rep_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_rep_date`.");
                }
            },
            rep_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rep_freq`.");
                }
            },
            no_ei_structures: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `no_ei_structures`.");
                }
            },
            npa_class: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa_class`.");
                }
            },
            remark: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `remark`.");
                }
            },
            months_os_comb: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `months_os_comb`.");
                }
            },
            mor_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `mor_type`.");
                }
            },
            from_mor_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `from_mor_date`.");
                }
            },
            to_mor_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `to_mor_date`.");
                }
            },
            recalc_ei_amt_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `recalc_ei_amt_flag`.");
                }
            },
            mor_int_calc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `mor_int_calc`.");
                }
            },
            bullet_pay_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bullet_pay_flag`.");
                }
            },
            restrct_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `restrct_flag`.");
                }
            },
            residential_mortgage: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `residential_mortgage`.");
                }
            },
            risk_weight: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `risk_weight`.");
                }
            },
            internal_rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `internal_rating`.");
                }
            },
            external_rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `external_rating`.");
                }
            },
            contractual_tenor: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `contractual_tenor`.");
                }
            },
            residual_tenor: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `residual_tenor`.");
                }
            },
            cust_constitution_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_constitution_code`.");
                }
            },
            prod_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `prod_code`.");
                }
            },
            p_gl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `p_gl_code`.");
                }
            },
            m_npaclass: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `m_npaclass`.");
                }
            },
            acrd_int: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `acrd_int`.");
                }
            },
            cust_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_id`.");
                }
            },
            cust_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_name`.");
                }
            },
            group_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `group_id`.");
                }
            },
            group_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `group_name`.");
                }
            },
            branch_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `branch_code`.");
                }
            },
            sector: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sector`.");
                }
            },
            industry: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `industry`.");
                }
            },
            ltv: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ltv`.");
                }
            },
            overdue_acc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `overdue_acc`.");
                }
            },
            excess_acc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `excess_acc`.");
                }
            },
            loan_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `loan_type`.");
                }
            },
            resid_int: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `resid_int`.");
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy`.");
                }
            },
            hdfc_ltd_percent: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `hdfc_ltd_percent`.");
                }
            },
            sec_percent: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `sec_percent`.");
                }
            },
            overdue_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `overdue_type`.");
                }
            },
            alm_line: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `alm_line`.");
                }
            },
            emi_overdue_gl_cd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `emi_overdue_gl_cd`.");
                }
            },
            pre_emi_overdue_gl_cd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `pre_emi_overdue_gl_cd`.");
                }
            },
            excess_emi_gl_cd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `excess_emi_gl_cd`.");
                }
            },
            excess_pre_emi_gl_cd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `excess_pre_emi_gl_cd`.");
                }
            },
            lcr_fin_non_fin_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `lcr_fin_non_fin_flag`.");
                }
            },
            undrawn_loans: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `undrawn_loans`.");
                }
            },
            undrawn_ccod: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `undrawn_ccod`.");
                }
            },
            purpose: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `purpose`.");
                }
            },
            drawing_power: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `drawing_power`.");
                }
            },
            tenor: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `tenor`.");
                }
            },
            turn_over: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `turn_over`.");
                }
            },
            line_of_activity: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `line_of_activity`.");
                }
            },
            rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rating`.");
                }
            },
            net_wt: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `net_weight`.");
                }
            },
            curr_os_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `excess_pre_emi_gl_cd`.");
                }
            },
        };
        Ok(input_account)
    }
}
