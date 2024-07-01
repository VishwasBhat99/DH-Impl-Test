use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct AdjKey {
    pub start_date: i64,
    pub adj_id: i32,
}

impl Display for AdjKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}|{}", self.start_date, self.adj_id)
    }
}
