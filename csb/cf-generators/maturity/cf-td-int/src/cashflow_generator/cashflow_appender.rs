use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use protobuf;
use rbdate::timestamp;
use statics::DEFAULT_INT;

pub fn create_account_with_cashflows(
    acc: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = 0.0;
    let mut tot_prin_amt = 0.0;

    out_acc.acc_no = acc.acc_no;
    out_acc.br_cd = acc.br_cd;
    out_acc.cust_id = acc.cust_id;
    out_acc.ucic_id = acc.ucic_id;
    out_acc.ccy = acc.ccy;
    out_acc.prod_cd = acc.prod_cd;
    out_acc.gl_cd = acc.gl_cd;
    out_acc.gl_comp_portion = acc.gl_comp_portion;
    out_acc.acc_open_dt = timestamp(acc.acc_open_dt);
    out_acc.effc_dt = if let Some(dt) = acc.effc_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.bal_os = acc.bal_os;
    out_acc.bal_os_cly = acc.bal_os_cly;
    out_acc.int_comp_type = acc.int_comp_type;
    out_acc.compo_int_amt = acc.compo_int_amt;
    out_acc.int_rt = acc.int_rt;
    out_acc.mat_dt = timestamp(acc.mat_dt);
    out_acc.dep_amt = acc.dep_amt;
    out_acc.dep_amt_lcy = acc.dep_amt_lcy;
    out_acc.int_amt = acc.int_amt;
    out_acc.int_acc_amt = acc.int_acc_amt;
    out_acc.non_with_flag = acc.non_with_flag;
    out_acc.notice_day = acc.notice_day;
    out_acc.cust_const_code = acc.cust_const_code;
    out_acc.cntrct_num = acc.cntrct_num;
    out_acc.as_on = if let Some(dt) = acc.as_on {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.comp_freq = acc.comp_freq;
    out_acc.pay_freq = acc.pay_freq;
    out_acc.over_dt = if let Some(dt) = acc.over_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.lst_int_acr_dt = if let Some(dt) = acc.lst_int_acr_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.int_pay_amt = acc.int_pay_amt;
    out_acc.is_overdue = acc.is_overdue;
    out_acc.max_date = timestamp(acc.max_date);
    out_acc.resid_days = acc.resid_days;
    out_acc.over_int_rt = acc.over_int_rt;

    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.tot_int_amt = tot_int_amt;
    out_acc.tot_prin_amt = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
