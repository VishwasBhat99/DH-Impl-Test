use super::*;

pub fn create_cust_master(config_param: &ConfigurationParameters, log: &Logger) {
    let mut cust_master_output_line = String::new();

    let mut cust_writer = match buf_file_wrtr(config_param.cust_master_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create cust master ouput file: `{}` on location `{}` : {}",
            config_param.output_file_path_casa(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };
    let file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in file.lines().enumerate().skip(1) {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_file_path(),
                line_num + 1,
                error
            ),
        };
        let info: Vec<&str> = line.split(config_param.field_delimiter()).collect();
        let mut row: Vec<&str> = Vec::new();
        for data in info {
            row.push(data.trim());
        }

        if row.len() < 19 {
            let report_string = format!("Less no of fields present in line no:{}, line is:{}", line_num, line);
            log_info!(log, "{}", report_string);
            continue;
        }

        let cust_no = row[4].to_string();
        if cust_no.is_empty() {
            log_error!(log, "no cust number found");
            continue;
        }
        if cust_no.parse::<i64>().is_err() {
            log_error!(log, "invalid cust_id:{}", cust_no);
        }
        let formated_cust_no = get_formatted_cust_no(cust_no.as_str());
        let cust_category = row[18].to_string();
        if cust_category.is_empty() {
            log_error!(log, "cust no:`{}` , category empty", formated_cust_no);
        }
        let output_line = get_cust_master_output(formated_cust_no.as_str(), cust_category.as_str());

        cust_master_output_line.push_str(output_line.as_str());
        cust_master_output_line.push_str("\n");
    }
    match cust_writer.write_all(cust_master_output_line.as_bytes()) {
        Ok(_) => println!("Successfully written cust master file."),
        Err(error) => panic!(
            "Unable to write custer master lines to file `{}`: {}.",
            config_param.cust_master_file_path(),
            error,
        ),
    };
}

fn get_formatted_cust_no(cust_no: &str) -> String {
    cust_no.trim_start_matches('0').to_string()
}

fn get_cust_master_output(cust_no: &str, cust_category: &str) -> String {
    format!("{}~#~{}", cust_no, cust_category)
}
