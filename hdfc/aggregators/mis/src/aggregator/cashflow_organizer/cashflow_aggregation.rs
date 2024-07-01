use sdb_dyn_proto_rdr::compound_types::Cashflow;

#[derive(Debug, Copy, Clone)]
pub struct CashflowAggregatedOnDateBuilder {
    rate_outstanding_amount_weighted: f64,
    total_outstanding_amount: f64,
}

impl CashflowAggregatedOnDateBuilder {
    pub fn new() -> CashflowAggregatedOnDateBuilder {
        CashflowAggregatedOnDateBuilder {
            rate_outstanding_amount_weighted: 0.0,
            total_outstanding_amount: 0.0,
        }
    }

    pub fn add_from_builder(&mut self, other: CashflowAggregatedOnDateBuilder) {
        self.rate_outstanding_amount_weighted += other.rate_outstanding_amount_weighted;
        self.total_outstanding_amount += other.total_outstanding_amount;
    }

    pub fn add_outstanding_cf(&mut self, cf: &Cashflow, interest_rate: f64) {
        self.add_outstanding(f64::from(cf.get_principal_amount()), interest_rate);
    }

    fn add_outstanding(&mut self, o_a: f64, r: f64) {
        self.total_outstanding_amount += o_a;
        self.rate_outstanding_amount_weighted += o_a * r;
    }

    pub fn to_cf_aggregated(self) -> LLGAggregateOnDay {
        let o_a = {
            if self.total_outstanding_amount == 0.0 {
                // This will result in NaN in the output string. We don't want that.
                // NaNs occur only for the interest aggregates.
                CashflowAggregated {
                    amount: 0.0,
                    rate: 0.0,
                }
            } else {
                CashflowAggregated {
                    amount: self.total_outstanding_amount,
                    rate: self.rate_outstanding_amount_weighted / self.total_outstanding_amount,
                }
            }
        };

        LLGAggregateOnDay { o_a }
    }

    pub fn values_multiplied_by(&mut self, multiplier: f64) {
        self.rate_outstanding_amount_weighted *= multiplier;
        self.total_outstanding_amount *= multiplier;
    }
}

#[derive(Debug)]
pub struct CashflowAggregated {
    pub amount: f64,
    pub rate: f64,
}

// TODO: Keep in mind that there's only one field that's different here. Look into breaking it apart into constituent parts if that solves a problem.
#[derive(Debug)]
pub struct LLGAggregateOnDay {
    pub o_a: CashflowAggregated,
}

impl CashflowAggregated {
    fn new() -> CashflowAggregated {
        CashflowAggregated {
            amount: 0.0,
            rate: 0.0,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:.2}|{:.2}|", self.amount, self.rate)
    }

    pub fn empty_value_string() -> String {
        format!("{}", CashflowAggregated::new().to_string())
    }
}
