use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut total_interest_amount = DEFAULT_FLOAT;
    let mut total_principal_amount = DEFAULT_FLOAT;

    out_acc.deal_no = account.deal_no;
    out_acc.book_value = account.book_value;
    out_acc.ccy = account.ccy;
    out_acc.cntr_party_id = account.cntr_party_id;
    out_acc.cntr_party_name = account.cntr_party_name;
    out_acc.cntr_party_type = account.cntr_party_type;
    out_acc.repo_dt = if let Some(dt) = account.repo_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.repo_mat_dt = if let Some(dt) = account.repo_mat_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.int_rate = account.int_rate;
    out_acc.int_amt = account.int_amt;
    out_acc.treas_gl_cd = account.treas_gl_cd;
    out_acc.client_type = account.client_type;
    out_acc.clients_name1 = account.clients_name1;
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
    out_acc.w4b_cd = account.w4b_cd;
    out_acc.balm_llg = account.balm_llg;
    out_acc.care_llg = account.care_llg;
    out_acc.ba_llg = account.ba_llg;

    for cf in &cashflows {
        total_interest_amount += cf.int_amt;
        total_principal_amount += cf.prin_amt;
    }
    out_acc.total_interest_amount = total_interest_amount;
    out_acc.total_principal_amount = total_principal_amount;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
