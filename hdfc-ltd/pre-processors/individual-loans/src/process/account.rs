use crate::configuration_parameters::ConfigurationParameters;

#[derive(Debug, Clone, Default)]
/// The structure in which the gl-mapping for each prod_code is expected in a excel file
pub struct ProdGLData {
    pub prod_code: String,
    pub gl_code: String,
    pub emi_overdue_gl_code: String,
    pub pre_emi_overdue_gl_code: String,
    pub emi_excess_gl_code: String,
    pub pre_emi_excess_gl_code: String,
}

impl ProdGLData {
    pub fn new(master_data: &[calamine::DataType]) -> ProdGLData {
        ProdGLData {
            prod_code: master_data[0].to_string(),
            gl_code: master_data[1].to_string(),
            emi_overdue_gl_code: master_data[2].to_string(),
            pre_emi_overdue_gl_code: master_data[3].to_string(),
            emi_excess_gl_code: master_data[4].to_string(),
            pre_emi_excess_gl_code: master_data[5].to_string(),
        }
    }
    pub fn def(config_params: &ConfigurationParameters) -> ProdGLData {
        ProdGLData {
            prod_code: "NA".to_string(),
            gl_code: config_params.default_gls()[0].to_string(),
            emi_overdue_gl_code: config_params.default_gls()[1].to_string(),
            pre_emi_overdue_gl_code: config_params.default_gls()[2].to_string(),
            emi_excess_gl_code: config_params.default_gls()[3].to_string(),
            pre_emi_excess_gl_code: config_params.default_gls()[4].to_string(),
        }
    }
}
