use crate::configuration_parameters::ConfigurationParameters;
use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub acid: String,
    pub foracid: String,
    pub sol_id: String,
    pub acct_opn_date: Option<NaiveDate>,
    pub gl_sub_head_code: String,
    pub schm_code: String,
    pub schm_type: String,
    pub acct_crncy_code: String,
    pub rep_shdl_num: i64,
    pub rep_shdl_date: Option<NaiveDate>,
    pub dis_shdl_num: i64,
    pub dis_shdl_date: Option<NaiveDate>,
    pub dis_amt: f64,
    pub clr_bal_amt: f64,
    pub sanct_lim: f64,
    pub rephasement_principal: f64,
    pub ei_perd_end_date: Option<NaiveDate>,
    pub cust_id: String,
    pub cust_name: String,
    pub ei_schm_flg: String,
    pub int_basis: String,
    pub ei_formula_flg: String,
    pub ei_intcalc_freq: String,
    pub ei_method: String,
    pub int_rate: f64,
    pub int_type: String,
    pub next_repricing_date: Option<NaiveDate>,
    pub last_repricing_date: Option<NaiveDate>,
    pub repricing_freq: String,
    pub float_rate_benchmark: String,
    pub spread: f64,
    pub npa_flg: String,
    pub npa_classification: String,
    pub npa_amt: f64,
    pub cust_country_cd: String,
    pub cust_credit_rating: String,
    pub cust_sector_cd: String,
    pub cust_industry_cd: String,
    pub exchangert: f64,
    pub custom1: String,
    pub custom2: String,
    pub custom3: String,
    pub gnt_type: String,
    pub status_code: String,
    pub occupation_code: String,
    pub sector: String,
    pub sector_code: String,
    pub subsector_code: String,
    pub staffflag: String,
    pub cre_free_text_1: String,
    pub prov_perc: i64,
    pub ltv: f64,
    pub npa_prov: f64,
    pub dumm3: String,
    pub dumm4: String,
    pub dumm5: String,
    pub dumm6: String,
    pub dumm7: String,
    pub dumm8: f64,
    pub dumm9: f64,
    pub dumm10: f64,
    pub constcatgorycode: String,
    pub ratingagc: String,
    pub rating: String,
    pub supperannuation_flag: String,
    pub turn_amt1: f64,
    pub turn_amt2: f64,
    pub turn_amt3: f64,
    pub ftp_char1: String,
    pub ftp_char2: String,
    pub ftp_amt1: f64,
    pub ftp_amt2: f64,
    pub ftp_date1: Option<NaiveDate>,
    pub ftp_date2: Option<NaiveDate>,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            acid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acid`.");
                }
            },
            foracid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `foracid`.");
                }
            },
            sol_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sol_id`.");
                }
            },
            acct_opn_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `acct_opn_date`.");
                }
            },
            gl_sub_head_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_sub_head_code`.");
                }
            },
            schm_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `schm_code`.");
                }
            },
            schm_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `schm_type`.");
                }
            },
            acct_crncy_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acct_crncy_code`.");
                }
            },
            rep_shdl_num: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `rep_shdl_num`.");
                }
            },
            rep_shdl_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `rep_shdl_date`.");
                }
            },
            dis_shdl_num: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `dis_shdl_num`.");
                }
            },
            dis_shdl_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `dis_shdl_date`.");
                }
            },
            dis_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `dis_amt`.");
                }
            },
            clr_bal_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `clr_bal_amt`.");
                }
            },
            sanct_lim: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `sanct_lim`.");
                }
            },
            rephasement_principal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rephasement_principal`.");
                }
            },
            ei_perd_end_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `ei_perd_end_date`.");
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
            ei_schm_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ei_schm_flg`.");
                }
            },
            int_basis: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_basis`.");
                }
            },
            ei_formula_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ei_formula_flg`.");
                }
            },
            ei_intcalc_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ei_intcalc_freq`.");
                }
            },
            ei_method: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ei_method`.");
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_rate`.");
                }
            },
            int_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_type`.");
                }
            },
            next_repricing_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_repricing_date`.");
                }
            },
            last_repricing_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_repricing_date`.");
                }
            },
            repricing_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `repricing_freq`.");
                }
            },
            float_rate_benchmark: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `float_rate_benchmark`.");
                }
            },
            spread: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `spread`.");
                }
            },
            npa_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa_flg`.");
                }
            },
            npa_classification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa_classification`.");
                }
            },
            npa_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `npa_amt`.");
                }
            },
            cust_country_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_country_cd`.");
                }
            },
            cust_credit_rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_credit_rating`.");
                }
            },
            cust_sector_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_sector_cd`.");
                }
            },
            cust_industry_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_industry_cd`.");
                }
            },
            exchangert: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `exchangert`.");
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
            custom3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `custom3`.");
                }
            },
            gnt_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gnt_type`.");
                }
            },
            status_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `status_code`.");
                }
            },
            occupation_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `occupation_code`.");
                }
            },
            sector: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sector`.");
                }
            },
            sector_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sector_code`.");
                }
            },
            subsector_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `subsector_code`.");
                }
            },
            staffflag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `staffflag`.");
                }
            },
            cre_free_text_1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cre_free_text_1`.");
                }
            },
            prov_perc: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `prov_perc`.");
                }
            },
            ltv: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ltv`.");
                }
            },
            npa_prov: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `npa_prov`.");
                }
            },
            dumm3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `dumm3`.");
                }
            },
            dumm4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `dumm4`.");
                }
            },
            dumm5: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `dumm5`.");
                }
            },
            dumm6: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `dumm6`.");
                }
            },
            dumm7: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `dumm7`.");
                }
            },
            dumm8: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `dumm8`.");
                }
            },
            dumm9: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `dumm9`.");
                }
            },
            dumm10: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `dumm10`.");
                }
            },
            constcatgorycode: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `constcatgorycode`.");
                }
            },
            ratingagc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ratingagc`.");
                }
            },
            rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rating`.");
                }
            },
            supperannuation_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `supperannuation_flag`.");
                }
            },
            turn_amt1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `turn_amt1`.");
                }
            },
            turn_amt2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `turn_amt2`.");
                }
            },
            turn_amt3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `turn_amt3`.");
                }
            },
            ftp_char1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ftp_char1`.");
                }
            },
            ftp_char2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ftp_char2`.");
                }
            },
            ftp_amt1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ftp_amt1`.");
                }
            },
            ftp_amt2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ftp_amt2`.");
                }
            },
            ftp_date1: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `ftp_date1`.");
                }
            },
            ftp_date2: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `ftp_date2`.");
                }
            },
        };
        Ok(input_account)
    }
}

#[derive(Debug, Clone)]
pub struct RepaymentData {
    pub acid: String,
    pub shdl_num: u16,
    pub num_of_flows: u16,
    pub flow_start_date: NaiveDate,
    pub flow_amt: f64,
    pub lr_freq_type: String,
    pub dmy1: String,
    pub dmy2: String,
    pub flow_id: String,
    pub dmy3: String,
    pub num_of_paid_flows: u16,
    pub lr_freq_start_dd: u16,
    pub lr_holiday_stat: String,
}

impl RepaymentData {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        input_acc: &[&str],
        row: usize,
    ) -> RepaymentData {
        RepaymentData {
            acid: get_str(input_file, input_acc, 0, row),
            shdl_num: get_str(input_file, input_acc, 1, row)
                .parse::<u16>()
                .unwrap_or(0),
            num_of_flows: get_str(input_file, input_acc, 2, row)
                .parse::<u16>()
                .unwrap_or(0),
            flow_start_date: get_date(config_params, input_file, input_acc, 3, row),
            flow_amt: get_str(input_file, input_acc, 4, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            lr_freq_type: get_str(input_file, input_acc, 5, row),
            dmy1: get_str(input_file, input_acc, 6, row),
            dmy2: get_str(input_file, input_acc, 7, row),
            flow_id: get_str(input_file, input_acc, 8, row),
            dmy3: get_str(input_file, input_acc, 9, row),
            num_of_paid_flows: get_str(input_file, input_acc, 10, row)
                .parse::<u16>()
                .unwrap_or(0),
            lr_freq_start_dd: get_str(input_file, input_acc, 11, row)
                .parse::<u16>()
                .unwrap_or(0),
            lr_holiday_stat: get_str(input_file, input_acc, 12, row),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OverdueData {
    pub acid: String,
    pub ovd_amt: f64,
    pub ovd_date: NaiveDate,
}

impl OverdueData {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        input_acc: &[&str],
        row: usize,
    ) -> OverdueData {
        OverdueData {
            acid: get_str(input_file, input_acc, 0, row),
            ovd_amt: get_str(input_file, input_acc, 1, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            ovd_date: get_date(config_params, input_file, input_acc, 2, row),
        }
    }
}

pub fn get_str(input_file: &str, data: &[&str], index: usize, row: usize) -> String {
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
    config_params: &ConfigurationParameters,
    input_file: &str,
    data: &[&str],
    index: usize,
    row: usize,
) -> NaiveDate {
    let date_parser = rbdate::DateParser::new("%d-%m-%Y".to_string(), false);
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
