use aggregator::cashflow_organizer::cashflow_aggregation::LLGAggregateOnDay;
use aggregator::llg::llg_key::LLGKey;

#[derive(Debug)]
pub struct LLGSummaryRecordGrouped {
    pub subtype_id: LLGKey,
    pub o_a: LLGSummaryRecord,
}

#[derive(Debug)]
pub struct LLGSummaryRecord {
    pub amount: f64,
    pub rate: f64,
}

pub fn summary_for_llg(
    llg: LLGKey,
    aggregate_record: &Vec<Option<LLGAggregateOnDay>>,
) -> LLGSummaryRecordGrouped {
    let mut total_outstanding_amount = 0.0;
    let mut o_a_weighted_rate_builder = WeightedAverageBuilder::new();

    for aggregates in aggregate_record.iter() {
        match aggregates {
            Some(a) => {
                total_outstanding_amount += a.o_a.amount;
                o_a_weighted_rate_builder.build_with(a.o_a.rate, a.o_a.amount);
            }
            None => {}
        }
    }

    let outstanding_summary = LLGSummaryRecord {
        amount: total_outstanding_amount,
        rate: o_a_weighted_rate_builder.into_weighted_average(),
    };

    LLGSummaryRecordGrouped {
        subtype_id: llg,
        o_a: outstanding_summary,
    }
}

// TODO: Include this inside the CashflowAggregatedBuilder
struct WeightedAverageBuilder {
    numerator_builder: f64,
    divisor_builder: f64,
}

impl WeightedAverageBuilder {
    fn new() -> WeightedAverageBuilder {
        WeightedAverageBuilder {
            numerator_builder: 0.0,
            divisor_builder: 0.0,
        }
    }

    fn build_with(&mut self, num: f64, div: f64) {
        self.numerator_builder += num * div;
        self.divisor_builder += div;
    }

    fn into_weighted_average(self) -> f64 {
        if self.divisor_builder == 0.0 {
            0.0
        } else {
            self.numerator_builder / self.divisor_builder
        }
    }
}
