use super::StoreData;
use calamine::{open_workbook_auto, Reader};
use config::{Files, SheetNames};
use slog::Logger;
use stamper::StamperData;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::SystemTime;
use structs::{
    AccData, COAData, DivisionMapping, FinnoneProdToDiv, LNMAlternateAccs, NPAData, WriteOff,
};

pub mod config;
pub mod stamper;
mod statics;
pub mod structs;

pub fn read_files(config_files: Files, logger: &Logger, _diag_logger: &Logger) -> StoreData {
    //Init hashMaps for all files to be read
    let mut stamper_map: HashMap<String, StamperData> = HashMap::new();
    let mut coa_map: HashMap<String, COAData> = HashMap::new();
    let mut finnone_map: HashMap<String, FinnoneProdToDiv> = HashMap::new();
    let mut lnm_map: HashMap<String, LNMAlternateAccs> = HashMap::new();
    let mut npa_prev_month_map: HashMap<String, NPAData> = HashMap::new();
    let mut npa_ason_map: HashMap<String, NPAData> = HashMap::new();
    let mut npa_prev_year_map: HashMap<String, NPAData> = HashMap::new();
    let mut writeoff_prev_month_map: HashMap<String, WriteOff> = HashMap::new();
    let mut writeoff_ason_map: HashMap<String, WriteOff> = HashMap::new();
    let mut writeoff_prev_year_map: HashMap<String, WriteOff> = HashMap::new();
    let mut division_map: HashMap<String, DivisionMapping> = HashMap::new();
    let mut accs_src_sys_map: HashMap<String, AccData> = HashMap::new();

    let sheet_names = SheetNames::new_from_path(&config_files.sheet_names_file);
    let t0 = SystemTime::now();

    //Reading Finnone Product to Division Mapping File
    let mut finnone_data = open_workbook_auto(config_files.finnone_prod_to_div_mapping_file)
        .expect("Could Not Read Finnone Product to Division Mapping File");
    if let Some(Ok(reader)) = finnone_data.worksheet_range(
        &sheet_names
            .finnone_prod_to_div_mapping
            .unwrap_or_else(|| "Sheet1".to_string()),
    ) {
        for data in reader.rows().skip(1) {
            let data_inst = FinnoneProdToDiv::new(data);
            finnone_map.insert(get_str(&data[0]), data_inst);
        }
    }
    let finnone_timer = SystemTime::now();
    let finnone_duration = finnone_timer
        .duration_since(t0)
        .expect("Could not calculate total duration.");
    info!(
        logger,
        "Time to read/store Finnone Product to Division Mapping File: {:?}", finnone_duration
    );

    //Reading LNM Alternate Account Numbers File
    let mut lnm_data = open_workbook_auto(config_files.lnm_alternate_accs_file)
        .expect("Could Not Read LNM Alternate Account Numbers File");
    if let Some(Ok(reader)) = lnm_data.worksheet_range(
        &sheet_names
            .lnm_alternate_accs
            .unwrap_or_else(|| "Sheet1".to_string()),
    ) {
        for data in reader.rows().skip(1) {
            let data_inst = LNMAlternateAccs::new(data);
            lnm_map.insert(get_str(&data[1]), data_inst);
        }
    }
    let lnm_timer = SystemTime::now();
    let lnm_duration = lnm_timer
        .duration_since(finnone_timer)
        .expect("Could not calculate total duration.");
    info!(
        logger,
        "Time to read/store LNM Alternate Accs File: {:?}", lnm_duration
    );

    //Reading NPA AsOn and SLI File
    let mut npa_ason_data = open_workbook_auto(config_files.npa_as_on_month_file)
        .expect("Could Not Read NPA AsOn File");
    if let Some(Ok(reader)) = npa_ason_data.worksheet_range(
        &sheet_names
            .npa_as_on_month
            .unwrap_or_else(|| "Sheet1".to_string()),
    ) {
        for data in reader.rows().skip(1) {
            let data_inst = NPAData::new(data);
            npa_ason_map.insert(get_str(&data[2]), data_inst);
            let acc_inst = AccData::new_npa(data);
            accs_src_sys_map.insert(get_str(&data[2]), acc_inst);
        }
    }
    let mut npa_ason_data_sli = open_workbook_auto(config_files.npa_as_on_month_sli_file)
        .expect("Could Not Read NPA AsOn SLI File");
    if let Some(Ok(reader)) = npa_ason_data_sli.worksheet_range(
        &sheet_names
            .npa_as_on_month_sli
            .unwrap_or_else(|| "Sheet1".to_string()),
    ) {
        for data in reader.rows().skip(1) {
            let data_inst = NPAData::new(data);
            npa_ason_map.insert(get_str(&data[2]), data_inst);
            let acc_inst = AccData::new_npa(data);
            accs_src_sys_map.insert(get_str(&data[2]), acc_inst);
        }
    }
    let npa_ason_timer = SystemTime::now();
    let npa_ason_duration = npa_ason_timer
        .duration_since(lnm_timer)
        .expect("Could not calculate total duration.");
    info!(
        logger,
        "Time to read/store NPA AsOn File: {:?}", npa_ason_duration
    );

    //Reading NPA Previous Month and SLI File
    let mut npa_prev_month_data = open_workbook_auto(config_files.npa_prev_month_file)
        .expect("Could Not Read NPA Prev Month File");
    if let Some(Ok(reader)) = npa_prev_month_data.worksheet_range(
        &sheet_names
            .npa_prev_month
            .unwrap_or_else(|| "Sheet1".to_string()),
    ) {
        for data in reader.rows().skip(1) {
            let data_inst = NPAData::new(data);
            npa_prev_month_map.insert(get_str(&data[2]), data_inst);
            let acc_inst = AccData::new_npa(data);
            accs_src_sys_map.insert(get_str(&data[2]), acc_inst);
        }
    }
    let mut npa_prev_month_data_sli = open_workbook_auto(config_files.npa_prev_month_sli_file)
        .expect("Could Not Read NPA Prev Month SLI File");
    if let Some(Ok(reader)) = npa_prev_month_data_sli.worksheet_range(
        &sheet_names
            .npa_prev_month_sli
            .unwrap_or_else(|| "Sheet1".to_string()),
    ) {
        for data in reader.rows().skip(1) {
            let data_inst = NPAData::new(data);
            npa_prev_month_map.insert(get_str(&data[2]), data_inst);
            let acc_inst = AccData::new_npa(data);
            accs_src_sys_map.insert(get_str(&data[2]), acc_inst);
        }
    }
    let npa_prevmon_timer = SystemTime::now();
    let npa_prevmon_duration = npa_prevmon_timer
        .duration_since(npa_ason_timer)
        .expect("Could not calculate total duration.");
    info!(
        logger,
        "Time to read/store NPA Prev Month File: {:?}", npa_prevmon_duration
    );

    //Reading NPA Previous Financial Year and SLI File
    let mut npa_prev_year_data = open_workbook_auto(config_files.npa_prev_year_file)
        .expect("Could Not Read NPA Prev Year File");
    if let Some(Ok(reader)) = npa_prev_year_data.worksheet_range(
        &sheet_names
            .npa_prev_year
            .unwrap_or_else(|| "Sheet1".to_string()),
    ) {
        for data in reader.rows().skip(1) {
            let data_inst = NPAData::new(data);
            npa_prev_year_map.insert(get_str(&data[2]), data_inst);
            let acc_inst = AccData::new_npa(data);
            accs_src_sys_map.insert(get_str(&data[2]), acc_inst);
        }
    }
    let mut npa_prev_year_data_sli = open_workbook_auto(config_files.npa_prev_year_sli_file)
        .expect("Could Not Read NPA Prev Year SLI File");
    if let Some(Ok(reader)) = npa_prev_year_data_sli.worksheet_range(
        &sheet_names
            .npa_prev_year_sli
            .unwrap_or_else(|| "Sheet1".to_string()),
    ) {
        for data in reader.rows().skip(1) {
            let data_inst = NPAData::new(data);
            npa_prev_year_map.insert(get_str(&data[2]), data_inst);
            let acc_inst = AccData::new_npa(data);
            accs_src_sys_map.insert(get_str(&data[2]), acc_inst);
        }
    }
    let npa_year_timer = SystemTime::now();
    let npa_year_duration = npa_year_timer
        .duration_since(npa_prevmon_timer)
        .expect("Could not calculate total duration.");
    info!(
        logger,
        "Time to read/store NPA Prev Year File: {:?}", npa_year_duration
    );

    //Reading Write-Off AsOn File
    let mut writeoff_ason_data = open_workbook_auto(config_files.write_off_as_on_month_file)
        .expect("Could Not Read Write-Off AsOn File");
    if let Some(Ok(reader)) = writeoff_ason_data.worksheet_range(
        &sheet_names
            .write_off_as_on_month
            .unwrap_or_else(|| "Sheet1".to_string()),
    ) {
        for data in reader.rows().skip(1) {
            let data_inst = WriteOff::new(data);
            writeoff_ason_map.insert(get_str(&data[1]), data_inst);
            let acc_inst = AccData::new_writeoff(data);
            accs_src_sys_map.insert(get_str(&data[1]), acc_inst);
        }
    }
    let writeoff_ason_timer = SystemTime::now();
    let writeoff_ason_duration = writeoff_ason_timer
        .duration_since(npa_year_timer)
        .expect("Could not calculate total duration.");
    info!(
        logger,
        "Time to read/store WriteOff AsOn File: {:?}", writeoff_ason_duration
    );

    //Reading Write-Off Previous Month File
    let mut writeoff_prev_month_data = open_workbook_auto(config_files.write_off_prev_month_file)
        .expect("Could Not Read Write-Off Prev Month File");
    if let Some(Ok(reader)) = writeoff_prev_month_data.worksheet_range(
        &sheet_names
            .write_off_prev_month
            .unwrap_or_else(|| "Sheet1".to_string()),
    ) {
        for data in reader.rows().skip(1) {
            let data_inst = WriteOff::new(data);
            writeoff_prev_month_map.insert(get_str(&data[1]), data_inst);
            let acc_inst = AccData::new_writeoff(data);
            accs_src_sys_map.insert(get_str(&data[1]), acc_inst);
        }
    }
    let writeoff_prevmon_timer = SystemTime::now();
    let writeoff_prevmon_duration = writeoff_prevmon_timer
        .duration_since(writeoff_ason_timer)
        .expect("Could not calculate total duration.");
    info!(
        logger,
        "Time to read/store WriteOff Prev Month File: {:?}", writeoff_prevmon_duration
    );

    //Reading Write-Off Previous Financial Year File
    let mut writeoff_prev_year_data = open_workbook_auto(config_files.write_off_prev_year_file)
        .expect("Could Not Read Write-Off Prev Year File");
    if let Some(Ok(reader)) = writeoff_prev_year_data.worksheet_range(
        &sheet_names
            .write_off_prev_year
            .unwrap_or_else(|| "Sheet1".to_string()),
    ) {
        for data in reader.rows().skip(1) {
            let data_inst = WriteOff::new(data);
            writeoff_prev_year_map.insert(get_str(&data[1]), data_inst);
            let acc_inst = AccData::new_writeoff(data);
            accs_src_sys_map.insert(get_str(&data[1]), acc_inst);
        }
    }
    let writeoff_year_timer = SystemTime::now();
    let writeoff_year_duration = writeoff_year_timer
        .duration_since(writeoff_prevmon_timer)
        .expect("Could not calculate total duration.");
    info!(
        logger,
        "Time to read/store WriteOff Prev Year File: {:?}", writeoff_year_duration
    );

    //Reading COA Data File
    let mut coa_data =
        open_workbook_auto(config_files.coa_master_file).expect("Could Not Read COA Data File");
    if let Some(Ok(reader)) = coa_data.worksheet_range(
        &sheet_names
            .coa_master
            .unwrap_or_else(|| "Sheet1".to_string()),
    ) {
        for data in reader.rows().skip(1) {
            let data_inst = COAData::new(data);
            coa_map.insert(get_str(&data[0]), data_inst);
        }
    }
    let coa_timer = SystemTime::now();
    let coa_duration = coa_timer
        .duration_since(writeoff_year_timer)
        .expect("Could not calculate total duration.");
    info!(
        logger,
        "Time to read/store COA Mapping File: {:?}", coa_duration
    );

    //Reading Division Mapping File
    let mut div_data = open_workbook_auto(config_files.division_mapping_file)
        .expect("Could Not Division Mapping File");
    if let Some(Ok(reader)) = div_data.worksheet_range(
        &sheet_names
            .division_mapping
            .unwrap_or_else(|| "Sheet1".to_string()),
    ) {
        for data in reader.rows().skip(1) {
            let data_inst = DivisionMapping::new(data);
            division_map.insert(get_str(&data[0]), data_inst);
        }
    }
    let div_timer = SystemTime::now();
    let div_duration = div_timer
        .duration_since(coa_timer)
        .expect("Could not calculate total duration.");
    info!(
        logger,
        "Time to read/store Division Mapping File: {:?}", div_duration
    );

    // Read and Store Stamper Files
    for file in config_files.stamper_files {
        let stamper_file = File::open(&file.stamper_file_path).unwrap_or_else(|_| {
            panic!(
                "Could Not Read Stamper File: `{:?}`",
                &file.stamper_file_path
            )
        });
        let stamper_reader = BufReader::new(stamper_file);
        let stamper_timer1 = SystemTime::now();

        for (line_num, lines) in stamper_reader.lines().enumerate() {
            let line = match lines {
                Ok(line) => line,
                Err(error) => {
                    error!(
                        logger,
                        "Unable to read file `{}` at line number: `{}` : {}",
                        file.stamper_file_path,
                        line_num + 1,
                        error
                    );
                    continue;
                }
            };
            let stamper_data: Vec<&str> = line.split('|').collect();
            let acc_id = stamper_data[0].to_string();
            match StamperData::new_from_line(line.to_owned()) {
                Ok(stamper) => {
                    if npa_ason_map.contains_key(&acc_id) {
                        stamper_map.insert(acc_id.trim().to_string(), stamper);
                    }
                }
                Err(e) => {
                    error!(
                        logger,
                        "Couldn't parse StamperData: `{}` at line number: `{}` : {}",
                        file.stamper_file_path,
                        line_num,
                        e
                    );
                }
            }
        }
        let stamper_timer2 = SystemTime::now();
        let stamper_duration = stamper_timer2
            .duration_since(stamper_timer1)
            .expect("Could not calculate total duration.");
        info!(
            logger,
            "Time to read/store {:?} Stamper File: {:?}", file.source, stamper_duration
        );
    }

    StoreData {
        stamper_data: stamper_map,
        coa_data: coa_map,
        finnone_data: finnone_map,
        lnm_data: lnm_map,
        npa_prev_month_data: npa_prev_month_map,
        npa_ason_data: npa_ason_map,
        npa_prev_year_data: npa_prev_year_map,
        writeoff_prev_month_data: writeoff_prev_month_map,
        writeoff_ason_data: writeoff_ason_map,
        writeoff_prev_year_data: writeoff_prev_year_map,
        division_data: division_map,
        accs_src_sys: accs_src_sys_map,
    }
}

pub fn get_str(data: &calamine::DataType) -> String {
    data.to_string()
        .trim()
        .trim_matches(|p| p == '"')
        .to_string()
}
