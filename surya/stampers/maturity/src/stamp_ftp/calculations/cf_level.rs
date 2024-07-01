use super::*;

pub fn stamp_cf_level(
    mut one_acc_out: &mut OneAccountView,
    static_params: &mut StaticParams,
    dyn_params: &mut DynamicParams,
    derived_fields: &mut DerivedFields,
    config_params: &ConfigurationParameters,
) {
    let (fix_adjs, var_adjs) = stamp_adjs(&mut one_acc_out, &static_params, &derived_fields);
    let mut ttl_bals = TotalBalances::new(fix_adjs, var_adjs, &static_params);
    calc_bm_rates_cf_level(
        one_acc_out,
        static_params,
        dyn_params,
        derived_fields,
        &mut ttl_bals,
    );

    ttl_bals.derive_rates();

    one_acc_out.base_rate = ttl_bals.base_rate;
    if static_params.config_params.apply_base_curve_2() {
        one_acc_out.ftp_rate = fix_adjs + var_adjs + one_acc_out.base_rate;
        for index in 0..static_params.config_params.adj_count_for_bc_2() {
            one_acc_out.ftp_rate += one_acc_out.adj_rates[index as usize];
        }
    } else {
        one_acc_out.ftp_rate = fix_adjs + var_adjs + one_acc_out.base_rate
    }

    let avg_bal = get_avg_bal(&mut one_acc_out, &static_params, &dyn_params);

    one_acc_out.ftp_amt_ccy = calc_int(
        rounded_f64(avg_bal, config_params.bal_prec()),
        rounded_f64(one_acc_out.ftp_rate, config_params.rate_prec()),
        ttl_bals.run_duration as f64 / static_params.no_of_days_in_year as f64,
    );

    one_acc_out.ftp_amt_hcy = one_acc_out.ftp_amt_ccy;

    if static_params.config_params.is_int_calc_required() {
        one_acc_out.acr_int_amt_ccy = calc_int(
            avg_bal,
            one_acc_out.int_rate,
            ttl_bals.run_duration as f64 / static_params.no_of_days_in_year as f64,
        );
    } else {
        let int_income_expense = get_int_exp(&mut one_acc_out, &static_params, &dyn_params);
        one_acc_out.acr_int_amt_ccy = int_income_expense;
    }

    one_acc_out.acr_int_amt_hcy = one_acc_out.acr_int_amt_ccy;

    one_acc_out.lock_spread = one_acc_out.int_rate - one_acc_out.base_rate - fix_adjs;
    if static_params.config_params.apply_base_curve_2() {
        one_acc_out.lock_spread = one_acc_out.int_rate - one_acc_out.base_rate;
        for index in 0..static_params.config_params.adj_count_for_bc_2() {
            one_acc_out.lock_spread -= one_acc_out.adj_rates[index as usize];
        }
    }

    // Writing spread data into spread file
    write!(
        static_params.spread_writer,
        "{}",
        one_acc_out.print_spread(derived_fields.parsed_method.id)
    )
    .expect("Error while writing spread file.");
}