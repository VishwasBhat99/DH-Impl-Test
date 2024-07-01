use std::collections::HashMap;
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Tenor {
    pub start_tenor: i64,
    pub end_tenor: i64,
}

impl Tenor {
    pub fn new(start_tenor: String, end_tenor: String) -> Tenor {
        Tenor {
            start_tenor: start_tenor.trim().parse::<i64>().unwrap_or(0),
            end_tenor: end_tenor.trim().parse::<i64>().unwrap_or(0),
        }
    }
}

pub fn get_tenor(org_tenor: String, tenor_map: &HashMap<Tenor, String>) -> String {
    let org_tenor = org_tenor.trim().parse::<i64>().unwrap_or(0);
    for (tenor, org_tenor_cat) in tenor_map.iter() {
        if tenor.start_tenor <= org_tenor && tenor.end_tenor >= org_tenor {
            return org_tenor_cat.to_string();
        }
    }
    String::from("NA")
}

pub fn get_clubbed_tenor_rate(
    pdt_code: &str,
    cust_type_staff: &[String],
    cust_type_senior: &[String],
    clubbed_tenor: &str,
    tenor_rt_staff_map: &HashMap<String, String>,
    tenor_rt_senior_map: &HashMap<String, String>,
    tenor_rt_others_map: &HashMap<String, String>,
) -> f64 {
    let cust_type: String;
    if cust_type_staff.contains(&pdt_code.to_string()) {
        cust_type = "staff".to_string();
    } else if cust_type_senior.contains(&pdt_code.to_string()) {
        cust_type = "senior".to_string();
    } else {
        cust_type = "others".to_string();
    }
    //Return the derived clubbed tenor rate.
    if cust_type == "staff" {
        return match tenor_rt_staff_map.get(clubbed_tenor) {
            Some(val) => val.to_string().trim().parse::<f64>().unwrap_or(0.0),
            None => 0.00,
        };
    } else if cust_type == "senior" {
        return match tenor_rt_senior_map.get(clubbed_tenor) {
            Some(val) => val.to_string().trim().parse::<f64>().unwrap_or(0.0),
            None => 0.00,
        };
    } else {
        return match tenor_rt_others_map.get(clubbed_tenor) {
            Some(val) => val.to_string().trim().parse::<f64>().unwrap_or(0.0),
            None => 0.00,
        };
    }
}
