use super::{ConfigurationParameters, ParamDownloadData, ValuesDownloadData};

pub fn get_op_line(
    config_params: &ConfigurationParameters,
    msf_percent: f64,
    slr_ndtl: f64,
    percent: f64,
) -> String {
    let mut op_line = String::new();

    let msf_amt = (slr_ndtl * (msf_percent / 100.0)) * percent;

    let afacility_amt = slr_ndtl
        * (config_params
            .afacility_percent()
            .parse::<f64>()
            .expect("Invalid afacility percent")
            / 100.0);
    let param_data = ParamDownloadData::new(&config_params, msf_amt);
    let values_data = ValuesDownloadData::new(&config_params, afacility_amt);

    op_line.push_str(&param_data.print());
    op_line.push_str("\n");
    op_line.push_str(&values_data.print());

    op_line
}
