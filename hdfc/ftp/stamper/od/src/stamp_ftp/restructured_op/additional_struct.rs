use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug)]
pub struct AddOutput {
    pub ftp_month: String,
    pub acc_num: String,
    pub cust_name: String,
    pub avg_bal: f64,
    pub accr_int: f64,
    pub accr_int_rate: f64,
    pub yld_to_call: f64,
    pub int_rate: f64,
    pub base_rate: f64,
    pub adj_lp: f64,
    pub adj_tp: f64,
    pub adj_pcfc: f64,
    pub adj_psl: f64,
    pub adj_ews: f64,
    pub adj_smf: f64,
    pub adj_mo: f64,
    pub adj_8: f64,
    pub adj_9: f64,
    pub adj_10: f64,
    pub final_ftp_rate: f64,
    pub ftp_rate_without_psl: f64,
    pub margin_rate: f64,
    pub base_tpr_amount: f64,
    pub final_ftp_amount: f64,
    pub ftp_amt_without_psl: f64,
    pub psl_amount: f64,
    pub total_lp_amount: f64,
    pub total_psl_amount_without_ews_smf: f64,
    pub total_ews_amount: f64,
    pub total_smf_amount: f64,
    pub margin_amount: f64,
    pub value_date: String,
    pub maturity_date: String,
    pub last_reprice_date: String,
    pub next_reprice_date: String,
    pub mis1: String,
    pub mis2: String,
    pub psl_code: String,
    pub prod_code: String,
    pub rate_flag: String,
    pub branch: String,
    pub source_file_name: String,
    pub currency: String,
    pub gl_code: String,
    pub cust_id: String,
    pub alm_line: String,
    pub trade_date: String,
    pub initial_dep_amt: f64,
    pub current_outstanding: f64,
    pub input_benchmark: String,
    pub pdo: String,
    pub npa: String,
    pub ftp_method: String,
    pub ftp_rate_curve: String,
    pub org_tenor: String,
    pub repricing_tenor: String,
    pub fixed_spread: f64,
    pub variable_spread: f64,
    pub first_month_ftp: f64,
    pub bc_as_on_rule: String,
    pub tenor_start_date_rule: String,
    pub tenor_end_rate_rule: String,
    pub bc_as_on_applied: String,
    pub tenor_start_date_applied: String,
    pub tenor_end_date_applied: String,
    pub concat_4_point: String,
    pub concat_2_point: String,
    pub ews_flag: String,
    pub bdp_division: String,
    pub bdp_coa: String,
    pub adj_id_lp: String,
    pub adj_id_tp: String,
    pub adj_id_pcfc: String,
    pub adj_id_psl: String,
    pub adj_id_ews: String,
    pub adj_id_smf: String,
    pub adj_id_mo: String,
    pub adj_id_8: String,
    pub adj_id_9: String,
    pub adj_id_10: String,
}
#[derive(Debug, Clone, Default)]
pub struct AmbData {
    pub vf_casa_acct_number: String,
    pub nf_casa_avg_bal_cr: f64,
    pub nf_casa_avg_bal_dr: f64,
    pub vf_casa_src_syst: String,
    pub nf_ods_fw_cbrmst_cod_cc_brn: String,
    pub vf_aact_prod_code: String,
    pub vf_cod_mis_comp_1: String,
    pub vf_ods_fw_cbrmst_cod_10: String,
    pub accr_int: f64,
}

impl AmbData {
    pub fn new(input_file: &str, data: &[&str], row: usize) -> AmbData {
        AmbData {
            vf_casa_acct_number: get_str(input_file, data, 0, row),
            nf_casa_avg_bal_cr: get_float(input_file, data, 1, row),
            nf_casa_avg_bal_dr: get_float(input_file, data, 2, row),
            vf_casa_src_syst: get_str(input_file, data, 3, row),
            nf_ods_fw_cbrmst_cod_cc_brn: get_str(input_file, data, 4, row),
            vf_aact_prod_code: get_str(input_file, data, 5, row),
            vf_cod_mis_comp_1: get_str(input_file, data, 6, row),
            vf_ods_fw_cbrmst_cod_10: get_str(input_file, data, 7, row),
            accr_int: get_float(input_file, data, 8, row),
        }
    }
}
pub fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

pub fn days_in_year(year: i32) -> u32 {
    if is_leap_year(year) {
        366
    } else {
        365
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
