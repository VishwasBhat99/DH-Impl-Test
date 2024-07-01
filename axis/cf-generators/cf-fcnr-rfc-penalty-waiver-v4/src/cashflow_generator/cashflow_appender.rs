use crate::configuration_parameters::ConfigurationParameters;
use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::gen_cashflows::generate_cashflows;
use cashflow_generator::CompData;
use chrono::NaiveDate;
use rbdate::timestamp;
use std::collections::HashMap;

pub static DEFAULT_INT: i64 = 0;

pub fn create_account_with_cashflows(
    acc: InputAccount,
    config_map: HashMap<String, CompData>,
    config_params: &ConfigurationParameters,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let cashflows = generate_cashflows(&acc.clone(), &config_params, &mut out_acc, config_map);
    out_acc.acid = acc.acid;
    out_acc.foracid = acc.foracid;
    out_acc.bacid = acc.bacid;
    out_acc.clr_bal_amt = acc.clr_bal_amt;
    out_acc.un_clr_bal_amt = acc.un_clr_bal_amt;
    out_acc.sol_id = acc.sol_id;
    out_acc.cust_id = acc.cust_id;
    out_acc.acct_ownership = acc.acct_ownership;
    out_acc.ledg_num = acc.ledg_num;
    out_acc.drwng_power = acc.drwng_power;
    out_acc.mode_of_oper_code = acc.mode_of_oper_code;
    out_acc.lien_amt = acc.lien_amt;
    out_acc.sanct_lim = acc.sanct_lim;
    out_acc.gl_sub_head_code = acc.gl_sub_head_code;
    out_acc.schm_code = acc.schm_code;
    out_acc.schm_type = acc.schm_type;
    out_acc.crncy_code = acc.crncy_code;
    out_acc.acct_crncy_code = acc.acct_crncy_code;
    out_acc.acct_cls_flg = acc.acct_cls_flg;
    out_acc.del_flg = acc.del_flg;
    out_acc.acct_opn_date = if let Some(dt) = acc.acct_opn_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.entity_cre_flg = acc.entity_cre_flg;
    out_acc.acct_cls_date = if let Some(dt) = acc.acct_cls_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.last_tran_date = if let Some(dt) = acc.last_tran_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.notional_rate_code = acc.notional_rate_code;
    out_acc.emp_id = acc.emp_id;
    out_acc.notional_rate = acc.notional_rate;
    out_acc.limit_b2kid = acc.limit_b2kid;
    out_acc.adim1_gam = acc.adim1_gam;
    out_acc.adim2_gam = acc.adim2_gam;
    out_acc.adim3_gam = acc.adim3_gam;
    out_acc.int_rate = acc.int_rate;
    out_acc.bm_id = acc.bm_id;
    out_acc.spread = acc.spread;
    out_acc.reprice_freq = acc.reprice_freq;
    out_acc.last_reprice_dt = if let Some(dt) = acc.last_reprice_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.next_reprice_dt = if let Some(dt) = acc.next_reprice_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.code1 = acc.code1;
    out_acc.code2 = acc.code2;
    out_acc.code3 = acc.code3;
    out_acc.code4 = acc.code4;
    out_acc.adim1_gac = acc.adim1_gac;
    out_acc.adim2_gac = acc.adim2_gac;
    out_acc.adim3_gac = acc.adim3_gac;
    out_acc.cust_name = acc.cust_name;
    out_acc.cmg_pan_gir_num = acc.cmg_pan_gir_num;
    out_acc.cmg_cust_const = acc.cmg_cust_const;
    out_acc.adim1_cmg = acc.adim1_cmg;
    out_acc.adim2_cmg = acc.adim2_cmg;
    out_acc.adim3_cmg = acc.adim3_cmg;
    out_acc.out_bal_amt = acc.out_bal_amt;
    out_acc.cust_grp_id = acc.cust_grp_id;
    out_acc.ucif_cust_const = acc.ucif_cust_const;
    out_acc.exch_rt = acc.exch_rt;
    out_acc.out_bal_amt_con = acc.out_bal_amt_con;
    out_acc.segment_code = acc.segment_code;
    out_acc.nfs = acc.nfs;
    out_acc.oth_del_flg = acc.oth_del_flg;
    out_acc.open_effective_date = if let Some(dt) = acc.open_effective_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.oth_schm_type = acc.oth_schm_type;
    out_acc.int_tbl_code = acc.int_tbl_code;
    out_acc.int_version = acc.int_version;
    out_acc.int_tbl_ver_num = acc.int_tbl_ver_num;
    out_acc.min_int_pcnt_cr = acc.min_int_pcnt_cr;
    out_acc.max_int_pcnt_cr = acc.max_int_pcnt_cr;
    out_acc.cust_cr_pref_pcnt = acc.cust_cr_pref_pcnt;
    out_acc.id_cr_pref_pcnt = acc.id_cr_pref_pcnt;
    out_acc.nrml_int_pcnt = acc.nrml_int_pcnt;
    out_acc.id_dr_pref_pcnt = acc.id_dr_pref_pcnt;
    out_acc.base_int_tbl_code = acc.base_int_tbl_code;
    out_acc.base_pcnt_dr = acc.base_pcnt_dr;
    out_acc.base_pcnt_cr = acc.base_pcnt_cr;
    out_acc.base_pcnt = acc.base_pcnt;
    out_acc.deposit_period_mths = acc.deposit_period_mths;
    out_acc.deposit_period_days = acc.deposit_period_days;
    out_acc.deposit_amount = acc.deposit_amount;
    out_acc.oth_acct_crncy_code = acc.oth_acct_crncy_code;
    out_acc.deposit_type = acc.deposit_type;
    out_acc.spl_catg_ind = acc.spl_catg_ind;
    out_acc.nrml_int_pcnt_cr = acc.nrml_int_pcnt_cr;
    out_acc.base_differential_exists = acc.base_differential_exists;
    out_acc.deposit_status = acc.deposit_status;
    out_acc.maturity_amount = acc.maturity_amount;
    out_acc.maturity_date = if let Some(dt) = acc.maturity_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.rcre_time = if let Some(dt) = acc.rcre_time {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.auto_renewed_counter = acc.auto_renewed_counter;
    out_acc.overdue_flg = acc.overdue_flg;
    out_acc.final_int_rate = acc.final_int_rate;
    out_acc.source_name = "0001".to_string();
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
