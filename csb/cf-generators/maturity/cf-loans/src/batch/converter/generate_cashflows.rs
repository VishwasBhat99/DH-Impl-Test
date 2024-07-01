use super::{
    days_with_convn, macros, timestamp, Cashflow, Conventions, Input, Logger, NaiveDate,
    DEFAULT_FLOAT, DEFAULT_INT,
};
use chrono::Datelike;
use rbdate::{get_month_end_date, incr_dt_by_mon_presrv_eom};

pub fn generate(
    input_account: &Input,
    convention: Conventions,
    as_on_date: NaiveDate,
    is_contractual: bool,
    log: &Logger,
) -> Vec<Cashflow> {
    let mut cfs_vec = Vec::with_capacity(15);
    let mut tot_prin_in_op = 0.0;
    let mut ost_amt = input_account.bal.abs();
    let mat_dt = match input_account.last_emi_dt {
        Some(dt) => dt,
        None => match input_account.lm_exp {
            Some(dt) => dt,
            None => {
                log_error!(
                    log,
                    "`lm_exp`: `{:?}` is not well-formatted for account: `{}`.",
                    input_account.lm_exp,
                    input_account.acc_no,
                );
                as_on_date
            }
        },
    };
    let acc_open_dt = match input_account.acnts_opening_dt {
        Some(dt) => dt,
        None => {
            log_error!(
                log,
                "`account_open_date`: `{:?}` is not well-formatted for account: `{}`.",
                input_account.acnts_opening_dt,
                input_account.acc_no,
            );
            as_on_date
        }
    };

    if acc_open_dt > mat_dt {
        log_error!(
            log,
            "`account_open_date`: `{}` is greater than `maturity_date`: `{}` for account: `{}`.",
            acc_open_dt,
            mat_dt,
            input_account.acc_no,
        );

        return vec![new_cashflow(timestamp(mat_dt), ost_amt, DEFAULT_FLOAT)];
    }

    if input_account.loan_type.to_uppercase().trim() == "NO EMI" {
        let int_amt = match cal_int_amt(
            acc_open_dt,
            mat_dt,
            ost_amt,
            input_account.int_rt,
            convention,
        ) {
            Ok(amt) => amt,
            Err(error) => {
                log_error!(log, "{} for account: `{}`.", error, input_account.acc_no);
                0.0
            }
        };
        let cf = new_cashflow(timestamp(mat_dt), ost_amt, int_amt);
        cfs_vec.push(cf);
        tot_prin_in_op += ost_amt;
        ost_amt = 0.0;
    } else {
        let schedules: Vec<&str> = input_account.schedules.split(',').collect();
        let mut index = 0;
        let len = schedules.len();
        let mut ccy = vec![""; len];
        let mut amt = vec![DEFAULT_FLOAT; len];
        let mut freq = vec![DEFAULT_INT; len];
        let mut from_dt = vec![NaiveDate::from_ymd(1970, 01, 01); len + 1];
        for schedule in schedules.iter() {
            let data: Vec<&str> = schedule.split(':').collect();
            if data.len() != 4 {
                let int_amt = match cal_int_amt(
                    acc_open_dt,
                    mat_dt,
                    ost_amt,
                    input_account.int_rt,
                    convention,
                ) {
                    Ok(amt) => amt,
                    Err(error) => {
                        log_error!(log, "{} for account: `{}`.", error, input_account.acc_no);
                        0.0
                    }
                };
                let cf = new_cashflow(timestamp(mat_dt), ost_amt, int_amt);
                cfs_vec.push(cf);
                log_error!(
                    log,
                    "`schedules` not found for account: `{}`.",
                    input_account.acc_no
                );
                return cfs_vec;
            }
            ccy[index] = data[0];
            amt[index] = data[1].parse().unwrap_or(DEFAULT_FLOAT);
            freq[index] = data[2].parse().unwrap_or(1);
            from_dt[index] = match NaiveDate::parse_from_str(data[3], "%d-%m-%Y") {
                Ok(dt) => dt,
                Err(error) => {
                    log_error!(
                        log,
                        "Invalid `from_dt`: `{}` for account: `{}`: `{}`.",
                        from_dt[index],
                        input_account.acc_no,
                        error
                    );
                    mat_dt
                }
            };
            index += 1;
        }
        from_dt[index] = mat_dt;

        let mut last_pay_date = if let Some(dt) = input_account.last_paid_emi_dt {
            dt
        } else {
            mat_dt
        };
        index = get_start_index(last_pay_date, &from_dt, len);
        let mut start_date = get_first_date(last_pay_date);
        let mut end_date = last_pay_date;
        let mut int_amt = 0.0;
        let is_last_date_of_month: bool = last_pay_date == get_month_end_date(last_pay_date);
        if !is_last_date_of_month {
            if let Some(paid_amt) = amt.get(0) {
                ost_amt += paid_amt;
                int_amt += match cal_int_amt(
                    start_date,
                    end_date,
                    ost_amt,
                    input_account.int_rt,
                    convention,
                ) {
                    Ok(amt) => amt,
                    Err(error) => {
                        log_error!(log, "{} for account: `{}`.", error, input_account.acc_no);
                        0.0
                    }
                };
                ost_amt -= paid_amt;
                start_date = end_date.succ();
            }
        }

        for idx in index..len {
            while last_pay_date <= from_dt[idx + 1] && ost_amt > 0.0 {
                if freq[idx] == 0 {
                    let int_amt = match cal_int_amt(
                        acc_open_dt,
                        mat_dt,
                        ost_amt,
                        input_account.int_rt,
                        convention,
                    ) {
                        Ok(amt) => amt,
                        Err(error) => {
                            log_error!(log, "{} for account: `{}`.", error, input_account.acc_no);
                            0.0
                        }
                    };
                    let cf = new_cashflow(timestamp(mat_dt), ost_amt, int_amt);
                    cfs_vec.push(cf);
                    log_warn!(
                        log,
                        "`interest_pay_frequency` is `0` for account: `{}`.",
                        input_account.acc_no
                    );
                    return cfs_vec;
                }
                let cf_date = incr_dt_by_mon_presrv_eom(last_pay_date, freq[idx] as usize);
                let mut next_cf_date;
                if let Some(dt) = cf_date {
                    next_cf_date = dt;
                    if next_cf_date > mat_dt {
                        break;
                    }
                } else {
                    log_error!(
                        log,
                        "Cannot calculate next cf date for account: {}",
                        input_account.acc_no
                    );
                    next_cf_date = mat_dt;
                }
                if is_last_date_of_month {
                    next_cf_date = get_month_end_date(next_cf_date);
                }
                end_date = get_month_end_date(start_date);
                int_amt += match cal_int_amt(
                    start_date,
                    end_date,
                    ost_amt,
                    input_account.int_rt,
                    convention,
                ) {
                    Ok(amt) => amt,
                    Err(error) => {
                        log_error!(log, "{} for account: `{}`.", error, input_account.acc_no);
                        cfs_vec.clear();
                        ost_amt = input_account.bal.abs();
                        break;
                    }
                };
                end_date = next_cf_date;
                let i_a = int_amt;
                ost_amt += i_a;
                let mut prin_amt = amt[idx] - i_a;
                prin_amt = prin_amt.abs();

                let cf_dt_timestmp = rbdate::timestamp(next_cf_date);
                let cf = new_cashflow(cf_dt_timestmp, prin_amt, i_a);
                if !is_last_date_of_month {
                    int_amt += match cal_int_amt(
                        start_date,
                        end_date,
                        ost_amt,
                        input_account.int_rt,
                        convention,
                    ) {
                        Ok(amt) => amt,
                        Err(error) => {
                            log_error!(log, "{} for account: `{}`.", error, input_account.acc_no);
                            cfs_vec.clear();
                            ost_amt = input_account.bal.abs();
                            break;
                        }
                    };
                }
                start_date = next_cf_date.succ();

                cfs_vec.push(cf);
                ost_amt -= amt[idx].abs();
                tot_prin_in_op += prin_amt;
                last_pay_date = next_cf_date;
                int_amt -= i_a;
            }
        }
    }

    if ost_amt <= 0.0 {
        let cf_dt_timestmp = rbdate::timestamp(as_on_date);
        if let Some(mut cf) = cfs_vec.pop() {
            cf.prin_amt += ost_amt;
            cfs_vec.push(cf);
        } else {
            let cf = new_cashflow(cf_dt_timestmp, ost_amt, 0.0);
            cfs_vec.push(cf);
        }
        tot_prin_in_op += ost_amt;
    }

    let mut tot_cf_amt = 0.0;
    for cfs in cfs_vec.iter() {
        tot_cf_amt += cfs.prin_amt;
    }
    let adj_amt = input_account.bal.abs() - tot_cf_amt.abs();
    let cf_dt_timestmp = rbdate::timestamp(mat_dt);
    let cf = new_cashflow(cf_dt_timestmp, adj_amt, 0.0);
    cfs_vec.push(cf);
    tot_prin_in_op += adj_amt;

    if (tot_prin_in_op - input_account.bal) > 0.1 {
        let mismatched_amounts_error_string = format!(
            "Total principal amount calculated doesn't match outstanding amount for Account: {}\
             PrincipalAmount: {}, OutstandingAmount: {}",
            input_account.acc_no, tot_prin_in_op, input_account.bal
        );
        if !is_contractual {
            log_warn!(
                log,
                "Mismatch occured while calculating cashflows: {}",
                mismatched_amounts_error_string
            );
        }
    }

    log_debug!(
        log,
        "`account`: `{}`, cashflows: `{:?}`",
        input_account.acc_no,
        cfs_vec
    );

    cfs_vec
}

fn new_cashflow(d: i64, p: f64, i: f64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.set_date(d);
    cf.set_prin_amt(p);
    cf.set_int_amt(i);

    cf
}

fn cal_int_amt(
    last_pay_date: NaiveDate,
    next_cf_date: NaiveDate,
    ost_amt: f64,
    int_rate: f64,
    convention: Conventions,
) -> Result<f64, String> {
    let days = match days_with_convn(last_pay_date, next_cf_date, &convention) {
        Ok(day) => day,
        Err(error) => {
            return Err(format!("{}", error));
        }
    };
    let no_of_days = days.days_btw_dts as f64 + 1.0;
    let days_in_yr = days.day_in_yr as f64;
    let int_amt = (ost_amt * int_rate * no_of_days) / (days_in_yr * 100.0);
    Ok(int_amt)
}

fn get_start_index(lst_pay_dt: NaiveDate, from_dt: &Vec<NaiveDate>, len: usize) -> usize {
    let mut index = 0;
    while from_dt[index] <= lst_pay_dt && index < len {
        index += 1;
    }
    if index > 0 {
        index - 1
    } else {
        index
    }
}

fn get_first_date(date: NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd(date.year(), date.month(), 1)
}
