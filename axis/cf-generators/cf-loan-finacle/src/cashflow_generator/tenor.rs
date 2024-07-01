use std::collections::HashMap;
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
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

pub fn get_tenor(org_tenor: i64, tenor_map: &HashMap<Tenor, i64>) -> i64 {
    for (tenor, org_tenor_cat) in tenor_map.iter() {
        if tenor.start_tenor <= org_tenor && tenor.end_tenor > org_tenor {
            return *org_tenor_cat;
        }
    }
    tenor_map.len() as i64
}
