use chrono::NaiveDate;
use slog::Logger;
use stamp_ftp::bm_reader::IntermediateBmPoints;

pub fn calc_yieldrate(
    lst_bm: &mut Vec<IntermediateBmPoints>,
    days_diff: i64,
    date: NaiveDate,
    log: &Logger,
) -> f64 {
    let mut yield_rate = 0.0;

    for i in 0..lst_bm.len() {
        if !(i == lst_bm.len() - 1) {
            if days_diff <= lst_bm[i].days_diff {
                yield_rate = lst_bm[i].rate;
                break;
            }
            if days_diff >= lst_bm[i].days_diff && days_diff <= lst_bm[i + 1].days_diff {
                //calculate rate using interpolation
                if days_diff <= 31 {
                    yield_rate = calc_yieldratedaily(
                        days_diff,
                        lst_bm[i + 1].days_diff,
                        lst_bm[i].days_diff,
                        lst_bm[i + 1].rate,
                        lst_bm[i].rate,
                    );
                } else {
                    yield_rate = calc_yieldratemonthly(
                        days_diff,
                        lst_bm[i + 1].month,
                        lst_bm[i].month,
                        lst_bm[i + 1].rate,
                        lst_bm[i].rate,
                    );
                }

                break;
            } else {
                continue;
            }
        } else {
            yield_rate = lst_bm[i].rate;
            break;
        }
    }

    yield_rate
}

pub fn calc_yieldratedaily(
    days_diff: i64,
    upperpoint: i64,
    lowerpoint: i64,
    upperpointrate: f64,
    lowerpointrate: f64,
) -> f64 {
    let yield_rate = (((days_diff - lowerpoint) as f64 * (upperpointrate - lowerpointrate))
        / (upperpoint - lowerpoint) as f64)
        + lowerpointrate;
    return yield_rate;
}

pub fn calc_yieldratemonthly(
    days_diff: i64,
    upperpoint: i64,
    lowerpoint: i64,
    upperpointrate: f64,
    lowerpointrate: f64,
) -> f64 {
    let month_diff = upperpoint - lowerpoint;

    let month_rate = (upperpointrate - lowerpointrate) / month_diff as f64;

    let req_mnth = (days_diff as f64 / 365.00 * 12.00).round() as i64;

    let yield_rate = lowerpointrate + (month_rate * (req_mnth - lowerpoint) as f64);

    return yield_rate;
}
