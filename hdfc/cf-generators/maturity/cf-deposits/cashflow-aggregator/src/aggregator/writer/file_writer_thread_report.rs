use aggregator::llg_key::LLGKey;
use aggregator::reports::llgs_report::PrincipalInterestAmountReport;
use std::collections::HashMap;

#[derive(Debug)]
pub struct AggregateWriterThreadReport {
    pub thread_no: u32,
    pub total_records_written: u32,
    pub llg_amounts_report: HashMap<
        LLGKey,
        PrincipalInterestAmountReport
    >
}