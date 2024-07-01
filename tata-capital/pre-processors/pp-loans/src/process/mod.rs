use crate::configuration_parameters::ConfigurationParameters;
use crate::process::input_account::*;
use crate::process::output_account::{format_output, get_writer, OutputAccount};
use calamine::{open_workbook_auto, Reader};
use chrono::prelude::*;
use health_report::HealthReport;
use rbdate::{incr_dt_by_mon_presrv_eom_checked, increment_date_by_months, NaiveDate};
use slog::Logger;
use std::cmp::min;
use std::collections::HashMap;
use std::default;
use std::path::Component;
use std::{fs, io::Write};
mod input_account;
mod output_account;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_proc = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;

    //Reading TCFSL NPA FILE
    let mut tcfsla_map: HashMap<String, (String, String)> = HashMap::new();
    let mut tcfsl_npa_file_path = open_workbook_auto(config_params.tcfsl_npa_file())
        .expect("Unable to open the tcfsl xlsb file.");
    info!(
        logger,
        "Sheets present in tcfsl_npa_file: `{:?}`",
        tcfsl_npa_file_path.sheet_names()
    );
    if !tcfsl_npa_file_path
        .sheet_names()
        .contains(&config_params.tcfsl_npa_sheet_name().to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in TCFSL NPA File: `{}`",
            config_params.tcfsl_npa_sheet_name(),
            config_params.tcfsl_npa_file()
        );
    }
    info!(
        logger,
        "Reading Sheet: `{}` from Master-File",
        config_params.tcfsl_npa_sheet_name(),
    );
    if let Some(Ok(tcfsl_npa_reader)) =
        tcfsl_npa_file_path.worksheet_range(&config_params.tcfsl_npa_sheet_name())
    {
        for (row_no, row) in tcfsl_npa_reader.rows().enumerate().skip(1) {
            let tcfsl_data = TcfslNpa::new_from_excel(row);
            let acct_number = tcfsl_data.acct_no;
            let asset_classification = tcfsl_data.asset_classification;
            let schm_code = tcfsl_data.schm_code;
            tcfsla_map.insert(acct_number, (asset_classification, schm_code));
        }
    }

    //Reading Cashflow file
    let mut finnone_cashflow_map: HashMap<String, Vec<(FinnoneCashflows, NaiveDate)>> =
        HashMap::new();
    let finnone_cashflows_file = fs::read_to_string(config_params.finnone_cashflow_file_path())
        .expect("Could not read cashflows file");
    for (line_no, line) in finnone_cashflows_file.lines().enumerate().skip(1) {
        let input_vec: Vec<&str> = line.split("*|~").collect::<Vec<&str>>();
        let cashflows_data: FinnoneCashflows = FinnoneCashflows::new(
            config_params,
            config_params.finnone_cashflow_file_path(),
            &input_vec,
            line_no + 1,
        );
        let loan_id: String = cashflows_data.loan_id.clone();
        let trimed_loan_id: &str = loan_id.trim_start_matches('0');
        finnone_cashflow_map
            .entry(trimed_loan_id.to_string())
            .and_modify(|prev_data| {
                let nearest_date = min(
                    prev_data[0].0.repayment_date.clone(),
                    cashflows_data.repayment_date.clone(),
                );
                prev_data.push((cashflows_data.clone(), nearest_date));
            })
            .or_insert(vec![(
                cashflows_data.clone(),
                cashflows_data.repayment_date,
            )]);
    }

    //Reading Write off merged file

    let mut writeoff_merged_map: HashMap<String, WriteOffMerged> = HashMap::new();
    let write_off_merged_file = fs::read_to_string(config_params.writeoff_merged_file_path())
        .expect("Could not read write off merged map");
    for (line_no, line) in write_off_merged_file.lines().enumerate().skip(1) {
        let input_vec: Vec<&str> = line.split("*|~").collect::<Vec<&str>>();
        let mut writeoff_data: WriteOffMerged = WriteOffMerged::new(
            config_params,
            config_params.writeoff_merged_file_path(),
            &input_vec,
            line_no + 1,
        );
        let loan_id = writeoff_data.loan_id.clone();
        writeoff_merged_map.insert(loan_id, writeoff_data);
    }

    let mut op_writer = get_writer(config_params.output_file());
    let as_on_date = config_params.as_on_date();
    //Reading Input file
    let finnone_fsl_file = fs::read_to_string(config_params.finnone_fsl_file_path())
        .expect("Could not read Input file");
    for (line_no, line) in finnone_fsl_file.lines().enumerate().skip(1) {
        acc_enc += 1;
        let input_vec: Vec<&str> = line.split("*|~").collect::<Vec<&str>>();
        let mut input_data: InputData = InputData::new(
            config_params,
            config_params.finnone_fsl_file_path(),
            &input_vec,
            line_no + 1,
        );
        let loan_id = input_data.loanid.clone();
        let trimed_loan_id = loan_id.trim_start_matches('0');
        let pre_emi_ovd_amt = input_data.totalinterestaccrued;
        let mut due_dt = input_data.maturity_date;
        let def_finnone_cashflow_vec: Vec<(FinnoneCashflows, NaiveDate)> = Vec::new();
        let finnone_cashflow_vec = finnone_cashflow_map
            .get(&trimed_loan_id.to_string())
            .unwrap_or(&def_finnone_cashflow_vec);
        let cashflow_vec_len = finnone_cashflow_vec.len();
        if cashflow_vec_len == 0 {
            acc_proc += 1;
            let def_type = ("P".to_string(), "".to_string());
            let trimed_loan_id = loan_id.trim_start_matches('0');
            let mut final_npa_typ = "P".to_string();
            let (npa_type, schm_code) = tcfsla_map.get(trimed_loan_id.clone()).unwrap_or(&def_type);

            if tcfsla_map.contains_key(trimed_loan_id.clone())
                && schm_code.to_uppercase() == "FINNONE"
            {
                final_npa_typ = npa_type.to_string();
            }
            let def_writeoff_data = WriteOffMerged::default();
            let writeoff_data = writeoff_merged_map
                .get(&loan_id)
                .unwrap_or(&def_writeoff_data);
            let mut division = "NULL".to_string();
            if writeoff_merged_map.contains_key(&loan_id) {
                if writeoff_data.source_system.to_uppercase() == "FINNONE" {
                    division = writeoff_data.asset_class.clone();
                }
            }
            let due_date = input_data.maturity_date.clone();
            let component = "PRINCIPAL";
            let principal_amt = input_data.principalcomponent;
            writeln!(
                op_writer,
                "{}",
                format_output(
                    input_data.clone(),
                    division,
                    loan_id,
                    due_dt,
                    component.to_string(),
                    principal_amt,
                    final_npa_typ,
                    pre_emi_ovd_amt,
                    config_params
                )
            )
            .expect("Error in Writing Output");
        } else {
            for cash_flows in finnone_cashflow_vec {
                acc_proc += 2;

                let mut last_rep_date = cash_flows.1.clone();
                if last_rep_date < *as_on_date {
                    last_rep_date = input_data.maturity_date;
                }
                let def_type = ("P".to_string(), "".to_string());
                let trimed_loan_id: &str = loan_id.trim_start_matches('0');
                let mut final_npa_typ = "P".to_string();
                let (npa_type, schm_code) =
                    tcfsla_map.get(trimed_loan_id.clone()).unwrap_or(&def_type);
                if tcfsla_map.contains_key(trimed_loan_id.clone())
                    && schm_code.to_uppercase() == "FINNONE"
                {
                    final_npa_typ = npa_type.to_string();
                }
                let def_writeoff_data = WriteOffMerged::default();
                let writeoff_data = writeoff_merged_map
                    .get(&loan_id)
                    .unwrap_or(&def_writeoff_data);
                let mut division = "NULL".to_string();
                if writeoff_merged_map.contains_key(&loan_id) {
                    division = writeoff_data.asset_class.clone();
                }
                let due_date = cash_flows.0.repayment_date;
                let mut principal_amt = cash_flows.0.principal_amt;
                let int_amt = cash_flows.0.intrest_amt;
                let mut component = "PRINCIPAL";

                writeln!(
                    op_writer,
                    "{}",
                    format_output(
                        input_data.clone(),
                        division.clone(),
                        loan_id.clone(),
                        due_dt,
                        component.to_string(),
                        principal_amt,
                        final_npa_typ.clone(),
                        pre_emi_ovd_amt,
                        config_params
                    )
                )
                .expect("Error in Writing Output");
                component = "INTEREST";
                principal_amt = int_amt;
                writeln!(
                    op_writer,
                    "{}",
                    format_output(
                        input_data.clone(),
                        division,
                        loan_id.clone(),
                        due_dt,
                        component.to_string(),
                        principal_amt,
                        final_npa_typ,
                        pre_emi_ovd_amt,
                        config_params
                    )
                )
                .expect("Error in Writing Output");
            }
        }
    }

    let health_report = HealthReport::new(acc_enc, acc_proc, 0, 0.0, 0.0, 0);
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file());
}
