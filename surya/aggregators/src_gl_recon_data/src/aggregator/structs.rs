#[derive(Deserialize)]
pub struct RequiredFields {
    pub fields: Vec<FieldInfo>,
}
#[derive(Deserialize)]
pub struct FieldInfo {
    pub field_name: String,
    pub output_file_type: String,
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct AggregatorKeys {
    pub src_field_cd: String,
    pub gl_type: String,
    pub src_gl_cd: String,
    pub gl_desc: String,
    pub src_ccy: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AggregatorValues {
    pub gl_amt: f64,
}

impl AggregatorValues {
    pub fn aggregateamount(&mut self, aggrvalue: AggregatorValues) {
        self.gl_amt += aggrvalue.gl_amt;
    }

    pub fn values_multiplied_by(&mut self, multiplier: f64) {
        self.gl_amt *= multiplier;
    }
}
