mod account_struct;
use super::*;
use process::process::account_struct::ExtraFieldData;
use std::io::{BufRead, Write};

pub fn process_la(
    input_buffer: BufReader<File>,
    config_params: &ConfigurationParameters,
    logger: &Logger,
) {
    let account_map = get_excel_data(config_params, logger);
    let mut output_writer =
        File::create(config_params.output_file_path()).expect("unable to create output file");
    for line in input_buffer.lines() {
        let each_line = line.expect("unable to read line");
        let mut extrafeild = get_extra_feilds(each_line.as_str());
        extrafeild.restruct_flag = set_restruct_flag(
            extrafeild.acc_id.as_str(),
            extrafeild.restruct_flag.as_str(),
            &account_map,
        );
        write_struct(&extrafeild, &mut output_writer);
    }
}

pub fn get_extra_feilds(line: &str) -> account_struct::ExtraFieldData {
    let split_vec: Vec<&str> = line.split('|').collect();
    vec_to_struct(&split_vec)
}

pub fn vec_to_struct(vec: &Vec<&str>) -> account_struct::ExtraFieldData {
    account_struct::ExtraFieldData {
        acc_id: vec[0].to_string(),
        sanc_dt: vec[1].to_string(),
        occp_cd: vec[2].to_string(),
        sens_sec: vec[3].to_string(),
        prior_subtype: vec[4].to_string(),
        restruct_flag: vec[5].to_string(),
        restruct_dt: vec[6].to_string(),
        mor_prd: vec[7].to_string(),
        rating: vec[8].to_string(),
        consitin: vec[9].to_string(),
        pan: vec[10].to_string(),
        limit_amt: vec[11].to_string(),
        gross_adv: vec[12].to_string(),
        exp_amt: vec[13].to_string(),
        unvail_amt: vec[14].to_string(),
        gold_gram: vec[15].to_string(),
        fund_flag: vec[16].to_string(),
    }
}

pub fn get_excel_data(
    config_params: &ConfigurationParameters,
    _logger: &Logger,
) -> HashMap<String, bool> {
    let mut worsheet =
        open_workbook_auto(config_params.sheet_file_path()).expect("unable to read excel data");
    let mut map: HashMap<String, bool> = HashMap::new();
    if let Some(Ok(r)) = worsheet.worksheet_range("Account wise") {
        for row in r.rows().skip(1) {
            let acc_number = row[2].to_string().trim().to_owned();
            let high_provision = row[15].to_string().to_owned();
            if high_provision.to_lowercase().contains("yes") {
                map.insert(acc_number, true);
            } else {
                map.insert(acc_number, false);
            }
        }
    }
    return map;
}

pub fn set_restruct_flag(
    account_number: &str,
    _previous_value: &str,
    account_map: &HashMap<String, bool>,
) -> String {
    match account_map.get(account_number) {
        None => "0".to_string(),
        Some(val) => {
            if *val {
                "1".to_string()
            } else {
                "0".to_string()
            }
        }
    }
}

pub fn write_struct(extrafeilds: &ExtraFieldData, output_writer: &mut File) {
    let line = format!("{}\n", extrafeilds.to_string());
    output_writer
        .write_all(line.as_bytes())
        .expect("unable to write line to output file");
}
