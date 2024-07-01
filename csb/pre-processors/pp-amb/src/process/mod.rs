use self::get_dates::GetDates;
use configuration_parameters::ConfigurationParameters;
use std::fs::{create_dir, metadata, remove_file, File};
use std::io::prelude::*;

mod get_dates;

pub fn process_name(config_params: &ConfigurationParameters) {
    let req_cols: Vec<&str> = (config_params.req_cols()).split("|").collect();
    let mut index;
    let dates = GetDates::new(config_params.as_on_date());
    let mut nxt_aggr_dt = dates.start_date;
    let date_format = config_params.date_format();
    let as_on_dt = config_params.as_on_date().format(date_format).to_string();

    while nxt_aggr_dt <= dates.end_date {
        let mut total_inp_accounts = 0;
        let date_folder_name = nxt_aggr_dt.format(date_format).to_string();

        let inp_file_path = config_params
            .input_file_path()
            .replace(&as_on_dt, &date_folder_name);
        let directory_path = &inp_file_path
            .rsplit_once("/")
            .expect("Could not extract directory path.");
        //getting file name
        let ip_file: Vec<&str> = directory_path.1.split("_").collect();
        let op_path = inp_file_path.replace(
            &directory_path.1,
            &(ip_file[0].to_string() + &"-extracted-" + &date_folder_name + ".txt"),
        );

        if config_params.operation() == "extract" {
            if !metadata(inp_file_path.to_string()).is_ok() {
                // create the empty file in the directory
                let _ = create_dir(directory_path.0).is_ok();
                let _ = File::create(&inp_file_path);
            }

            let mut file = File::open(&inp_file_path).expect("Cannot Open the Input file.");
            let mut input_file = String::new();
            file.read_to_string(&mut input_file)
                .expect("Could not read the value.");

            //Create output file
            let mut output_file = File::create(op_path).expect("Create Failed.");

            for line in input_file.lines() {
                let mut output = String::new();
                index = 0;
                total_inp_accounts += 1;
                let fields: Vec<&str> = line.split("|").collect();
                output.push_str(fields[req_cols[index].parse::<usize>().unwrap() - 1]);
                output.push_str("|");
                index += 1;
                output.push_str(fields[req_cols[index].parse::<usize>().unwrap() - 1]);
                output.push_str("|");
                index += 1;
                output.push_str(fields[req_cols[index].parse::<usize>().unwrap() - 1]);
                output.push_str("|");
                index += 1;
                output.push_str(fields[req_cols[index].parse::<usize>().unwrap() - 1]);
                output.push_str("\n");
                output_file
                    .write_all(output.as_bytes())
                    .expect("Couldn't write to output file.");
            }
            println!(
                "total input accounts encountered for {}: {}",
                date_folder_name, total_inp_accounts
            );
        } else {
            remove_file(op_path).ok();
        }
        nxt_aggr_dt = nxt_aggr_dt.succ();
    }
}
