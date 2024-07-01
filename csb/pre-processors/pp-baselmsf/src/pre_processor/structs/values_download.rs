use configuration_parameters::ConfigurationParameters;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ValuesDownloadInput {
    pub slr_ndtl: String,
    pub slr_req: String,
    pub crr_ndtl: String,
    pub crr_req: String,
    pub as_on: String,
}

#[derive(Debug)]
pub struct ValuesDownloadData {
    pub as_on: String,
    pub entity: String,
    pub ccy: String,
    pub afacility_llg: String,
    pub afacility_amt: String,
}

impl ValuesDownloadData {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}",
            self.as_on,
            self.entity,
            self.ccy,
            self.afacility_llg,
            self.afacility_amt,
            self.afacility_amt,
        )
    }
}

impl ValuesDownloadData {
    pub fn new(config_params: &ConfigurationParameters, afacility_amt: f64) -> Self {
        ValuesDownloadData {
            as_on: config_params.as_on_date().format("%d-%m-%Y").to_string(),
            entity: config_params.entity().to_string(),
            ccy: config_params.ccy().to_string(),
            afacility_llg: config_params.afacility_llg().to_string(),
            afacility_amt: afacility_amt.to_string(),
        }
    }
}
