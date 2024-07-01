use crate::configuration_parameters::ConfigurationParameters;
use crate::process::MetaDataFields;
use slog::Logger;
use std::collections::HashMap;

pub fn get_cf_fields(
    config_params: &mut ConfigurationParameters,
    _log: &Logger,
    _diag_log: &Logger,
    ip_map: &mut HashMap<String, MetaDataFields>,
    cf_vec: &mut Vec<usize>,
) {
    cf_vec.push(
        if config_params.cf_fields_col()[0] == *"0.0" || config_params.cf_fields_col()[0] == *"" {
            999
        } else {
            if &ip_map
                .get(&config_params.cf_fields_col()[1].to_string())
                .expect("Error getting cf-field from input-metadata")
                .typ
                != "double"
            {
                panic!(
                    "Invalid Datatype for CF-Field:`{}`passed as interest-amount, expected F64",
                    config_params.cf_fields_col()[0]
                )
            }
            (&ip_map
                .get(&config_params.cf_fields_col()[0].to_string())
                .unwrap_or_else(|| {
                    panic!(
                        "CF-Field:`{}`passed as interest-amount not present in Input Metadata",
                        config_params.cf_fields_col()[0]
                    )
                })
                .position
                - 1)
            .into()
        },
    );

    cf_vec.push(
        if config_params.cf_fields_col()[1] == *"0.0" || config_params.cf_fields_col()[1].is_empty()
        {
            999
        } else {
            if &ip_map
                .get(&config_params.cf_fields_col()[1].to_string())
                .expect("Error getting cf-field from input-metadata")
                .typ
                != "double"
            {
                panic!(
                    "Invalid Datatype for CF-Field:`{}`passed as principal-amount, expected F64",
                    config_params.cf_fields_col()[1]
                )
            }
            (&ip_map
                .get(&config_params.cf_fields_col()[1].to_string())
                .unwrap_or_else(|| {
                    panic!(
                        "CF-Field:`{}`passed as principal-amount not present in Input Metadata",
                        config_params.cf_fields_col()[1]
                    )
                })
                .position
                - 1)
            .into()
        },
    );

    cf_vec.push(
        (&ip_map
            .get(&config_params.cf_fields_col()[2].to_string())
            .unwrap_or_else(|| {
                panic!(
                    "CF-Field:`{}`passed as cashflow-date not present in Input Metadata",
                    config_params.cf_fields_col()[2]
                )
            })
            .position
            - 1)
        .into(),
    );
    if &ip_map
        .get(&config_params.cf_fields_col()[2].to_string())
        .expect("Error getting cf-field from input-metadata")
        .typ
        != "int64"
    {
        panic!(
            "Invalid Datatype for CF-Field:`{}`passed as cashflow-date, expected Date",
            config_params.cf_fields_col()[2]
        )
    }
}
