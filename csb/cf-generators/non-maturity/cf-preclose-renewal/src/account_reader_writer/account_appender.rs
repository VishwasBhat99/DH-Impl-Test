use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;
use chrono::NaiveDate;
use rbdate::num_days_start_to_end;
use statics::*;

pub fn create_account_without_cashflows(acc: InputAccount) -> OutputAccount {
    let mut out_acc = OutputAccount::new();
    let default_dt = NaiveDate::parse_from_str("01-01-1900", "%d-%m-%Y").unwrap();
    out_acc.as_on_date = if let Some(dt) = acc.as_on_date {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.acc_id = acc.acc_id;
    out_acc.gl_code = acc.gl_code;
    out_acc.acc_open_dt = if let Some(dt) = acc.acc_open_dt {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.curr_out_bl_ccy = acc.curr_out_bl_ccy;
    out_acc.curr_out_bl_lcy = acc.curr_out_bl_lcy;
    out_acc.premat_renew_value_ccy = acc.premat_renew_value_ccy;
    out_acc.premat_renew_value_lcy = acc.premat_renew_value_lcy;
    out_acc.ccy = acc.ccy;

    out_acc.preclose_renew_dt = if let Some(dt) = acc.preclose_renew_dt {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };

    out_acc.int_rate = acc.int_rate;

    out_acc.actual_mat_dt = if let Some(dt) = acc.actual_mat_dt {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };

    out_acc.prod_cd = acc.prod_cd;
    out_acc.add_dim_1 = acc.add_dim_1;
    out_acc.add_dim_2 = acc.add_dim_2;
    out_acc.add_dim_3 = acc.add_dim_3;
    out_acc.add_dim_4 = acc.add_dim_4;
    out_acc.add_dim_5 = acc.add_dim_5;
    out_acc.add_dim_6 = acc.add_dim_6;
    out_acc.add_dim_7 = acc.add_dim_7;
    out_acc.add_dim_8 = acc.add_dim_8;
    out_acc.add_dim_9 = acc.add_dim_9;
    out_acc.add_dim_10 = acc.add_dim_10;
    out_acc.event_type = acc.event_type.to_owned();
    let open_dt = acc.acc_open_dt.unwrap_or(default_dt);
    let mat_dt = acc.actual_mat_dt.unwrap_or(default_dt);
    let preclosure_days = num_days_start_to_end(open_dt, mat_dt);
    out_acc.pre_closure_days = preclosure_days;

    let preclose_dt = acc.preclose_renew_dt.unwrap_or(default_dt);
    let contratual_mat_days = num_days_start_to_end(preclose_dt,mat_dt);
    out_acc.contratual_mat_days = contratual_mat_days;
    out_acc.bucket_days =  match &acc.event_type.as_str() {
        &"TDP" => preclosure_days,
        &"TDR" => contratual_mat_days,
        _ => 0,  
    };
    out_acc
}
