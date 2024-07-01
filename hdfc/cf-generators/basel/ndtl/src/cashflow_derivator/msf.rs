use rbdate::NaiveDate;
use std::collections::HashMap;

#[derive(Debug, Hash, Default, Eq, PartialEq)]
pub struct Msf {
    pub from_date: String,
    pub to_date: String,
}

impl Msf {
    pub fn new(from_date: String, to_date: String) -> Msf {
        Msf {
            from_date: from_date,
            to_date: to_date,
        }
    }
}

pub fn get_msf_desc(date: NaiveDate, msf_map: &HashMap<Msf, String>) -> String {
    for (msf, val) in msf_map.iter() {
        let from_date = NaiveDate::parse_from_str(&msf.from_date, "%d-%m-%Y");
        let to_date = NaiveDate::parse_from_str(&msf.to_date, "%d-%m-%Y");
        if from_date.unwrap() <= date && to_date.unwrap() >= date {
            return val.to_string();
        }
    }
    String::from("NA")
}
