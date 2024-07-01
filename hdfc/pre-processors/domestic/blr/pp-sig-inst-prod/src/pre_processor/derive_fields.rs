use super::*;
use configuration_parameters::ConfigurationParameters;
use std::collections::HashMap;

pub fn get_op_line(
    inp_rec: &mut InputRecord,
    prod_dec_map: &mut HashMap<i64, ProdDescOutput>,
    smry_aggr_data: &mut HashMap<AggregatedKey, AggregatedValue>,
) {
    if prod_dec_map.contains_key(&inp_rec.llg_id) {
        let prod_desc = prod_dec_map
            .get(&inp_rec.llg_id)
            .expect("Error while getting product details.");
        let aggr_key = AggregatedKey::new(&inp_rec, &prod_desc);
        let aggr_val = AggregatedValue::new(&inp_rec);
        smry_aggr_data
            .entry(aggr_key)
            .and_modify(|data| data.add(aggr_val))
            .or_insert(aggr_val);
    }
}

pub fn get_liability_bal(
    config_params: &ConfigurationParameters,
    log: &Logger,
    diag_log: &Logger,
) -> f64 {
    info!(
        diag_log,
        "Start of getting liability balance from file {}.",
        config_params.liability_bal_file()
    );
    let mut liability_bal: f64 = 0.0;
    let mut input_reader = read_file(config_params.liability_bal_file());
    for (line_num, lines) in input_reader.deserialize().enumerate() {
        liability_bal = extract_lines(line_num, lines, config_params.liability_bal_file(), log);
        break;
    }
    info!(
        diag_log,
        "End of getting liability balance from file {}.",
        config_params.liability_bal_file()
    );

    liability_bal
}
