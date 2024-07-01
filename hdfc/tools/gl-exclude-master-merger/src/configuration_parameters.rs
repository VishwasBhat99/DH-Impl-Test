use clap::{App, Arg};

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub static_adder: String,
    pub static_remover: String,
    pub dynamic_master: String,
    pub excld_master: String,
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let static_adder = matches
            .value_of("static_adder")
            .expect("Error getting `static_adder` value.")
            .to_string();
        let static_remover = matches
            .value_of("static_remover")
            .expect("Error getting `static_remover` value.")
            .to_string();
        let dynamic_master = matches
            .value_of("dynamic_master")
            .expect("Error getting `dynamic_master` value.")
            .to_string();
        let excld_master = matches
            .value_of("excld_master")
            .expect("Error getting `excld_master` value.")
            .to_string();
        ConfigurationParameters {
            static_adder,
            static_remover,
            dynamic_master,
            excld_master,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn static_adder(&self) -> &str {
        &self.static_adder
    }
    pub fn static_remover(&self) -> &str {
        &self.static_remover
    }
    pub fn dynamic_master(&self) -> &str {
        &self.dynamic_master
    }
    pub fn excld_master(&self) -> &str {
        &self.excld_master
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Merge GL Master Files.")
        .arg(
            Arg::with_name("static_adder")
                .long("static-adder")
                .value_name("Static Adder")
                .help("Path Static Adder File.")
                .required(true),
        )
        .arg(
            Arg::with_name("static_remover")
                .long("static-remover")
                .value_name("Static Remover")
                .help("Path Static Remover File.")
                .required(true),
        )
        .arg(
            Arg::with_name("dynamic_master")
                .long("dynamic-master")
                .value_name("dynamic_master")
                .help("Path Dynamic Master File.")
                .required(true),
        )
        .arg(
            Arg::with_name("excld_master")
                .long("exclude-master")
                .value_name("excld_master")
                .help("Path Exclude Master File.")
                .required(true),
        )
        .get_matches()
}
