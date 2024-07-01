use crate::configuration_parameters::ConfigurationParameters;
use crate::process::account::*;
use calamine::{open_workbook_auto, Reader};
use health_report::HealthReport;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufWriter, Write};

mod account;
mod io;

pub fn process(config_params: &ConfigurationParameters, _logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_proc = 0;
    let header = "SOLLINE_ID|SOL_NAME|SOL_TYPE|SOL_CAT1|SOL_CAT2|SOL_CAT3|SOL_CAT4|SOL_CAT5|HL_HO|HL_RO|HL_AD1|HL_AD2|HL_AD3";

    //Read and Store Sol Type Mapper File
    let mut sol_type_mapper: HashMap<String, String> = HashMap::new();
    let sol_type_reader = match new_buf_rdr(config_params.sol_type_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found sol-type file: `{}` due to: `{}`.",
            config_params.sol_type_file(),
            error
        ),
    };
    for (line_no, lines) in sol_type_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.sol_type_file(),
                line_no + 1,
                error
            ),
        };
        let fields = line.split('|').collect::<Vec<&str>>();
        //Key: Sol-Type and Value: Sol-Type-Mapping-Values (BR or RO or ZO or HO or CO)
        sol_type_mapper.insert(fields[0].to_string(), fields[1].to_string());
    }

    //Init Writers
    let mut writers_pool: HashMap<String, BufWriter<File>> = HashMap::new();
    let mut index_mapper: HashMap<String, u32> = HashMap::new();
    for val in config_params.writer_vec().iter() {
        let (suffix, column_cell) = val.split_at(val.find(':').unwrap());
        let mut new_writer = io::get_new_writer(
            suffix.to_string(),
            &config_params.output_file().replace(".txt", ""),
        );
        writeln!(new_writer, "{}", header).expect("Header can not be written");
        writers_pool.insert(suffix.to_string(), new_writer);
        let column_cell_ascii = column_cell
            .trim_matches(':')
            .chars()
            .next()
            .expect("Error in casting &str into char");

        //(ASCII Value - 65) will give index to be referred from excel file
        index_mapper.insert(suffix.to_string(), column_cell_ascii as u32 - 65);
    }

    //Reading Input File
    let mut input_reader =
        open_workbook_auto(config_params.input_file()).expect("Unable to open Input File.");
    println!(
        "Sheets present in Input-File: `{:?}`",
        input_reader.sheet_names()
    );
    if !input_reader
        .sheet_names()
        .contains(&config_params.input_sheet_name().to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in Input-File: `{}`",
            config_params.input_sheet_name(),
            config_params.input_file(),
        );
    }
    println!(
        "Reading Sheet: `{}` from Input-File",
        config_params.input_sheet_name(),
    );
    if let Some(Ok(reader)) = input_reader.worksheet_range(config_params.input_sheet_name()) {
        for row in reader.rows().skip(1) {
            acc_enc += 1;
            let sol_type = row[3].to_string().trim().to_uppercase();
            let soltype_val = sol_type_mapper
                .get(&sol_type)
                .unwrap_or(&"NA".to_string())
                .to_string();
            let mut sol_data = SolDimData::new(row, &soltype_val.to_owned());
            if sol_type != "BRCH" && sol_type != "ALL" {
                if writers_pool.contains_key(&sol_type) {
                    let writer = writers_pool
                        .get_mut(&sol_type)
                        .unwrap_or_else(|| panic!("Writer Not found for Sol-Type: {}", sol_type));
                    acc_proc += 1;
                    writeln!(writer, "{}", format_output(&sol_data))
                        .expect("Output Line can not be written");
                }
            } else {
                for (sol_type, writer) in writers_pool.iter_mut() {
                    let hl_ro_index = match index_mapper.get(&sol_type.to_string()) {
                        Some(val) => val,
                        None => panic!(
                            "Sol-Type `{}` not found in Writer-Vec parameter: {:?}",
                            sol_type,
                            config_params.writer_vec()
                        ),
                    };
                    let init_soltype = sol_data.sol_type.to_string();
                    if sol_data.sol_type == "ALL" {
                        sol_data.hl_ro = "ALL".to_string();
                        sol_data.sol_type = "RO".to_string();
                    } else {
                        sol_data.hl_ro = row[*hl_ro_index as usize].to_string();
                    }
                    acc_proc += 1;
                    writeln!(writer, "{}", format_output(&sol_data))
                        .expect("Output Line can not be written");
                    sol_data.sol_type = init_soltype;
                }
            }
        }
    }

    let health_report = HealthReport::new(acc_enc, acc_proc, 0, 0.0, 0.0, 0);
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file());
}
