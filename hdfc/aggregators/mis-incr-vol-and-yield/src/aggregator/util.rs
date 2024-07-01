use rbdate::incr_dt_by_mon_presrv_eom;
use rbdate::NaiveDate;

pub fn add_to_prev_data(a: Vec<f64>, b: &Vec<f64>) -> Vec<f64> {
    let max_len = std::cmp::max(a.len(), b.len());
    let mut final_data: Vec<f64> = vec![0.0; max_len];
    for idx in 0..max_len {
        final_data[idx] = a[idx] + b[idx];
    }
    final_data
}

pub fn get_days(info: &str, as_on_date: &NaiveDate) -> i64 {
    let mut alpha_code: Vec<&str> = info.split(|c: char| c.is_numeric()).collect();
    alpha_code.retain(|&x| x != "");
    let mut num_code: Vec<&str> = info.split(|c: char| c.is_alphabetic()).collect();
    num_code.retain(|&x| x != "");
    let mut days = 0;
    for (i, num_val) in num_code.iter().enumerate() {
        let period = num_val.to_string() + alpha_code[i];
        days += num_days(&period, as_on_date);
    }
    days
}

fn num_days(info: &str, as_on_date: &NaiveDate) -> i64 {
    if info.contains("D") {
        let period: i64 = info
            .trim_matches('D')
            .parse::<i64>()
            .expect("Invalid from day format");
        return period;
    } else if info.contains("M") {
        let period: usize = info
            .trim_matches('M')
            .parse::<usize>()
            .expect("Invalid from month format");
        let new_date = incr_dt_by_mon_presrv_eom(*as_on_date, period)
            .expect("Cannot add month to as on date as per prd slab config");
        return rbdate::num_days_start_to_end(*as_on_date, new_date);
    } else if info.contains("Y") {
        let period: usize = info
            .trim_matches('Y')
            .parse::<usize>()
            .expect("Invalid from year format");
        let new_date = incr_dt_by_mon_presrv_eom(*as_on_date, period * 12)
            .expect("Cannot add month to as on date as per prd slab config");
        return rbdate::num_days_start_to_end(*as_on_date, new_date);
    } else {
        panic!("Invalid period type in prd config file.");
    }
}
