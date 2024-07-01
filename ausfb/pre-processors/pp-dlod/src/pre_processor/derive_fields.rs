use super::input_account::{InputAccount, Ref1};
use rbdate::{incr_dt_by_days, incr_dt_by_mon_presrv_eom_checked, incr_dt_by_yrs, NaiveDate};
use std::collections::HashMap;

pub fn get_op_line(
    account: &InputAccount,
    as_on_date: NaiveDate,
    ref1_map: &HashMap<String, Ref1>,
    ref2_map: &HashMap<String, String>,
    ref3_map: &HashMap<String, String>,
    ta_config_map: &HashMap<String, String>,
    dlod_cashflow_map: &HashMap<String, bool>,
    dlod_cashflow_map2: &HashMap<String, bool>,
    odfd_cashflow_map: &HashMap<String, String>,
    rtl_cashflow_map: &HashMap<String, bool>,
    cust_entity_map: &HashMap<String, String>,
    crm_master_map: &HashMap<String, String>,
) -> String {
    let maturity_date =
        NaiveDate::parse_from_str(&account.maturity_date, "%d-%b-%Y").unwrap_or(as_on_date);
    let ref1_values = match ref1_map.get(&account.account_id.replace("'", "").replace("\"", "")) {
        Some(val) => val.to_owned(),
        None => Ref1 {
            asset_type: "NA".to_string(),
            cod_acc_no: account.account_id.to_owned(),
            cod_limit_no: 0,
            loan_limit_amount: 0.0,
            index_code: "NA".to_string(),
            index_name: "NA".to_string(),
            index_rate: 0.0,
            effective_roi: 0.0,
            reset_frequency: "NA".to_string(),
            next_reset_date: account.maturity_date.to_string(),
            tenure: 0.0,
        },
    };
    let next_reset_date = NaiveDate::parse_from_str(&ref1_values.next_reset_date, "%d-%b-%Y")
        .unwrap_or(maturity_date);
    let derived_reset_date = if next_reset_date == NaiveDate::from_ymd(1800, 1, 1)
        || next_reset_date == NaiveDate::from_ymd(1900, 1, 1)
    {
        maturity_date
    } else {
        next_reset_date
    };

    let final_reset_date = if &derived_reset_date >= &as_on_date {
        derived_reset_date
    } else {
        let mut updated_derived_reset_date = derived_reset_date;
        while updated_derived_reset_date <= as_on_date {
            updated_derived_reset_date = add_freq(
                updated_derived_reset_date,
                ref1_values.reset_frequency.to_owned(),
                as_on_date,
            );
        }
        updated_derived_reset_date
    };

    let temp_npa_status = "STANDARD".to_string();
    let npa_status = match ref2_map.get(
        &account
            .account_id
            .to_string()
            .replace("'", "")
            .replace("\"", ""),
    ) {
        Some(val) => val.to_string(),
        None => match ref3_map.get(&account.account_id.replace("'", "").replace("\"", "")) {
            Some(classification) => classification.to_string(),
            None => temp_npa_status,
        },
    };

    let npa_final_status = if npa_status.to_uppercase().contains("SUB-STANDARD") {
        "SUB-STANDARD".to_string()
    } else if npa_status.to_uppercase().contains("LOS") {
        "LOSS".to_string()
    } else if npa_status.to_uppercase().contains("STANDARD") {
        "STANDARD".to_string()
    } else if npa_status.to_uppercase().contains("DOUBTFUL") {
        "DOUBTFUL".to_string()
    } else {
        "STANDARD".to_string()
    };

    let classification = if ta_config_map
        .contains_key(&account.scheme_type.replace("'", "").replace("\"", ""))
    {
        "TA".to_string()
    } else if odfd_cashflow_map.contains_key(&account.account_id.replace("'", "").replace("\"", ""))
    {
        "ODFD".to_string()
    } else {
        match dlod_cashflow_map.get(&account.account_id.replace("'", "").replace("\"", "")) {
            Some(val) => {
                if val == &true {
                    "DLOD-OD-OTHERS".to_string()
                } else {
                    "DLOD".to_string()
                }
            }
            None => {
                match dlod_cashflow_map2.get(&account.account_id.replace("'", "").replace("\"", ""))
                {
                    Some(val) => {
                        if val == &true {
                            "DLOD-OD-OTHERS".to_string()
                        } else {
                            "DLOD".to_string()
                        }
                    }
                    None => {
                        match rtl_cashflow_map
                            .get(&account.account_id.replace("'", "").replace("\"", ""))
                        {
                            Some(val) => {
                                if val == &true {
                                    "RTL-OD-OTHERS".to_string()
                                } else {
                                    "RTL".to_string()
                                }
                            }
                            None => "OD-OTHERS".to_string(),
                        }
                    }
                }
            }
        }
    };
    let lcr_category = match cust_entity_map.get(
        &account
            .customer_type
            .to_string()
            .replace("'", "")
            .replace("\"", ""),
    ) {
        Some(lcr_cat) => lcr_cat.to_owned(),
        None => "NA".to_string(),
    };
    let final_category = match crm_master_map.get(
        &account
            .customer_id
            .to_string()
            .replace("'", "")
            .replace("\"", ""),
    ) {
        Some(final_cat) => final_cat.to_owned(),
        None => lcr_category.to_string(),
    };

    format!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|0.0|0.0|0|0",
    account.customer_id.replace("'", "").replace("\"", ""),
    account.account_id.replace("'", "").replace("\"", ""),
    account.product_type.replace("'", "").replace("\"", ""),
    account.scheme_type.replace("'", "").replace("\"", ""),
    account.product_code.replace("'", "").replace("\"", ""),
    account.currency.replace("'", "").replace("\"", ""),
    account.customer_type.replace("'", "").replace("\"", ""),
    account.gl_account_principal.replace("'", "").replace("\"", ""),
    NaiveDate::parse_from_str(&account.open_date, "%d-%b-%Y").unwrap_or(as_on_date).format("%d-%m-%Y"),
    NaiveDate::parse_from_str(&account.value_date, "%d-%b-%Y").unwrap_or(as_on_date).format("%d-%m-%Y"),
    NaiveDate::parse_from_str(&account.maturity_date, "%d-%b-%Y").unwrap_or(as_on_date).format("%d-%m-%Y"),
    account.limit_amount.replace("'", "").replace("\"", ""),
    account.current_bal_amount.replace("'", "").replace("\"", ""),
    account.flg_fixed_floating.replace("'", "").replace("\"", ""),
    account.interest_paid.replace("'", "").replace("\"", ""),
    account.interest_received.replace("'", "").replace("\"", ""),
    account.flg_performing_npa.replace("'", "").replace("\"", ""),
    ref1_values.asset_type.replace("'", "").replace("\"", ""),
    ref1_values.cod_acc_no.replace("'", "").replace("\"", ""),
    ref1_values.cod_limit_no,
    ref1_values.loan_limit_amount,
    ref1_values.index_code.replace("'", "").replace("\"", ""),
    ref1_values.index_name.replace("'", "").replace("\"", ""),
    ref1_values.index_rate,
    ref1_values.effective_roi,
    ref1_values.reset_frequency.replace("'", "").replace("\"", ""),
    next_reset_date.format("%d-%m-%Y"),
    ref1_values.tenure,
    classification,
    derived_reset_date.format("%d-%m-%Y"),
    final_reset_date.format("%d-%m-%Y"),
    npa_status,
    npa_final_status,
    lcr_category,
    final_category,
    as_on_date.format("%d-%m-%Y"),
    as_on_date.format("%d-%m-%Y"),
    )
}

