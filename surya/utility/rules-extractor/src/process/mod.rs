use configuration_parameters::ConfigurationParameters;
use dbpool::OracleConnectionManager;
use macros;
use r2d2::Pool;
use slog::Logger;
use std::fs::File;
use std::io::prelude::*;
use std::str;

struct FileDef {
    file_id: String,
    extract_file_name: String,
}

pub fn process(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
    pool: Pool<OracleConnectionManager>,
) {
    //vector to store structs from SourceDataFileDef table
    let mut file_def_vec: Vec<FileDef> = Vec::new();
    let conn = &pool
        .get()
        .expect("Failed to get connection from pool.")
        .conn;
    match conn {
        Some(db) => {
            let sql1 = "select count(\"FileID\") from \"Sh_DHRules\"";
            let rows = db
                .conn
                .query(sql1, &[])
                .expect("Query Failed to Fetch count(FileID) from Sh_DHRules.");
            for row_result in &rows {
                let row = row_result.expect("Failed to read query1 output.");
                let count: i64 = row
                    .get(0)
                    .expect("Failed to get value of count from query1 output.");
                //Extract Rule Files only if there are no records in Sh_DHRules.
                if count == 0 {
                    let country = config_params.country_id();
                    let sql2 = "select \"FileID\", \"ExtractFileName\" from \"SourceDataFileDef\" where \"CountryID\"=:1";
                    let rows = db
                        .conn
                        .query(sql2, &[&country])
                        .expect("Query Failed to Fetch Data from SourceDataFileDef.");
                    for row_result in &rows {
                        let row = row_result
                            .expect("Failed to read query output from SourceDataFileDef.");
                        let src_fileid: String = row.get("FileID").expect("Error reading FileID.");
                        let src_extfilename: String = row
                            .get("ExtractFileName")
                            .expect("Error reading ExtractFileName.");
                        file_def_vec.push(FileDef {
                            file_id: src_fileid,
                            extract_file_name: src_extfilename,
                        });
                    }

                    for filedef_instance in file_def_vec.iter() {
                        let id = &filedef_instance.file_id;
                        let sql3 = "select \"RuleFile\" from \"DHRules\" where \"FileID\"=:1";
                        let rows = db
                            .conn
                            .query(sql3, &[id])
                            .expect("Query Failed to Fetch Data from DHRules.");
                        for row_result in &rows {
                            let row =
                                row_result.expect("Failed to read query output from DHRules.");
                            log_debug!(diag_logger, "row: {:?}", row);
                            let rule_file: Vec<u8> = match row.get("RuleFile") {
                                Ok(data) => data,
                                Err(err) => {
                                    log_error!(
                                        logger,
                                        "Could not extract data for :{}. Error:{}",
                                        id,
                                        err
                                    );
                                    continue;
                                }
                            };

                            match str::from_utf8(&rule_file) {
                                Ok(_val) => {
                                    let mut filepath = config_params.output_path().to_string();
                                    filepath.push('/');
                                    let mut filename =
                                        filedef_instance.extract_file_name.to_string();
                                    filename.push_str(".txt");
                                    filepath.push_str(&filename);
                                    let mut extract_file = File::create(&filepath)
                                        .expect("Failed to create extracted output file in path.");
                                    extract_file
                                        .write_all(&rule_file)
                                        .expect("Failed to write to final output file.");
                                    log_info!(logger, "File:{} extracted successfully.", &filename);
                                }
                                Err(err) => {
                                    log_error!(logger, "{}.", err);
                                    return;
                                }
                            }
                        }
                    }
                } else {
                    log_error!(
                        logger,
                        "Rules File(s) under modification in Sh_DHRules. Failed to extract rules files."
                    );
                    panic!("Rules File(s) under modification in Sh_DHRules. Failed to extract rules files.");
                }
            }
        }
        None => {}
    };
}
