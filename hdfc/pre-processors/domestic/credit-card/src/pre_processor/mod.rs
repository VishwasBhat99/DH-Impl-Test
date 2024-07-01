use configuration_parameters::ConfigurationParameters;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use slog::Logger;
use macros;

pub fn process(config_param: ConfigurationParameters, log: &Logger) {
    let mut output_file = File::create(&config_param.output_file_path()).expect("Create Failed.");
    let mut output = String::new();
    let mut output_append = String::new();
    let mut tot_mast_acc = 0;
    let del = config_param.delimiter();

    let mut inp_file = File::open(config_param.input_file_path()).expect("Cannot open input file.");
    let mut input = String::new();
    inp_file.read_to_string(&mut input).unwrap();

    let master = File::open(&config_param.master_file_path()).expect("Could Not Read Master File");
    let master_reader = BufReader::new(master);

    // skipped header
    for (count, line) in master_reader.lines().enumerate().skip(1) {
        let line = line
            .expect("Could Not Read Line in Master File.")
            .to_string();
        tot_mast_acc += 1;
        let master_fields: Vec<&str> = line.split(del).collect();
        // skip footer and invalid accounts
        if master_fields.len() < 12 {
            log_info!(log, "Skipped line no: {:?}", count + 1);
            continue;
        }
        output_append.push_str(&line.to_string());
        output_append.push_str("\n");
    }
    log_info!(log, "Processed {} master file records.", tot_mast_acc);
    if tot_mast_acc < input.lines().count() {
        let empty_accs = input.lines().count() - tot_mast_acc;
        log_info!(log, "Processing total {} empty accounts.", empty_accs - 1);
        for _ in 0..empty_accs {
            output_append.push_str(del);
            output_append.push_str(del);
            output_append.push_str(del);
            output_append.push_str(del);
            output_append.push_str(del);
            output_append.push_str(del);
            output_append.push_str(del);
            output_append.push_str(del);
            output_append.push_str(del);
            output_append.push_str(del);
            output_append.push_str("\n");
        }
    }
    let mut append_master_input = output_append.split("\n");

    for (count, line) in input.lines().enumerate() {
        if count == 0 {
            continue;
        }
        let length = line.len();
        let min_len = 3 + 3 + 19 + 19 + 3 - 1; // lengths of ORG,LOGO,ACCT,LOAN_NBR,TENURE
        if length >= min_len {
            output.push_str(&line[0..3]); //ORG
            output.push_str(del);
            output.push_str(&line[3..6]); //LOGO
            output.push_str(del);
            output.push_str(&line[6..25]); //ACCT
            output.push_str(del);
            output.push_str(&line[25..44]); //LOAN_NBR
            output.push_str(del);
            output.push_str(&line[44..47]); //TENURE
            output.push_str(del);

            let mut index = 47;
            while index <= length - 33 {
                output.push_str(&line[index..index + 11]);
                index += 11;
                output.push_str(del);
                output.push_str(&line[index..index + 11]);
                index += 11;
                output.push_str(del);
                output.push_str(&line[index..index + 11]);
                index += 11;
                output.push_str(del);
                output.push_str(&line[index..index + 8]);
                index += 8;
                output.push_str(del);
            }
            output.push_str(&line[index..length]); //FILLER
            output.push_str(append_master_input.next().unwrap()); //append the master column values
            output.push_str("\n");
            output_file
                .write_all(output.as_bytes())
                .expect("Unable write to Output File.");
            output.clear();
        } else {
            log_info!(log, "Invalid Input record: {:?}", line);
        }
    }
}
