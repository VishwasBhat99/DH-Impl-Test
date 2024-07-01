use super::{timestamp, Account, AccountDescriptor, Cashflow, Input, DEFAULT_FLOAT, DEFAULT_INT};

pub fn create_account_with_cashflows(
    acc: Input,
    cfs_vec: Vec<Cashflow>,
) -> (Account, AccountDescriptor) {
    let cashflows = protobuf::RepeatedField::from_vec(cfs_vec);
    let cashflows_count = cashflows.len() as u64;
    let mut op = Account::new();
    let mut tot_prin_amt = DEFAULT_FLOAT;
    let mut tot_int_amt = DEFAULT_FLOAT;

    op.acc_no = acc.acc_no;
    op.acnts_internal_acnum = acc.acnts_internal_acnum;
    op.acnts_brn_cd = acc.acnts_brn_cd;
    op.acnts_client_num = acc.acnts_client_num;
    op.ucic = acc.ucic;
    op.acnts_curr_cd = acc.acnts_curr_cd;
    op.acnts_prod_cd = acc.acnts_prod_cd;
    op.gl_cd = acc.gl_cd;
    op.acnts_opening_dt = if let Some(dt) = acc.acnts_opening_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    op.bal = acc.bal;
    op.bal_ccy = acc.bal_ccy;
    op.int_rt = acc.int_rt;
    op.int_type = acc.int_type;
    op.int_bench = acc.int_bench;
    op.int_spread = acc.int_spread;
    op.last_reset_dt = if let Some(dt) = acc.last_reset_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    op.next_reset_dt = if let Some(dt) = acc.next_reset_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    op.reset_no_of_months = acc.reset_no_of_months;
    op.disbursal_amount = acc.disbursal_amount;
    op.last_emi_dt = if let Some(dt) = acc.last_emi_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    op.lm_exp = if let Some(dt) = acc.lm_exp {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    op.lim = acc.lim;
    op.lm_ccy = acc.lm_ccy;
    op.ext_rating_agency = acc.ext_rating_agency;
    op.ext_rating = acc.ext_rating;
    op.int_rating = acc.int_rating;
    op.asset_cd = acc.asset_cd;
    op.prov_amt = acc.prov_amt;
    op.prov_dt = if let Some(dt) = acc.prov_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    op.constitn = acc.constitn;
    op.loan_type = acc.loan_type;
    op.def_amt = acc.def_amt;
    op.def_dt = if let Some(dt) = acc.def_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    op.last_paid_emi_dt = if let Some(dt) = acc.last_paid_emi_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    op.w4b_cd = acc.w4b_cd;
    op.balm_llg = acc.balm_llg;
    op.care_llg = acc.care_llg;
    op.ba_llg = acc.ba_llg;
    op.client_type = acc.client_type;
    op.clients_name = acc.clients_name;
    op.clients_bsr_type_flg = acc.clients_bsr_type_flg;
    op.clients_busdivn_code = acc.clients_busdivn_code;
    op.clients_const_code = acc.clients_const_code;
    op.clients_pan_gir_num = acc.clients_pan_gir_num;
    op.clients_risk_categorization = acc.clients_risk_categorization;
    op.clients_risk_cntry = acc.clients_risk_cntry;
    op.clients_segment_code = acc.clients_segment_code;
    op.corpcl_orgn_qualifier = acc.corpcl_orgn_qualifier;
    op.corpcl_indus_code = acc.corpcl_indus_code;
    op.corpcl_nature_of_bus1 = acc.corpcl_nature_of_bus1;
    op.corpcl_nature_of_bus2 = acc.corpcl_nature_of_bus2;
    op.corpcl_nature_of_bus3 = acc.corpcl_nature_of_bus3;
    op.corpcl_central_state_flg = acc.corpcl_central_state_flg;
    op.corpcl_public_sector_flg = acc.corpcl_public_sector_flg;
    op.corpcl_primary_dlr_flg = acc.corpcl_primary_dlr_flg;
    op.corpcl_multilateral_bank = acc.corpcl_multilateral_bank;
    op.corpcl_connp_inv_num = acc.corpcl_connp_inv_num;
    op.corpcl_bc_gross_turnover = acc.corpcl_bc_gross_turnover;
    op.npa_asset_cd = acc.npa_asset_cd;
    op.npa_dt = if let Some(dt) = acc.npa_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    op.acc_bal = acc.acc_bal;
    op.pwo = acc.pwo;
    op.ho_bal = acc.ho_bal;
    op.npa_prov = acc.npa_prov;
    op.ho_prov = acc.ho_prov;
    op.suspence_bal = acc.suspence_bal;
    op.suspence_writeoff = acc.suspence_writeoff;
    op.ho_suspence = acc.ho_suspence;
    op.claim = acc.claim;
    op.primary = acc.primary;
    op.col = acc.col;
    op.priority = acc.priority;
    op.main_sector = acc.main_sector;
    op.industry = acc.industry;
    op.npa_amt = acc.npa_amt;
    op.sanc_dt = if let Some(dt) = acc.sanc_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    op.occp_cd = acc.occp_cd;
    op.sens_sec = acc.sens_sec;
    op.prior_subtype = acc.prior_subtype;
    op.restruct_flag = acc.restruct_flag;
    op.restruct_dt = if let Some(dt) = acc.restruct_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    op.mor_prd = acc.mor_prd;
    op.rating = acc.rating;
    op.consitin = acc.consitin;
    op.pan = acc.pan;
    op.limit_amt = acc.limit_amt;
    op.gross_adv = acc.gross_adv;
    op.exp_amt = acc.exp_amt;
    op.unvail_amt = acc.unvail_amt;
    op.gold_gram = acc.gold_gram;
    op.fund_flag = acc.fund_flag;
    op.ltv_value = acc.ltv_value;
    op.pt_i64_1 = acc.pt_i64_1;
    op.pt_i64_2 = acc.pt_i64_2;
    op.pt_i64_3 = acc.pt_i64_3;
    op.pt_i64_4 = acc.pt_i64_4;
    op.pt_i64_5 = acc.pt_i64_5;
    op.pt_f64_1 = acc.pt_f64_1;
    op.pt_f64_2 = acc.pt_f64_2;
    op.pt_f64_3 = acc.pt_f64_3;
    op.pt_f64_4 = acc.pt_f64_4;
    op.pt_f64_5 = acc.pt_f64_5;
    op.pt_str_1 = acc.pt_str_1;
    op.pt_str_2 = acc.pt_str_2;
    op.pt_str_3 = acc.pt_str_3;
    op.pt_str_4 = acc.pt_str_4;
    op.pt_str_5 = acc.pt_str_5;
    op.org_code = acc.org_code;

    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }

    op.set_cashflows(cashflows);

    let cd = AccountDescriptor {
        cashflows_count,
        total_amount_input: acc.bal,
        total_principal_output: tot_prin_amt,
        total_interest_output: tot_int_amt,
    };

    (op, cd)
}
