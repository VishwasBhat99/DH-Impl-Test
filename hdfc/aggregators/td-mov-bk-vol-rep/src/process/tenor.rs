use std::collections::HashMap;

#[derive(Debug, Hash, Default, Eq, PartialEq)]
pub struct Tenor {
    pub from_days: i64,
    pub to_days: i64,
}

impl Tenor {
    pub fn new(from_days: String, to_days: String) -> Tenor {
        Tenor {
            from_days: from_days.to_string().trim().parse::<i64>().unwrap_or(0),
            to_days: to_days.to_string().trim().parse::<i64>().unwrap_or(0),
        }
    }
}

pub fn get_tenor_desc(day: i64, tenor_map: &HashMap<Tenor, String>) -> String {
    for (tenor, desc) in tenor_map.iter() {
        if tenor.from_days <= day && tenor.to_days >= day {
            return desc.to_string();
        }
    }
    String::from("NA")
}
