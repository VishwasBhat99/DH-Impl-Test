use rbdate::NaiveDateTime;
use stamp_ftp::one_acc_view::One_acc_view;

pub fn append_out(one_acc_op: &One_acc_view) -> String {
    let mut op_line: String = String::new();

    op_line = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\
        {}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        one_acc_op.account_number,
        one_acc_op.cust_name,
        one_acc_op.average_balance,
        one_acc_op.accr_int,
        one_acc_op.yld_to_call,
        one_acc_op.int_rate,
        one_acc_op.base_rate,
        one_acc_op.final_ftp_rate,
        get_date_str(one_acc_op.value_date),
        get_date_str(one_acc_op.maturity_date),
        get_date_str(one_acc_op.lst_rep_dt),
        get_date_str(one_acc_op.nxt_rep_dt),
        one_acc_op.mis1,
        one_acc_op.mis2,
        one_acc_op.psl_code,
        one_acc_op.prod_type,
        one_acc_op.rate_flag,
        one_acc_op.repr_spread,
        one_acc_op.source_file_name,
        one_acc_op.ccy,
        one_acc_op.gl,
        one_acc_op.cust_id,
        one_acc_op.final_ftp_amt,
        one_acc_op.alm_line,
        one_acc_op.trade_dt,
        one_acc_op.init_dep_amt,
        one_acc_op.outstanding_bal,
        one_acc_op.base_rate,
        one_acc_op.adj1,
        one_acc_op.adj2,
        one_acc_op.adj3,
        one_acc_op.adj4,
        one_acc_op.adj5,
        one_acc_op.adj6,
        one_acc_op.input_benchmark,
        one_acc_op.pdo,
        one_acc_op.npa,
        one_acc_op.method,
        one_acc_op.rate_curve,
        one_acc_op.org_tenor,
        one_acc_op.rep_tenor,
        one_acc_op.fx_spread,
        one_acc_op.var_spread,
        one_acc_op.first_ftp,
        get_date_str(one_acc_op.bc_as_on_rule),
        get_date_str(one_acc_op.tenor_start_date_rule),
        get_date_str(one_acc_op.tenor_end_date_rule),
        get_date_str(one_acc_op.bc_as_on_applied),
        get_date_str(one_acc_op.tenor_start_date_applied),
        get_date_str(one_acc_op.tenor_end_date_applied),
    );

    op_line
}

pub fn get_date_str(date: i64) -> String {
    let start_date = NaiveDateTime::from_timestamp(date, 0)
        .date()
        .format("%d-%m-%Y");

    start_date.to_string()
}
