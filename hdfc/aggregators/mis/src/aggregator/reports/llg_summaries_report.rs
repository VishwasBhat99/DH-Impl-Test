use aggregator::llg::llg_key::LLGKey;
use aggregator::reports::llgs_report::OutstandingAmountReport;
use aggregator::writer::summaries::LLGSummaryRecordGrouped;
use serde::ser::{Serialize, SerializeMap, Serializer};
use std::collections::HashMap;

#[derive(Debug)]
pub struct LLGSummariesReport {
    pub report: HashMap<LLGKey, OutstandingAmountReport>,
}

impl LLGSummariesReport {
    pub fn new() -> LLGSummariesReport {
        LLGSummariesReport {
            report: HashMap::new(),
        }
    }

    pub fn build_with(&mut self, grouped_summary: &LLGSummaryRecordGrouped) {
        let mut report_for_llg = {
            // TODO: This is horribly wasteful. I'm getting around an API limitation
            // in the interest of time.
            if self.report.contains_key(&grouped_summary.subtype_id) {
                self.report.remove(&grouped_summary.subtype_id).unwrap()
            } else {
                OutstandingAmountReport::new()
            }
        };
        report_for_llg.outstanding_amount += grouped_summary.o_a.amount;

        self.report
            .insert(grouped_summary.subtype_id.clone(), report_for_llg);
    }
}

impl Serialize for LLGSummariesReport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.report.len()))?;
        for (k, v) in &self.report {
            map.serialize_entry(&format!("{}", k), v)?;
        }
        map.end()
    }
}
