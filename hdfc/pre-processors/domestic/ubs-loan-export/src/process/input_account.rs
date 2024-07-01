use crate::configuration_parameters::ConfigurationParameters;
use calamine::DataType;
use rbdate::NaiveDate;

#[derive(Debug, Clone, Default)]
pub struct NpaData {
    pub src_system: String,
    pub accref_num: String,
    pub spec_prov: f64,
    pub tot_prov: i64,
    pub net_npa: f64,
    pub npa_date: String,
    pub asst_class: String,
}

impl NpaData {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        data: &[&str],
        row: usize,
    ) -> NpaData {
        NpaData {
            src_system: get_str(input_file, data, 0, row),
            accref_num: get_str(input_file, data, 1, row),
            spec_prov: get_str(input_file, data, 2, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            tot_prov: get_str(input_file, data, 3, row)
                .parse::<i64>()
                .unwrap_or(0),
            net_npa: get_str(input_file, data, 4, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            npa_date: get_str(input_file, data, 5, row),
            asst_class: get_str(input_file, data, 6, row),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MasterData {
    pub branch: String,
    pub contract_ref_no: String,
    pub counter_party: String,
    pub user_ref_no: String,
    pub cust_name1: String,
    pub contract_ccy: String,
    pub ccy_num: i64,
    pub booking_date: NaiveDate,
    pub value_dt: NaiveDate,
    pub mat_date: NaiveDate,
    pub product_code: String,
    pub product_desc: String,
    pub user_defined_status: String,
    pub payment_method: String,
    pub contract_status: String,
    pub closure_dt: NaiveDate,
    pub amt: f64,
    pub lcy_amt: f64,
    pub ac_ccy_outstand_bal: f64,
    pub lcy_outstand_bal: f64,
    pub gl: String,
    pub description: String,
    pub accrual_freq: String,
    pub rate_code: String,
    pub rate_typ: String,
    pub rate_spread: f64,
    pub int_rate: f64,
    pub last_rep_date: NaiveDate,
    pub next_rep_date: NaiveDate,
    pub comp_mis_1: i64,
    pub comp_mis_2: i64,
    pub comp_mis_3: i64,
    pub txn_mis_2: i64,
    pub due_dt_principal: NaiveDate,
    pub due_dt_interest: NaiveDate,
    pub frequency: String,
    pub principal_start_date: NaiveDate,
    pub intrest_start_date: NaiveDate,
    pub intcalcmeth: i64,
    pub benchmark_rate: String,
    pub riskbase_pricing: String,
    pub weaker_section: String,
    pub msme: String,
    pub call: String,
    pub call_option_date: NaiveDate,
    pub put: String,
    pub put_option_date: NaiveDate,
    pub new_last_reset_date: NaiveDate,
    pub new_next_reset_date: NaiveDate,
}

impl MasterData {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        data: &[&str],
        row: usize,
    ) -> MasterData {
        MasterData {
            branch: get_str(input_file, data, 0, row),
            contract_ref_no: get_str(input_file, data, 1, row),
            counter_party: get_str(input_file, data, 2, row),
            user_ref_no: get_str(input_file, data, 3, row),
            cust_name1: get_str(input_file, data, 4, row),
            contract_ccy: get_str(input_file, data, 5, row),
            ccy_num: get_int(input_file, data, 6, row),
            booking_date: get_date(config_params, input_file, data, 7, row),
            value_dt: get_date(config_params, input_file, data, 8, row),
            mat_date: get_date(config_params, input_file, data, 9, row),
            product_code: get_str(input_file, data, 10, row),
            product_desc: get_str(input_file, data, 11, row),
            user_defined_status: get_str(input_file, data, 12, row),
            payment_method: get_str(input_file, data, 13, row),
            contract_status: get_str(input_file, data, 14, row),
            closure_dt: get_date(config_params, input_file, data, 15, row),
            amt: get_float(input_file, data, 16, row),
            lcy_amt: get_float(input_file, data, 17, row),
            ac_ccy_outstand_bal: get_float(input_file, data, 18, row),
            lcy_outstand_bal: get_float(input_file, data, 19, row),
            gl: get_str(input_file, data, 20, row),
            description: get_str(input_file, data, 21, row),
            accrual_freq: get_str(input_file, data, 22, row),
            rate_code: get_str(input_file, data, 23, row),
            rate_typ: get_str(input_file, data, 24, row),
            rate_spread: get_float(input_file, data, 25, row),
            int_rate: get_float(input_file, data, 26, row),
            last_rep_date: get_date(config_params, input_file, data, 27, row),
            next_rep_date: get_date(config_params, input_file, data, 28, row),
            comp_mis_1: get_int(input_file, data, 29, row),
            comp_mis_2: get_int(input_file, data, 30, row),
            comp_mis_3: get_int(input_file, data, 31, row),
            txn_mis_2: get_int(input_file, data, 32, row),
            due_dt_principal: get_date(config_params, input_file, data, 33, row),
            due_dt_interest: get_date(config_params, input_file, data, 34, row),
            frequency: get_str(input_file, data, 35, row),
            principal_start_date: get_date(config_params, input_file, data, 36, row),
            intrest_start_date: get_date(config_params, input_file, data, 37, row),
            intcalcmeth: get_int(input_file, data, 38, row),
            benchmark_rate: get_str(input_file, data, 39, row),
            riskbase_pricing: get_str(input_file, data, 40, row),
            weaker_section: get_str(input_file, data, 41, row),
            msme: get_str(input_file, data, 42, row),
            call: get_str(input_file, data, 43, row),
            call_option_date: get_date(config_params, input_file, data, 44, row),
            put: get_str(input_file, data, 45, row),
            put_option_date: get_date(config_params, input_file, data, 46, row),
            new_last_reset_date: get_date(config_params, input_file, data, 47, row),
            new_next_reset_date: get_date(config_params, input_file, data, 48, row),
        }
    }
}

#[derive(Debug, Clone, Default)]

pub struct CashflowData {
    pub reference: String,
    pub component: String,
    pub schedule_due_dt: NaiveDate,
    pub amount_due: f64,
    pub amount_settled: f64,
}

impl CashflowData {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        data: &[&str],
        row: usize,
    ) -> CashflowData {
        CashflowData {
            reference: get_str(input_file, data, 0, row),
            component: get_str(input_file, data, 1, row),
            schedule_due_dt: get_date(config_params, input_file, data, 2, row),
            amount_due: get_float(input_file, data, 3, row),
            amount_settled: get_float(input_file, data, 4, row),
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
    let date_parser = rbdate::DateParser::new("%d-%b-%Y".to_string(), false);
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
