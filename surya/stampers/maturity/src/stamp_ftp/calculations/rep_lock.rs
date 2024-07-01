use super::*;
use chrono::Datelike;

pub fn stamp_rep_lock(
    mut one_acc_out: &mut OneAccountView,
    mut static_params: &mut StaticParams,
    mut dyn_params: &mut DynamicParams,
    derived_fields: &mut DerivedFields,
    spread_rates: &SpreadReader,
    config_params: &ConfigurationParameters,
) {
    let (fix_adjs, var_adjs) = stamp_adjs(&mut one_acc_out, &static_params, &derived_fields);
    let mut ttl_bals = TotalBalances::new(fix_adjs, var_adjs, &static_params);
    let tot_adjs = fix_adjs + var_adjs;

    let lrp_dt = date_from_timestamp(one_acc_out.lst_repricing_dt);
    let prev_int_rt = spread_rates.int_rate as f64;
    let prev_spread = spread_rates.spread as f64;
    let wt_avg_tpr = if lrp_dt.month() == static_params.config_params.from_date().month()
        && lrp_dt.year() == static_params.config_params.from_date().year()
    {
        let prev_days =
            num_days_start_to_end(*static_params.config_params.from_date(), lrp_dt) as f64;
        let new_days = ttl_bals.run_duration as f64 - prev_days;
        ((prev_int_rt - prev_spread) * prev_days
            + (one_acc_out.int_rate as f64 - prev_spread) * new_days)
            / ttl_bals.run_duration as f64
    } else {
        one_acc_out.int_rate as f64 - prev_spread
    };

    one_acc_out.ftp_rate = wt_avg_tpr + var_adjs;
    one_acc_out.base_rate = one_acc_out.ftp_rate - tot_adjs;
    one_acc_out.base_rate_curve_id = spread_rates.curve_id_1;

    let mut lst_bm: IntermediateBMPoints = Vec::new();
    get_data_from_cfs(
        derived_fields,
        &mut ttl_bals,
        &mut lst_bm,
        &mut static_params,
        &mut dyn_params,
        one_acc_out,
    );

    let avg_bal = get_avg_bal(&mut one_acc_out, &static_params, &dyn_params);

    one_acc_out.lock_spread = one_acc_out.int_rate - one_acc_out.base_rate - fix_adjs;
    one_acc_out.ftp_amt_ccy = calc_int(
        rounded_f64(avg_bal, config_params.bal_prec()),
        rounded_f64(one_acc_out.ftp_rate, config_params.rate_prec()),
        ttl_bals.run_duration as f64 / static_params.no_of_days_in_year as f64,
    );

    one_acc_out.lock_spread = spread_rates.spread;

    // Writing updated spread data into spread file
    write!(
        &mut static_params.spread_writer,
        "{}",
        one_acc_out.print_rep_spread(derived_fields.parsed_method.id, prev_spread as f32)
    )
    .expect("Error while writing spread file.");
}