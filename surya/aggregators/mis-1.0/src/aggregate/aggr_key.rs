use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct AggrKey {
    pub cust_id: String,
    pub dims: Vec<String>,
}

impl Display for AggrKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}",
            self.cust_id,
            &self
                .dims
                .clone()
                .into_iter()
                .map(|dim| dim.to_string())
                .collect::<Vec<String>>()
                .join("|")
        )
    }
}
