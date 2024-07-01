use std::ops::AddAssign;

pub struct AccountDescriptor {
    pub cashflows_count: u64,
    pub total_amount_input: f64,
    pub total_principal_output: f64,
    pub total_interest_output: f64,
}

impl AddAssign for AccountDescriptor {
    fn add_assign(&mut self, other: AccountDescriptor) {
        self.cashflows_count += other.cashflows_count;
        self.total_amount_input += other.total_amount_input;
        self.total_principal_output += other.total_principal_output;
        self.total_interest_output += other.total_interest_output;
    }
}

impl AccountDescriptor {
    pub fn new() -> AccountDescriptor {
        AccountDescriptor {
            cashflows_count: 0,
            total_amount_input: 0.0,
            total_principal_output: 0.0,
            total_interest_output: 0.0,
        }
    }
}
