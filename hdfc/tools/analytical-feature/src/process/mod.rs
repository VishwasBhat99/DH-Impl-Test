use self::io::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
mod mapped_data;
use self::mapped_data::*;
use macros;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_loggerr: &Logger) {
    let mut op_writer = get_writer(&config_params.output_file());

    let cust_id_map_cust_type = get_cust_id_to_cust_type(config_params, logger);
    let cust_id_map_division = get_cust_id_to_division(config_params, logger);
    let lcr_classification_map_lcr_run_off =
        get_lcr_classification_to_lcr_run_off(config_params, logger);
    let cust_type_map_cust_type_desc = get_cust_type_to_cust_type_desc(config_params, logger);

    let retail_input_file_path = config_params.retail_input_file();
    write_op(
        retail_input_file_path,
        logger,
        &cust_id_map_cust_type,
        &cust_id_map_division,
        &lcr_classification_map_lcr_run_off,
        &cust_type_map_cust_type_desc,
        &mut op_writer,
    );

    let non_retail_input_file_path = config_params.non_retail_input_file();
    write_op(
        non_retail_input_file_path,
        logger,
        &cust_id_map_cust_type,
        &cust_id_map_division,
        &lcr_classification_map_lcr_run_off,
        &cust_type_map_cust_type_desc,
        &mut op_writer,
    );
}

fn get_val(val: &str) -> &str {
    if val == "" {
        return "NA";
    }

    val
}

fn write_op(
    input_file_path: &str,
    logger: &Logger,
    cust_id_map_cust_type: &HashMap<String, String>,
    cust_id_map_division: &HashMap<String, String>,
    lcr_classification_map_lcr_run_off: &HashMap<String, String>,
    cust_type_map_cust_type_desc: &HashMap<String, String>,
    op_writer: &mut BufWriter<File>,
) {
    let input_file = File::open(input_file_path).expect(&format!("Could Not Read '{}'", input_file_path));
    let input_reader = BufReader::new(input_file);

    for (index, line) in input_reader.lines().enumerate() {
        let line = line.expect(&format!("Could Not Read Line.no {}", (index+1))).to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        if input_fields.len() >= 74 {
            write!(
            op_writer,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            get_val(input_fields[1]),
            if cust_id_map_cust_type.contains_key(input_fields[1]){
                &cust_id_map_cust_type[input_fields[1]]
            }
            else{
                "NA"
            },
            if cust_id_map_cust_type.contains_key(input_fields[1]) {
               if cust_type_map_cust_type_desc.contains_key(&cust_id_map_cust_type[input_fields[1]]) {
                    &cust_type_map_cust_type_desc[&cust_id_map_cust_type[input_fields[1]]]
               }
               else{
                   "NA"
               }
            }
            else{
                "NA"
            },
            get_val(input_fields[14]),
            get_val(input_fields[15]),
            if cust_id_map_division.contains_key(input_fields[1]) {
                &cust_id_map_division[input_fields[1]]
            }
            else{
                "NA"
            },
            get_val(input_fields[19]),
            get_val(input_fields[2]),
            if lcr_classification_map_lcr_run_off.contains_key(input_fields[2]) {
                &lcr_classification_map_lcr_run_off[input_fields[2]]
            }
            else{
                "NA"
            },
            get_val(input_fields[20]),
            get_val(input_fields[21]),
            get_val(input_fields[22]),
            get_val(input_fields[23]),
            get_val(input_fields[24]),
            get_val(input_fields[33]),
            get_val(input_fields[34]),
            get_val(input_fields[35]),
            get_val(input_fields[36]),
            get_val(input_fields[37]),
            get_val(input_fields[38]),
            get_val(input_fields[63]),
            get_val(input_fields[64]),
            get_val(input_fields[65]),
            get_val(input_fields[66]),
            get_val(input_fields[67]),
            get_val(input_fields[68]),
            get_val(input_fields[69]),
            get_val(input_fields[70]),
            get_val(input_fields[71]),
            get_val(input_fields[72]),
            get_val(input_fields[73]),
        ).expect(&format!("Could not write output in line-no. {}", (index+1)));
        } else {
            log_error!(
                logger,
                "fields are less in the line-no.{} in '{}'. line is '{}'.",
                (index + 1).to_string(),
                input_file_path,
                line
            );
        }
    }
}
