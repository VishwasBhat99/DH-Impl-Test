use std::collections::HashMap;

use super::account::AccData;
use aggregator::account_field_names::AccFieldNames;
use aggregator::llg_key::LLGKey;
use configuration_parameters::ConfigurationParameters;
use rbdate::{NaiveDate, NaiveDateTime};
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::{get_field_value, AccountWithCFs};
use sdb_dyn_proto_rdr::reader::Reader;
use slog::Logger;

#[allow(dead_code, unused_imports)]
pub fn fetch_acc_data(
    mut account: AccountWithCFs,
    keys: &AccFieldNames,
    num_day_bkts: &Vec<i64>,
    config_params: &ConfigurationParameters,
    _logger: &Logger,
    exrt_map: &mut HashMap<String, f64>,
) -> AccData {
    let tot_bkts;
    if config_params.final_bkt_required() == true {
        tot_bkts = num_day_bkts.len() + 1;
    } else {
        tot_bkts = num_day_bkts.len();
    }
    let mut acc_data: Vec<f64> = vec![0.0; tot_bkts];

    // takes default empty values if cashflows are not present

    let default_vec = Vec::new();

    let reader = Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let base_ccy = config_params.base_ccy().to_string();
    let ccy = get_field_value(&account, &reader, keys.currency.to_string())
        .unwrap_or(base_ccy.to_string());

    let mut llg_id = 0;
    if config_params.rules_file_path() != "NONE" {
        let rules = AggRules::new_from_path(config_params.rules_file_path(), &reader);
        llg_id = match rules.llg_for_acc(&account) {
            Some(c) => c.llg,
            None => config_params.default_llg(),
        };
    }

    if &keys.cashflows == "cashflows" {
        // takes default empty values if cashflows are not present
        let mut account_of_cfs = account
            .remove_cfs_for_key(&keys.cashflows)
            .unwrap_or(default_vec);

        for cf in account_of_cfs.iter_mut() {
            if config_params.amt_type() == "INT" {
                get_acc_data(
                    config_params,
                    &mut acc_data,
                    cf.get_date(),
                    if !config_params.is_consolidated() {
                        cf.get_interest_amount() * exrt_map.get(&ccy.to_string()).unwrap_or(&1.0)
                    } else {
                        cf.get_interest_amount()
                    },
                    tot_bkts,
                    num_day_bkts,
                );
            } else {
                get_acc_data(
                    config_params,
                    &mut acc_data,
                    cf.get_date(),
                    if !config_params.is_consolidated() {
                        cf.get_principal_amount() * exrt_map.get(&ccy.to_string()).unwrap_or(&1.0)
                    } else {
                        cf.get_principal_amount()
                    },
                    tot_bkts,
                    num_day_bkts,
                );
            }
        }
    } else {
        let mut amount = get_field_value(&account, &reader, keys.cashflows.to_string())
            .unwrap()
            .parse::<f64>()
            .expect("Cannot parse amount to `f64`.");
        if !config_params.is_consolidated() {
            amount = amount * exrt_map.get(&ccy.to_string()).unwrap_or(&1.0)
        }
        let bucket_number = config_params.is_nonmat_bucket_available();
        if bucket_number != "NA" {
            let bkt_idx = get_field_value(&account, &reader, bucket_number.to_string())
                .unwrap()
                .parse::<usize>()
                .expect("cannot parse to usize");
            let len = acc_data.len();
            if bkt_idx > len {
                acc_data[len - 1] += amount;
            } else {
                acc_data[bkt_idx - 1] += amount;
            }
        } else {
            acc_data[0] += amount;
        }
    }

    let def_val = "NA".to_string();
    let scheme_id = get_field_value(&account, &reader, keys.scheme_id.to_string())
        .unwrap_or(def_val.to_string());
    let rate_flag = account
        .get_string_for_key(&keys.rate_flag)
        .unwrap_or(&def_val);
    let rep_index = account
        .get_string_for_key(&keys.rep_index)
        .unwrap_or(&def_val);
    let alm_line = account
        .get_string_for_key(&keys.alm_line)
        .unwrap_or(&def_val);
    let al_line = account
        .get_string_for_key(&keys.al_line)
        .unwrap_or(&def_val);
    let p1 = account.get_string_for_key(&keys.p1).unwrap_or(&def_val);
    // writing report id in place of p2
    // This is temp and report id needs to be brought as a separate field
    let p2 = config_params.report_id();
    // writing llg id to P3 if rules file is present
    let p3;
    if config_params.rules_file_path() != "NONE" {
        p3 = llg_id.to_string();
    } else {
        p3 = account
            .get_string_for_key(&keys.p3)
            .unwrap_or(&def_val)
            .to_string();
    }
    let grp_key = LLGKey::new(
        scheme_id,
        rate_flag.to_string(),
        rep_index.to_string(),
        alm_line.to_string(),
        al_line.to_string(),
        p1.to_string(),
        p2.to_string(),
        p3.to_string(),
    );

    AccData {
        grp_key: grp_key,
        acc_data: acc_data,
    }
}

pub fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}

pub fn get_acc_data(
    config_params: &ConfigurationParameters,
    acc_data: &mut Vec<f64>,
    date: i64,
    amount: f64,
    tot_bkts: usize,
    num_day_bkts: &Vec<i64>,
) {
    let mut is_bucketed = false;
    let cf_date = naivedate_from_timestamp(date);
    let cf_tenor = if &cf_date > config_params.as_on_date() {
        rbdate::num_days_start_to_end(*config_params.as_on_date(), cf_date)
    } else {
        -1
    };
    let mut idx: usize = 0;
    for day in num_day_bkts {
        if &cf_tenor < day {
            acc_data[idx] += amount;
            is_bucketed = true;
            break;
        } else {
            idx += 1;
        }
    }
    //The cfs interest/principal amounts above 180 Months are bucketed in final bucket
    if is_bucketed == false && idx == tot_bkts - 1 && config_params.final_bkt_required() == true {
        acc_data[idx] += amount;
    }
}
