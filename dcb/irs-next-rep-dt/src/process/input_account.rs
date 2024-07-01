use crate::configuration_parameters::ConfigurationParameters;
use calamine::DataType;
use rbdate::NaiveDate;

#[derive(Debug, Clone, Default)]
pub struct InputData {
    pub acid: String,
    pub foracid: String,
    pub sol_id: String,
    pub acct_opn_dt: NaiveDate,
    pub gl_sub_head_code: String,
    pub schm_code: String,
    pub schm_typ: String,
    pub acct_crncy_code: String,
    pub rep_shdl_num: i64,
    pub rep_shdl_date: NaiveDate,
    pub dis_shdl_num: i64,
    pub dis_shdl_date: NaiveDate,
    pub dis_amt: f64,
    pub clr_bal_amt: f64,
    pub sanct_lim: String,
    pub rephasement_principal: f64,
    pub ei_perd_end_date: NaiveDate,
    pub cust_id: String,
    pub cust_name: String,
    pub ei_schm_flg: String,
    pub int_basis: String,
    pub ei_formula_flg: String,
    pub ei_intcalc_freq: String,
    pub ei_method: String,
    pub int_rate: f64,
    pub int_type: String,
    pub next_rep_date: NaiveDate,
    pub last_rep_date: NaiveDate,
    pub rep_freq: String,
    pub float_rate_bench_mark: String,
    pub spread: String,
    pub npa_flag: String,
    pub npa_classification: String,
    pub npa_amt: f64,
    pub cust_country_cd: String,
    pub cust_credit_rating: String,
    pub cust_sector_cd: String,
    pub cust_industry_cd: String,
    pub exchange_rt: String,
    pub custom1: String,
    pub custom2: String,
    pub custom3: String,
}

impl InputData {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        data: &[&str],
        row: usize,
    ) -> InputData {
        InputData {
            acid: get_str(input_file, data, 0, row),
            foracid: get_str(input_file, data, 1, row),
            sol_id: get_str(input_file, data, 2, row),
            acct_opn_dt: get_date(config_params, input_file, data, 3, row),
            gl_sub_head_code: get_str(input_file, data, 4, row),
            schm_code: get_str(input_file, data, 5, row),
            schm_typ: get_str(input_file, data, 6, row),
            acct_crncy_code: get_str(input_file, data, 7, row),
            rep_shdl_num: get_int(input_file, data, 8, row),
            rep_shdl_date: get_date(config_params, input_file, data, 9, row),
            dis_shdl_num: get_int(input_file, data, 10, row),
            dis_shdl_date: get_date(config_params, input_file, data, 11, row),
            dis_amt: get_float(input_file, data, 12, row),
            clr_bal_amt: get_float(input_file, data, 13, row),
            sanct_lim: get_str(input_file, data, 14, row),
            rephasement_principal: get_float(input_file, data, 15, row),
            ei_perd_end_date: get_date(config_params, input_file, data, 16, row),
            cust_id: get_str(input_file, data, 17, row),
            cust_name: get_str(input_file, data, 18, row),
            ei_schm_flg: get_str(input_file, data, 19, row),
            int_basis: get_str(input_file, data, 20, row),
            ei_formula_flg: get_str(input_file, data, 21, row),
            ei_intcalc_freq: get_str(input_file, data, 22, row),
            ei_method: get_str(input_file, data, 23, row),
            int_rate: get_float(input_file, data, 24, row),
            int_type: get_str(input_file, data, 25, row),
            next_rep_date: get_date(config_params, input_file, data, 26, row),
            last_rep_date: get_date(config_params, input_file, data, 27, row),
            rep_freq: get_str(input_file, data, 28, row),
            float_rate_bench_mark: get_str(input_file, data, 29, row),
            spread: get_str(input_file, data, 30, row),
            npa_flag: get_str(input_file, data, 31, row),
            npa_classification: get_str(input_file, data, 32, row),
            npa_amt: get_float(input_file, data, 33, row),
            cust_country_cd: get_str(input_file, data, 34, row),
            cust_credit_rating: get_str(input_file, data, 35, row),
            cust_sector_cd: get_str(input_file, data, 36, row),
            cust_industry_cd: get_str(input_file, data, 37, row),
            exchange_rt: get_str(input_file, data, 38, row),
            custom1: get_str(input_file, data, 39, row),
            custom2: get_str(input_file, data, 40, row),
            custom3: get_str(input_file, data, 41, row),
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct RepMaster {
    pub bench_mark_code: String,
    pub f1: String,
    pub incr_bucket:String,
}

impl RepMaster {
    pub fn new_from_excel(data: &[DataType]) -> RepMaster {
        RepMaster {
            bench_mark_code: get_str_from_xlsx(data, 0),
            f1: get_str_from_xlsx(data, 1),
            incr_bucket:get_str_from_xlsx(data, 2)
        }
    }
    pub fn new(input_file: &str, data: &[&str], row: usize) -> RepMaster {
        RepMaster {
            bench_mark_code: get_str(input_file, data, 0, row),
            f1: get_str(input_file, data, 1, row),
            incr_bucket:get_str(input_file, data, 2, row)
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
pub fn get_float(input_file: &str, data: &[&str], index: usize, row: usize) -> f64 {
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
        .parse::<f64>()
        .unwrap_or(0.0)
}
pub fn get_int(input_file: &str, data: &[&str], index: usize, row: usize) -> i64 {
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
        .parse::<i64>()
        .unwrap_or(0)
}
pub fn get_str_from_xlsx(data: &[DataType], index: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` for row: `{:?}`",
                index + 1,
                data
            )
        })
        .to_string()
        .replace("\n", " ")
        .trim()
        .to_string()
}
