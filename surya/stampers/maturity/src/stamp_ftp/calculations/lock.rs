use super::*;

pub fn stamp_lock(
    mut one_acc_out: &mut OneAccountView,
    mut static_params: &mut StaticParams,
    mut dyn_params: &mut DynamicParams,
    mut derived_fields: &mut DerivedFields,
    spread_rates: &SpreadReader,
    config_params: &ConfigurationParameters,
) {
    let (fix_adjs, var_adjs) = stamp_adjs(&mut one_acc_out, &static_params, &derived_fields);
    let mut ttl_bals = TotalBalances::new(fix_adjs, var_adjs, &static_params);
    let tot_adjs = fix_adjs + var_adjs;
    one_acc_out.ftp_rate = one_acc_out.int_rate - spread_rates.spread + var_adjs;
    one_acc_out.base_rate = one_acc_out.ftp_rate - tot_adjs;
    one_acc_out.base_rate_curve_id = spread_rates.curve_id_1;

    let mut lst_bm: IntermediateBMPoints = Vec::new();
    get_data_from_cfs(
        &mut derived_fields,
        &mut ttl_bals,
        &mut lst_bm,
        &mut static_params,
        &mut dyn_params,
        one_acc_out,
    );

    let avg_bal = get_avg_bal(&mut one_acc_out, &static_params, &dyn_params);

    one_acc_out.ftp_amt_ccy = calc_int(
        rounded_f64(avg_bal, config_params.bal_prec()),
        rounded_f64(one_acc_out.ftp_rate, config_params.rate_prec()),
        ttl_bals.run_duration as f64 / static_params.no_of_days_in_year as f64,
    );

    one_acc_out.lock_spread = spread_rates.spread;
}