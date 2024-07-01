use super::{format::*, io::*, script_reader::*, structs::*, SystemTime};
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use slog::Logger;
use std::fs::read_dir;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let st_tm_read = SystemTime::now();
    let mut op_line = String::new();
    op_line.push_str(&format!(
        "#!/usr/bin/env bash\n\n# Executor Id: {}\n",
        config_param.executor_id()
    ));
    let paths = read_dir(config_param.scenario_file_path())
        .expect("Error while getting scenario file path.");
    for path in paths {
        let reader = read_file(
            &path
                .expect("Error while listing file in scenario directory.")
                .path()
                .display()
                .to_string(),
        );
        let stream: Stream = serde_json::from_reader(reader).expect("JSON was not well-formatted");
        for flow in stream.flows {
            if flow.executor_id == config_param.executor_id() {
                for mut process in flow.process {
                    // Taking entire folder for loaders
                    let dir_names: Vec<&str> = process.process_binary.split('/').collect();
                    if let Some(pos) = dir_names.iter().position(|&dir| {
                        dir.to_lowercase().contains("load") || dir.to_lowercase().contains("recon")
                    }) {
                        let new_path = dir_names.split_at(pos + 1);
                        process.process_binary = new_path.0.join("/");
                    }
                    // Handling recursive script call
                    else if process.process_binary.contains(".sh")
                        && !process.process_binary.contains("pfc")
                        && !process.process_binary.contains("#")
                        && process.process_binary.contains("programs")
                    {
                        op_line.push_str(&format_output(
                            process.process_binary.to_string(),
                            &config_param,
                        ));
                        let scripts = reading_scripts(&process.process_binary, &log);
                        for script in scripts {
                            if script.len() != 0 {
                                op_line.push_str(&format_output(script, &config_param));
                            }
                        }
                    }

                    if process.process_binary.contains("programs") {
                        op_line.push_str(&format_output(process.process_binary, &config_param));
                    } else {
                        log_error!(log, "Skipped File: `{}`.", process.process_binary);
                    }
                }
            }
        }
    }
    let ed_tm_read = SystemTime::now();
    let duration = ed_tm_read
        .duration_since(st_tm_read)
        .expect("Could not calculate total read process duration.");
    debug!(diag_log, "Read Process Total Duration: {:?}.", duration);
    let st_tm_writer = SystemTime::now();

    let mut op_writer = get_writer(config_param.output_file_path());
    output_writer(&mut op_writer, op_line, config_param.output_file_path());

    let ed_tm_writer = SystemTime::now();
    let duration = ed_tm_writer
        .duration_since(st_tm_writer)
        .expect("Could not calculate total duration for write process.");
    debug!(
        diag_log,
        "Writing Script Generator, Total Duration: {:?}.", duration
    );
}
