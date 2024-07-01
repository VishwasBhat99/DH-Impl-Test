use calamine::DataType;
use chrono::NaiveDate;
extern crate serde;
extern crate serde_derive;
use crate::configuration_parameters::ConfigurationParameters;

use self::serde_derive::Deserialize;
use self::serde_derive::Serialize;

#[derive(Debug, Default)]
pub struct CashflowData {
    pub date_1: String,
    pub intrestportion: String,
    pub principal_payment: String,
    pub ubs_account_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]

pub struct YieldData {
    pub account_no: String,
    pub yield_rate: f64,
}

#[derive(Debug, Default)]

pub struct MasterFileData {
    pub ubs_account_number: String,
    pub cust_id: String,
    pub deal_name: String,
    pub principal_amount: String,
    pub deal_start_date: String,
    pub cf_end_date: String,
    pub accrued_interest: String,
    pub compounding_frequency: String,
    pub deal_value: String,
    pub gl: String,
    pub system: String,
    pub curr_nominal_int_rate: String,
    pub ratings: String,
    pub rating_aggency: String,
    pub asset_class: String,
    pub division: String,
    pub typ: String,
    pub originator: String,
    pub contract_yield: String,
    pub current_annual_yield: String,
    pub reset_frequency: String,
    pub interest_rate_type: String,
    pub expected_rate_reset_date: String,
    pub mis_code_1: String,
    pub mis_code_2: String,
}

impl CashflowData {
    pub fn new_from_xlsx(cashflow_data: &[DataType]) -> CashflowData {
        CashflowData {
            date_1: get_str_from_xlsx(cashflow_data, 2),
            intrestportion: get_str_from_xlsx(cashflow_data, 5),
            principal_payment: get_str_from_xlsx(cashflow_data, 6),
            ubs_account_number: get_str_from_xlsx(cashflow_data, 8),
        }
    }
}

impl MasterFileData {
    pub fn new_from_xlsx(master_data: &[DataType]) -> MasterFileData {
        MasterFileData {
            ubs_account_number: get_str_from_xlsx(master_data, 0),
            cust_id: get_str_from_xlsx(master_data, 1),
            deal_name: get_str_from_xlsx(master_data, 3),
            principal_amount: get_str_from_xlsx(master_data, 4),
            deal_start_date: get_str_from_xlsx(master_data, 5),
            cf_end_date: get_str_from_xlsx(master_data, 7),
            accrued_interest: get_str_from_xlsx(master_data, 9),
            compounding_frequency: get_str_from_xlsx(master_data, 10),
            deal_value: get_str_from_xlsx(master_data, 11),
            gl: get_str_from_xlsx(master_data, 12),
            system: get_str_from_xlsx(master_data, 13),
            curr_nominal_int_rate: get_str_from_xlsx(master_data, 14),
            ratings: get_str_from_xlsx(master_data, 15),
            rating_aggency: get_str_from_xlsx(master_data, 17),
            asset_class: get_str_from_xlsx(master_data, 18),
            division: get_str_from_xlsx(master_data, 19),
            typ: get_str_from_xlsx(master_data, 20),
            originator: get_str_from_xlsx(master_data, 21),
            contract_yield: get_str_from_xlsx(master_data, 24),
            current_annual_yield: get_str_from_xlsx(master_data, 25),
            reset_frequency: get_str_from_xlsx(master_data, 26),
            interest_rate_type: get_str_from_xlsx(master_data, 27),
            expected_rate_reset_date: get_str_from_xlsx(master_data, 29),
            mis_code_1: get_str_from_xlsx(master_data, 31),
            mis_code_2: get_str_from_xlsx(master_data, 32),
        }
    }
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
        .replace("\n"," ")
        .trim()
        .to_string()
}
