use self::io::*;
use self::metadata_val::AccountMetadata;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::new_buf_rdr;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use slog::Logger;
use std::io::BufRead;
use std::io::Write;
use std::time::SystemTime;
mod io;
mod metadata_val;

pub fn fixed_len_to_txt(
    config_params: ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
) {
    let start_time = SystemTime::now();
    let mut tot_acc_encntrd = 0;
    let skp_acc = 0;
    let ttl_amt: f64 = 0.0;
    let metadata_reader = AccountMetadata::new_from_path(config_params.metadata_file_path());
    let mut op_writer = get_writer(config_params.output_file_path());
    let mut metadata_vec: Vec<FieldDesc> = Vec::new();
    // this counter wiil check position duplication and ordering
    let mut counter = 1;
    // This field will check the overlapping of fields start positiong of current field vs end position previous fields
    let mut prev_field_end_pos = 0;
    for val in metadata_reader.fields.iter() {
        if val.position != counter {
            panic!("position is not correct for {} field in metadata", val.name);
        }
        if prev_field_end_pos >= val.start_pos {
            panic!(
                "start_pos of {} should be greater than the prev field end position",
                val.name
            );
        }
        metadata_vec.push(FieldDesc::new(
            val.name.clone(),
            val.typ.clone(),
            val.start_pos.clone(),
            val.max_len.clone(),
        ));
        prev_field_end_pos = val.start_pos + val.max_len;
        counter += 1;
    }
    //write the header
    let mut output_header_vec = Vec::new();
    for val in &metadata_vec {
        output_header_vec.push(val.name.clone());
    }
    writeln!(op_writer, "{}", output_header_vec.join("|"))
        .expect("output header line can not be written");

    //read the fixed length file
    let fix_len_file = match new_buf_rdr(config_params.input_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found fix_len_file: `{}` : {}.",
            config_params.input_file(),
            error
        ),
    };
    for (line_num, lines) in fix_len_file
        .lines()
        .enumerate()
        .skip(*config_params.skip_header())
    {
        tot_acc_encntrd += 1;
        let fixed_len_input_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_file(),
                line_num + 1,
                error
            ),
        };
        let mut fixed_len_input_fields: Vec<String> = Vec::new();
        for val in metadata_vec.iter() {
            // get the value from fixed length file using start and max_len
            let curr_field_val =
                fixed_len_input_line[val.start_pos - 1..val.start_pos + val.max_len].trim();

            //convert the field value as per metadata
            match &val.typ[..] {
                "I64" => {
                    fixed_len_input_fields.push(curr_field_val.parse().unwrap_or(0).to_string())
                }
                "I32" => {
                    fixed_len_input_fields.push(curr_field_val.parse().unwrap_or(0).to_string())
                }
                "F64" => {
                    fixed_len_input_fields.push(curr_field_val.parse().unwrap_or(0.0).to_string())
                }
                "F32" => {
                    fixed_len_input_fields.push(curr_field_val.parse().unwrap_or(0.0).to_string())
                }
                "String" => fixed_len_input_fields.push(curr_field_val.to_string()),
                _ => panic!(
                    "Invalid property type encountered in account metadata: {}",
                    val.typ
                ),
            };
        }
        let output_line = fixed_len_input_fields.join(config_params.field_delimeter());
        writeln!(op_writer, "{}", output_line).expect(&format!("{} can not be written", line_num));
        op_writer.flush().expect("Unable to flush the writer.");
    }
    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skp_acc,
        skp_acc,
        ttl_amt,
        ttl_amt,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}
#[derive(Serialize, Deserialize, Debug)]
struct FieldDesc {
    pub name: String,
    pub typ: String,
    pub start_pos: usize,
    pub max_len: usize,
}
impl FieldDesc {
    fn new(name: String, typ: String, start_pos: usize, max_len: usize) -> FieldDesc {
        FieldDesc {
            name,
            typ,
            start_pos,
            max_len,
        }
    }
}
