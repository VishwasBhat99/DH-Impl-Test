use chrono::Datelike;
use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use sdb_day_convention::Conventions;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_args_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    from_date: NaiveDate,
    to_date: NaiveDate,
    config_file_path: String,
    output_file_path: String,
    method_rules_file_path: String,
    bc_rule_file_path: String,
    llg_rule_file_path: String,
    fix_adj_rule_file_path: String,
    var_adj_rule_file_path: String,
    bc_file_path: String,
    spread_file_path: String,
    adj_rate_file_path: String,
    bal_slab_file: String,
    default_method_file_path: String,
    default_basecurve_file_path: String,
    default_llg_file_path: String,
    fixed_adj_count: i32,
    var_adj_count: i32,
    log_file_path: String,
    diagnostics_file_path: String,
    method_req_fields_file_path: String,
    log_level: String,
    ccy: String,
    rate_prec: i8,
    bal_prec: i8,
    is_perf_diagnostics_enabled: bool,
    apply_base_curve_2: bool,
    is_absolute: bool,
    rate_flag: String,
    aorl_rule_file_path: String,
    default_aorl_flag: String,
    is_extrapolation_req: bool,
    skip_amb_header: bool,
    day_count_basis: Conventions,
    bmrate_accuracy: String,
    is_int_calc_required: bool,
    is_int_rate_from_amb: bool,
    adj_count_for_bc_2: i32,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) -> () {
        info!(logger, "from_date:{}", self.from_date());
        info!(logger, "to_date:{}", self.to_date());
        info!(logger, "config_file: {}", self.config_file_path());
        info!(
            logger,
            "day count basis convention: {:?}",
            self.day_count_basis()
        );
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(
            logger,
            "method_req_fields_file_path: {}",
            self.method_req_fields_file_path()
        );
        info!(logger, "ccy: {}", self.ccy());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(
            logger,
            "method_rule_file: {}",
            self.method_rules_file_path()
        );
        info!(logger, "bc_rule_file: {}", self.bc_rule_file_path());
        info!(logger, "llg_rule_file: {}", self.llg_rule_file_path());
        info!(logger, "adj_rule_file: {}", self.fix_adj_rule_file_path());
        info!(
            logger,
            "var_adj_rule_file: {}",
            self.var_adj_rule_file_path()
        );
        info!(logger, "bc_file: {}", self.bc_file_path());
        info!(logger, "ftp_rates_file: {}", self.spread_file_path());
        info!(logger, "adj_rate_file_path: {}", self.adj_rate_file_path());
        info!(logger, "bal_slab_file: {}", self.bal_slab_file());
        info!(
            logger,
            "default_method_file_path: {}",
            self.default_method_file_path()
        );
        info!(
            logger,
            "default_basecurve_file_path: {}",
            self.default_basecurve_file_path()
        );
        info!(
            logger,
            "default_llg_file_path: {}",
            self.default_llg_file_path()
        );
        info!(logger, "fixed_adj_count: {}", self.fixed_adj_count());
        info!(logger, "var_adj_count: {}", self.var_adj_count());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "rate_prec: {}", self.rate_prec());
        info!(logger, "bal_prec: {}", self.bal_prec());
        info!(logger, "is_absolute: {}", self.is_absolute());
        info!(logger, "skip_amb_header: {}", self.skip_amb_header());
        info!(logger, "rate_flag: {}", self.rate_flag());
        info!(
            logger,
            "is_extrapolation_req: {}",
            self.is_extrapolation_req()
        );
        info!(logger, "aorl_rule_file: {}", self.aorl_rule_file_path());
        info!(logger, "default_aorl_flag: {}", self.default_aorl_flag());
        info!(logger, "bmrate_accuracy: {}", self.bmrate_accuracy());
        info!(
            logger,
            "is_int_calc_required: {}",
            self.is_int_calc_required()
        );
        info!(
            logger,
            "is_int_rate_from_amb: {}",
            self.is_int_rate_from_amb()
        );
        info!(logger, "adj_count_for_bc_2: {}", self.adj_count_for_bc_2());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let is_def_from_date = matches
            .value_of("is_def_from_date")
            .expect("Error getting `is_def_from_date` value.")
            .parse::<bool>()
            .expect("Cannot parse `is_def_from_date` value as bool.");

        let conv = matches
            .value_of("day_count_basis")
            .expect("Error getting `day_count_basis convention`.")
            .to_string();
        let day_count_basis = match conv.as_str() {
            "ACTbyACT" => Conventions::ACTbyACT,
            "ACTby360" => Conventions::ACTby360,
            "Thirtyby360" => Conventions::Thirtyby360,
            _ => Conventions::ACTby365,
        };
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let to_date = date_parser.parse(
            matches
                .value_of("to_date")
                .expect("Error getting `to_date` value."),
        );

        let from_date = if is_def_from_date {
            NaiveDate::from_ymd(to_date.year(), to_date.month(), 01)
        } else {
            date_parser.parse(
                matches
                    .value_of("from_date")
                    .expect("Error getting `from_date` value."),
            )
        };

        let config_file_path = matches
            .value_of("config_file")
            .expect("Error getting `config_file_path` value.")
            .to_string();
        let method_rules_file_path = matches
            .value_of("method_rule_file")
            .expect("Error getting `method_rules_file_path` value.")
            .to_string();
        let bc_rule_file_path = matches
            .value_of("bc_rule_file")
            .expect("Error getting `bc_rule_file_path` value.")
            .to_string();
        let llg_rule_file_path = matches
            .value_of("llg_rule_file")
            .expect("Error getting `llg_rule_file_path` value.")
            .to_string();
        let fix_adj_rule_file_path = matches
            .value_of("fix_adj_rule_file")
            .expect("Error getting `fix_adj_rule_file_path` value.")
            .to_string();
        let var_adj_rule_file_path = matches
            .value_of("var_adj_rule_file")
            .expect("Error getting `var_adj_rule_file_path` value.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path` value.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path` value.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_file_path` value.")
            .to_string();
        let method_req_fields_file_path = matches
            .value_of("method_req_fields_file_path")
            .expect("Error getting `method_req_fields_file_path` value.")
            .to_string();
        let bc_file_path = matches
            .value_of("bc_file")
            .expect("Error getting `bc_file_path` value.")
            .to_string();
        let spread_file_path = matches
            .value_of("spread_file_path")
            .expect("Error getting `spread_file_path` value.")
            .to_string();
        let adj_rate_file_path = matches
            .value_of("adj_rate_file_path")
            .expect("Error getting `adj_rate_file_path` value.")
            .to_string();
        let bal_slab_file = matches
            .value_of("bal_slab_file")
            .expect("Error getting `bal_slab_file` value.")
            .to_string();
        let ccy = matches
            .value_of("ccy")
            .expect("Error getting `ccy` value.")
            .to_string();
        let default_method_file_path = matches
            .value_of("default_method_file_path")
            .expect("Error getting `default_method_file_path` value.")
            .to_string();
        let default_basecurve_file_path = matches
            .value_of("default_basecurve_file_path")
            .expect("Error getting `default_basecurve_file_path` value.")
            .to_string();
        let default_llg_file_path = matches
            .value_of("default_llg_file_path")
            .expect("Error getting `default_llg_file_path` value.")
            .to_string();
        let fixed_adj_count = matches
            .value_of("fixed_adj_count")
            .expect("Error getting `fixed_adj_count` value.")
            .to_string()
            .parse::<i32>()
            .expect("Cannot parse `fixed_adj_count` value as i32.");
        let var_adj_count = matches
            .value_of("var_adj_count")
            .expect("Error getting `var_adj_count` value.")
            .to_string()
            .parse::<i32>()
            .expect("Cannot parse `var_adj_count` value as i32.");
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level` value.")
            .to_string();
        let rate_prec = matches
            .value_of("rate_prec")
            .expect("Error getting `rate_prec` value.")
            .parse::<i8>()
            .expect("Cannot parse `rate_prec` value as i8.");
        let bal_prec = matches
            .value_of("bal_prec")
            .expect("Error getting `bal_prec` value.")
            .parse::<i8>()
            .expect("Cannot parse `bal_prec` value as i8.");
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag` value.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` value as bool.");

        let apply_base_curve_2 = matches
            .value_of("apply_base_curve_2")
            .expect("Error getting `apply_base_curve_2` value.")
            .parse::<bool>()
            .expect("Cannot parse `apply_base_curve_2` value as bool.");
        let is_absolute = matches
            .value_of("is_absolute")
            .expect("Error getting `is_absolute` value.")
            .parse::<bool>()
            .expect("Cannot parse `is_absolute` value as bool.");
        let rate_flag = matches
            .value_of("rate_flag")
            .expect("Error getting `rate_flag` value.")
            .to_string();
        let is_extrapolation_req = matches
            .value_of("is_extrapolation_req")
            .expect("Error getting `is_extrapolation_req` value.")
            .parse::<bool>()
            .expect("Cannot parse `is_extrapolation_req` value as bool.");
        let aorl_rule_file_path = matches
            .value_of("aorl_rule_file_path")
            .expect("Error getting `aorl_rule_file_path` value.")
            .to_string();
        let default_aorl_flag = matches
            .value_of("default_aorl_flag")
            .expect("Error getting `default_aorl_flag` value.")
            .to_string();
        let skip_amb_header = matches
            .value_of("skip_amb_header")
            .expect("Error getting `skip_amb_header` value.")
            .parse::<bool>()
            .expect("Cannot parse `skip_amb_header` value as bool.");
        let bmrate_accuracy = matches
            .value_of("bmrate_accuracy")
            .expect("Error getting `bmrate_accuracy` value.")
            .to_string();
        let is_int_calc_required = matches
            .value_of("is_int_calc_required")
            .expect("Error getting `is_int_calc_required` value.")
            .parse::<bool>()
            .expect("Cannot parse `is_int_calc_required` value as bool.");
        let is_int_rate_from_amb = matches
            .value_of("is_int_rate_from_amb")
            .expect("Error getting `is_int_rate_from_amb` value.")
            .parse::<bool>()
            .expect("Cannot parse `is_int_rate_from_amb` value as bool.");
        let adj_count_for_bc_2 = matches
            .value_of("adj_count_for_bc_2")
            .expect("Error getting `adj_count_for_bc_2` value.")
            .parse::<i32>()
            .expect("Cannot parse `adj_count_for_bc_2` value as i32.");
        ConfigurationParameters {
            from_date,
            to_date,
            config_file_path,
            output_file_path,
            method_rules_file_path,
            bc_rule_file_path,
            llg_rule_file_path,
            fix_adj_rule_file_path,
            var_adj_rule_file_path,
            bc_file_path,
            spread_file_path,
            adj_rate_file_path,
            bal_slab_file,
            default_method_file_path,
            default_basecurve_file_path,
            default_llg_file_path,
            fixed_adj_count,
            var_adj_count,
            log_file_path,
            diagnostics_file_path,
            method_req_fields_file_path,
            log_level,
            ccy,
            rate_prec,
            bal_prec,
            is_perf_diagnostics_enabled,
            apply_base_curve_2,
            is_absolute,
            rate_flag,
            aorl_rule_file_path,
            default_aorl_flag,
            is_extrapolation_req,
            skip_amb_header,
            day_count_basis,
            bmrate_accuracy,
            is_int_calc_required,
            is_int_rate_from_amb,
            adj_count_for_bc_2,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn from_date(&self) -> &NaiveDate {
        &self.from_date
    }
    pub fn to_date(&self) -> &NaiveDate {
        &self.to_date
    }
    pub fn config_file_path(&self) -> &str {
        &self.config_file_path
    }
    pub fn day_count_basis(&self) -> &Conventions {
        &self.day_count_basis
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn method_rules_file_path(&self) -> &str {
        &self.method_rules_file_path
    }
    pub fn bc_rule_file_path(&self) -> &str {
        &self.bc_rule_file_path
    }
    pub fn llg_rule_file_path(&self) -> &str {
        &self.llg_rule_file_path
    }
    pub fn fix_adj_rule_file_path(&self) -> &str {
        &self.fix_adj_rule_file_path
    }
    pub fn var_adj_rule_file_path(&self) -> &str {
        &self.var_adj_rule_file_path
    }
    pub fn bc_file_path(&self) -> &str {
        &self.bc_file_path
    }
    pub fn spread_file_path(&self) -> &str {
        &self.spread_file_path
    }
    pub fn adj_rate_file_path(&self) -> &str {
        &self.adj_rate_file_path
    }
    pub fn bal_slab_file(&self) -> &str {
        &self.bal_slab_file
    }
    pub fn default_method_file_path(&self) -> &str {
        &self.default_method_file_path
    }
    pub fn default_basecurve_file_path(&self) -> &str {
        &self.default_basecurve_file_path
    }
    pub fn default_llg_file_path(&self) -> &str {
        &&self.default_llg_file_path
    }
    pub fn fixed_adj_count(&self) -> i32 {
        self.fixed_adj_count
    }
    pub fn var_adj_count(&self) -> i32 {
        self.var_adj_count
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn method_req_fields_file_path(&self) -> &str {
        &self.method_req_fields_file_path
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn ccy(&self) -> &str {
        &self.ccy
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
    pub fn apply_base_curve_2(&self) -> bool {
        self.apply_base_curve_2
    }
    pub fn rate_prec(&self) -> i8 {
        self.rate_prec
    }
    pub fn bal_prec(&self) -> i8 {
        self.bal_prec
    }
    pub fn is_absolute(&self) -> bool {
        self.is_absolute
    }
    pub fn is_extrapolation_req(&self) -> bool {
        self.is_extrapolation_req
    }
    pub fn skip_amb_header(&self) -> bool {
        self.skip_amb_header
    }
    pub fn rate_flag(&self) -> &str {
        &self.rate_flag
    }
    pub fn aorl_rule_file_path(&self) -> &str {
        &self.aorl_rule_file_path
    }
    pub fn default_aorl_flag(&self) -> &str {
        &self.default_aorl_flag
    }
    pub fn bmrate_accuracy(&self) -> &str {
        &self.bmrate_accuracy
    }
    pub fn is_int_calc_required(&self) -> bool {
        self.is_int_calc_required
    }
    pub fn is_int_rate_from_amb(&self) -> bool {
        self.is_int_rate_from_amb
    }
    pub fn adj_count_for_bc_2(&self) -> i32 {
        self.adj_count_for_bc_2
    }
}

fn get_args_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
    .version("1.1.5360")
        .author("Tarun Jaiswal <tarun.j@surya-soft.com>")
        .about("This app helps convert inputs to outputs at lightning speed!")
        .arg(
            Arg::with_name("from_date")
                .long("from-date")
                .value_name("DATE")
                .help("Start date of the FTP process date range")
                .required(false)
        )
        .arg(
            Arg::with_name("to_date")
                .long("to-date")
                .value_name("DATE")
                .help("End date of the FTP process date range")
                .required(true)
        )
        .arg(
            Arg::with_name("config_file")
                .long("config-file")
                .value_name("FILE")
                .help("Path to config file that needs to be processed")
                .required(true),
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file")
                .required(true)
        )
        .arg(
            Arg::with_name("method_rule_file")
                .long("method-rules-file")
                .value_name("FILE")
                .help("Path to the method rules file")
                .required(true)
        )
        .arg(
            Arg::with_name("bc_rule_file")
                .long("bc-rule-file")
                .value_name("FILE")
                .help("Path to the basecurve rules file")
                .required(true)
        )
        .arg(
            Arg::with_name("llg_rule_file")
                .long("llg-rule-file")
                .value_name("FILE")
                .help("Path to the LLG rules file")
                .required(true)
        )
        .arg(
            Arg::with_name("fix_adj_rule_file")
                .long("fix-adj-rule-file")
                .value_name("FILE")
                .help("Path to the fixed adjustment rules file")
                .required(true)
        )
        .arg(
            Arg::with_name("var_adj_rule_file")
                .long("var-adj-rule-file")
                .value_name("FILE")
                .help("Path to the variable adjustment rules file")
                .required(true)
        )
        .arg(
            Arg::with_name("bc_file")
                .long("bc-file")
                .value_name("FILE")
                .help("Path to the basecurve file")
                .required(true)
        )
        .arg(
            Arg::with_name("spread_file_path")
                .long("spread-file")
                .value_name("FILE")
                .help("Path to the FTP Rates file")
                .required(true)
        )
         .arg(
            Arg::with_name("adj_rate_file_path")
                .long("adj-rates-file")
                .value_name("FILE")
                .help("Path to the adjustment Rates file")
                .required(true)
        )
        .arg(
            Arg::with_name("bal_slab_file")
                .long("bal-slab")
                .value_name("FILE")
                .help("Path to the balance slab Rates file")
                .required(true)
        )
        .arg(
            Arg::with_name("default_method_file_path")
                .long("default-method-file")
                .value_name("Default Method FILE")
                .help("Path to Default Method File")
                .required(true)
        )
        .arg(
            Arg::with_name("default_basecurve_file_path")
                .long("default-basecurve-file")
                .value_name("Default Basecurve FILE")
                .help("Path to Default Basecurve File")
                .required(true)
        )
        .arg(
            Arg::with_name("default_llg_file_path")
                .long("default-llg-file")
                .value_name("Default LLG FILE")
                .help("Path to Default LLG File")
                .required(true)
        )
        .arg(
            Arg::with_name("fixed_adj_count")
                .long("fixed-adjustments-count")
                .value_name("fixed adjustments count")
                .help("count of fixed adjustments")
                .required(true)
        )
        .arg(
            Arg::with_name("var_adj_count")
                .long("var-adjustments-count")
                .value_name("Variable adjustments count")
                .help("Count of variable adjustments")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("method_req_fields_file_path")
                .long("method-req-fields-file-path")
                .value_name("File")
                .help("Method Required Fields File Path")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("apply_base_curve_2")
                .long("apply-base-curve-2")
                .value_name("BASE CURVE 2")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether base curve 2 will be applied.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("ccy")
                .long("ccy")
                .value_name("Currency")
                .help("Currency")
                .required(true)
        )
        .arg(
            Arg::with_name("is_def_from_date")
                .long("is-def-from-date")
                .value_name("Default To Date")
                .help("The flag that decides whether to_date to be considered first day of the month or not.")
                .possible_values(&["true", "false"])
                .default_value("true")
                .required(false)
        )
        .arg(
            Arg::with_name("rate_prec")
                .long("rate-prec")
                .value_name("Rate Precision")
                .help("The flag that decides the round off factor for rate fields.")
                .default_value("4")
                .required(false)
        )
        .arg(
            Arg::with_name("bal_prec")
                .long("bal-prec")
                .value_name("Balance Precision")
                .help("The flag that decides the round off factor for balance fields.")
                .default_value("4")
                .required(false)
        )
        .arg(
            Arg::with_name("rate_flag")
                .long("rate-flag")
                .value_name("Rate Flag")
                .help("The flag that decides the value of rate flag field.")
                .default_value("")
                .required(false)
        )
        .arg(
            Arg::with_name("is_absolute")
                .long("is-absolute")
                .value_name("Is Absolute")
                .help("The flag that decides the round off factor for balance fields.")
                .possible_values(&["true", "false"])
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("aorl_rule_file_path")
                .long("aorl-rule-file")
                .value_name("FILE")
                .help("Path to the basecurve rules file")
                .required(true)
        )
        .arg(
            Arg::with_name("default_aorl_flag")
                .long("default-aorl-flag")
                .value_name("AorL Flag")
                .help("The flag that decides the value of AorL field.")
                .default_value("")
                .possible_values(&["A", "L",""])
                .required(true)
        )
        .arg(
            Arg::with_name("is_extrapolation_req")
                .long("is-extrapolation-required")
                .value_name("Extrapolation Flag")
                .help("The flag that decides to calulate rate values using extrapolation.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("skip_amb_header")
                .long("skip-amb-header")
                .value_name("Amb Header Flag")
                .help("The flag that decides to skip the header in amb file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("day_count_basis")
                .long("day-count-basis")
                .value_name("Day count basis convention flag")
                .possible_values(&["ACTbyACT", "ACTby360", "Thirtyby360", "ACTby365"])
                .help("Conventions")
                .default_value("ACTby365")
                .required(true)
        )
        .arg(
            Arg::with_name("bmrate_accuracy")
                .long("bmrate-accuracy")
                .value_name("BMRate calculation flag")
                .possible_values(&["D", "M"])
                .help("The flag that decides to calculate bm rates based on exact days or months.")
                .default_value("M")
                .required(true)
        )
        .arg(
            Arg::with_name("is_int_calc_required")
                .long("is-int-calc-required")
                .value_name("Interest amount calculation flag")
                .possible_values(&["true", "false"])
                .help("The flag that decides to calculate interest amount based on amb file.")
                .default_value("true")
                .required(false)
        )
        .arg(
            Arg::with_name("is_int_rate_from_amb")
                .long("is-int-rate-from-amb")
                .value_name("Interest rate calculation flag")
                .possible_values(&["true", "false"])
                .help("The flag that decides to calculate interest rate based on amb file or input file.")
                .default_value("true")
                .required(false)
        )
        .arg(
            Arg::with_name("adj_count_for_bc_2",)
                .long("adj-count-for-bc-2")
                .value_name("Adj rates calculation flag")
                .help("The flag that decides to apply basecurve to how many adj rates.")
                .default_value("1")
                .required(false)
        )
        .get_matches()
}
