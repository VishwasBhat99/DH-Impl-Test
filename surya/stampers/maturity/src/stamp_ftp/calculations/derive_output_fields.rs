use super::*;

pub fn stamp_adjs(
    one_acc_out: &mut OneAccountView,
    static_params: &StaticParams,
    derived_fields: &DerivedFields,
) -> (f64, f64) {
    let mut fix_adjs = DEFAULT_FLOAT;
    let mut var_adjs = DEFAULT_FLOAT;
    let fix_count = static_params.config_params.fixed_adj_count() as usize;
    for index in 0..fix_count {
        if index >= derived_fields.fix_adjs.len() {
            one_acc_out.adj_codes.push(DEFAULT_INT as i32);
            one_acc_out.adj_rates.push(DEFAULT_FLOAT);
            continue;
        }
        let adj_key = AdjKey::new(one_acc_out.val_dt, derived_fields.fix_adjs[index]);
        one_acc_out.adj_codes.push(derived_fields.fix_adjs[index]);
        if let Some(adj_rate) = static_params.adj_rates.adjs.get(&adj_key) {
            one_acc_out.adj_rates.push(*adj_rate);
            fix_adjs += *adj_rate;
        } else {
            one_acc_out.adj_rates.push(DEFAULT_FLOAT);
            log_debug!(
                static_params.log,
                "Adj rates not found for account: {}, adj id: {}, date: {}.",
                one_acc_out.account_id,
                one_acc_out.adj_codes[index],
                one_acc_out.val_dt
            );
        }
    }

    for index in 0..static_params.config_params.var_adj_count() as usize {
        if index >= derived_fields.var_adjs.len() {
            one_acc_out.adj_codes.push(DEFAULT_INT as i32);
            one_acc_out.adj_rates.push(DEFAULT_FLOAT);
            continue;
        }
        let adj_key = AdjKey::new(one_acc_out.val_dt, derived_fields.var_adjs[index]);
        one_acc_out.adj_codes.push(derived_fields.var_adjs[index]);
        if let Some(adj_rate) = static_params.adj_rates.adjs.get(&adj_key) {
            one_acc_out.adj_rates.push(*adj_rate);
            var_adjs += *adj_rate;
        } else {
            one_acc_out.adj_rates.push(DEFAULT_FLOAT);
            log_debug!(
                static_params.log,
                "Adj rates not found for account: {}, adj id: {}, date: {}..",
                one_acc_out.account_id,
                one_acc_out.adj_codes[fix_count + index],
                one_acc_out.val_dt
            );
        }
    }
    if static_params.config_params.apply_base_curve_2() {
        for index in 0..static_params.config_params.adj_count_for_bc_2() {
            fix_adjs -= one_acc_out.adj_rates[index as usize] as f64;
            one_acc_out.adj_rates[index as usize] = 0.0;
        }
    }

    (fix_adjs, var_adjs)
}

pub fn calc_int(prin_amt: f64, rate: f64, time: f64) -> f64 {
    prin_amt * rate * time / 100.0
}

pub fn get_avg_bal(
    one_acc_out: &mut OneAccountView,
    static_params: &StaticParams,
    dyn_params: &DynamicParams,
) -> f64 {
    if let Some(bals) = dyn_params.avg_bal.avg_bal.get(&one_acc_out.account_id) {
        bals.bal
    } else {
        log_debug!(
            static_params.log,
            "Average Balance not found for account: {}.",
            one_acc_out.account_id
        );
        DEFAULT_FLOAT
    }
}

pub fn get_int_exp(
    one_acc_out: &mut OneAccountView,
    static_params: &StaticParams,
    dyn_params: &DynamicParams,
) -> f64 {
    if let Some(bals) = dyn_params.avg_bal.avg_bal.get(&one_acc_out.account_id) {
        bals.int_income_expense
    } else {
        log_debug!(
            static_params.log,
            "Average Balance not found for account: {}.",
            one_acc_out.account_id
        );
        DEFAULT_FLOAT
    }
}

pub fn get_int_rate(
    one_acc_out: &mut OneAccountView,
    static_params: &StaticParams,
    dyn_params: &DynamicParams,
) -> f64 {
    if let Some(bals) = dyn_params.avg_bal.avg_bal.get(&one_acc_out.account_id) {
        bals.rate
    } else {
        log_debug!(
            static_params.log,
            "Interest Rate not found for account: {}.",
            one_acc_out.account_id
        );
        DEFAULT_FLOAT
    }
}
