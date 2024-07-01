use super::{HashMap, NaiveDate};

#[derive(Debug, Default)]
pub struct Balance {
    pub opening: f64,
    pub closing: f64,
}

#[derive(Debug, Default)]
pub struct BalanceWithDate {
    pub store: HashMap<NaiveDate, Balance>,
}

#[derive(Debug, Default)]
pub struct Output {
    pub opening: String,
    pub closing: String,
}

impl Balance {
    pub fn new() -> Balance {
        Default::default()
    }
    pub fn insert(&mut self, op: f64, cl: f64) {
        self.opening = op;
        self.closing = cl;
    }
    pub fn add_amts(&mut self, op: f64, cl: f64) {
        self.opening += op;
        self.closing += cl;
    }
}

impl BalanceWithDate {
    pub fn new() -> BalanceWithDate {
        BalanceWithDate {
            store: HashMap::new(),
        }
    }
    pub fn print(&mut self, llg_code: i32, currency: &str) -> Output {
        let mut op_bal_out_line = String::new();
        let mut cl_bal_out_line = String::new();
        for (date, bals) in self.store.drain() {
            op_bal_out_line.push_str(&format!(
                "{}|{}|{}|DIM1|DIM2|DIM3|DIM4|DIM5|{}|0.0\n",
                llg_code,
                date.format("%d-%m-%Y"),
                currency,
                bals.opening
            ));
            cl_bal_out_line.push_str(&format!(
                "{}|{}|{}|DIM1|DIM2|DIM3|DIM4|DIM5|{}|0.0\n",
                llg_code,
                date.format("%d-%m-%Y"),
                currency,
                bals.closing
            ));
        }
        Output {
            opening: op_bal_out_line,
            closing: cl_bal_out_line,
        }
    }
}
