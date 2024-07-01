use super::*;

pub fn stamp_margin(
    mut one_acc_out: &mut OneAccountView,
    static_params: &mut StaticParams,
    dyn_params: &mut DynamicParams,
    method: i32,
    derived_fields: &mut DerivedFields,
    config_params: &ConfigurationParameters,
) {
    let mut adj_rate = DEFAULT_FLOAT;
    // Margin Method 1 - get the adj margin from bal slab file
    if method == 1041 {
        adj_rate = static_params
            .bal_slab
            .get_adj_rate(one_acc_out.cust_agg_bal);
    }
    // Margin Method 3 - get the margin based on adj rules file
    if method == 1043 {
        let (fix_adjs, var_adjs) = stamp_adjs(&mut one_acc_out, &static_params, &derived_fields);
        adj_rate = fix_adjs + var_adjs;
    }
    let ttl_bals = TotalBalances::new(adj_rate, 0.0, &static_params);
    // Margin method 2
    if method == 1042 {
        let mut lst_bm: IntermediateBMPoints = Vec::new();
        let (bm_key, full_file_path) = get_base_curve_file_path(
            one_acc_out,
            static_params,
            derived_fields.parsed_method.curve_pick_date,
            derived_fields.basecurve,
        );

        let mut yield_rate = DEFAULT_FLOAT;
        if Path::new(&full_file_path).exists() {
            if let Some(prev_bm_rate) = static_params.saved_bm_rates.get(&bm_key) {
                lst_bm = prev_bm_rate.to_vec();
            } else {
                bm_reader::get_bm_points(
                    &full_file_path,
                    derived_fields.parsed_method.curve_pick_date,
                    &mut lst_bm,
                );
                static_params.saved_bm_rates.insert(bm_key, lst_bm.clone());
            }
            yield_rate = calc_yield_rate(
                &mut lst_bm,
                0,
                static_params.config_params.is_extrapolation_req(),
                static_params.config_params.bmrate_accuracy().to_string(),
            )
            .unwrap_or(DEFAULT_FLOAT);
            ig_neg_val_f64(&mut yield_rate);
            one_acc_out.base_rate = yield_rate;
        } else {
            log_debug!(
                static_params.log,
                "Benchmark file not found at path: {} for account: {}.",
                full_file_path,
                one_acc_out.account_id
            );
        }
        one_acc_out.ftp_rate = yield_rate;
    } else {
        one_acc_out.ftp_rate = one_acc_out.int_rate + adj_rate;
        one_acc_out.base_rate = one_acc_out.int_rate;
    }

    let avg_bal = get_avg_bal(&mut one_acc_out, &static_params, &dyn_params);
    one_acc_out.ftp_amt_ccy = calc_int(
        avg_bal,
        one_acc_out.ftp_rate,
        ttl_bals.run_duration as f64 / static_params.no_of_days_in_year as f64,
    );
    one_acc_out.ftp_amt_hcy = one_acc_out.ftp_amt_ccy;

    if static_params.config_params.is_int_calc_required() {
        one_acc_out.acr_int_amt_ccy = calc_int(
            rounded_f64(avg_bal, config_params.bal_prec()),
            rounded_f64(one_acc_out.int_rate, config_params.rate_prec()),
            ttl_bals.run_duration as f64 / static_params.no_of_days_in_year as f64,
        );
    } else {
        let int_income_expense = get_int_exp(&mut one_acc_out, &static_params, &dyn_params);
        one_acc_out.acr_int_amt_ccy = int_income_expense;
    }

    one_acc_out.acr_int_amt_hcy = one_acc_out.acr_int_amt_ccy;
    if method != 1043 {
        stamp_default(&mut one_acc_out);
    }
}