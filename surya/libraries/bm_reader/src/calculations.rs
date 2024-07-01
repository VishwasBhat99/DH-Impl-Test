use super::*;

pub fn calc_yield_rate(
    lst_bm: &mut IntermediateBMPoints,
    days_diff: i64,
    is_exp_req: bool,
    bmrate_accuracy: String,
) -> Result<f64, Error> {
    let mut yield_rate = 0.0;

    for index in 0..lst_bm.len() {
        if index != lst_bm.len() - 1 {
            //case when days diff is equal to vertex
            if days_diff == lst_bm[index + 1].days_diff {
                yield_rate = lst_bm[index + 1].rate;
                break;
            }
            // case where tenor < lower vertex tenor 
            if days_diff <= lst_bm[index].days_diff {
                // case where days_diff is zero
                if (lst_bm[index].rate / lst_bm[index].days_diff as f64).is_nan() {
                    yield_rate = lst_bm[index].rate;
                } else {
                    yield_rate = (lst_bm[index].rate / lst_bm[index].days_diff as f64) * days_diff as f64;
                }
                break;
            }
            if days_diff >= lst_bm[index].days_diff && days_diff <= lst_bm[index + 1].days_diff {
                //calculate rate using interpolation
                if days_diff <= 31 {
                    yield_rate = match calc_yield_rate_daily(
                        days_diff,
                        lst_bm[index + 1].days_diff,
                        lst_bm[index].days_diff,
                        lst_bm[index + 1].rate,
                        lst_bm[index].rate,
                    ) {
                        Ok(val) => val,
                        Err(error) => return Err(error),
                    };
                } else {
                    yield_rate = match calc_yield_rate_monthly(
                        days_diff,
                        lst_bm[index + 1].month,
                        lst_bm[index].month,
                        lst_bm[index + 1].rate,
                        lst_bm[index].rate,
                        lst_bm[index + 1].days_diff,
                        lst_bm[index].days_diff,
                        bmrate_accuracy,
                    ) {
                        Ok(val) => val,
                        Err(error) => return Err(error),
                    };
                }
                break;
            } else {
                continue;
            }
        } else {
            if is_exp_req && lst_bm.len() >= 2 {
                yield_rate = lst_bm[index - 1].rate as f64
                    + ((days_diff - lst_bm[index - 1].days_diff) as f64)
                        * (lst_bm[index].rate - lst_bm[index - 1].rate)
                        / (lst_bm[index].days_diff - lst_bm[index - 1].days_diff) as f64;
            } else {
                yield_rate = lst_bm[index].rate;
            }
            break;
        }
    }

    Ok(yield_rate)
}

fn calc_yield_rate_daily(
    days_diff: i64,
    upperpoint: i64,
    lowerpoint: i64,
    upperpointrate: f64,
    lowerpointrate: f64,
) -> Result<f64, Error> {
    Ok(
        (((days_diff - lowerpoint) as f64 * (upperpointrate - lowerpointrate))
            / (upperpoint - lowerpoint) as f64)
            + lowerpointrate,
    )
}

fn calc_yield_rate_monthly(
    days_diff: i64,
    upperpoint: i64,
    lowerpoint: i64,
    upperpointrate: f64,
    lowerpointrate: f64,
    upperdays: i64,
    lowerdays: i64,
    bmrate_accuracy: String,
) -> Result<f64, Error> {
    if lowerpoint < 12 {
        Ok(lowerpointrate
            + ((days_diff - lowerdays) as f64
                * ((upperpointrate - lowerpointrate) / (upperdays - lowerdays) as f64) as f64))
    } else {
        if bmrate_accuracy == "M" {
        let month_diff = upperpoint - lowerpoint;
        let month_rate = (upperpointrate - lowerpointrate) / month_diff as f64;
        let req_mnth = (days_diff as f64 / 365.00 * 12.00) as f64;
        Ok(lowerpointrate + (month_rate * (req_mnth - lowerpoint as f64) as f64))
        } else {
            Ok(lowerpointrate
                + ((days_diff - lowerdays) as f64
                    * ((upperpointrate - lowerpointrate) / (upperdays - lowerdays) as f64) as f64))
        }
    }
}
