use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;
use rbdate::timestamp;
use statics::*;

pub fn create_account_without_cashflows(acc: InputAccount) -> OutputAccount {
    let mut out_acc = OutputAccount::new();
    out_acc.acc_num = acc.acc_num;
    out_acc.br_code = acc.br_code;
    out_acc.client_id = acc.client_id;
    out_acc.lc_typ = acc.lc_typ;
    out_acc.ccy = acc.ccy;
    out_acc.gl_cd = acc.gl_cd;
    out_acc.acc_open_dt = if let Some(dt) = acc.acc_open_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.lc_amt = acc.lc_amt;
    out_acc.bal_os = acc.bal_os;
    out_acc.lc_dt = if let Some(dt) = acc.lc_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.cancel_dt = if let Some(dt) = acc.cancel_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.lst_dt_of_negotiation = if let Some(dt) = acc.lst_dt_of_negotiation {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.acc_typ_cd = acc.acc_typ_cd;
    out_acc.acc_typ_desc = acc.acc_typ_desc;
    out_acc.prod_code = acc.prod_code;
    out_acc.as_on = if let Some(dt) = acc.as_on {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };

    out_acc.client_type = acc.client_type;
    out_acc.clients_name = acc.clients_name;
    out_acc.clients_bsr_type_flg = acc.clients_bsr_type_flg;
    out_acc.clients_busdivn_code = acc.clients_busdivn_code;
    out_acc.clients_const_code = acc.clients_const_code;
    out_acc.clients_pan_gir_num = acc.clients_pan_gir_num;
    out_acc.clients_risk_categorization = acc.clients_risk_categorization;
    out_acc.clients_risk_cntry = acc.clients_risk_cntry;
    out_acc.clients_segment_code = acc.clients_segment_code;
    out_acc.corpcl_orgn_qualifier = acc.corpcl_orgn_qualifier;
    out_acc.corpcl_indus_code = acc.corpcl_indus_code;
    out_acc.corpcl_nature_of_bus1 = acc.corpcl_nature_of_bus1;
    out_acc.corpcl_central_state_flg = acc.corpcl_central_state_flg;
    out_acc.corpcl_public_sector_flg = acc.corpcl_public_sector_flg;
    out_acc.corpcl_primary_dlr_flg = acc.corpcl_primary_dlr_flg;
    out_acc.corpcl_multilateral_bank = acc.corpcl_multilateral_bank;
    out_acc.corpcl_connp_inv_num = acc.corpcl_connp_inv_num;
    out_acc.corpcl_bc_gross_turnover = acc.corpcl_bc_gross_turnover;
    out_acc.w4b_cd = acc.w4b_cd;
    out_acc.balm_llg = acc.balm_llg;
    out_acc.care_llg = acc.care_llg;
    out_acc.ba_llg = acc.ba_llg;

    out_acc
}
