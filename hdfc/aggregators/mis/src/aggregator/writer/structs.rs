use aggregator::cashflow_organizer::cashflow_aggregation::LLGAggregateOnDay;
use aggregator::llg::llg_key::LLGKey;

#[derive(Clone, Debug)]
pub struct LLGRecordPrefix {
    pub subtype_id: LLGKey,
    pub as_on: String,
}

impl LLGRecordPrefix {
    pub fn to_string(&self) -> String {
        format!(
            "{}|{}|{}|",
            self.as_on, self.subtype_id.llg_cd, self.subtype_id.ccy,
        )
    }
}

#[derive(Debug)]
pub struct LLGAggregatedRecord {
    pub prefix: LLGRecordPrefix,
    pub aggregates: Vec<Option<LLGAggregateOnDay>>,
}
