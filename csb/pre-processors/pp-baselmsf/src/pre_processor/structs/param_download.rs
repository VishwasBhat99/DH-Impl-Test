use configuration_parameters::ConfigurationParameters;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ParamDownloadInput {
    pub version_id: String,
    pub ndtl: String,
    pub slr: String,
    pub crr: String,
    pub param_value: String,
    pub as_on: String,
}

#[derive(Debug)]
pub struct ParamDownloadData {
    pub as_on: String,
    pub entity: String,
    pub ccy: String,
    pub msf_llg: String,
    pub msf_amt: String,
}

impl ParamDownloadData {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}",
            self.as_on, self.entity, self.ccy, self.msf_llg, self.msf_amt, self.msf_amt,
        )
    }
}

impl ParamDownloadData {
    pub fn new(config_params: &ConfigurationParameters, msf_amt: f64) -> Self {
        ParamDownloadData {
            as_on: config_params.as_on_date().format("%d-%m-%Y").to_string(),
            entity: config_params.entity().to_string(),
            ccy: config_params.ccy().to_string(),
            msf_llg: config_params.msf_llg().to_string(),
            msf_amt: msf_amt.to_string(),
        }
    }
}
