pub struct InputAccountReaderReport {
    total_input_lines_count: u64,
    well_formed_lines_count: u64,
    malformed_lines_count: u64,
    input_accounts_parsed_count: u64,
    input_accounts_not_parsed_count: u64,
}

impl InputAccountReaderReport {
    pub fn increment_total_lines_count(&mut self) {
        self.total_input_lines_count += 1;
    }
    pub fn increment_well_formed_lines_count(&mut self) {
        self.well_formed_lines_count += 1;
    }
    pub fn increment_malformed_lines_count(&mut self) {
        self.malformed_lines_count += 1;
    }
    pub fn increment_input_accounts_parsed_count(&mut self) {
        self.input_accounts_parsed_count += 1;
    }
    pub fn increment_input_accounts_not_parsed_count(&mut self) {
        self.input_accounts_not_parsed_count += 1;
    }
}
