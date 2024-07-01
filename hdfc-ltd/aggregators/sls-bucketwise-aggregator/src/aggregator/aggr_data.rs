use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone)]

pub struct AggrData {
    pub data: Vec<f64>,
}

impl AggrData {
    pub fn append_data(&mut self, new_data: AggrData) {
        for (key, val) in new_data.data.iter().enumerate() {
            self.data[key] += val;
        }
    }
}
impl Display for AggrData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let output_line: Vec<String> = self.data.iter().map(|val| val.to_string()).collect();
        write!(f, "{}", output_line.join("|"))
    }
}
