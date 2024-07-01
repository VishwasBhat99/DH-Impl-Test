use super::*;
use rbdate::NaiveDate;
use std::collections::HashMap;

pub fn format_output(
    sub_cat_desc: &String,
    cat_bucket_id: &i64,
    sum_hm: &HashMap<String, AmtSum>,
    cat_value: &CategoryValue,
    as_on_date: NaiveDate,
    currency: &str,
) -> String {
    let sum_val = sum_hm.get(&cat_value.sub_cat_id).unwrap_or(&AmtSum {
        sum_limbalccy: 0.0,
        sum_limccyint: 0.0,
    });
    let wavg_int_rate = if sum_val.sum_limbalccy != 0.0 {
        cat_value.lim_ccy_int / sum_val.sum_limbalccy
    } else {
        cat_value.lim_ccy_int
    };
    let waavg_yld = if sum_val.sum_limbalccy != 0.0 {
        sum_val.sum_limccyint / sum_val.sum_limbalccy
    } else {
        sum_val.sum_limccyint
    };

    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        as_on_date.format("%d-%m-%Y"),
        cat_value.category_id,
        cat_value.cat_desc,
        cat_value.sub_cat_id,
        sub_cat_desc,
        cat_bucket_id,
        cat_value.bucket_desc,
        currency,
        cat_value.os_bal_ccy,
        cat_value.os_bal_ccy,
        cat_value.disb_bal_ccy,
        cat_value.disb_bal_ccy,
        wavg_int_rate,
        cat_value.lim_bal_ccy,
        cat_value.lim_bal_ccy,
        cat_value.lim_ccy_int,
        cat_value.lim_ccy_int,
        waavg_yld,
        cat_value.disp_order,
        cat_value.level_code,
        cat_value.is_visible
    )
}
