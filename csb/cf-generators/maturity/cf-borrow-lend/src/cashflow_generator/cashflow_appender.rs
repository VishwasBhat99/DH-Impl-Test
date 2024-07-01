use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    tot_prin_amt: &mut f64,
    tot_int_amt: &mut f64,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    out_acc.deal_num = account.deal_num;
    out_acc.os_bal = account.os_bal;
    out_acc.ccy = account.ccy;
    out_acc.instrument = account.instrument;
    out_acc.counter_party_id = account.counter_party_id;
    out_acc.counter_party_name = account.counter_party_name;
    out_acc.counter_party_type = account.counter_party_type;
    out_acc.borrowing_dt = if let Some(dt) = account.borrowing_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.maturity_dt = if let Some(dt) = account.maturity_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.int_rt = account.int_rt;
    out_acc.int_rate_classification = account.int_rate_classification;
    out_acc.next_reprice_dt = if let Some(dt) = account.next_reprice_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.coupan_pay_strt_dt = if let Some(dt) = account.coupan_pay_strt_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.coupan_pay_freq = account.coupan_pay_freq;
    out_acc.spread = account.spread;
    out_acc.treasury_gl_code = account.treasury_gl_code;
    out_acc.cbs_gl_code = account.cbs_gl_code;
    out_acc.w4b_cd = account.w4b_cd;
    out_acc.balm_llg = account.balm_llg;
    out_acc.care_llg = account.care_llg;
    out_acc.ba_llg = account.ba_llg;
    out_acc.client_type = account.client_type;
    out_acc.clients_name = account.clients_name;
    out_acc.clients_bsr_type_flg = account.clients_bsr_type_flg;
    out_acc.clients_busdivn_code = account.clients_busdivn_code;
    out_acc.clients_const_code = account.clients_const_code;
    out_acc.clients_pan_gir_num = account.clients_pan_gir_num;
    out_acc.clients_risk_categorization = account.clients_risk_categorization;
    out_acc.clients_risk_cntry = account.clients_risk_cntry;
    out_acc.clients_segment_code = account.clients_segment_code;
    out_acc.corpcl_orgn_qualifier = account.corpcl_orgn_qualifier;
    out_acc.corpcl_indus_code = account.corpcl_indus_code;
    out_acc.corpcl_nature_of_bus1 = account.corpcl_nature_of_bus1;
    out_acc.corpcl_central_state_flg = account.corpcl_central_state_flg;
    out_acc.corpcl_public_sector_flg = account.corpcl_public_sector_flg;
    out_acc.corpcl_primary_dlr_flg = account.corpcl_primary_dlr_flg;
    out_acc.corpcl_multilateral_bank = account.corpcl_multilateral_bank;
    out_acc.corpcl_connp_inv_num = account.corpcl_connp_inv_num;
    out_acc.corpcl_bc_gross_turnover = account.corpcl_bc_gross_turnover;

    for cf in &cashflows {
        *tot_int_amt += cf.int_amt;
        *tot_prin_amt += cf.prin_amt;
    }
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
