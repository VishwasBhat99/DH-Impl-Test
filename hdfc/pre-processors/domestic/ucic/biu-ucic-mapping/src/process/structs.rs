use crate::configuration_parameters::ConfigurationParameters;

#[derive(Debug)]
pub struct BiuRecord {
    pub cust_id: String,
    pub t1: String,
    pub t2: String,
    pub t3: String,
    pub t4: String,
    pub division: String,
}

impl BiuRecord {
    pub fn from_line(line: &str, config_params: &ConfigurationParameters) -> Option<BiuRecord> {
        let fields: Vec<&str> = line.split(config_params.input_field_delimiter()).collect();
        if fields.len() >= 6 {
            Some(BiuRecord {
                cust_id: fields[0].to_string(),
                t1: fields[1].to_string(),
                t2: fields[2].to_string(),
                t3: fields[3].to_string(),
                t4: fields[4].to_string(),
                division: fields[5].to_string(),
            })
        } else {
            None
        }
    }
}
