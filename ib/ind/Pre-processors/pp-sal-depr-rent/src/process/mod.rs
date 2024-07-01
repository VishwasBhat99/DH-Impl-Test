use crate::configuration_parameters::ConfigurationParameters;
use crate::process::account::*;
use calamine::{open_workbook_auto, Reader};
use health_report::HealthReport;
use slog::Logger;
use std::collections::HashMap;
use std::{fs, io::Write};

mod account;
mod config;

pub fn process(config_params: &ConfigurationParameters, _logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_proc = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;

    let files_config = config::get_files(config_params.config_file_path());
    let default_tuple = ("NA".to_string(), "NA".to_string());

    //Read AL PL Mapper File
    let mut al_pl_master_file = open_workbook_auto(&files_config.al_pl_file)
        .expect("Unable to open `al_pl_master_input_file.xlsx`.");
    println!(
        "Sheets present in AL-PL-Master-File: `{:?}`",
        al_pl_master_file.sheet_names()
    );
    if !al_pl_master_file
        .sheet_names()
        .contains(&files_config.al_pl_sheet_name.to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in AL-PL-Master-File: `{}`",
            files_config.al_pl_sheet_name, files_config.al_pl_file,
        );
    }
    println!(
        "Reading Sheet: `{}` from AL-PL-Master-File",
        files_config.al_pl_sheet_name,
    );
    let mut ogl_code_desc_mapper: HashMap<String, (String, String)> = HashMap::new();
    if let Some(Ok(reader)) = al_pl_master_file.worksheet_range(&files_config.al_pl_sheet_name) {
        for row in reader.rows().skip(1) {
            ogl_code_desc_mapper.insert(
                row[0].to_string().trim().to_string(),
                (
                    row[4].to_string().trim().to_string(),
                    row[5].to_string().trim().to_string(),
                ),
            );
        }
    }

    //Read SolDim File
    let soldim_input_reader = fs::read_to_string(&files_config.soldim_input_file)
        .expect("Could Not Read Soldim Input File");
    let mut br_zone_mapper: HashMap<String, String> = HashMap::new();
    let mut zone_list: Vec<String> = Vec::new();
    for line in soldim_input_reader.lines().skip(1) {
        let vec_soldim = line.split('|').collect::<Vec<&str>>();
        let mut soldim_input = SolDimData::def();
        soldim_input.sollineid = vec_soldim[0].to_string();
        soldim_input.hl_ro = vec_soldim[9].trim().to_string();
        br_zone_mapper.insert(
            soldim_input.sollineid.trim().to_string(),
            soldim_input.hl_ro,
        );
        if *vec_soldim[2].trim() == files_config.zone_code {
            zone_list.push(soldim_input.sollineid.trim().to_string());
        }
    }

    let mut op_writer = get_writer(config_params.output_file());
    writeln!(
        op_writer,
        "BookDt|GLCode|GLDesc|SolLineID|DivLineID|PrdLineID|AdLineid1|AdLineID2|Cycle|CCY|Cost Amount HCY"
    ).expect("Error Writing Header into Output File");

    //Reading OGL File
    let ogl_reader = fs::read_to_string(&files_config.ogl_file).expect("Could Not Read OGL File");
    let mut sal_ogl_sum = 0.0;
    let mut depr_ogl_sum = 0.0;
    let mut ogl_rent_mapper: HashMap<String, f64> = HashMap::new();
    let mut br_ogl_mapper: HashMap<String, String> = HashMap::new();
    for (line_no, line) in ogl_reader.lines().enumerate().skip(1) {
        acc_enc += 1;
        let ogl_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let ogl_data = Account::new_from_txt(&ogl_vec);
        let book_date = rbdate::NaiveDate::parse_from_str(&ogl_data.bookdt, "%d-%m-%Y")
            .unwrap_or(*config_params.as_on_date());
        if book_date != *config_params.as_on_date() {
            panic!(
                "Incorrect Date: `{}` found in `{:?}` File\nError at Row: `{}`\nExpected Run-Date: `{}`",
                book_date,
                files_config.ogl_file,
                line_no+1,
                config_params.as_on_date()
            );
        }
        ip_amt += ogl_data.cost_amount_hcy;
        if ogl_vec[3].to_string().to_uppercase() == files_config.sal_code.to_uppercase()
            && ogl_code_desc_mapper
                .get(&ogl_data.gl_code.to_string())
                .unwrap_or(&("NA".to_string(), "NA".to_string()))
                .0
                == files_config.sal_desc.to_uppercase()
        {
            sal_ogl_sum += ogl_vec[10].to_string().parse::<f64>().unwrap();
        }
        if ogl_vec[3].to_string().to_uppercase() == files_config.depr_code.to_uppercase()
            && ogl_code_desc_mapper
                .get(&ogl_data.gl_code.to_string())
                .unwrap_or(&("NA".to_string(), "NA".to_string()))
                .0
                == files_config.depr_desc.to_uppercase()
        {
            depr_ogl_sum += ogl_vec[10].to_string().parse::<f64>().unwrap();
        }
        if ogl_code_desc_mapper
            .get(&ogl_data.gl_code.to_string())
            .unwrap_or(&("NA".to_string(), "NA".to_string()))
            .0
            == files_config.rent_desc.to_uppercase()
        {
            ogl_rent_mapper
                .entry(ogl_vec[3].to_string())
                .and_modify(|data| *data += ogl_vec[10].to_string().parse::<f64>().unwrap())
                .or_insert(ogl_vec[10].to_string().parse::<f64>().unwrap());
        }
        let output_data: Account = get_op_data(&ogl_data, config_params, ogl_data.cost_amount_hcy);
        let output_str = format_output(
            &output_data,
            output_data.cost_amount_hcy,
            &ogl_code_desc_mapper,
            output_data.gl_code.to_string(),
        );
        br_ogl_mapper.insert(output_data.sol_lineid, output_data.gl_code);
        acc_proc += 1;
        op_amt += output_data.cost_amount_hcy;
        op_writer
            .write_all(output_str.as_bytes())
            .expect("Error writing depriciation accounts to output file");
    }

    //Read Salary File
    let mut sal_file = open_workbook_auto(&files_config.sal_input_file)
        .expect("Unable to open `sal_input_master_file.xlsx`.");
    let mut sal_total_sum = 0.0;
    let mut sal_output: Account = ::std::default::Default::default();
    println!(
        "Sheets present in Salary-Input-Master-File: `{:?}`",
        sal_file.sheet_names()
    );
    if !sal_file
        .sheet_names()
        .contains(&files_config.sal_sheet_name.to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in Salary-Input-Master-File: `{}`",
            files_config.sal_sheet_name, files_config.sal_input_file,
        );
    }
    println!(
        "Reading Sheet: `{}` from Salary-Input-Master-File",
        files_config.sal_sheet_name,
    );
    if let Some(Ok(sal_reader)) = sal_file.worksheet_range(&files_config.sal_sheet_name) {
        for (row_no, row) in sal_reader.rows().enumerate().skip(1) {
            acc_enc += 1;
            let salary_master = Account::new_from_xlsx(row);
            let book_date = rbdate::datevalue_to_naive_date(&salary_master.bookdt)
                .unwrap_or(*config_params.as_on_date());
            if book_date != *config_params.as_on_date() {
                panic!(
                    "Incorrect Date: `{}` found in `{:?}` File\nError in Row: `{}` from Sheet: `{}`\nExpected Run-Date: `{}`",
                    book_date,
                    files_config.sal_input_file,
                    row_no+1,
                    files_config.sal_sheet_name,
                    config_params.as_on_date()
                );
            }
            ip_amt += salary_master.cost_amount_hcy;
            if salary_master.sol_lineid.to_uppercase() == files_config.sal_code.to_uppercase() {
                sal_output =
                    get_op_data(&salary_master, config_params, salary_master.cost_amount_hcy);
            } else {
                let sal_output_data: Account =
                    get_op_data(&salary_master, config_params, salary_master.cost_amount_hcy);
                let sal_exclude_str = format_output(
                    &sal_output_data,
                    salary_master.cost_amount_hcy,
                    &ogl_code_desc_mapper,
                    sal_output.sol_lineid.to_string(),
                );
                acc_proc += 1;
                op_amt += salary_master.cost_amount_hcy;
                op_writer
                    .write_all(sal_exclude_str.as_bytes())
                    .expect("Error writing salary accounts to output file");
                sal_total_sum += salary_master.cost_amount_hcy;
            }
        }
    }

    let sal_str = format_output(
        &sal_output,
        sal_ogl_sum - sal_total_sum,
        &ogl_code_desc_mapper,
        sal_output.sol_lineid.to_string(),
    );
    op_writer
        .write_all(sal_str.as_bytes())
        .expect("Error writing salary accounts to output file");

    //Read Depriciation File
    let mut depr_file = open_workbook_auto(&files_config.depr_input_file)
        .expect("Unable to open `depr_input_master_file.xlsx`.");
    let mut depr_total_sum = 0.0;
    let mut depr_output: Account = ::std::default::Default::default();
    println!(
        "Sheets present in Depr-Input-Master-File: `{:?}`",
        depr_file.sheet_names()
    );
    if !depr_file
        .sheet_names()
        .contains(&files_config.depr_sheet_name.to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in Depr-Input-Master-File: `{}`",
            files_config.depr_sheet_name, files_config.depr_input_file,
        );
    }
    println!(
        "Reading Sheet: `{}` from Depr-Input-Master-File",
        files_config.depr_sheet_name,
    );
    if let Some(Ok(depr_reader)) = depr_file.worksheet_range(&files_config.depr_sheet_name) {
        for (row_no, row) in depr_reader.rows().enumerate().skip(1) {
            acc_enc += 1;
            let depr_master = Account::new_from_xlsx(row);
            let book_date = rbdate::datevalue_to_naive_date(&depr_master.bookdt)
                .unwrap_or(*config_params.as_on_date());
            if book_date != *config_params.as_on_date() {
                panic!(
                    "Incorrect Date: `{}` found in `{:?}` File\nError in Row: `{}` from Sheet: `{}`\nExpected Run-Date: `{}`",
                    book_date,
                    files_config.depr_input_file,
                    row_no+1,
                    files_config.depr_sheet_name,
                    config_params.as_on_date()
                );
            }
            ip_amt += depr_master.cost_amount_hcy;
            if depr_master.sol_lineid.to_uppercase() == files_config.depr_code.to_uppercase() {
                depr_output = get_op_data(&depr_master, config_params, depr_master.cost_amount_hcy);
            } else {
                let depr_output_data: Account =
                    get_op_data(&depr_master, config_params, depr_master.cost_amount_hcy);
                let depr_exclude_str = format_output(
                    &depr_output_data,
                    depr_master.cost_amount_hcy,
                    &ogl_code_desc_mapper,
                    depr_output.sol_lineid.to_string(),
                );
                acc_proc += 1;
                op_amt += depr_master.cost_amount_hcy;
                op_writer
                    .write_all(depr_exclude_str.as_bytes())
                    .expect("Error writing depriciation accounts to output file");
                depr_total_sum += depr_master.cost_amount_hcy;
            }
        }
    }
    let depr_str = format_output(
        &depr_output,
        depr_ogl_sum - depr_total_sum,
        &ogl_code_desc_mapper,
        depr_output.sol_lineid.to_string(),
    );
    op_writer
        .write_all(depr_str.as_bytes())
        .expect("Error writing depriciation accounts to output file");

    //Read Rent File and store in Rent Master Hashmap
    let mut rent_mapper: HashMap<String, Account> = HashMap::new();
    let mut rent_file = open_workbook_auto(&files_config.rent_input_file)
        .expect("Unable to open `rent_input_master_file.xlsx`.");
    println!(
        "Sheets present in Rent-Input-Master-File: `{:?}`",
        rent_file.sheet_names()
    );
    if !rent_file
        .sheet_names()
        .contains(&files_config.rent_sheet_name.to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in Rent-Input-Master-File: `{}`",
            files_config.rent_sheet_name, files_config.rent_input_file,
        );
    }
    println!(
        "Reading Sheet: `{}` from Rent-Input-Master-File",
        files_config.rent_sheet_name,
    );
    if let Some(Ok(rent_reader)) = rent_file.worksheet_range(&files_config.rent_sheet_name) {
        for (row_no, row) in rent_reader.rows().enumerate().skip(1) {
            acc_enc += 1;
            let rent_master = Account::new_from_xlsx(row);
            let book_date = rbdate::datevalue_to_naive_date(&rent_master.bookdt)
                .unwrap_or(*config_params.as_on_date());
            if book_date != *config_params.as_on_date() {
                panic!(
                    "Incorrect Date: `{}` found in `{:?}` File\nError in Row: `{}` from Sheet: `{}`\nExpected Run-Date: `{}`",
                    book_date,
                    files_config.rent_input_file,
                    row_no+1,
                    files_config.rent_sheet_name,
                    config_params.as_on_date()
                );
            }
            ip_amt += rent_master.cost_amount_hcy;
            let zone = br_zone_mapper
                .get(&row[3].to_string())
                .unwrap_or(&default_tuple.0);

            if !zone_list.contains(&row[3].to_string()) {
                rent_mapper
                    .entry(zone.to_string())
                    .and_modify(|data| {
                        data.cost_amount_hcy += rent_master.cost_amount_hcy;
                    })
                    .or_insert(rent_master.clone());
            }
            if zone_list.contains(&row[3].to_string()) {
                continue;
            }
            let rent_output_data: Account =
                get_op_data(&rent_master, config_params, rent_master.cost_amount_hcy);
            let rent_str = format_output(
                &rent_output_data,
                rent_master.cost_amount_hcy,
                &ogl_code_desc_mapper,
                rent_output_data.sol_lineid.to_string(),
            );
            acc_proc += 1;
            op_amt += rent_master.cost_amount_hcy;
            op_writer
                .write_all(rent_str.as_bytes())
                .expect("Error writing rent accounts to output file");
        }
    }

    //Rent File
    for (sollineid, rent_master) in rent_mapper {
        let rent_from_ogl = ogl_rent_mapper.get(&sollineid).unwrap_or(&0.0);
        let rent_sum = rent_master.cost_amount_hcy;
        let cost_amount_hcy = rent_from_ogl - rent_sum;
        let mut rent_output_data: Account =
            get_op_data(&rent_master, config_params, cost_amount_hcy);
        rent_output_data.sol_lineid = sollineid;
        let rent_str = format_output(
            &rent_output_data,
            cost_amount_hcy,
            &ogl_code_desc_mapper,
            rent_output_data.sol_lineid.to_string(),
        );
        op_amt += cost_amount_hcy;
        op_writer
            .write_all(rent_str.as_bytes())
            .expect("Error writing Rent accounts to output file");
    }

    let health_report = HealthReport::new(acc_enc, acc_proc, acc_enc - acc_proc, ip_amt, op_amt, 0);
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file());
}
