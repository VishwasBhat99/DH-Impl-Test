use super::io_helpers;
use rbdate::current_time_utc;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde_json;
use slog::Logger;
use std::io::Write;

pub struct BatchProcessReport {
    pub time_taken_seconds: u64,
    pub thread_count: u8,
    pub batch_size: u32,
    pub inputs_processed: u32,
    pub total_cashflows: u64,
    pub total_successful_records: u32,
    pub total_faulty_records: u32,
    pub total_amount_input: f64,
    pub total_principal_output: f64,
    pub total_interest_output: f64,
}

impl BatchProcessReport {
    pub fn print_report(&self, logger: &Logger) -> () {
        let report_string = format!(
            "Batch Size: {}\tThreads: {}\nTo process {} lines, it took {} seconds",
            self.batch_size, self.thread_count, self.inputs_processed, self.time_taken_seconds
        );
        info!(logger, "{}", &report_string);
        println!("{}", &report_string);
    }

    pub fn serialise_to_file(&self, file_path: &str) {
        let report_json = serde_json::to_string_pretty(&self).unwrap();
        let mut file_writer = io_helpers::buf_file_writer(file_path, ".json", None);
        file_writer
            .write(&report_json.as_bytes())
            .expect("Cannot write Json process report.");
        io_helpers::flush_contents(file_writer, "JSON Writer");
    }
}

impl Serialize for BatchProcessReport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("BatchProcessReport", 10)?;
        s.serialize_field("cashflowGenerationDate", &current_time_utc())?;
        s.serialize_field("totalTimeTakenSeconds", &self.time_taken_seconds)?;
        s.serialize_field("inputRecords", &self.inputs_processed)?;
        s.serialize_field("outputRecords", &self.total_successful_records)?;
        s.serialize_field("erroneousRecords", &self.total_faulty_records)?;
        s.serialize_field("totalCashflowsGenerated", &self.total_cashflows)?;
        s.serialize_field("totalAmountInInput", &self.total_amount_input)?;
        s.serialize_field("totalPrincipalInOutput", &self.total_principal_output)?;
        s.serialize_field("totalInterestInOutput", &self.total_interest_output)?;
        s.end()
    }
}
