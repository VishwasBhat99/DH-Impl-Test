use sdb_day_convention::Conventions;

use self::acc_level::stamp_acc_level;
pub use self::bm_rates::*;
use self::cf_level::stamp_cf_level;
use self::default::stamp_default;
use self::derive_output_fields::*;
use self::lock::stamp_lock;
use self::margin::stamp_margin;
use self::rep_lock::stamp_rep_lock;
use super::*;
use std::path::Path;

mod acc_level;
mod bm_rates;
mod cf_level;
mod default;
mod derive_output_fields;
mod lock;
mod margin;
mod rep_lock;

pub fn calc_ftp(
    mut one_acc_out: &mut OneAccountView,
    mut static_params: &mut StaticParams,
    mut dyn_params: &mut DynamicParams,
    mut derived_fields: &mut DerivedFields,
    old_acc_map: &mut OldAccountMap,
    config_params: &ConfigurationParameters,
) {
    // Take the interest rate from amb file
    if static_params.config_params.is_int_rate_from_amb() {
        one_acc_out.int_rate = get_int_rate(&mut one_acc_out, &static_params, &dyn_params);
    } else {
        one_acc_out.int_rate = one_acc_out.int_rate;
    }
    let avg_bal = get_avg_bal(&mut one_acc_out, &static_params, &dyn_params);
    one_acc_out.avg_balance_ccy = avg_bal;
    one_acc_out.avg_balance_hcy = avg_bal;
    // assign the derived LLG id to rl1 field
    one_acc_out.rl1 = derived_fields.llg_id.to_string();
    // get the day count basis value evaluated
    one_acc_out.day_count_basis = match static_params.config_params.day_count_basis() {
        Conventions::ACTbyACT => "ACTbyACT".to_string(),
        Conventions::ACTby360 => "ACTby360".to_string(),
        Conventions::Thirtyby360 => "Thirtyby360".to_string(),
        Conventions::ACTby365 => "ACTby365".to_string(),
        Conventions::AccruedThirtyby360 => panic!("Invalid day convention"),
    };
    let inter_rate: f64 = one_acc_out.int_rate.to_string().parse().unwrap_or(0.0);
    let days: f64 = rbdate::get_days_from_month(*static_params.config_params.to_date())
        .to_string()
        .parse()
        .unwrap_or(0.0);
    if static_params.config_params.is_int_calc_required() {
        one_acc_out.acr_int_amt_ccy = calc_int(
            avg_bal,
            inter_rate,
            days / static_params.no_of_days_in_year as f64,
        );
    } else {
        one_acc_out.acr_int_amt_ccy = get_int_exp(&mut one_acc_out, &static_params, &dyn_params);
    }
    one_acc_out.acr_int_amt_hcy = one_acc_out.acr_int_amt_ccy;

    // assign a_or_l field based on rule evaluated
    one_acc_out.a_or_l = derived_fields.a_or_l_value.to_string();

    append_rules_based_dates(one_acc_out, &derived_fields.parsed_method);
    one_acc_out.ftp_method = get_method_name(derived_fields.method_id).to_string();
    match derived_fields.method_id {
        1001 => stamp_acc_level(
            &mut one_acc_out,
            &mut static_params,
            &mut dyn_params,
            &derived_fields,
            config_params,
        ),
        1002 => stamp_acc_level(
            &mut one_acc_out,
            &mut static_params,
            &mut dyn_params,
            &derived_fields,
            config_params,
        ),
        1003 => stamp_acc_level(
            &mut one_acc_out,
            &mut static_params,
            &mut dyn_params,
            &derived_fields,
            config_params,
        ),
        1011 => stamp_cf_level(
            &mut one_acc_out,
            &mut static_params,
            &mut dyn_params,
            &mut derived_fields,
            config_params,
        ),
        1012 => stamp_cf_level(
            &mut one_acc_out,
            &mut static_params,
            &mut dyn_params,
            &mut derived_fields,
            config_params,
        ),
        1013 => stamp_cf_level(
            &mut one_acc_out,
            &mut static_params,
            &mut dyn_params,
            &mut derived_fields,
            config_params,
        ),
        1014 => stamp_cf_level(
            &mut one_acc_out,
            &mut static_params,
            &mut dyn_params,
            &mut derived_fields,
            config_params,
        ),
        1015 => stamp_cf_level(
            &mut one_acc_out,
            &mut static_params,
            &mut dyn_params,
            &mut derived_fields,
            config_params,
        ),
        1021 => stamp_cf_level(
            &mut one_acc_out,
            &mut static_params,
            &mut dyn_params,
            &mut derived_fields,
            config_params,
        ),
        1022 => stamp_acc_level(
            &mut one_acc_out,
            &mut static_params,
            &mut dyn_params,
            &derived_fields,
            config_params,
        ),
        1023 => stamp_acc_level(
            &mut one_acc_out,
            &mut static_params,
            &mut dyn_params,
            &derived_fields,
            config_params,
        ),
        1031 => {
            if let Some(rates) = old_acc_map.get(&one_acc_out.account_id) {
                if rates.method == derived_fields.method_id || rates.method == 0 {
                    stamp_lock(
                        &mut one_acc_out,
                        &mut static_params,
                        &mut dyn_params,
                        derived_fields,
                        rates,
                        config_params,
                    );
                } else {
                    stamp_cf_level(
                        &mut one_acc_out,
                        &mut static_params,
                        &mut dyn_params,
                        &mut derived_fields,
                        config_params,
                    );
                }
            } else {
                stamp_cf_level(
                    &mut one_acc_out,
                    &mut static_params,
                    &mut dyn_params,
                    &mut derived_fields,
                    config_params,
                );
            }
        }
        1032 => {
            if let Some(rates) = old_acc_map.get(&one_acc_out.account_id) {
                if rates.method == derived_fields.method_id || rates.method == 0 {
                    stamp_lock(
                        &mut one_acc_out,
                        &mut static_params,
                        &mut dyn_params,
                        derived_fields,
                        rates,
                        config_params,
                    );
                } else {
                    stamp_acc_level(
                        &mut one_acc_out,
                        &mut static_params,
                        &mut dyn_params,
                        &derived_fields,
                        config_params,
                    );
                }
            } else {
                stamp_acc_level(
                    &mut one_acc_out,
                    &mut static_params,
                    &mut dyn_params,
                    &derived_fields,
                    config_params,
                );
            }
        }
        1033 => {
            if let Some(rates) = old_acc_map.get(&one_acc_out.account_id) {
                if rates.method == derived_fields.method_id || rates.method == 0 {
                    stamp_lock(
                        &mut one_acc_out,
                        &mut static_params,
                        &mut dyn_params,
                        derived_fields,
                        rates,
                        config_params,
                    );
                } else {
                    stamp_acc_level(
                        &mut one_acc_out,
                        &mut static_params,
                        &mut dyn_params,
                        &derived_fields,
                        config_params,
                    );
                }
            } else {
                stamp_acc_level(
                    &mut one_acc_out,
                    &mut static_params,
                    &mut dyn_params,
                    &derived_fields,
                    config_params,
                );
            }
        }
        1034 => {
            if let Some(rates) = old_acc_map.get(&one_acc_out.account_id) {
                if rates.method == derived_fields.method_id || rates.method == 0 {
                    stamp_lock(
                        &mut one_acc_out,
                        &mut static_params,
                        &mut dyn_params,
                        derived_fields,
                        rates,
                        config_params,
                    );
                } else {
                    stamp_acc_level(
                        &mut one_acc_out,
                        &mut static_params,
                        &mut dyn_params,
                        &derived_fields,
                        config_params,
                    );
                }
            } else {
                stamp_acc_level(
                    &mut one_acc_out,
                    &mut static_params,
                    &mut dyn_params,
                    &derived_fields,
                    config_params,
                );
            }
        }
        1036 => {
            if let Some(rates) = old_acc_map.get(&one_acc_out.account_id) {
                if rates.method == derived_fields.method_id || rates.method == 0 {
                    stamp_rep_lock(
                        &mut one_acc_out,
                        &mut static_params,
                        &mut dyn_params,
                        derived_fields,
                        rates,
                        config_params,
                    );
                } else {
                    stamp_acc_level(
                        &mut one_acc_out,
                        &mut static_params,
                        &mut dyn_params,
                        &derived_fields,
                        config_params,
                    );
                }
            } else {
                stamp_acc_level(
                    &mut one_acc_out,
                    &mut static_params,
                    &mut dyn_params,
                    &derived_fields,
                    config_params,
                );
            }
        }
        1041 => stamp_margin(
            &mut one_acc_out,
            &mut static_params,
            &mut dyn_params,
            derived_fields.method_id,
            derived_fields,
            config_params,
        ),
        1042 => stamp_margin(
            &mut one_acc_out,
            &mut static_params,
            &mut dyn_params,
            derived_fields.method_id,
            derived_fields,
            config_params,
        ),
        1043 => stamp_margin(
            &mut one_acc_out,
            &mut static_params,
            &mut dyn_params,
            derived_fields.method_id,
            derived_fields,
            config_params,
        ),
        _ => stamp_default(&mut one_acc_out),
    }
}