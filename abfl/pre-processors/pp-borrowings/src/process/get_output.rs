use crate::configuration_parameters::ConfigurationParameters;
use crate::process::account::{InputAccount, OutputAccount};
use crate::process::get_prod_type::get_prod_type;
use slog::Logger;
use std::collections::HashMap;

pub fn get_op_data(
    input_account: &InputAccount,
    prod_type_map: &mut HashMap<&str, &str>,
    config_params: &ConfigurationParameters,
    _log: &Logger,
) -> OutputAccount {
    let date_parser = rbdate::DateParser::new("%d-%b-%Y".to_string(), false);
    let def_data = OutputAccount::new();
    if input_account.issuer_number.is_empty() {
        return def_data;
    }
    let data = OutputAccount {
        cust_no: input_account.issuer_number.to_string(),
        reference: input_account.isin.to_string(),
        cust_name: "".to_string(),
        branch_cd: "".to_string(),
        norm_int_rt: input_account.coupon_rate.to_string(),
        acurl_freq: "".to_string(),
        book_dt: date_parser
            .parse_opt(&input_account.issue_date)
            .unwrap_or(*config_params.as_on_date())
            .format("%d-%m-%Y")
            .to_string(),
        val_dt: date_parser
            .parse_opt(&input_account.mat_date)
            .unwrap_or(*config_params.as_on_date())
            .format("%d-%m-%Y")
            .to_string(),
        mat_dt: date_parser
            .parse_opt(&input_account.mat_date)
            .unwrap_or(*config_params.as_on_date())
            .format("%d-%m-%Y")
            .to_string(),
        due_dt: date_parser
            .parse_opt(&input_account.mat_date)
            .unwrap_or(*config_params.as_on_date())
            .format("%d-%m-%Y")
            .to_string(),
        user_def_stats: "".to_string(),
        prod_cd: input_account.instrument_type.to_string(),
        gl: "".to_string(),
        curr: input_account.currency.to_string(),
        prin_ost_bal: input_account.outstanding_bal.to_string(),
        component: "PRINCIPAL".to_string(),
        amt_due: input_account.outstanding_bal.to_string(),
        amt_setld: input_account.acquisition_val.to_string(),
        cf_amt: input_account.outstanding_bal.to_string(),
        spread: "".to_string(),
        bucket_category: "".to_string(),
        is_secured: if input_account.instrument_type == "15J" {
            "Unsecured".to_string()
        } else {
            "Secured".to_string()
        },
        product_type: get_prod_type(prod_type_map, &input_account.instrument_type).to_string(),
        composition_percentage: "".to_string(),
        old_rt_typ: input_account.coupon_type.to_string(),
        old_benchmark: input_account.benchmark.to_string(),
        nxt_call_dt: date_parser
            .parse_opt(&input_account.next_repricing_date)
            .unwrap_or(*config_params.as_on_date())
            .format("%d-%m-%Y")
            .to_string(),
        nxt_put_dt: "".to_string(),
        rt_flag_new: input_account.coupon_type.to_string(),
        rt_cd_new: "".to_string(),
        ucid: "".to_string(),
        alm_line: "".to_string(),
        ia_llg: "".to_string(),
        balm_llg: "".to_string(),
        coupon_freq: input_account.repricing_freq.to_string(),
        nxt_repricing_dt: date_parser
            .parse_opt(&input_account.next_coupon_date)
            .unwrap_or_else(|| {
                rbdate::NaiveDate::parse_from_str(&input_account.mat_date, "%d-%b-%Y")
                    .unwrap_or(*config_params.as_on_date())
            })
            .format("%d-%m-%Y")
            .to_string(),
        lst_repricing_dt: date_parser
            .parse_opt(&input_account.next_in_payout_date)
            .unwrap_or(*config_params.as_on_date())
            .format("%d-%m-%Y")
            .to_string(),
        as_on_dt: config_params.as_on_date().format("%d-%m-%Y").to_string(),
        int_basis: input_account.int_calc_basis.to_string(),
        int_calc_typ: input_account.coupon_freq.to_string(),
        cust_typ: "".to_string(),
        npa_typ: "".to_string(),
        bmid: input_account.prod_type.to_string(),
        division: "".to_string(),
    };
    data
}
