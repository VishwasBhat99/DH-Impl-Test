use chrono::{Duration, NaiveDate};
use rbdate::incr_dt_by_mon_presrv_eom_checked;
use std::collections::HashMap;

pub fn bucket_eq_dist_op(
    daywise_amt: HashMap<NaiveDate, f64>,
    monthwise_amt: HashMap<NaiveDate, f64>,
    after_two_years_amt: f64,
    as_on_date: NaiveDate,
    tot_amt_op: &mut f64,
) -> Vec<f64> {
    let mut out_distirbution_vec: Vec<f64> = Vec::new();
    let mut bucket_dt: NaiveDate = as_on_date.succ();
    //First 184 buckets are aggregated day-wise.
    let last_dt_for_daily_bucket = incr_dt_by_mon_presrv_eom_checked(
        bucket_dt
            .pred_opt()
            .expect("can not get preceding calander date for bucket_dt"),
        6,
    )
    .expect("Error while incrementing last date for daily bucket.");
    let mut day_num = 1;
    while bucket_dt <= last_dt_for_daily_bucket {
        let val = get_data(bucket_dt, &daywise_amt);
        *tot_amt_op += val;
        out_distirbution_vec.push(val);
        day_num += 1;
        bucket_dt += Duration::days(1);
    }

    while day_num <= 184 {
        out_distirbution_vec.push(0.00);
        day_num += 1;
    }
    // println!("{:?}",monthwise_amt);
    //Next buckets are aggregated monthwise upto 24 months.
    for i in 7..=24 {
        let start_dt = incr_dt_by_mon_presrv_eom_checked(as_on_date, i - 1 as usize)
            .expect("Could not aggregate monthwise data. End date could not be fetched.")
            .succ();
        let val = get_data(start_dt, &monthwise_amt);
        *tot_amt_op += val;
        out_distirbution_vec.push(val);
    }
    //Amounts after 2 years are aggregated to a single bucket.
    out_distirbution_vec.push(after_two_years_amt);
    *tot_amt_op += after_two_years_amt;
    out_distirbution_vec
}
pub fn bucket_eneq_dist_op(
    daywise_amt: HashMap<NaiveDate, f64>,
    monthwise_amt: HashMap<NaiveDate, f64>,
    config_data: &Vec<i64>,
    after_two_years_amt: f64,
    as_on_date: NaiveDate,
    mut tot_amt_op: &mut f64,
) -> Vec<f64> {
    let mut out_distribution_vec: Vec<f64> = vec![0.0; 203];
    let mut bucket_dt: NaiveDate = as_on_date.succ();
    //First 184 buckets are aggregated day-wise.
    let mut day_num = 0;
    //distribution upto one month
    daywise_distribution(
        as_on_date,
        &mut bucket_dt,
        1,
        &daywise_amt,
        config_data[0] as usize,
        &mut out_distribution_vec,
        &mut day_num,
        &mut tot_amt_op,
    );

    //distribution upto 1-3 months
    daywise_distribution(
        as_on_date,
        &mut bucket_dt,
        3,
        &daywise_amt,
        config_data[1] as usize,
        &mut out_distribution_vec,
        &mut day_num,
        &mut tot_amt_op,
    );
    //distribution upto 3-6 months
    daywise_distribution(
        as_on_date,
        &mut bucket_dt,
        6,
        &daywise_amt,
        config_data[2] as usize,
        &mut out_distribution_vec,
        &mut day_num,
        &mut tot_amt_op,
    );
    //distribution upto 6-9 months
    monthwise_distribution(
        as_on_date,
        &mut bucket_dt,
        9,
        &monthwise_amt,
        config_data[3] as usize,
        &mut out_distribution_vec,
        &mut day_num,
        &mut tot_amt_op,
    );
    //distribution upto 9-12 months
    monthwise_distribution(
        as_on_date,
        &mut bucket_dt,
        12,
        &monthwise_amt,
        config_data[4] as usize,
        &mut out_distribution_vec,
        &mut day_num,
        &mut tot_amt_op,
    );
    //distribution upto 12-24 months
    monthwise_distribution(
        as_on_date,
        &mut bucket_dt,
        24,
        &monthwise_amt,
        config_data[5] as usize,
        &mut out_distribution_vec,
        &mut day_num,
        &mut tot_amt_op,
    );
    //append the amount in the last bucket
    *tot_amt_op += after_two_years_amt;
    out_distribution_vec[202] = after_two_years_amt;
    out_distribution_vec
}

pub fn get_data(bucket_dt: NaiveDate, data: &HashMap<NaiveDate, f64>) -> f64 {
    let data = match data.get(&bucket_dt) {
        Some(amt) => *amt,
        None => 0.00,
    };
    data
}
pub fn daywise_distribution(
    as_on_date: NaiveDate,
    bucket_dt: &mut NaiveDate,
    months: usize,
    daywise_amt: &HashMap<NaiveDate, f64>,
    bucket_no: usize,
    dist_vec: &mut Vec<f64>,
    bucket_num: &mut usize,
    tot_amt_op: &mut f64,
) {
    let last_dt_for_daily_bucket = incr_dt_by_mon_presrv_eom_checked(as_on_date, months)
        .expect("Error while incrementing last date for daily bucket.");
    while *bucket_dt <= last_dt_for_daily_bucket {
        let val = get_data(*bucket_dt, &daywise_amt);
        *tot_amt_op += val;
        if bucket_no == 0 {
            dist_vec[*bucket_num] = val;
        } else {
            dist_vec[bucket_no as usize - 1] += val;
        }
        *bucket_num += 1;
        *bucket_dt += Duration::days(1);
    }
}
pub fn monthwise_distribution(
    as_on_date: NaiveDate,
    bucket_dt: &mut NaiveDate,
    months: usize,
    monthwise_amt: &HashMap<NaiveDate, f64>,
    bucket_no: usize,
    dist_vec: &mut Vec<f64>,
    bucket_num: &mut usize,
    tot_amt_op: &mut f64,
) {
    let last_dt_for_monthly_bucket = incr_dt_by_mon_presrv_eom_checked(as_on_date, months)
        .expect("Error while incrementing last date for monthly bucket.");
    while *bucket_dt <= last_dt_for_monthly_bucket {
        let val = get_data(*bucket_dt, &monthwise_amt);
        *tot_amt_op += val;
        if bucket_no == 0 {
            dist_vec[*bucket_num] = val;
        } else {
            dist_vec[bucket_no as usize - 1] += val;
        }
        *bucket_num += 1;
        *bucket_dt = incr_dt_by_mon_presrv_eom_checked(*bucket_dt, 1)
            .expect("bucket_date can not increased by one month");
    }
}
