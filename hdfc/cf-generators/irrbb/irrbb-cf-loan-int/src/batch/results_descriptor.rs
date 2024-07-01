use std::ops::AddAssign;

// TODO: Re-evaluate the `u32`s to `u64`s.
#[derive(Debug)]
pub struct ResultsDescriptor {
    pub inputs_count: u32,
    pub cashflows_count: u64,
    pub successful_outputs_count: u32,
    pub erroneous_outputs_count: u32,
    pub total_amount_input: f64,
    pub total_principal_output: f64,
    pub total_interest_output: f64,
}

impl AddAssign for ResultsDescriptor {
    fn add_assign(&mut self, other: ResultsDescriptor) {
        self.inputs_count += other.inputs_count;
        self.cashflows_count += other.cashflows_count;
        self.successful_outputs_count += other.successful_outputs_count;
        self.erroneous_outputs_count += other.erroneous_outputs_count;
        self.total_amount_input += other.total_amount_input;
        self.total_principal_output += other.total_principal_output;
        self.total_interest_output += other.total_interest_output;
    }
}

impl ResultsDescriptor {
    pub fn new() -> ResultsDescriptor {
        ResultsDescriptor {
            inputs_count: 0,
            cashflows_count: 0,
            successful_outputs_count: 0,
            erroneous_outputs_count: 0,
            total_amount_input: 0.0,
            total_principal_output: 0.0,
            total_interest_output: 0.0,
        }
    }
}
