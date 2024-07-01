use aggregator::cashflows::CashflowVec;
use chrono::Duration;
use chrono::NaiveDate;
use rbdate::{incr_dt_by_mon_presrv_eom_checked, timestamp};

#[derive(Clone, Debug)]
pub struct Cashflow {
    pub amt: Vec<f64>,
    pub weighted_rate: Vec<f64>,
    pub spread: f64,
    pub outstanding_amt: f64,
    pub outstanding_rate: f64,
}
pub fn aggregate_existing(
    mut count: usize,
    mut map_value: Cashflow,
    as_on_date: &NaiveDate,
    ex_rt: &f64,
    cashflow_vec: CashflowVec,
    rate: f64,
    spread_rt: &f64,
    is_npa_acc: &bool,
) -> Cashflow {
    let mut day_count = 0;
    let mut outstanding_amt: f64 = 0.0;

    let len = cashflow_vec.date.len().clone();
    let mut start_date = as_on_date.pred();
    let mut count_flag = true;
    let end_date =
        incr_dt_by_mon_presrv_eom_checked(*as_on_date, 24).expect("increase by 24 months");
    for (_, amt) in cashflow_vec.date_amt_map.iter() {
        outstanding_amt = outstanding_amt + (ex_rt * amt);
    }

    let out_amt_smry = map_value.outstanding_amt + outstanding_amt;
    let out_rate_smry = map_value.outstanding_rate + rate * outstanding_amt;
    let spread_value = map_value.spread + (spread_rt * outstanding_amt);
    let vec_len = map_value.amt.len();
    let mut date_vec = cashflow_vec.date.clone();
    date_vec.sort();
    if *is_npa_acc {
        while day_count < vec_len {
            map_value.amt[day_count] = map_value.amt[day_count] + outstanding_amt;
            map_value.weighted_rate[day_count] = map_value.weighted_rate[day_count] + 0.0;
            day_count += 1;
        }
    } else {
        for date in date_vec.clone() {
            if date >= as_on_date.and_hms(0, 0, 0).timestamp() {
                break;
            }
            outstanding_amt = outstanding_amt
                - (cashflow_vec
                    .date_amt_map
                    .get(&date)
                    .expect("Cannot deduct amount from outstanding amount")
                    * ex_rt);
        }
        start_date += Duration::days(1);
        while start_date <= end_date {
            if count_flag == true && count < len {
                if cashflow_vec
                    .date_amt_map
                    .contains_key(&timestamp(start_date))
                {
                    outstanding_amt = outstanding_amt
                        - (cashflow_vec
                            .date_amt_map
                            .get(&timestamp(start_date))
                            .expect("Cannot deduct amount from outstanding amount")
                            * ex_rt);

                    count += 1;
                }
            }

            map_value.amt[day_count] = map_value.amt[day_count] + outstanding_amt;
            map_value.weighted_rate[day_count] =
                map_value.weighted_rate[day_count] + (rate * outstanding_amt);
            start_date += Duration::days(1);
            day_count += 1;
            if outstanding_amt <= 0.0 || count >= len {
                count_flag = false;
            }
        }
    }

    Cashflow {
        amt: map_value.amt,
        weighted_rate: map_value.weighted_rate,
        spread: spread_value,
        outstanding_amt: out_amt_smry,
        outstanding_rate: out_rate_smry,
    }
}
pub fn aggregate_new(
    mut count: usize,
    as_on_date: &NaiveDate,
    ex_rt: &f64,
    cashflow_vec: CashflowVec,
    rate: f64,
    spread_rt: &f64,
    is_npa_acc: &bool,
) -> Cashflow {
    let mut outstanding_amt: f64 = 0.0;
    let mut amt_vec: Vec<f64> = Vec::new();
    let mut rate_vec: Vec<f64> = Vec::new();
    let len = cashflow_vec.date.len().clone();
    let mut start_date = as_on_date.pred();
    let mut count_flag = true;
    let end_date =
        incr_dt_by_mon_presrv_eom_checked(*as_on_date, 24).expect("Cannot increase by 24 months");
    let mut date_vec = cashflow_vec.date.clone();
    date_vec.sort();
    for (_, amt) in cashflow_vec.date_amt_map.iter() {
        outstanding_amt = outstanding_amt + (ex_rt * amt);
    }
    let out_amt_smry = outstanding_amt;
    let out_rate_smry = rate * outstanding_amt;
    let spread_value: f64 = spread_rt * outstanding_amt;

    if *is_npa_acc {
        while start_date.succ() <= end_date {
            amt_vec.push(outstanding_amt);
            rate_vec.push(0.0);
            start_date += Duration::days(1);
        }
    } else {
        for date in date_vec.clone() {
            if date >= as_on_date.and_hms(0, 0, 0).timestamp() {
                break;
            }
            outstanding_amt = outstanding_amt
                - (cashflow_vec
                    .date_amt_map
                    .get(&date)
                    .expect("Cannot deduct amount from outstanding amount")
                    * ex_rt);
        }
        start_date += Duration::days(1);
        while start_date <= end_date {
            if count_flag && count < len {
                if cashflow_vec
                    .date_amt_map
                    .contains_key(&timestamp(start_date))
                {
                    outstanding_amt = outstanding_amt
                        - (cashflow_vec
                            .date_amt_map
                            .get(&timestamp(start_date))
                            .expect("Cannot deduct amount from outstanding amount")
                            * ex_rt);
                    count += 1;
                }
            }
            amt_vec.push(outstanding_amt);
            rate_vec.push(rate * outstanding_amt);
            start_date = start_date.succ();
            if outstanding_amt <= 0.0 || count >= len {
                count_flag = false;
            }
        }
    }

    Cashflow {
        amt: amt_vec,
        weighted_rate: rate_vec,
        spread: spread_value,
        outstanding_amt: out_amt_smry,
        outstanding_rate: out_rate_smry,
    }
}
