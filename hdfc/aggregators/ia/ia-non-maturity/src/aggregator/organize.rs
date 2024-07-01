#[derive(Clone, Debug)]
pub struct Cashflow {
    pub amt: f64,
    pub weighted_rate: f64,
}
pub fn aggregate_existing(amount: f64, map_value: Cashflow, ex_rt: &f64, rate: f64) -> Cashflow {
    let out_amount = map_value.amt + (ex_rt * amount);
    let weighted_rt = map_value.weighted_rate + (rate * ex_rt * amount);
    return Cashflow {
        amt: out_amount,
        weighted_rate: weighted_rt,
    };
}
pub fn aggregate_new(ex_rt: &f64, amount: f64, rate: f64) -> Cashflow {
    let out_amount = ex_rt * amount;
    let weighted_rt = rate * out_amount;
    return Cashflow {
        amt: out_amount,
        weighted_rate: weighted_rt,
    };
}
