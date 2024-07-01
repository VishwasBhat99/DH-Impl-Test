use aggregator::input_account::DailyData;
use chrono::{Datelike, NaiveDate};
use rbdate::date_from_timestamp;

pub fn is_first_day_of_month(date: NaiveDate) -> bool {
    return date.day() == 01;
}

pub fn first_occurence_of_account(input_account: DailyData, date: NaiveDate) -> String {
    let day = date.day() as usize;
    let index = day * 3;
    let mut data = vec!["0.0"; 98];
    let cls_dt_timestamp = input_account.acc_cls_dt.parse::<i64>().unwrap_or(0);
    let cls_dt = date_from_timestamp(cls_dt_timestamp)
        .format("%d-%m-%Y")
        .to_string()
        .to_string();
    data[0] = input_account.acc_num.as_str();
    data[index - 2] = input_account.out_bal.as_str();
    data[index - 1] = input_account.int_rt.as_str();
    data[index] = input_account.int_posted.as_str();
    data[94] = input_account.curr_status.as_str();
    data[95] = input_account.class.as_str();
    data[96] = cls_dt.as_str();
    data[97] = input_account.gl_cd.as_str();

    let op_str = data.join("|");

    op_str
}

pub fn previous_occurence_of_account(
    input_account: DailyData,
    inp_str: String,
    date: NaiveDate,
) -> String {
    let mut data = inp_str.split('|').collect::<Vec<&str>>();
    let day = date.day() as usize;
    let index = day * 3;
    let cls_dt_timestamp = input_account.acc_cls_dt.parse::<i64>().unwrap_or(0);
    let cls_dt = date_from_timestamp(cls_dt_timestamp)
        .format("%d-%m-%Y")
        .to_string()
        .to_string();
    data[0] = input_account.acc_num.as_str();
    data[index - 2] = input_account.out_bal.as_str();
    data[index - 1] = input_account.int_rt.as_str();
    data[index] = input_account.int_posted.as_str();
    data[94] = input_account.curr_status.as_str();
    data[95] = input_account.class.as_str();
    data[96] = cls_dt.as_str();
    data[97] = input_account.gl_cd.as_str();

    let op_str = data.join("|");

    op_str
}
