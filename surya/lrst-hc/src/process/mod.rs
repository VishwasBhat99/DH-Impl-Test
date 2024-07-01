use self::rules_extract::extract;
use crate::macros;
use configuration_parameters::ConfigurationParameters;
use dbpool::OracleConnectionManager;
use r2d2::Pool;
use sdb_agg_rules_txt::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::env;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

mod rules_extract;

pub fn process(
    pool: Pool<OracleConnectionManager>,
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    let account_metadata_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let rule_filenames_list = extract(pool, &config_params, &logger, &diag_logger);
    for (scen_id, rulefile) in rule_filenames_list.into_iter() {
        //Rules_File for HairCut Derivation
        let mut rule_filepath_hc = config_params.rules_file_path().to_string();
        let rule_file_hc = format!("{}.txt", rulefile.rules_hc);
        rule_filepath_hc.push_str(&rule_file_hc);
        //Rules_File for ProductID Derivation
        let mut rule_filepath_pid = config_params.rules_file_path().to_string();
        let rule_file_pid = format!("{}.txt", rulefile.rules_pid);
        rule_filepath_pid.push_str(&rule_file_pid);

        let mut op_filepath = config_params.output_file_path().to_string();
        let file = format!("op-{}.txt", scen_id);
        op_filepath.push_str(&file);
        let mut output_file = match buf_file_wrtr(&op_filepath, None) {
            Ok(create) => create,
            Err(error) => {
                panic!(
                    "Could not create file: `{}` on location `{}` : {:?}.",
                    op_filepath,
                    env::current_exe()
                        .expect("Unable to find current directory path!")
                        .display(),
                    error
                );
            }
        };
        log_debug!(logger, "The rules_hc file path is:: {:?}", rule_filepath_hc);
        let rules_hc = AggRules::new_from_path(&rule_filepath_hc, &account_metadata_reader);
        log_debug!(
            logger,
            "The rules_pid file path is:: {:?}",
            rule_filepath_pid
        );
        let rules_pid = AggRules::new_from_path(&rule_filepath_pid, &account_metadata_reader);
        let input_file =
            File::open(config_params.input_file_path()).expect("Cannot open input file.");
        let mut idx_count = 1;
        for line_opt in BufReader::new(input_file).lines() {
            let record = line_opt.expect("Cannot read line from input file.");
            let fields: Vec<&str> = record.split('|').collect();
            let mut op = String::new();
            let mut cntr = 0;
            let mut pid_str = String::new();
            op.push_str(&config_params.as_on_date().format("%d-%m-%Y").to_string());
            op.push('|');
            op.push_str(&scen_id);
            op.push('|');
            op.push_str(&idx_count.to_string());
            op.push('|');
            let pid = match rules_pid.llg_for_acc(&record, &account_metadata_reader) {
                Some(p) => p.llg,
                None => config_params.default_llg_code(),
            };
            let mut new_pid = format!("{:04}", pid);
            for field in fields {
                if cntr == 1 {
                    let prodid_tuple = field.split_at(3);
                    pid_str.push_str(prodid_tuple.0);
                    pid_str.push_str(&new_pid.to_string());
                    op.push_str(&pid_str);
                } else {
                    op.push_str(field);
                }
                op.push('|');
                cntr += 1;
            }
            let hc_id = match rules_hc.llg_for_acc(&record, &account_metadata_reader) {
                Some(c) => c.llg,
                None => config_params.default_llg_code(),
            };
            op.push_str(&hc_id.to_string());
            op.push('\n');
            let _ = output_file.write(op.as_bytes());
            idx_count += 1;
        }
    }
}
