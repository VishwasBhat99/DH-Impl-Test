#[derive(Debug, Clone)]
pub struct BIUFields {
    pub acc_no: String,
    pub t1: String,
    pub t2: String,
    pub t3: String,
    pub t4: String,
    pub div: String,
}

impl Default for BIUFields {
    fn default() -> Self {
        BIUFields {
            acc_no: String::from("N"),
            t1: String::from("N"),
            t2: String::from("N"),
            t3: String::from("N"),
            t4: String::from("N"),
            div: String::from("N"),
        }
    }
}

impl BIUFields {
    pub fn new() -> Self {
        BIUFields {
            ..Default::default()
        }
    }

    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}\n",
            self.acc_no, self.t1, self.t2, self.t3, self.t4, self.div,
        )
    }
}
