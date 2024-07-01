///Aggregated Value Structure
#[derive(Debug, Clone, PartialEq)]
pub struct AggrVal {
    pub count: i64,
    pub out_bal: f64,
    pub limit_bal: f64,
}

impl AggrVal {
    pub fn new(count: i64, out_bal: f64, limit_bal: f64) -> AggrVal {
        AggrVal {
            count,
            out_bal,
            limit_bal,
        }
    }
}

impl AggrVal {
    pub fn aggregateamount(&mut self, aggrvalue: AggrVal) {
        self.count += 1;
        self.out_bal += aggrvalue.out_bal;
        self.limit_bal = aggrvalue.limit_bal;
    }
}
