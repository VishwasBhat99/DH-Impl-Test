use account::Cashflow;
pub struct OPFields {
    pub entity: String,
    pub source: String,
    pub in_out: String,
    pub sub_type: String,
    pub counter_party: String,
    pub currency: String,
    pub avaliabile_limit: f64,
    pub deal_amount_lcy: f64,
    pub cf_date: i64,
    pub cp_parent_id: String,
    pub cashflows: Cashflow,
}
