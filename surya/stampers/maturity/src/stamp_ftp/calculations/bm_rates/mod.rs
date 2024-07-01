mod ttl_bals;
pub use self::ttl_bals::*;
use super::*;
use rbdate::date_from_timestamp;

pub fn get_base_curve_file_path(
    _one_acc_out: &mut OneAccountView,
    static_params: &StaticParams,
    curve_pick_date: NaiveDate,
    id: i32,
) -> (BMKey, String) {
    let bm_key = BMKey {
        date: curve_pick_date.format("%d-%m-%Y").to_string(),
        base_curve_id: id,
    };

    _one_acc_out.base_rate_curve_id = bm_key.base_curve_id;
    // Deriving adj basecurve file path
    let full_file_path = format!(
        "{}{}_{}.txt",
        static_params.config_params.bc_file_path(),
        bm_key.date,
        bm_key.base_curve_id
    );
    (bm_key, full_file_path)
}

pub fn calc_bm_rates_acc_level(
    mut one_acc_out: &mut OneAccountView,
    static_params: &mut StaticParams,
    derived_fields: &DerivedFields,
    ttl_bals: &mut TotalBalances,
) {
    let mut lst_bm: IntermediateBMPoints = Vec::new();
    let (bm_key, full_file_path) = get_base_curve_file_path(
        one_acc_out,
        static_params,
        derived_fields.parsed_method.curve_pick_date,
        derived_fields.basecurve,
    );

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

        if derived_fields.parsed_method.tenor_end_date
            > derived_fields.parsed_method.curve_pick_date
        {
            ttl_bals.residual_days = num_days_start_to_end(
                derived_fields.parsed_method.curve_pick_date,
                derived_fields.parsed_method.tenor_end_date,
            );
        }
        ig_neg_val_i64(&mut ttl_bals.residual_days);
        let mut yield_rate: f64 = calc_yield_rate(
            &mut lst_bm,
            ttl_bals.residual_days,
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
}

pub fn calc_bm_rates_acc_level_adj1(
    one_acc_out: &mut OneAccountView,
    static_params: &mut StaticParams,
    derived_fields: &DerivedFields,
    ttl_bals: &mut TotalBalances,
    adj_id: i32,
) -> f64 {
    let mut lst_bm: IntermediateBMPoints = Vec::new();
    let (bm_key, full_file_path) = get_base_curve_file_path(
        one_acc_out,
        static_params,
        derived_fields.parsed_method.curve_pick_date,
        adj_id,
    );
    let mut yield_rate_adj1 = 0.0;

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

        if derived_fields.parsed_method.tenor_end_date
            > derived_fields.parsed_method.curve_pick_date
        {
            ttl_bals.residual_days = num_days_start_to_end(
                derived_fields.parsed_method.curve_pick_date,
                derived_fields.parsed_method.tenor_end_date,
            );
        }
        ig_neg_val_i64(&mut ttl_bals.residual_days);
        yield_rate_adj1 = calc_yield_rate(
            &mut lst_bm,
            ttl_bals.residual_days,
            static_params.config_params.is_extrapolation_req(),
            static_params.config_params.bmrate_accuracy().to_string(),
        )
        .unwrap_or(DEFAULT_FLOAT);
    } else {
        log_debug!(
            static_params.log,
            "Benchmark file not found at path: {} for account: {}.",
            full_file_path,
            one_acc_out.account_id
        );
    }
    yield_rate_adj1
}

pub fn calc_bm_rates_cf_level(
    one_acc_out: &mut OneAccountView,
    mut static_params: &mut StaticParams,
    mut dyn_params: &mut DynamicParams,
    mut derived_fields: &mut DerivedFields,
    mut ttl_bals: &mut TotalBalances,
) {
    let mut lst_bm: IntermediateBMPoints = Vec::new();

    let (bm_key, full_file_path) = get_base_curve_file_path(
        one_acc_out,
        static_params,
        derived_fields.parsed_method.curve_pick_date,
        derived_fields.basecurve,
    );

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
        get_data_from_cfs(
            &mut derived_fields,
            &mut ttl_bals,
            &mut lst_bm,
            &mut static_params,
            &mut dyn_params,
            one_acc_out,
        );
    } else {
        log_debug!(
            static_params.log,
            "Benchmark file not found at path: {} for account: {}.",
            full_file_path,
            one_acc_out.account_id
        );

        get_data_from_cfs(
            &mut derived_fields,
            &mut ttl_bals,
            &mut lst_bm,
            &mut static_params,
            &mut dyn_params,
            one_acc_out,
        );
    }
}

