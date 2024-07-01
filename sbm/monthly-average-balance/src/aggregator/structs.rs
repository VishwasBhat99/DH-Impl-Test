use super::input_account::InputParsedAccount;
use statics::DEFAULT_FLOAT;

#[derive(Debug, Clone)]
pub struct AggregateData {
    pub bal: f64,
    pub int_rt: f64,
}

impl AggregateData {
    pub fn new() -> AggregateData {
        AggregateData {
            bal: DEFAULT_FLOAT,
            int_rt: DEFAULT_FLOAT,
        }
    }

    pub fn add(&mut self, account: &InputParsedAccount) {
        self.add_to_store(account.amt, account.int_rt);
    }

    fn add_to_store(&mut self, p_a: f64, int_rt: f64) {
        self.bal += p_a;
        self.int_rt += int_rt;
    }

    pub fn average(&mut self, no_of_days: f64) {
        self.int_rt = if self.int_rt != 0.0 && self.bal != 0.0 {
            self.int_rt / self.bal
        } else if self.int_rt != 0.0 && self.bal == 0.0 {
            self.int_rt
        } else {
            0.0
        };
        self.bal /= no_of_days;
    }

    pub fn to_string(&self) -> String {
        format!("|{:.2}|{:.2}\n", self.bal, self.int_rt)
    }
}
