use aggregator::reports::input_report::InputReport;
use aggregator::reports::llg_summaries_report::LLGSummariesReport;
use aggregator::reports::llgs_report::LLGsReport;
use sdb_io;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde_json;
use std::io::Write;

pub mod input_report;
pub mod llg_summaries_report;
pub mod llgs_report;

pub struct AggregationReport {
    input_report: InputReport,
    llgs_report: LLGsReport,
    llg_summaries_report: LLGSummariesReport,
}

impl AggregationReport {
    pub fn new(
        input_report: InputReport,
        llgs_report: LLGsReport,
        llg_summaries_report: LLGSummariesReport,
    ) -> AggregationReport {
        AggregationReport {
            input_report,
            llgs_report,
            llg_summaries_report,
        }
    }
}

impl AggregationReport {
    pub fn serialise_to_path(self, path: &str) {
        let rprt_path = format!("{}-heath-check-report", path);
        let report_json = serde_json::to_string_pretty(&self).unwrap();

        let full_path = format!("{}.json", &rprt_path);
        let mut wrtr = match sdb_io::buf_file_wrtr(&full_path, None) {
            Ok(w) => w,
            Err(e) => panic!(format!(
                "Cannot write to file at path: '{}'. Error: {}",
                full_path, e
            )),
        };

        wrtr.write(&report_json.as_bytes()).unwrap();
        wrtr.flush()
            .expect("Unable to flush report writer contents");
    }
}

impl Serialize for AggregationReport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("report", 4)?;
        s.serialize_field("input", &self.input_report)?;
        s.serialize_field("llgs", &self.llgs_report)?;
        s.serialize_field("llgSummaries", &self.llg_summaries_report)?;

        s.end()
    }
}
