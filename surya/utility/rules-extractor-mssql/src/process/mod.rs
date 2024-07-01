use configuration_parameters::ConfigurationParameters;
use macros;
use odbc::*;
use slog::Logger;
use std::fs::File;
use std::io::Write;

struct FileDef {
    file_id: String,
    extract_file_name: String,
}

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let con_str = config_params.connection_string();
    let env = create_environment_v3()
        .map_err(|e| e.unwrap())
        .expect("Cannot create DB environment.");
    let conn = env
        .connect_with_connection_string(&con_str)
        .expect("Cannot establish a DB connection.");

    let stmt1 =
        Statement::with_parent(&conn).expect("Cannot create a statement instance to run queries.");
    let sql_cmd1 = format!(
        "select FileID, ExtractFileName from SourceDataFileDef where CountryID = \'{}\'",
        config_params.country_id()
    );
    //Vector to store struct FileDef from SourceDataFileDef table
    let mut file_def_vec: Vec<FileDef> = Vec::new();
    let mut f_id = String::new();
    let mut ext_file_name = String::new();
    match stmt1
        .exec_direct(&sql_cmd1)
        .expect("Failed to get record from SourceDataFileDef")
    {
        Data(mut stmt1) => {
            while let Some(mut cursor) = stmt1
                .fetch()
                .expect("Cannot read output of query on SourceDataFileDef.")
            {
                match cursor
                    .get_data::<&str>(1)
                    .expect("Cannot get FileID from SourceDataFileDef.")
                {
                    Some(val) => {
                        f_id = val.to_string();
                    }
                    None => {
                        log_info!(logger, "No FileID found : \"{}\"", sql_cmd1);
                    }
                };
                match cursor
                    .get_data::<&str>(2)
                    .expect("Cannot get ExtractFileName from SourceDataFileDef.")
                {
                    Some(val) => {
                        ext_file_name = val.to_string();
                    }
                    None => {
                        log_info!(logger, "No ExtractFileName found: \"{}\"", sql_cmd1);
                    }
                };
                let inst_filedef = FileDef {
                    file_id: f_id.to_string(),
                    extract_file_name: ext_file_name.to_string(),
                };
                file_def_vec.push(inst_filedef);
            }
        }
        NoData(_) => {
            log_info!(logger, "Query \"{}\" executed, no data returned", sql_cmd1);
        }
    }

    let stmt2 =
        Statement::with_parent(&conn).expect("Cannot create a statement instance to run queries.");

    let sql_cmd2 = format!("select FileID from Sh_DHRules");

    //Vector to store FileID from Sh_DHRules
    let mut sh_filedef_vec: Vec<String> = Vec::new();
    match stmt2
        .exec_direct(&sql_cmd2)
        .expect("Failed to get record from Sh_DHRules")
    {
        Data(mut stmt2) => {
            while let Some(mut cursor) = stmt2
                .fetch()
                .expect("Cannot read output of query on Sh_DHRules.")
            {
                match cursor
                    .get_data::<&str>(1)
                    .expect("Cannot read FileID from Sh_DHRules")
                {
                    Some(val) => {
                        sh_filedef_vec.push(val.to_string());
                    }
                    None => {
                        log_info!(logger, "No FileID found in Sh_DHRules: \"{}\"", sql_cmd2);
                    }
                }
            }
        }
        NoData(_) => {
            log_info!(logger, "Query \"{}\" executed, no data returned", sql_cmd2);
        }
    }

    for filedef_instance in file_def_vec.iter() {
        if sh_filedef_vec.contains(&filedef_instance.file_id) {
            log_info!(
                logger,
                "RuleFile under modification: {}.",
                filedef_instance.file_id
            );
            continue;
        }
        let id = &filedef_instance.file_id;
        let stmt3 = Statement::with_parent(&conn)
            .expect("Cannot create a statement instance to run queries.");
        let sql_cmd3 = format!(
            "select cast(RuleFile as varchar(max)) from DHRules where FileID = \'{}\'",
            id
        );

        match stmt3
            .exec_direct(&sql_cmd3)
            .expect("Failed to get record from DHRules")
        {
            Data(mut stmt3) => {
                while let Some(mut cursor) = stmt3
                    .fetch()
                    .expect("Cannot read output of query on DHRules.")
                {
                    match cursor
                        .get_data::<&str>(1)
                        .expect("Cannot read row data from query.")
                    {
                        Some(val) => {
                            let mut filepath = config_params.output_file_path().to_string();
                            filepath.push('/');
                            let mut filename = filedef_instance.extract_file_name.to_string();
                            filename.push_str(".txt");
                            filepath.push_str(&filename);
                            log_info!(logger, "Filepath to store rule file: {}", &filepath);
                            let mut extract_file = File::create(&filepath)
                                .expect("Failed to create extracted output file in path.");
                            write!(extract_file, "{}", val.to_string())
                                .expect("Failed to write to final output file.");
                            log_info!(logger, "File:{} extracted successfully.", &filename);
                        }
                        None => {
                            log_info!(logger, "No Rule data found for FileID: \"{}\"", id);
                        }
                    }
                }
            }
            NoData(_) => {
                log_info!(logger, "Query \"{}\" executed, no data returned", sql_cmd3);
            }
        }
    }
}
