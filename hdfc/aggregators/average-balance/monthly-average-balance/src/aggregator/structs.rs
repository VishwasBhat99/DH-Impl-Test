use super::input_account::InputParsedAccount;
use statics::DEFAULT_FLOAT;

#[derive(Debug, Clone)]
pub struct AggregateData {
    int_rt: f64,
    pub bal: f64,
}

impl AggregateData {
    pub fn new() -> AggregateData {
        AggregateData {
            int_rt: DEFAULT_FLOAT,
            bal: DEFAULT_FLOAT,
        }
    }

    pub fn add(&mut self, account: &InputParsedAccount) {
        self.add_to_store(account.amt, account.int_rt);
    }

    fn add_to_store(&mut self, p_a: f64, r: f64) {
        self.bal += p_a;
        self.int_rt += r;
    }

    pub fn average(&mut self, no_of_days: f64) {
        self.bal /= no_of_days;
        self.int_rt /= no_of_days;
    }

    pub fn to_string(&self) -> String {
        format!("|{:.2}|{:.2}\n", self.bal, self.int_rt)
    }
}
