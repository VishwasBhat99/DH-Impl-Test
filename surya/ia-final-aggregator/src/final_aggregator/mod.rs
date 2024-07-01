use self::organize::{organize_aggregate, organize_smry, BucketValue, LLGKey, SummaryValue};
use self::writer::write_to_file;
use configuration_parameters::ConfigurationParameters;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

mod organize;
mod writer;

pub fn aggregate(config_params: &ConfigurationParameters, log: &Logger) {
    let mut principal_file =
        File::open(config_params.principal_file_path()).expect("Cannot read principal file");
    let mut principla_contents = String::new();
    principal_file
        .read_to_string(&mut principla_contents)
        .expect("Cannot convert principal file to string");
    let mut rate_file = File::open(config_params.rate_file_path()).expect("Cannot read rate file");
    let smry_file =
        new_buf_rdr(config_params.summary_file_path()).expect("Cannot read summary file");
    let mut rate_contents = String::new();
    rate_file.read_to_string(&mut rate_contents).expect("Cannot convert rate file to string");
    let prin_lines: Vec<&str> = principla_contents.lines().collect();
    let rate_lines: Vec<&str> = rate_contents.lines().collect();
    let mut aggregate_map: HashMap<LLGKey, BucketValue> = HashMap::new();
    let mut summary_map: HashMap<LLGKey, SummaryValue> = HashMap::new();
    if prin_lines.len() != rate_lines.len() {
        panic!("Principal and Rate files data is incorrectly formatted")
    }
    for line_no in 0..prin_lines.len() {
        let principal_record = prin_lines[line_no];
        let rate_record = rate_lines[line_no];
        organize_aggregate(&mut aggregate_map, principal_record, rate_record)
    }
    for (line_no, line) in smry_file.lines().enumerate() {
        let smry_content = match line {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.summary_file_path(),
                line_no + 1,
                error
            ),
        };
        organize_smry(&mut summary_map, &smry_content);
    }
    write_to_file(config_params.output_file_path(), aggregate_map, summary_map)
}