pub fn get_data_from_cfs(
    derived_fields: &mut DerivedFields,
    ttl_bals: &mut TotalBalances,
    mut lst_bm: &mut IntermediateBMPoints,
    static_params: &mut StaticParams,
    dyn_params: &mut DynamicParams,
    one_acc_out: &mut OneAccountView,
) {
    let no_of_adjs = static_params.config_params.adj_count_for_bc_2();
    let mut final_adjs = vec![0.0; no_of_adjs as usize];
    for cf in derived_fields.cashflows.iter() {
        let cf_date = date_from_timestamp(cf.date);
        if derived_fields.method_id == 1015{
            lst_bm.clear();
            derived_fields.parsed_method.curve_pick_date = cf_date;
            let (bm_key, full_file_path) =
                get_base_curve_file_path(one_acc_out, static_params,cf_date ,derived_fields.basecurve);
            if Path::new(&full_file_path).exists() {
                if let Some(prev_bm_rate) = static_params.saved_bm_rates.get(&bm_key) {
                    *lst_bm = prev_bm_rate.to_vec();
                } else {
                    bm_reader::get_bm_points(
                        &full_file_path,
                        derived_fields.parsed_method.curve_pick_date,
                        &mut lst_bm,
                    );
                    static_params.saved_bm_rates.insert(bm_key, lst_bm.clone());
                }
            }
        }
        if derived_fields.method_id == 1013 || derived_fields.method_id == 1014 || derived_fields.method_id == 1015 {
            let mat_dt = naivedate_from_timestamp(one_acc_out.mat_dt);
            if cf_date < mat_dt {
                ttl_bals.residual_days = num_days_start_to_end(cf_date, mat_dt);
            }
        } else if cf_date > derived_fields.parsed_method.curve_pick_date {
            ttl_bals.residual_days =
                num_days_start_to_end(derived_fields.parsed_method.curve_pick_date, cf_date);
        }
        ig_neg_val_i64(&mut ttl_bals.residual_days);
        let mut yield_rate: f64 = calc_yield_rate(
            &mut lst_bm,
            ttl_bals.residual_days,
            static_params.config_params.is_extrapolation_req(),
            static_params.config_params.bmrate_accuracy().to_string(),
        )
        .unwrap_or(DEFAULT_FLOAT);
        ig_neg_val_f64(&mut yield_rate);

        let mut ftp_rate = ttl_bals.fix_adj + ttl_bals.var_adj + yield_rate;
        if static_params.config_params.apply_base_curve_2() {
            for index in 0..no_of_adjs {
                let curve_pick_date = derived_fields.parsed_method.curve_pick_date;
                one_acc_out.adj_rates[index as usize] = calc_bm_rates_cf_level_adj1(
                    one_acc_out,
                    static_params,
                    curve_pick_date,
                    ttl_bals,
                    one_acc_out.adj_codes[index as usize],
                );
                ftp_rate = ftp_rate + one_acc_out.adj_rates[index as usize] as f64;
            }
        }

        ttl_bals.ttl_prin_amt += cf.principal_amount;
        ttl_bals.ttl_int_amt += cf.interest_amount;
        let org_bal_tenor = ttl_bals.residual_days as f64 * cf.principal_amount;
        let base_rate_prod = yield_rate * org_bal_tenor;
        let end_rate_prod = ftp_rate * org_bal_tenor;
        ttl_bals.ttl_base_rate_prod += base_rate_prod;
        ttl_bals.ttl_end_rate_bal += end_rate_prod;
        ttl_bals.ttl_org_tenor_bal += org_bal_tenor;
        ttl_bals.ttl_ftp_amt +=
            ttl_bals.ttl_base_rate_prod / (ttl_bals.max_days_in_year as f64 * 100.0);

        for index in 0..no_of_adjs {
            final_adjs[index as usize] = final_adjs[index as usize]
                + (one_acc_out.adj_rates[index as usize] as f64 * org_bal_tenor);
        }

        if dyn_params.is_cf_req {
            write!(
                static_params.cf_det_writer,
                "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
                one_acc_out.account_id,
                one_acc_out.ftp_method,
                date_from_timestamp(one_acc_out.open_dt).format("%d-%m-%Y"),
                date_from_timestamp(cf.date).format("%d-%m-%Y"),
                date_from_timestamp(one_acc_out.mat_dt).format("%d-%m-%Y"),
                date_from_timestamp(one_acc_out.lst_repricing_dt).format("%d-%m-%Y"),
                cf.principal_amount,
                cf.interest_amount,
                yield_rate,
                one_acc_out.adj_rates[0],
                one_acc_out.adj_rates[1],
                one_acc_out.adj_rates[2],
                one_acc_out.adj_rates[3],
                one_acc_out.adj_rates[4],
                one_acc_out.adj_rates[5],
                ftp_rate,
                ttl_bals.residual_days,
                yield_rate,
                org_bal_tenor,
                base_rate_prod,
                end_rate_prod,
            )
            .expect("Error while writing cf_level details.");
        }
    }
    // AdjRate = SUM(adj generated for cfs)/SUM(org_tenor_bal of the cfs);
    if static_params.config_params.apply_base_curve_2() {
        for index in 0..final_adjs.len() {
            if ttl_bals.ttl_org_tenor_bal != 0.0 {
                one_acc_out.adj_rates[index as usize] =
                    final_adjs[index as usize] / ttl_bals.ttl_org_tenor_bal;
            } else {
                one_acc_out.adj_rates[index as usize] = 0.0;
            }
        }
    }
}

pub fn calc_bm_rates_cf_level_adj1(
    one_acc_out: &mut OneAccountView,
    static_params: &mut StaticParams,
    curve_pick_date: NaiveDate,
    ttl_bals: &mut TotalBalances,
    adj_id: i32,
) -> f64 {
    let mut lst_bm: IntermediateBMPoints = Vec::new();
    let mut yield_rate_adj1: f64 = 0.0;

    let (bm_key, full_file_path) =
        get_base_curve_file_path(one_acc_out, static_params, curve_pick_date, adj_id);

    if Path::new(&full_file_path).exists() {
        if let Some(prev_bm_rate) = static_params.saved_bm_rates.get(&bm_key) {
            lst_bm = prev_bm_rate.to_vec();
        } else {
            bm_reader::get_bm_points(&full_file_path, curve_pick_date, &mut lst_bm);
            static_params.saved_bm_rates.insert(bm_key, lst_bm.clone());
        }

        yield_rate_adj1 = calc_yield_rate(
            &mut lst_bm,
            ttl_bals.residual_days,
            static_params.config_params.is_extrapolation_req(),
            static_params.config_params.bmrate_accuracy().to_string(),
        )
        .unwrap_or(DEFAULT_FLOAT);
    } else {
        log_debug!(
            static_params.log,
            "Benchmark file not found at path: {} for account: {}.",
            full_file_path,
            one_acc_out.account_id
        );
    }
    return yield_rate_adj1;
}
