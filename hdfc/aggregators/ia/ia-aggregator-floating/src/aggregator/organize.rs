#[derive(Clone, Debug)]
pub struct Cashflow {
    pub amt: f64,
    pub weighted_rate: f64,
    pub spread: f64,
}

pub fn aggregate_existing(
    map_value: Cashflow,
    amount: &f64,
    ex_rt: &f64,
    rate: f64,
    spread_rt: &f64,
) -> Cashflow {
    let out_amount = map_value.amt + (ex_rt * amount);
    let weighted_rt = map_value.weighted_rate + (rate * amount);
    let spread_value = map_value.spread + (spread_rt * amount);

    Cashflow {
        amt: out_amount,
        weighted_rate: weighted_rt,
        spread: spread_value,
    }
}

pub fn aggregate_new(ex_rt: &f64, amount: &f64, rate: f64, spread_rt: &f64) -> Cashflow {
    let out_amount = ex_rt * amount;
    let weighted_rt = rate * out_amount;
    let spread_value: f64 = spread_rt * out_amount;

    Cashflow {
        amt: out_amount,
        weighted_rate: weighted_rt,
        spread: spread_value,
    }
}
