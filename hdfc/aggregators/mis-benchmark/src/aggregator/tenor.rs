#[derive(Debug, Hash, Default, Eq, PartialEq)]
pub struct Tenor {
    pub from_days: i64,
    pub to_days: i64,
}

impl Tenor {
    pub fn new(from_days: String, to_days: String) -> Tenor {
        Tenor {
            from_days: from_days.trim().parse::<i64>().unwrap_or(0),
            to_days: to_days.trim().parse::<i64>().unwrap_or(0),
        }
    }
}
