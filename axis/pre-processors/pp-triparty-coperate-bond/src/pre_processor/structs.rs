#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct CommonCode {
    pub code_type: String,
    pub cm_code: String,
}
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct BondMasterKey {
    pub isin: String,
    pub country_code: String,
}
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct BondMasterValue {
    pub guarantee_type: String,
    pub rating_id: String,
    pub is_financial: String,
}
