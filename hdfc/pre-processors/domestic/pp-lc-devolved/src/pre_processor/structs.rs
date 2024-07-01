use super::{HashMap, NaiveDate};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Records {
    pub ccy: String,
    pub dev_dt: NaiveDate,
}

#[derive(Debug, Default)]
pub struct RecordsWithBalance {
    pub store: HashMap<Records, f64>,
}

#[derive(Debug, Default)]
pub struct Output {
    pub dev_dt: String,
    pub ccy: String,
    pub balance: String,
}

impl Default for Records {
    fn default() -> Records {
        Records {
            ccy: String::default(),
            dev_dt: NaiveDate::from_ymd(1970, 1, 1),
        }
    }
}

impl Records {
    pub fn new() -> Records {
        Default::default()
    }

    pub fn insert(&mut self, ccy: String, dt: NaiveDate) {
        self.ccy = ccy;
        self.dev_dt = dt;
    }

    fn display(&self) -> String {
        format!(
            "{}|{}",
            self.dev_dt.format("%d-%m-%Y").to_string(),
            self.ccy
        )
    }
}

impl RecordsWithBalance {
    pub fn new() -> RecordsWithBalance {
        RecordsWithBalance {
            store: HashMap::new(),
        }
    }

    pub fn print(&mut self) -> String {
        let mut out_line = String::new();
        for (record, bal) in self.store.drain() {
            out_line.push_str(&format!("{}|{}\n", record.display(), bal));
        }
        out_line
    }
}
