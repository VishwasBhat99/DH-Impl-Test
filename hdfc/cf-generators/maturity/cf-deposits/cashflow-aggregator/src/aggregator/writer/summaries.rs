use aggregator::structs::LLGAggregateOnDay;
use aggregator::llg_key::LLGKey;
use aggregator::writer::structs::CashflowAggregatedType;

pub struct LLGSummaryRecordGrouped {
    pub currency: String,   // TODO: Remove this. Subtype Id is enough
    pub subtype_id: LLGKey,
    pub int: LLGSummaryRecord,
    pub slr: LLGSummaryRecord,
    pub irs: LLGSummaryRecord
}

pub struct LLGSummaryRecord {
    pub amount: f64,
    pub rate: f64,
    pub ttype: CashflowAggregatedType
}

pub fn summary_for_llg(
    llg: LLGKey,
    aggregate_record: &Vec<Option<LLGAggregateOnDay>>
) -> LLGSummaryRecordGrouped {

    let mut total_int_amount = 0.0;
    let mut total_slr_amount = 0.0;
    let mut total_irs_amount = 0.0;
    let mut int_weighted_rate_builder = WeightedAverageBuilder::new();
    let mut slr_weighted_rate_builder = WeightedAverageBuilder::new();
    let mut irs_weighted_rate_builder = WeightedAverageBuilder::new();

    for aggregates in aggregate_record.iter() {
        match aggregates {
            Some(a) => {
                total_int_amount += a.int.amount;
                total_slr_amount += a.slr.amount;
                total_irs_amount += a.irs.amount;

                // TODO: Bug here. Rates are not getting weighted. Easy fix.
                int_weighted_rate_builder.build_with(a.int.amount, a.int.rate);
                slr_weighted_rate_builder.build_with(a.slr.amount, a.slr.rate);
                irs_weighted_rate_builder.build_with(a.irs.amount, a.irs.rate);
            },
            None => { }
        }
    }

    let int_summary = LLGSummaryRecord {
        amount: total_int_amount,
        rate: int_weighted_rate_builder
            .into_weighted_average(),
        ttype: CashflowAggregatedType::Int
    };
    let slr_summary = LLGSummaryRecord {
        amount: total_slr_amount,
        rate: slr_weighted_rate_builder
            .into_weighted_average(),
        ttype: CashflowAggregatedType::Slr
    };
    let irs_summary = LLGSummaryRecord {
        amount: total_irs_amount,
        rate: irs_weighted_rate_builder
            .into_weighted_average(),
        ttype: CashflowAggregatedType::Irs
    };

    LLGSummaryRecordGrouped {
        currency: llg.clone().currency,
        subtype_id: llg,
        int: int_summary,
        slr: slr_summary,
        irs: irs_summary,
    }
}

// TODO: Include this inside the CashflowAggregatedBuilder
struct WeightedAverageBuilder {
    numerator_builder: f64,
    divisor_builder: f64
}

impl WeightedAverageBuilder {

    fn new() -> WeightedAverageBuilder {
        WeightedAverageBuilder {
            numerator_builder: 0.0,
            divisor_builder: 0.0
        }
    }

    fn build_with(&mut self, num: f64, div: f64) {
        self.numerator_builder += num * div;
        self.divisor_builder += div;
    }

    fn into_weighted_average(self) -> f64 {
        self.numerator_builder / self.divisor_builder
    }
}