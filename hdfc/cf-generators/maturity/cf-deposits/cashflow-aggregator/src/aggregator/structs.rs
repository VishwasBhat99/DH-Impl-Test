use sdb_dyn_proto_rdr::compound_types::Cashflow;

#[derive(Debug, Copy, Clone)]
pub struct CashflowAggregatedOnDateBuilder {
    rate_interest_amount_weighted: f64,
    date_interest_amount_weighted: f64,
    rate_principal_amount_weighted: f64,
    date_principal_amount_weighted: f64,
    total_interest_amount: f64,
    total_principal_amount: f64,

    // IRS cashflows on a day can be vastly different from regular cashflows on this day
    irs_rate_principal_amount_weighted: f64,
    irs_date_principal_amount_weighted: f64,
    irs_total_amount: f64
}

impl CashflowAggregatedOnDateBuilder {

    pub fn new() -> CashflowAggregatedOnDateBuilder {

        CashflowAggregatedOnDateBuilder {
            rate_interest_amount_weighted: 0.0,
            rate_principal_amount_weighted: 0.0,
            date_interest_amount_weighted: 0.0,
            date_principal_amount_weighted: 0.0,
            total_interest_amount: 0.0,
            total_principal_amount: 0.0,
            irs_rate_principal_amount_weighted: 0.0,
            irs_date_principal_amount_weighted: 0.0,
            irs_total_amount: 0.0
        }

    }

    pub fn add_from_builder(&mut self, other: CashflowAggregatedOnDateBuilder) {

        self.rate_interest_amount_weighted += other.rate_interest_amount_weighted;
        self.rate_principal_amount_weighted += other.rate_principal_amount_weighted;
        self.date_interest_amount_weighted += other.date_interest_amount_weighted;
        self.date_principal_amount_weighted += other.date_principal_amount_weighted;
        self.total_interest_amount += other.total_interest_amount;
        self.total_principal_amount += other.total_principal_amount;

        self.irs_rate_principal_amount_weighted += other.irs_rate_principal_amount_weighted;
        self.irs_date_principal_amount_weighted += other.irs_date_principal_amount_weighted;
        self.irs_total_amount += other.irs_total_amount;

    }

    pub fn add_slr_int_cf(&mut self, cf: &Cashflow, interest_rate: f64, day_num: i64) {

        self.add_slr_int(
            cf.get_interest_amount() as f64,
            cf.get_principal_amount() as f64,
            interest_rate,
            day_num as f64
        );

    }

    pub fn add_irs_cf(&mut self, cf: &Cashflow, interest_rate: f64, day_num: i64) {
        self.add_irs(
            cf.get_principal_amount(),
            interest_rate,
            day_num as f64
        );
    }

    fn add_slr_int(&mut self, i_a: f64, p_a: f64, r: f64, d: f64) {

        self.total_interest_amount += i_a;
        self.total_principal_amount += p_a;
        self.rate_interest_amount_weighted += i_a * r;
        self.rate_principal_amount_weighted += p_a * r;
        self.date_interest_amount_weighted += i_a * d;
        self.date_principal_amount_weighted += p_a * d;

    }

    fn add_irs(&mut self, irs_a: f64, irs_r: f64, irs_d: f64) {

        self.irs_total_amount += irs_a;
        self.irs_date_principal_amount_weighted += irs_a * irs_d;
        self.irs_rate_principal_amount_weighted += irs_a * irs_r;

    }

    pub fn to_cf_aggregated(self) -> LLGAggregateOnDay {

        let int = {
            if self.total_interest_amount == 0.0 {
                // This will result in NaN in the output string. We don't want that.
                // NaNs occur only for the interest aggregates.
                CashflowAggregated {
                    amount: 0.0,
                    rate: 0.0,
                    date: 0.0
                }
            } else {
                CashflowAggregated {
                    amount: self.total_interest_amount,
                    rate: self.rate_interest_amount_weighted / self.total_interest_amount,
                    date: self.date_interest_amount_weighted / self.total_interest_amount
                }
            }
        };

        let slr = CashflowAggregated {
            amount: self.total_principal_amount,
            rate: self.rate_principal_amount_weighted / self.total_principal_amount,
            date: self.date_principal_amount_weighted / self.total_principal_amount
        };

        let irs = CashflowAggregated {
            amount: self.irs_total_amount,
            rate: self.irs_rate_principal_amount_weighted / self.irs_total_amount,
            date: self.irs_date_principal_amount_weighted / self.irs_total_amount
        };

        LLGAggregateOnDay {
            int,
            slr,
            irs
        }

    }

    pub fn values_multiplied_by(&mut self, multiplier: f64) {
        self.rate_interest_amount_weighted *= multiplier;
        self.rate_principal_amount_weighted *= multiplier;
        self.date_interest_amount_weighted *= multiplier;
        self.date_principal_amount_weighted *= multiplier;
        self.total_interest_amount *= multiplier;
        self.total_principal_amount *= multiplier;
        self.irs_rate_principal_amount_weighted *= multiplier;
        self.irs_date_principal_amount_weighted *= multiplier;
        self.irs_total_amount *= multiplier;
    }

}

#[derive(Debug)]
pub struct CashflowAggregated {
    pub amount: f64,
    pub rate: f64,
    pub date: f64
}

// TODO: Keep in mind that there's only one field that's different here. Look into breaking it apart into constituent parts if that solves a problem.
#[derive(Debug)]
pub struct LLGAggregateOnDay {
    pub int: CashflowAggregated,
    pub slr: CashflowAggregated,
    pub irs: CashflowAggregated
}

impl CashflowAggregated {

    fn new() -> CashflowAggregated {
        CashflowAggregated {
            amount: 0.0,
            rate: 0.0,
            date: 0.0
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}|{}|{}|", self.amount, self.rate, self.date)
    }

    pub fn empty_value_string() -> String {
        format!("{}", CashflowAggregated::new().to_string())
    }

}