use std::default::Default;
#[derive(Debug, Clone, Default)]
pub struct LMRBond {
    pub class_1: String,
    pub class_2: String,
    pub class_3: String,
    pub tenure_classification: String,
    pub sys_identifier: String,
}
