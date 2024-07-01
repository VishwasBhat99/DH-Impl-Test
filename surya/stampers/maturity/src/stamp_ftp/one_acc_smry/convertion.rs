use super::*;

pub fn ccy_converted(
    one_acc_out: &mut OneAccountView,
    static_params: &mut StaticParams,
    dyn_params: &mut DynamicParams,
) {
    let exrt = if dyn_params.is_consolidated {
        let ccy_key = Currency {
            source: static_params.config_params.ccy().to_string(),
            target: one_acc_out.currency.to_string(),
        };
        *dyn_params.exrt_map.exrt_map.get(&ccy_key).unwrap_or(&1.0)
    } else {
        1.0
    };

    one_acc_out.ccy_val_mult_by(exrt);
}

pub fn hcy_converted(
    one_acc_out: &mut OneAccountView,
    static_params: &mut StaticParams,
    dyn_params: &mut DynamicParams,
) {
    let exrt = if !dyn_params.is_consolidated {
        let hcy_key = Currency {
            source: one_acc_out.currency.to_string(),
            target: static_params.config_params.ccy().to_string(),
        };
        *dyn_params.exrt_map.exrt_map.get(&hcy_key).unwrap_or(&1.0)
    } else {
        1.0
    };

    one_acc_out.hcy_val_mult_by(exrt);
}

pub fn act_int_amt_converted(
    one_acc_out: &mut OneAccountView,
    static_params: &mut StaticParams,
    dyn_params: &mut DynamicParams,
) {
    let exrt = if dyn_params.is_consolidated {
        let ccy_key = Currency {
            source: static_params.config_params.ccy().to_string(),
            target: one_acc_out.currency.to_string(),
        };
        *dyn_params.exrt_map.exrt_map.get(&ccy_key).unwrap_or(&1.0)
    } else {
        1.0
    };
    one_acc_out.act_int_ccy_val_mult_by(exrt, dyn_params.is_int_amt_consolidated);

    let exrt = if !dyn_params.is_consolidated {
        let hcy_key = Currency {
            source: one_acc_out.currency.to_string(),
            target: static_params.config_params.ccy().to_string(),
        };
        *dyn_params.exrt_map.exrt_map.get(&hcy_key).unwrap_or(&1.0)
    } else {
        1.0
    };
    one_acc_out.act_int_hcy_val_mult_by(exrt, dyn_params.is_int_amt_consolidated);
}
