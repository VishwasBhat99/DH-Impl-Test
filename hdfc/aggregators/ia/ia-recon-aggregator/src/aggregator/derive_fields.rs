use super::{AggregatedKey, AggregatedMap, AggregatedValue, ConfigurationParameters, InputRecord};

pub fn get_aggregated_pair(
    aggr_map: &mut AggregatedMap,
    input_record: &InputRecord,
    config_params: &ConfigurationParameters,
) {
    let mut aggr_key = AggregatedKey::new();
    aggr_key.insert(
        input_record.clone(),
        config_params.src_id(),
        config_params.is_src_tb(),
    );

    let mut aggr_val = AggregatedValue::new();
    aggr_val.add(input_record.clone());

    aggr_map
        .store
        .entry(aggr_key)
        .and_modify(|val| val.add(input_record.clone()))
        .or_insert(aggr_val);
}
