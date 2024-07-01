use self::derive::extra_fields::ExtraFieldData;

use self::derive::{
    derive_fields::*, derive_td_fields::*, get_alm_master_data, get_cust_master_data, get_npa_data,
    get_schedule_data, get_td_cr_cust_master_data,
};
use self::io::*;
use self::reconcilation::*;
use self::structs::{
    additional_passthroughs::AdditionalPassThroughs, alm_master::*, cust_master::*,
    input_account::InputAccount, loan_additional::LoanAdditional, npa::*, schedule::*,
    td_cr_cust_master::*,
};
use calamine::{open_workbook_auto, Reader};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::default::Default;
use std::io::BufRead;
use std::path::Path;
use std::time::SystemTime;

mod derive;
mod io;
mod reconcilation;
mod structs;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let st_tm_read = SystemTime::now();
    let mut op_line_emi: String = String::new();
    let mut op_line_non_emi: String = String::new();
    let mut op_line_td: String = String::new();
    let mut tot_rec = DEFAULT_INT;
    let mut skp_rec = DEFAULT_INT;

    let mut alm_master: HashMap<AlmMasterKey, AlmMaster> = HashMap::new();
    let mut alm_master_excel =
        open_workbook_auto(config_param.alm_master()).expect("Unable to open Alm Master File.");
    if let Some(Ok(reader)) = alm_master_excel.worksheet_range(config_param.alm_master_sheet_name())
    {
        for row in reader.rows() {
            get_alm_master_data(row, &mut alm_master);
        }
    }

    // Manual customer map file for CARE mapping
    let mut care_cust_map: HashMap<String, String> = HashMap::new();
    if Path::new(config_param.care_cust_mapping_file_path()).exists() {
        let mut care_cust_map_excel =
            open_workbook_auto(config_param.care_cust_mapping_file_path())
                .expect("Unable to open CARE customer mapping file.");
        if let Some(Ok(reader)) =
            care_cust_map_excel.worksheet_range(config_param.care_cust_mapping_sheet_name())
        {
            for row in reader.rows() {
                care_cust_map.insert(row[0].to_string(), row[1].to_string());
            }
        }
    }
    log_info!(
        log,
        "Number of Records in customer map file for CARE mapping File: {}",
        care_cust_map.len()
    );
    // Manual account map file for CARE mapping
    let mut care_acc_map: HashMap<String, String> = HashMap::new();
    if Path::new(config_param.care_acc_mapping_file_path()).exists() {
        let mut care_acc_map_excel = open_workbook_auto(config_param.care_acc_mapping_file_path())
            .expect("Unable to open CARE account mapping file.");
        if let Some(Ok(reader)) =
            care_acc_map_excel.worksheet_range(config_param.care_acc_mapping_sheet_name())
        {
            for row in reader.rows() {
                care_acc_map.insert(row[0].to_string(), row[1].to_string());
            }
        }
    }
    log_info!(
        log,
        "Number of Records in account map file for CARE mapping File: {}",
        care_acc_map.len()
    );

    let mut cust_master_reader = read_file(config_param.cust_master());
    let mut cust_master: CustMasterMap = CustMasterMap::new();
    for (line_num, lines) in cust_master_reader.deserialize().enumerate() {
        let cust_master_input: CustMasterInput =
            extract_lines(line_num, lines, config_param.cust_master(), log);
        get_cust_master_data(cust_master_input, &mut cust_master);
    }
    let mut cust_master_reader = read_file(config_param.cust_master());
    let mut td_cr_cust_master: TdCrCustMasterMap = TdCrCustMasterMap::new();
    for (line_num, lines) in cust_master_reader.deserialize().enumerate() {
        let td_cr_cust_master_input: TdCrCustMasterInput =
            extract_lines(line_num, lines, config_param.cust_master(), log);
        get_td_cr_cust_master_data(td_cr_cust_master_input, &mut td_cr_cust_master);
    }
    log_info!(
        log,
        "Number of Records in Customer Master Fields File: {}",
        cust_master.store.len()
    );
    log_info!(
        log,
        "Number of Records in TD Customer Master Fields File: {}",
        td_cr_cust_master.store.len()
    );

    let mut npa_reader = read_file(config_param.npa_file_path());
    let mut npa: NPAMap = NPAMap::new();
    for (line_num, lines) in npa_reader.deserialize().enumerate() {
        let npa_input: NPAInput = extract_lines(line_num, lines, config_param.npa_file_path(), log);
        get_npa_data(npa_input, &mut npa);
    }
    log_info!(
        log,
        "Number of Records in NPA Fields File: {}",
        npa.store.len()
    );

    let additional_passthroughs = AdditionalPassThroughs::new();
    let mut schedule_reader = read_file(config_param.schedule_file_path());
    let mut cashflows: HashMap<String, Vec<Cashflows>> = HashMap::new();
    for (line_num, lines) in schedule_reader.deserialize().enumerate().skip(1) {
        let mut schedules_input: Schedules =
            extract_lines(line_num, lines, config_param.schedule_file_path(), log);
        get_schedule_data(&mut schedules_input, &mut cashflows);
    }
    // Init additional fields Reader
    let mut extra_field_reader = read_file(config_param.extra_fields_file_path());
    let mut extra_field_map: HashMap<String, ExtraFieldData> = HashMap::new();
    for (_, lines) in extra_field_reader.deserialize().enumerate() {
        let extra_field_rec: ExtraFieldData =
            lines.expect("Cannot read line into Extra Fields Data struct!");
        extra_field_map.insert(extra_field_rec.acc_id.to_string(), extra_field_rec);
    }
    log_info!(
        log,
        "Number of Records in Extra Fields File: {}",
        extra_field_map.len()
    );
    // Init LTV Reader
    let ltv_reader =
        sdb_io::new_buf_rdr(config_param.ltv_file_path()).expect("Cannot open LTV file for read!");
    let mut ltv_map: HashMap<String, String> = HashMap::new();
    for (_, lines) in ltv_reader.lines().enumerate() {
        let line = lines.expect("Cannot read data from LTV file!");
        let line_info: Vec<&str> = line.split('|').collect();
        let acc_no = line_info[0].to_string();
        let ltv = line_info[1].to_string();
        ltv_map.insert(acc_no, ltv);
    }
    log_info!(
        log,
        "Number of Records in LTV Fields File: {}",
        ltv_map.len()
    );

    let mut loan_additional_reader = read_file(config_param.loan_additional_file_path());
    let mut loan_additional_map: HashMap<String, String> = HashMap::new();
    let mut loan_additional_map_org: HashMap<String, String> = HashMap::new();
    for (line_num, lines) in loan_additional_reader.deserialize().enumerate() {
        let mut loan_additional_input: LoanAdditional = extract_lines(
            line_num,
            lines,
            config_param.loan_additional_file_path(),
            log,
        );
        loan_additional_map.insert(
            (*loan_additional_input.acc_id).to_string(),
            loan_additional_input.int_benchmark_type,
        );
        loan_additional_map_org.insert(
            (*loan_additional_input.acc_id).to_string(),
            loan_additional_input.org_code,
        );
    }
    log_info!(
        log,
        "Number of records in Loan Additional File: {}",
        loan_additional_map.len()
    );

    let mut recon = ReconMap::new();
    let mut concats = String::new();
    let mut input_reader = read_file(config_param.input_file_path());
    let mut tot_amt = 0.0;
    for (line_num, lines) in input_reader.deserialize().enumerate().skip(1) {
        let mut input_account: InputAccount =
            extract_lines(line_num, lines, config_param.input_file_path(), log);
        tot_rec += 1;

        let amt = input_account.bal.parse().unwrap_or(DEFAULT_FLOAT);
        if amt == 0.0 {
            log_error!(
                log,
                "account: `{}` is already matured.",
                input_account.acc_no
            );
            skp_rec += 1;
            continue;
        }
        tot_amt += amt;

        if amt > 0.0 {
            op_line_td.push_str(&get_td_op_line(
                &mut input_account,
                &mut td_cr_cust_master,
                &mut npa,
                &mut alm_master,
                &mut concats,
                &additional_passthroughs,
                *config_param.as_on_date(),
                &loan_additional_map,
                &loan_additional_map_org,
            ));
        } else if input_account.loan_type.to_uppercase() == "EMI" {
            op_line_emi.push_str(&get_op_line(
                &mut input_account,
                &mut cust_master,
                &mut npa,
                &mut alm_master,
                &mut cashflows,
                &extra_field_map,
                &ltv_map,
                &mut concats,
                &care_cust_map,
                &care_acc_map,
                &loan_additional_map,
                &loan_additional_map_org,
            ));
        } else {
            op_line_non_emi.push_str(&get_op_line(
                &mut input_account,
                &mut cust_master,
                &mut npa,
                &mut alm_master,
                &mut cashflows,
                &extra_field_map,
                &ltv_map,
                &mut concats,
                &care_cust_map,
                &care_acc_map,
                &loan_additional_map,
                &loan_additional_map_org,
            ));
        }

        let recon_key = ReconKey::new(
            input_account.acnts_curr_cd,
            String::from("Loans"),
            input_account.gl_cd,
        );
        recon
            .store
            .entry(recon_key)
            .and_modify(|val| *val += amt)
            .or_insert(amt);
    }

    let ed_tm_read = SystemTime::now();
    let duration = ed_tm_read
        .duration_since(st_tm_read)
        .expect("Could not calculate total read process duration.");
    debug!(diag_log, "Read Process Total Duration: {:?}.", duration);

    let st_tm_writer = SystemTime::now();
    let mut emi_file_path = String::from(config_param.output_file_path());
    emi_file_path = emi_file_path.replace(".txt", "-emi.txt");
    let mut op_writer_emi = get_writer(&emi_file_path);
    output_writer(&mut op_writer_emi, op_line_emi, &emi_file_path);

    let mut non_emi_file_path = String::from(config_param.output_file_path());
    non_emi_file_path = non_emi_file_path.replace(".txt", "-non-emi.txt");
    let mut op_writer_non_emi = get_writer(&non_emi_file_path);
    output_writer(&mut op_writer_non_emi, op_line_non_emi, &non_emi_file_path);

    let mut td_file_path = String::from(config_param.output_file_path());
    td_file_path = td_file_path.replace(".txt", "-td.txt");
    let mut op_writer_td = get_writer(&td_file_path);
    output_writer(&mut op_writer_td, op_line_td, &td_file_path);

    let mut recon_writer = get_writer(config_param.rec_output_file_path());
    output_writer(
        &mut recon_writer,
        recon.print(*config_param.as_on_date(), "TermLoans"),
        config_param.rec_output_file_path(),
    );

    let mut concat_writer = get_writer(config_param.concat_file_path());
    output_writer(&mut concat_writer, concats, config_param.concat_file_path());

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_param.output_file_path());

    let ed_tm_writer = SystemTime::now();
    let duration = ed_tm_writer
        .duration_since(st_tm_writer)
        .expect("Could not calculate total duration for write process.");
    debug!(diag_log, "Writing Loans, Total Duration: {:?}.", duration);
}
