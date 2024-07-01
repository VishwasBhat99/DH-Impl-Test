use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();

    out_acc.account_number = account.account_number;
    out_acc.acid = account.acid;
    out_acc.foracid = account.foracid;
    out_acc.bacid = account.bacid;
    out_acc.solid = account.solid;
    out_acc.cust_id = account.cust_id;
    out_acc.schm_code = account.schm_code;
    out_acc.schm_type = account.schm_type;
    out_acc.bill_param_type = account.bill_param_type;
    out_acc.bill_b2k_id = account.bill_b2k_id;
    out_acc.bill_id = account.bill_id;
    out_acc.bill_amt = account.bill_amt;
    out_acc.bill_amt_inr = account.bill_amt_inr;
    out_acc.bill_crncy_code = account.bill_crncy_code;
    out_acc.due_date = if let Some(dt) = account.due_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.bp_acid = account.bp_acid;
    out_acc.del_flg = account.del_flg;
    out_acc.cls_flg = account.cls_flg;
    out_acc.reg_type = account.reg_type;
    out_acc.reg_sub_type = account.reg_sub_type;
    out_acc.bp_liab = account.bp_liab;
    out_acc.bp_liab_crncy = account.bp_liab_crncy;
    out_acc.bill_liab_inr = account.bill_liab_inr;
    out_acc.bill_stat = account.bill_stat;
    out_acc.bill_func_code = account.bill_func_code;
    out_acc.bill_liab = account.bill_liab;
    out_acc.bill_liab_hc_eq = account.bill_liab_hc_eq;
    out_acc.bill_liab_crncy = account.bill_liab_crncy;
    out_acc.bill_liab_crncy_der = account.bill_liab_crncy_der;
    out_acc.clr_bal_amt = account.clr_bal_amt;
    out_acc.un_clr_bal_amt = account.un_clr_bal_amt;
    out_acc.out_bal_amt = account.out_bal_amt;
    out_acc.acct_opn_date = if let Some(dt) = account.acct_opn_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.acct_crncy_code = account.acct_crncy_code;
    out_acc.cust_name = account.cust_name;
    out_acc.gl_sub_head_code = account.gl_sub_head_code;
    out_acc.npa_classification = account.npa_classification;
    out_acc.cust_hlth_code = account.cust_hlth_code;
    out_acc.cust_npa_class = account.cust_npa_class;
    out_acc.final_npa_class = account.final_npa_class;
    out_acc.int_rate = account.int_rate;
    out_acc.acct_exch_rt = account.acct_exch_rt;
    out_acc.cust_grp_id = account.cust_grp_id;
    out_acc.ucif_cust_const = account.ucif_cust_const;
    out_acc.exch_rt = account.exch_rt;
    out_acc.out_bal_amt_con = account.out_bal_amt_con;
    out_acc.segment_code = account.segment_code;
    out_acc.nfs = account.nfs;
    out_acc.overdue_flg = account.overdue_flg;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
