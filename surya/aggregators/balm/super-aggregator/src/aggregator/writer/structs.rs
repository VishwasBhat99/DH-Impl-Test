use aggregator::llg_key::LLGKey;
use aggregator::structs::LLGAggregateOnDay;

#[derive(Debug, Copy, Clone)]
pub enum CashflowAggregatedType {
    Slr,
    Irs,
    Int,
}

impl CashflowAggregatedType {
    pub fn to_string(&self) -> String {
        match self {
            CashflowAggregatedType::Slr => format!("SLR"),
            CashflowAggregatedType::Irs => format!("IRS"),
            CashflowAggregatedType::Int => format!("INT"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LLGRecordPrefix {
    pub subtype_id: LLGKey,
    pub as_on: String,
    pub currency_id: String,
    pub scheme_id: String,
}

impl LLGRecordPrefix {
    pub fn to_string(&self, ttype: CashflowAggregatedType) -> String {
        format!(
            "{}|{}|{}|{}|{}|",
            self.subtype_id.category,
            self.as_on,
            self.currency_id,
            self.scheme_id,
            ttype.to_string()
        )
    }
}

// TODO: Add slr_or_irs to the LLGConstantPrefix.
// TODO: One prefix per LLG Record.
#[derive(Debug)]
pub struct LLGAggregatedRecord {
    pub prefix: LLGRecordPrefix,
    pub aggregates: Vec<Option<LLGAggregateOnDay>>,
}
