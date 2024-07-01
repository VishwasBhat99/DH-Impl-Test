use super::HashMap;
use rbdate::NaiveDate;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct ReconKey {
    pub currency: String,
    pub gl_type: String,
    pub gl_code: String,
}

impl ReconKey {
    pub fn new(currency: String, gl_type: String, gl_code: String) -> ReconKey {
        ReconKey {
            currency,
            gl_type,
            gl_code,
        }
    }
}

impl Display for ReconKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}|{}|{}", self.currency, self.gl_type, self.gl_code)
    }
}

#[derive(Debug, Clone)]
pub struct ReconMap {
    pub store: HashMap<ReconKey, f64>,
}

impl ReconMap {
    pub fn new() -> Self {
        ReconMap {
            store: HashMap::new(),
        }
    }

    pub fn print(&self, as_on_dt: NaiveDate, src_file: &str) -> String {
        let mut out_line = String::new();
        for (key, value) in &self.store {
            out_line.push_str(&format!(
                "{}|{}|{}|{}|{}|{}\n",
                as_on_dt.format("%d-%m-%Y"),
                src_file,
                key.gl_type,
                key.gl_code,
                value,
                key.currency,
            ));
        }
        out_line
    }
}
