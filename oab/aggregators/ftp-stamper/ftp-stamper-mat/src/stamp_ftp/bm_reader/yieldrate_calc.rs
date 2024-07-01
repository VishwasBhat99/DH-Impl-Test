use stamp_ftp::bm_reader::IntermediateBmPoints;
use statics::DEFAULT_FLOAT;

pub fn calc_yieldrate(lst_bm: &mut Vec<IntermediateBmPoints>, days_diff: i64) -> f64 {
    let mut yield_rate = DEFAULT_FLOAT;

    for index in 0..lst_bm.len() {
        if !(index == lst_bm.len() - 1) {
            if days_diff <= lst_bm[index].days_diff {
                yield_rate = lst_bm[index].rate;
                break;
            }
            if days_diff >= lst_bm[index].days_diff && days_diff <= lst_bm[index + 1].days_diff {
                //calculate rate using interpolation
                if days_diff <= 31 {
                    yield_rate = calc_yieldratedaily(
                        days_diff,
                        lst_bm[index + 1].days_diff,
                        lst_bm[index].days_diff,
                        lst_bm[index + 1].rate,
                        lst_bm[index].rate,
                    );
                } else {
                    yield_rate = calc_yieldratemonthly(
                        days_diff,
                        lst_bm[index + 1].month,
                        lst_bm[index].month,
                        lst_bm[index + 1].rate,
                        lst_bm[index].rate,
                    );
                }
                break;
            } else {
                continue;
            }
        } else {
            yield_rate = lst_bm[index].rate;
            break;
        }
    }

    yield_rate
}

pub fn calc_yieldratedaily(
    days_diff: i64,
    upper_point: i64,
    lower_point: i64,
    upper_point_rate: f64,
    lower_point_rate: f64,
) -> f64 {
    let yield_rate = (((days_diff - lower_point) as f64 * (upper_point_rate - lower_point_rate))
        / (upper_point - lower_point) as f64)
        + lower_point_rate;

    yield_rate
}

pub fn calc_yieldratemonthly(
    days_diff: i64,
    upper_point: i64,
    lower_point: i64,
    upper_point_rate: f64,
    lower_point_rate: f64,
) -> f64 {
    let month_rate = (upper_point_rate - lower_point_rate) / (upper_point - lower_point) as f64;
    let req_month = (days_diff as f64 / 365.00 * 12.00).round() as i64;
    let yield_rate = lower_point_rate + (month_rate * (req_month - lower_point) as f64);

    yield_rate
}
