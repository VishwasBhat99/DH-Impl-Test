use crate::process::structs::*;
use crate::process::ConfigurationParameters;
use multimap::MultiMap;
use std::collections::HashMap;

pub fn get_processed_data_nonei(
    reader_map: &mut MultiMap<InputKeyNonEI, InputDataNonEI>,
    writer_map: &mut HashMap<OPKeyNonEI, OPDataNonEI>,
    config_params: &ConfigurationParameters,
) {
    for (key, data) in &mut reader_map.iter_all() {
        if data.len() == 1 {
            for val in data.clone() {
                let hashkey = OPKeyNonEI {
                    acid: key.acid.to_string(),
                    flow_start_date: val.flow_start_date,
                    cf_code: val.cf_code.to_string(),
                };
                let hashdata = OPDataNonEI {
                    shdl_num: val.shdl_num,
                    num_of_dmds: val.num_of_dmds,
                    flow_start_date: val.flow_start_date,
                    flow_amt: val.flow_amt,
                    lr_freq_type: val.lr_freq_type,
                    cf_code: val.cf_code,
                    num_of_flows: val.num_of_flows,
                };
                writer_map.insert(hashkey, hashdata);
            }
        }
        if data.len() > 1 {
            for val in data.clone() {
                if val.flow_start_date >= *config_params.as_on_date()
                    || val.flow_start_date
                        == data
                            .iter()
                            .filter(|s| s.flow_start_date < *config_params.as_on_date())
                            .map(|m| m.flow_start_date)
                            .max()
                            .unwrap_or(*config_params.as_on_date())
                {
                    let hashkey = OPKeyNonEI {
                        acid: key.acid.to_string(),
                        flow_start_date: val.flow_start_date,
                        cf_code: val.cf_code.to_string(),
                    };
                    let hashdata = OPDataNonEI {
                        shdl_num: val.shdl_num,
                        num_of_dmds: val.num_of_dmds,
                        flow_start_date: val.flow_start_date,
                        flow_amt: val.flow_amt,
                        lr_freq_type: val.lr_freq_type,
                        cf_code: val.cf_code,
                        num_of_flows: val.num_of_flows,
                    };
                    writer_map.insert(hashkey, hashdata);
                }
            }
        }
    }
}
