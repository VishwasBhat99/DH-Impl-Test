use std::default::Default;
#[derive(Debug, Clone, Default)]
pub struct OraFields {
    pub ora_mis1: String,
    pub ora_prod: String,
    pub ora_gl: String,
    pub ora_category: String,
}

#[derive(Debug, Clone, Default)]
pub struct MasterLLGFields {
    pub alm_line: String,
    pub ia_line: String,
}
