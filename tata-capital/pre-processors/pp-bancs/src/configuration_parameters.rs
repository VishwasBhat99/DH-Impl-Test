use clap;
use clap::{Arg, Command};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}
pub struct ConfigurationParameters {
    pub as_on_date: NaiveDate,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub input_file_path: String,
    pub writeoff_file_path: String,
    pub repayment_schedule_file_path: String,
    pub tcfsl_file_path: String,
    pub product_entity_mapping_file_path: String,
    pub product_id_file_path: String,
    pub product_fixed_floating_file_path: String,
    pub output_file_path: String,
    pub tcfsl_sheet_name: String,
    pub company_code: String,
    pub int_basis: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}
impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(logger, "company_code: {}", self.company_code());
        info!(logger, "writeoff_file_path: {}", self.writeoff_file_path());
        info!(
            logger,
            "repayment_schedule_file_path: {}",
            self.repayment_schedule_file_path()
        );
        info!(logger, "tcfsl_file_path: {}", self.tcfsl_file_path());
        info!(logger, "tcfsl_sheet_name: {}", self.tcfsl_sheet_name());
        info!(logger, "int_basis: {}", self.int_basis());
        info!(
            logger,
            "product_entity_mapping_file_path: {}",
            self.product_entity_mapping_file_path()
        );
        info!(
            logger,
            "product_fixed_floating_file_path: {}",
            self.product_fixed_floating_file_path()
        );
        info!(
            logger,
            "product_id_file_path: {}",
            self.product_id_file_path()
        );
        info!(logger, "output_file_path: {}", self.output_file_path());
    }
}
impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("`as-on-date` not well-formatted."),
        );
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level`.")
            .to_string();
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let writeoff_file_path = matches
            .value_of("writeoff_file")
            .expect("Error getting `writeoff_file_path`.")
            .to_string();
        let repayment_schedule_file_path = matches
            .value_of("repayment_schedule_file")
            .expect("Error getting `repayment_schedule_file_path`.")
            .to_string();
        let tcfsl_file_path = matches
            .value_of("tcfsl_file")
            .expect("Error getting `tcfsl_file_path`.")
            .to_string();
        let product_entity_mapping_file_path = matches
            .value_of("product_entity_mapping_file")
            .expect("Error getting `product_entity_mapping_file_path`.")
            .to_string();
        let product_id_file_path = matches
            .value_of("product_id_file")
            .expect("Error getting `product_id_file_path`.")
            .to_string();
        let product_fixed_floating_file_path = matches
            .value_of("product_fixed_floating_file")
            .expect("Error getting `product_fixed_floating_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let tcfsl_sheet_name = matches
            .value_of("tcfsl_sheet_name")
            .expect("Error getting `tcfsl_sheet_name`.")
            .to_string();
        let int_basis = matches
            .value_of("int_basis")
            .expect("Error getting `int_basis`.")
            .to_string();
        let company_code = matches
            .value_of("company_code")
            .expect("Error getting `company_code`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");
        ConfigurationParameters {
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            input_file_path,
            writeoff_file_path,
            repayment_schedule_file_path,
            tcfsl_file_path,
            product_entity_mapping_file_path,
            product_id_file_path,
            product_fixed_floating_file_path,
            output_file_path,
            tcfsl_sheet_name,
            int_basis,
            log_level,
            company_code,
            is_perf_diagnostics_enabled,
        }
    }
}
// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn writeoff_file_path(&self) -> &str {
        &self.writeoff_file_path
    }
    pub fn repayment_schedule_file_path(&self) -> &str {
        &self.repayment_schedule_file_path
    }
    pub fn tcfsl_file_path(&self) -> &str {
        &self.tcfsl_file_path
    }
    pub fn product_entity_mapping_file_path(&self) -> &str {
        &self.product_entity_mapping_file_path
    }
    pub fn product_id_file_path(&self) -> &str {
        &self.product_id_file_path
    }
    pub fn product_fixed_floating_file_path(&self) -> &str {
        &self.product_fixed_floating_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn tcfsl_sheet_name(&self) -> &str {
        &self.tcfsl_sheet_name
    }
    pub fn int_basis(&self) -> &str {
        &self.int_basis
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn company_code(&self) -> &str {
        &self.company_code
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
}
fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    Command::new(app_name)
        .about("This program generate the pre processor output for banncs")
        .author("Ravindar-01 <ravindar.sr@surya-soft.com>")
        .version("1.2.5454")
        .arg(
            Arg::new("log_file")
                .long("log-file")
                .value_name("Log File Path")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::new("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics File Path")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::new("input_file")
                .long("input-file")
                .value_name("Base Input File Path")
                .help("Path to read Base Input File.")
                .required(true)
        )
        .arg(
            Arg::new("writeoff_file")
                .long("writeoff-file")
                .value_name("writeoff File Path")
                .help("Path to read writeoff File.")
                .required(true)
        )
        .arg(
            Arg::new("repayment_schedule_file")
                .long("repayment-schedule-file")
                .value_name("Repayment Schedule File Path")
                .help("Path to read Repayment Schedule File.")
                .required(true)
        )
        .arg(
            Arg::new("tcfsl_file")
                .long("tcfsl-file")
                .value_name("TCFSL File Path")
                .help("Path to read TCFSL.")
                .required(true)
        )
        .arg(
            Arg::new("product_entity_mapping_file")
                .long("product-entity-mapping-file")
                .value_name("Product Entity Mapping File Path")
                .help("Path to read Product Entity Mapping.")
                .required(true)
        )
        .arg(
            Arg::new("product_id_file")
                .long("product-id-file")
                .value_name("Product Id File Path")
                .help("Path to read Product Id.")
                .required(true)
        )
        .arg(
            Arg::new("product_fixed_floating_file")
                .long("product-fixed-floating-file")
                .value_name("PRODUCT FIXED FLOATING FILE Path")
                .help("Path to read Product Fixed Floating File.")
                .required(true)
        )
        .arg(
            Arg::new("output_file")
                .long("output-file")
                .value_name("Output File Path")
                .help("Path to write Output.")
                .required(true)
        )
        .arg(
            Arg::new("tcfsl_sheet_name")
                .long("tcfsl-sheet-name")
                .value_name("tcfsl Sheet Name")
                .help("Path to write tcfsl Sheet Name.")
                .required(true)
        )
        .arg(
            Arg::new("int_basis")
                .long("int-basis")
                .value_name("int basis")
                .help("Path to int basis.")
                .required(true)
        )
        .arg(
            Arg::new("company_code")
                .long("company-code")
                .value_name("company code")
                .help("Path to company code.")
                .required(true)
        )
        .arg(
            Arg::new("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::new("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::new("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .get_matches()
}
