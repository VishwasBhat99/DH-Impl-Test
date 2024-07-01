use super::bkt_def::BktData;
use crate::configuration_parameters::ConfigurationParameters;
use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_without_cashflows::AccountWithoutCashflows;
use rbdate::timestamp;
use std::collections::HashMap;

pub fn create_account_with_cashflows(
    input_account: InputAccount,
    config_params: &ConfigurationParameters,
    mapping_master_map: &HashMap<String, String>,
    bkt_def_vec: &Vec<BktData>,
) -> AccountWithoutCashflows {
    let mut op = AccountWithoutCashflows::new();
    op.customer_no = input_account.customer_no;
    op.cust_acct_no = input_account.key_1;
    op.apprv_date = if input_account.apprv_date.is_some() {
        timestamp(
            input_account
                .apprv_date
                .unwrap_or(*config_params.as_on_date()),
        )
    } else {
        0
    };

    op.lst_fin_date = {
        if input_account.lst_fin_date.is_some() {
            timestamp(
                input_account
                    .lst_fin_date
                    .unwrap_or(*config_params.as_on_date()),
            )
        } else {
            0
        }
    };
    op.actl_mat_date = {
        if input_account.matdt.is_some() {
            timestamp(input_account.matdt.unwrap_or(*config_params.as_on_date()))
        } else {
            0
        }
    };

    op.closure_amount = input_account.premat_amt;
    op.int_rate = input_account.eff_int_rt;
    op.gl_class_code = input_account.gl_class_code.to_owned();
    op.currency_ind = input_account.ccy;
    let appro_date = input_account
        .apprv_date
        .unwrap_or(*config_params.as_on_date());
    let lst_fin_date = input_account
        .lst_fin_date
        .unwrap_or(*config_params.as_on_date());
    op.accnt_live_days = if appro_date <= lst_fin_date {
        rbdate::num_days_start_to_end(appro_date, lst_fin_date)
    } else {
        rbdate::num_days_start_to_end(lst_fin_date, appro_date) * (-1)
    };
    op.preclosure_bkt_id = bkt_def_vec
        .iter()
        .find(|val| val.from_bkt <= op.accnt_live_days && val.to_bkt >= op.accnt_live_days)
        .map(|val| val.bkt_id.to_string())
        .unwrap_or(bkt_def_vec[bkt_def_vec.len() - 1].bkt_id.to_string());
    let mat_date = input_account.matdt.unwrap_or(*config_params.as_on_date());
    op.actual_days_mat = if appro_date <= mat_date {
        rbdate::num_days_start_to_end(appro_date, mat_date)
    } else {
        rbdate::num_days_start_to_end(mat_date, appro_date) * (-1)
    };
    op.contractual_bkt_id = bkt_def_vec
        .iter()
        .find(|val| val.from_bkt <= op.actual_days_mat && val.to_bkt >= op.actual_days_mat)
        .map(|val| val.bkt_id.to_string())
        .unwrap_or(bkt_def_vec[bkt_def_vec.len() - 1].bkt_id.to_string());
    op.llg_type = mapping_master_map
        .get(&input_account.gl_class_code)
        .unwrap_or(&"NA".to_string())
        .to_string();

    op
}