pub fn add_freq(date: NaiveDate, reset_freq: String, as_on_date: NaiveDate) -> NaiveDate {
    let cap_reset_freq: String = reset_freq.trim().to_uppercase().to_string();
    let next_reset_date = match cap_reset_freq.as_str() {
        "ANNUAL" | "YEARLY" => incr_dt_by_yrs(date, 1),
        "QUARTERLY" => incr_dt_by_mon_presrv_eom_checked(date, 3).unwrap_or(as_on_date),
        "HALFYEARLY" | "HALF YEARLY" | "HALF-YEARLY" => {
            incr_dt_by_mon_presrv_eom_checked(date, 6).unwrap_or(as_on_date)
        }
        "MONTHLY" => incr_dt_by_mon_presrv_eom_checked(date, 1).unwrap_or(as_on_date),
        "BIMONTHLY" | "BI-MONTHLY" | "BI MONTHLY" => incr_dt_by_days(date, 14),
        _ => incr_dt_by_mon_presrv_eom_checked(date, 3).unwrap_or(as_on_date),
    };
    next_reset_date
}

pub fn check_cashflow(date: &NaiveDate, check_status: &bool, as_on_date: &NaiveDate) -> bool {
    //If the condition became true from a previous record cashflow, check_status is false.
    if check_status == &false {
        false
    } else if date < as_on_date {
        true
    } else {
        false
    }
}
