extern crate rbdate;

use rbdate::*;

mod tests;

pub fn npa_cfdate_adjusment(cfdate: NaiveDate, npa_classification: String) -> Option<NaiveDate> {
    match npa_classification
        .replace("-", "")
        .replace(" ", "")
        .to_lowercase()
        .as_str()
    {
        "substandard" => incr_dt_by_mon_presrv_eom(cfdate, 36),
        "doubtful1" | "doubtful2" | "doubtful3" | "lossasset" | "lossassetslessthan91days" => {
            incr_dt_by_mon_presrv_eom(cfdate, 60)
        }
        _ => incr_dt_by_mon_presrv_eom(cfdate, 0),
    }
}
