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
    pub llg: i32,
    pub gl_cd: String,
    pub ccy: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AggregatorValues {
    pub aggr_amt: f64,
}

impl AggregatorValues {
    pub fn aggregateamount(&mut self, aggrvalue: AggregatorValues) {
        self.aggr_amt += aggrvalue.aggr_amt;
    }

    pub fn values_multiplied_by(&mut self, multiplier: f64) {
        self.aggr_amt *= multiplier;
    }
}
