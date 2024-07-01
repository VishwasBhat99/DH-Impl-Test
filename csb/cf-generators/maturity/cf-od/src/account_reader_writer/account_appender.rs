use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_with_cashflows::Cashflow;
use account_reader_writer::account_with_cashflows::OutputAccount;
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(
    acc: InputAccount,
    cashflows: Vec<Cashflow>,
) -> OutputAccount {
    let mut out_acc = OutputAccount::new();
    out_acc.acc_no = acc.acc_no;
    out_acc.acnts_internal_acnum = acc.acnts_internal_acnum;
    out_acc.acnts_brn_code = acc.acnts_brn_code;
    out_acc.acnts_client_num = acc.acnts_client_num;
    out_acc.ucic = acc.ucic;
    out_acc.acnts_curr_code = acc.acnts_curr_code;
    out_acc.acnts_prod_code = acc.acnts_prod_code;
    out_acc.gl_cd = acc.gl_cd;
    out_acc.acc_open_dt = if let Some(dt) = acc.acc_open_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.bal = acc.bal;
    out_acc.balccy = acc.balccy;
    out_acc.int_rate = acc.int_rate;
    out_acc.int_type = acc.int_type;
    out_acc.int_bench = acc.int_bench;
    out_acc.int_spread = acc.int_spread;
    out_acc.last_reset_dt = if let Some(dt) = acc.last_reset_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.next_reset_dt = if let Some(dt) = acc.next_reset_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.reset_no_of_months = acc.reset_no_of_months;
    out_acc.int_accrued_amt = acc.int_accrued_amt;
    out_acc.constitn = acc.constitn;
    out_acc.lm_exp = if let Some(dt) = acc.lm_exp {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.lim = acc.lim;
    out_acc.lm_ccy = acc.lm_ccy;
    out_acc.ext_rating_agency = acc.ext_rating_agency;
    out_acc.ext_rating = acc.ext_rating;
    out_acc.int_rating = acc.int_rating;
    out_acc.asset_cd = acc.asset_cd;
    out_acc.provision_amt = acc.provision_amt;
    out_acc.prov_dt = if let Some(dt) = acc.prov_dt {
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
    out_acc.corpcl_nature_of_bus2 = acc.corpcl_nature_of_bus2;
    out_acc.corpcl_nature_of_bus3 = acc.corpcl_nature_of_bus3;
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
    out_acc.asset_code = acc.asset_code;
    out_acc.npa_dt = if let Some(dt) = acc.npa_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.account_balance = acc.account_balance;
    out_acc.pwo = acc.pwo;
    out_acc.written_off_dt = if let Some(dt) = acc.written_off_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.ho_balance = acc.ho_balance;
    out_acc.npa_provision = acc.npa_provision;
    out_acc.ho_provision = acc.ho_provision;
    out_acc.suspencebalance = acc.suspencebalance;
    out_acc.suspence_writeoff = acc.suspence_writeoff;
    out_acc.ho_suspence = acc.ho_suspence;
    out_acc.claim = acc.claim;
    out_acc.primary = acc.primary;
    out_acc.collateral = acc.collateral;
    out_acc.total_security = acc.total_security;
    out_acc.primary_valuation_dt = if let Some(dt) = acc.primary_valuation_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };

    out_acc.collateral_valuation_dt = if let Some(dt) = acc.collateral_valuation_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.gold_deficit = acc.gold_deficit;
    out_acc.fraud = acc.fraud;
    out_acc.wilful_default = acc.wilful_default;
    out_acc.subsidy = acc.subsidy;
    out_acc.priority = acc.priority;
    out_acc.priority_type = acc.priority_type;
    out_acc.main_sector = acc.main_sector;
    out_acc.sub_sector = acc.sub_sector;
    out_acc.activity = acc.activity;
    out_acc.industry = acc.industry;
    out_acc.categoryofborrower = acc.categoryofborrower;
    out_acc.org_gl_head = acc.org_gl_head;
    out_acc.npa_amt = acc.npa_amt;
    out_acc.sanc_dt = if let Some(dt) = acc.sanc_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.occp_cd = acc.occp_cd;
    out_acc.sens_sec = acc.sens_sec;
    out_acc.prior_subtype = acc.prior_subtype;
    out_acc.restruct_flag = acc.restruct_flag;
    out_acc.restruct_dt = if let Some(dt) = acc.restruct_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.mor_prd = acc.mor_prd;
    out_acc.rating = acc.rating;
    out_acc.consitin = acc.consitin;
    out_acc.pan = acc.pan;
    out_acc.limit_amt = acc.limit_amt;
    out_acc.gross_adv = acc.gross_adv;
    out_acc.exp_amt = acc.exp_amt;
    out_acc.unvail_amt = acc.unvail_amt;
    out_acc.gold_gram = acc.gold_gram;
    out_acc.fund_flag = acc.fund_flag;
    out_acc.pt_i64_1 = acc.pt_i64_1;
    out_acc.pt_i64_2 = acc.pt_i64_2;
    out_acc.pt_i64_3 = acc.pt_i64_3;
    out_acc.pt_i64_4 = acc.pt_i64_4;
    out_acc.pt_i64_5 = acc.pt_i64_5;
    out_acc.pt_f64_1 = acc.pt_f64_1;
    out_acc.pt_f64_2 = acc.pt_f64_2;
    out_acc.pt_f64_3 = acc.pt_f64_3;
    out_acc.pt_f64_4 = acc.pt_f64_4;
    out_acc.pt_f64_5 = acc.pt_f64_5;
    out_acc.pt_str_1 = acc.pt_str_1;
    out_acc.pt_str_2 = acc.pt_str_2;
    out_acc.pt_str_3 = acc.pt_str_3;
    out_acc.pt_str_4 = acc.pt_str_4;
    out_acc.pt_str_5 = acc.pt_str_5;

    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
