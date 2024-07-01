use crate::configuration_parameters::ConfigurationParameters;

pub fn format_output(path: String, config_param: &ConfigurationParameters) -> String {
    format!(
        "{} {} {}:{}\n",
        config_param.command(),
        path,
        config_param.target_server(),
        path,
    )
}
