use super::llg_key::LLGKey;

#[derive(Debug, Clone, PartialEq)]
pub struct AccData {
    pub grp_key: LLGKey,
    pub acc_data: Vec<f64>,
}
