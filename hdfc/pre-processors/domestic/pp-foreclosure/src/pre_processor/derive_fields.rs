use super::{remove_comma, ForeClosureAmounts, InputAccount};
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use std::collections::HashMap;

pub fn get_op_line(
    acc: &InputAccount,
    fore_clsr_aggr: &mut HashMap<NaiveDate, ForeClosureAmounts>,
) {
    let fc_pos_amt = remove_comma(&acc.fc_pos)
        .parse::<f64>()
        .unwrap_or(DEFAULT_FLOAT);
    let mut default_amts: ForeClosureAmounts = ForeClosureAmounts::new();
    default_amts.insert(fc_pos_amt);
    let mis_dt = match NaiveDate::parse_from_str(&acc.mis_dt, "%d-%m-%Y") {
        Ok(dt) => dt,
        Err(error) => panic!(
            "mis_date`: `{}` is not well-formatted as `DD-MM-YYYY`: `{}`.",
            acc.mis_dt, error,
        ),
    };
    fore_clsr_aggr
        .entry(mis_dt)
        .and_modify(|map| map.add_amts(fc_pos_amt))
        .or_insert(default_amts);
}
