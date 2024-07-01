use crate::configuration_parameters::ConfigurationParameters;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct OutputAccount {
    pub stress_app_type: String,
    pub class_id: String,
    pub topnrank: usize,
    pub cust_id: String,
    pub cust_name: String,
    pub as_on_date: String,
    pub country_id: String,
    pub llg_id: String,
    pub ccy_id: String,
    pub ccy_amt: f64,
    pub hcy_amt: f64,
    pub int_rate: f64,
}

impl OutputAccount {
    pub fn new(
        is_stable: bool,
        (cust_id, cust_name): (String, String),
        config_params: &ConfigurationParameters,
        topnrank: usize,
        ret_nonret_map: &mut HashMap<String, (String, f64, f64)>,
        class_llg_mapper: &mut HashMap<String, String>,
    ) -> OutputAccount {
        let (class_id, stable_amt, less_stable_amt) =
            ret_nonret_map.get(&cust_id.to_string()).unwrap_or_else(|| {
                panic!(
                    "Could not find Class ID from Ret-NonRetail for Customer:{}",
                    cust_id
                )
            });
        let def_llg = config_params.default_llg().to_string();
        let (out_amt, llg_id) = if is_stable {
            (
                stable_amt,
                class_llg_mapper
                    .get(&(class_id.to_string() + "S"))
                    .unwrap_or(&def_llg),
            )
        } else {
            (
                less_stable_amt,
                class_llg_mapper
                    .get(&(class_id.to_string() + "LS"))
                    .unwrap_or(&def_llg),
            )
        };
        OutputAccount {
            stress_app_type: config_params.stress_app_type().to_string(),
            class_id: class_id.to_string(),
            topnrank: topnrank + 1,
            cust_id,
            cust_name,
            as_on_date: config_params.as_on_date().format("%d-%m-%Y").to_string(),
            country_id: config_params.country_id().to_string(),
            llg_id: llg_id.to_string(),
            ccy_id: config_params.ccy_id().to_string(),
            ccy_amt: *out_amt,
            hcy_amt: *out_amt,
            int_rate: 0.0,
        }
    }
}

pub fn format_output(output_rec: OutputAccount) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{:.2}|{:.2}|{:.2}",
        output_rec.stress_app_type,
        output_rec.class_id,
        output_rec.topnrank,
        output_rec.cust_id,
        output_rec.cust_name,
        output_rec.as_on_date,
        output_rec.country_id,
        output_rec.llg_id,
        output_rec.ccy_id,
        output_rec.ccy_amt,
        output_rec.hcy_amt,
        output_rec.int_rate,
    )
}
