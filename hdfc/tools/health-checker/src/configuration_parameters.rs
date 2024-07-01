use clap::{App, Arg};
use statics::*;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub output_file_path: String,
    pub tot_acc: i64,
    pub tot_succ: i64,
    pub tot_fail: i64,
    pub tot_amt_inp: f64,
    pub tot_amt_op: f64,
    pub tot_cfs: i64,
    pub derive_fail_recs: bool,
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file` value.")
            .to_string();
        let tot_acc = matches
            .value_of("tot_acc")
            .expect("Error getting `tot_acc` value.")
            .parse::<i64>()
            .unwrap_or(DEFAULT_INT);
        let tot_succ = matches
            .value_of("tot_succ")
            .expect("Error getting `tot_succ` value.")
            .parse::<i64>()
            .unwrap_or(DEFAULT_INT);
        let tot_fail = matches
            .value_of("tot_fail")
            .expect("Error getting `tot_fail` value.")
            .parse::<i64>()
            .unwrap_or(DEFAULT_INT);
        let tot_amt_inp = matches
            .value_of("tot_amt_inp")
            .expect("Error getting `tot_amt_inp` value.")
            .parse::<f64>()
            .unwrap_or(DEFAULT_FLOAT);
        let tot_amt_op = matches
            .value_of("tot_amt_op")
            .expect("Error getting `tot_amt_op` value.")
            .parse::<f64>()
            .unwrap_or(DEFAULT_FLOAT);
        let tot_cfs = matches
            .value_of("tot_cfs")
            .expect("Error getting `tot_cfs` value.")
            .parse::<i64>()
            .unwrap_or(DEFAULT_INT);
        let derive_fail_recs = matches
            .value_of("derive_fail_recs")
            .expect("Error getting `derive_fail_recs` value.")
            .parse::<bool>()
            .unwrap_or(true);

        ConfigurationParameters {
            output_file_path,
            tot_acc,
            tot_succ,
            tot_fail,
            tot_amt_inp,
            tot_amt_op,
            tot_cfs,
            derive_fail_recs,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn tot_acc(&self) -> i64 {
        self.tot_acc
    }
    pub fn tot_succ(&self) -> i64 {
        self.tot_succ
    }
    pub fn tot_fail(&self) -> i64 {
        self.tot_fail
    }
    pub fn tot_amt_inp(&self) -> f64 {
        self.tot_amt_inp
    }
    pub fn tot_amt_op(&self) -> f64 {
        self.tot_amt_op
    }
    pub fn tot_cfs(&self) -> i64 {
        self.tot_cfs
    }
    pub fn derive_fail_recs(&self) -> bool {
        self.derive_fail_recs
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Health Check Report Writer!")
        .version("1.0.4758")
        .author("Tarun Jaiswal <tarun.j@surya-soft.com>")
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .short("o")
                .value_name("Output File")
                .help("Path to the output file.")
                .required(true),
        )
        .arg(
            Arg::with_name("tot_acc")
                .allow_hyphen_values(true)
                .long("account")
                .short("a")
                .value_name("Account Count")
                .help("Total Account Encountered.")
                .default_value("0")
                .required(false),
        )
        .arg(
            Arg::with_name("tot_succ")
                .allow_hyphen_values(true)
                .long("success")
                .short("s")
                .value_name("Success Count")
                .help("Total Success Count.")
                .default_value("0")
                .required(false),
        )
        .arg(
            Arg::with_name("tot_fail")
                .allow_hyphen_values(true)
                .long("fail")
                .short("f")
                .value_name("Fail Count")
                .help("Total Fail Count.")
                .default_value("0")
                .required(false),
        )
        .arg(
            Arg::with_name("tot_amt_inp")
                .allow_hyphen_values(true)
                .long("input-amount")
                .short("n")
                .value_name("Input Amount")
                .help("Total Amount in Input.")
                .default_value("0.0")
                .required(false),
        )
        .arg(
            Arg::with_name("tot_amt_op")
                .allow_hyphen_values(true)
                .long("output-amount")
                .short("u")
                .value_name("Output Amount")
                .help("Total Amount in Output.")
                .default_value("0.0")
                .required(false),
        )
        .arg(
            Arg::with_name("tot_cfs")
                .allow_hyphen_values(true)
                .long("total-cfs")
                .short("c")
                .value_name("Cashflow Count")
                .help("Total Cashflow Count")
                .default_value("0")
                .required(false),
        )
        .arg(
            Arg::with_name("derive_fail_recs")
                .long("derive-fail-recs")
                .short("d")
                .value_name("Derive Fail Recs")
                .help("Flag deciding whether to calculate Fail Recs if itis not passed.")
                .default_value("true")
                .required(false),
        )
        .get_matches()
}
