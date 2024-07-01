use chrono::NaiveDate;
use rbdate::{incr_dt_by_days, incr_dt_by_mon_presrv_eom_checked, incr_dt_by_yrs};
use std::collections::HashMap;
//Function to derive the Naivedate from Master file as DD-MM-YYYY format
pub fn naivedate_for_master(date: &str, as_on_date: &NaiveDate) -> NaiveDate {
    NaiveDate::parse_from_str(date, "%d-%b-%Y").unwrap_or(*as_on_date)
}
//Function to derive the Naivedate from Ref2 file as DD-MM-YYYY format
pub fn naivedate_for_ref2(date: &str, as_on_date: &NaiveDate) -> NaiveDate {
    NaiveDate::parse_from_str(date, "%d-%m-%y").unwrap_or(*as_on_date)
}
//function to calculate reset date
pub fn calc_reset_date(
    benchmark_name: String,
    cod_acct_no: String,
    ref3_file_map: &HashMap<String, bool>,
    ref2_file_map: &HashMap<String, Vec<String>>,
    as_on_date: NaiveDate,
    mat_date: NaiveDate,
) -> (NaiveDate, String) {
    if ref2_file_map.contains_key(&cod_acct_no) {
        let ref2_data = ref2_file_map.get(&cod_acct_no).unwrap();
        if ref2_data[1].to_uppercase().to_string() == "FLOATING" || ref2_data[0].starts_with('9') {
            if ref3_file_map.contains_key(&benchmark_name) {
                let next_reset_date = naivedate_for_ref2(&ref2_data[2], &as_on_date);

                if !ref2_data[2].is_empty()
                    && (next_reset_date
                        != NaiveDate::parse_from_str("01-Jan-1900", "%d-%b-%Y")
                            .unwrap_or(as_on_date)
                        || next_reset_date
                            != NaiveDate::parse_from_str("01-Jan-1800", "%d-%b-%Y")
                                .unwrap_or(as_on_date))
                {
                    if &next_reset_date >= &as_on_date {
                        (next_reset_date, "Floating".to_string())
                    } else {
                        let mut updated_next_reset_date = next_reset_date;
                        while &updated_next_reset_date <= &as_on_date {
                            updated_next_reset_date = add_freq(
                                updated_next_reset_date,
                                ref2_data[3].to_string(),
                                as_on_date,
                            );
                        }
                        (updated_next_reset_date, "Floating".to_string())
                    }
                } else {
                    (mat_date, "Fixed".to_string())
                }
            } else {
                (mat_date, "Fixed".to_string())
            }
        } else {
            (mat_date, "Fixed".to_string())
        }
    } else {
        (mat_date, "Fixed".to_string())
    }
}

//Function to Add the frequency in data
pub fn add_freq(date: NaiveDate, reset_freq: String, as_on_date: NaiveDate) -> NaiveDate {
    let cap_reset_freq: String = reset_freq.trim().to_uppercase().to_string();
    let next_reset_date = match cap_reset_freq.as_str() {
        "ANNUAL" | "YEARLY" => incr_dt_by_yrs(date, 1),
        "QUARTERLY" => incr_dt_by_mon_presrv_eom_checked(date, 3).unwrap_or(as_on_date),
        "HALFYEARLY" | "HALF YEARLY" | "HALF-YEARLY" => {
            incr_dt_by_mon_presrv_eom_checked(date, 6).unwrap_or(as_on_date)
        }
        "MONTHLY" => incr_dt_by_mon_presrv_eom_checked(date, 1).unwrap_or(as_on_date),
        "BIMONTHLY" | "BI-MONTHLY" | "BI MONTHLY" => incr_dt_by_days(date, 14),
        _ => incr_dt_by_mon_presrv_eom_checked(date, 3).unwrap_or(as_on_date),
    };
    next_reset_date
}
//function to remove the junk character from a string
pub fn remove_junk_char(input_string: &String) -> String {
    input_string
        .to_string()
        .replace("'", "")
        .replace("\"", "")
        .replace("`", "")
}
