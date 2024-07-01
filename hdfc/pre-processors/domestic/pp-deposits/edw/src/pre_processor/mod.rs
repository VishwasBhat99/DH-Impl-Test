use self::derive_fields::append_alm_balm_ia_line;
use self::derive_fields::append_as_on_date;
use self::derive_fields::append_currency;
use self::derive_fields::append_current_book_balance;
use self::derive_fields::append_division;
use self::derive_fields::append_gl_acc;
use self::derive_fields::append_int_rate;
use self::derive_fields::cost_center;
use self::recon::ReconKey;
use calamine::{open_workbook_auto, Reader, Sheets};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::NaiveDate;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;
use std::time::SystemTime;
mod derive_fields;
mod recon;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_ref_time = SystemTime::now();
    let mut ref_mis1: String = "".to_string();
    let input_file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(input_file) => input_file,
        Err(error) => panic!("{}", error),
    };
    let mut cust_ids: HashMap<String, String> = HashMap::new();
    if Path::new(config_param.cust_id_file()).exists() {
        let cust_id_file = match new_buf_rdr(config_param.cust_id_file()) {
            Ok(cust_id_file) => cust_id_file,
            Err(error) => panic!("{}", error),
        };
        let cust_id_reader = BufReader::new(cust_id_file);
        for line in cust_id_reader.lines() {
            let cust_id_info: String = match line {
                Ok(cust_id_info) => cust_id_info,
                Err(error) => {
                    panic!("Cannot read line from cust id file: {:?}", error);
                }
            };
            let mut fields: Vec<&str> = cust_id_info.split("|").collect();
            cust_ids.insert(fields[1].to_string(), fields[0].to_string());
        }
    }
    let mut ref_excel1 =
        open_workbook_auto(config_param.ref_file_path_1()).expect("Error while opening `R1` file.");
    check_sheet_name(config_param.ref_file_path_1().to_string(), &"Sheet1".to_string(), &ref_excel1);
    let mut ref_map1: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel1.worksheet_range("Sheet1") {
        for row in reader.rows() {
            let mut alm_concat = String::new();
            alm_concat.push_str(&row[4].to_string());
            alm_concat.push_str("_");
            alm_concat.push_str(&row[1].to_string());
            alm_concat.push_str("_");
            alm_concat.push_str(&row[5].to_string());
            ref_map1.insert(row[0].to_string(), alm_concat);
            ref_mis1 = row[2].to_string();
        }
    }
    let mut ref_excel2 = open_workbook_auto(config_param.ref_file_path_2())
        .expect("Error while opening `ALM Master File`.");
    let mut alm_llg: HashMap<String, String> = HashMap::new();
    let mut ia_llg: HashMap<String, String> = HashMap::new();
    let mut balm_llg: HashMap<String, String> = HashMap::new();
    check_sheet_name(
        config_param.ref_file_path_2().to_string(),
        &config_param.alm_master_sheet_name().to_string(),
        &ref_excel2,
    );
    if let Some(Ok(reader)) = ref_excel2.worksheet_range(config_param.alm_master_sheet_name()) {
        for row in reader.rows() {
            alm_llg.insert(row[0].to_string(), row[6].to_string());
            ia_llg.insert(row[0].to_string(), row[7].to_string());
            balm_llg.insert(row[0].to_string(), row[9].to_string());
        }
    }
    let mut ref_excel3 =
        open_workbook_auto(config_param.ref_file_path_3()).expect("Error while opening `R3`.");
    check_sheet_name(config_param.ref_file_path_3().to_string(), &"Sheet1".to_string(), &ref_excel3);
    let mut ref_map3: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel3.worksheet_range("Sheet1") {
        for row in reader.rows() {
            let cost_center = row[1].to_string();
            ref_map3.insert(row[0].to_string(), cost_center);
        }
    }
    let mut currency_codes: HashMap<String, String> = HashMap::new();
    let mut is_header: bool = true;
    let mut ref_excel4 =
        open_workbook_auto(config_param.ref_file_path_4()).expect("Error while opening `R4`.");
    check_sheet_name(config_param.ref_file_path_4().to_string(), &"Sheet1".to_string(), &ref_excel4);
    if let Some(Ok(reader)) = ref_excel4.worksheet_range("Sheet1") {
        for row in reader.rows() {
            if is_header {
                is_header = false;
                continue;
            }
            currency_codes.insert(row[0].to_string(), row[1].to_string());
        }
    }
    let mut ref_excel5 = open_workbook_auto(config_param.ref_file_path_5())
        .expect("Error while opening `MIS1_Desc.xlsx` file.");
    check_sheet_name(config_param.ref_file_path_5().to_string(), &"Sheet1".to_string(), &ref_excel5);
    let mut mis1_desc: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel5.worksheet_range("Sheet1") {
        for row in reader.rows() {
            mis1_desc.insert(row[0].to_string(), row[2].to_string());
        }
    }

    //total fd file
    let mut total_fd = open_workbook_auto(config_param.total_fd_file())
        .expect("Error while opening `TOTAL FD FILE`.");

    let sheet_name = total_fd
        .sheet_names()
        .first()
        .expect("excel is empty")
        .to_owned();
    check_sheet_name(config_param.total_fd_file().to_string(), &sheet_name.to_string(), &total_fd);
    let mut total_fd_data: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = total_fd.worksheet_range(&sheet_name) {
        for row in reader.rows() {
            total_fd_data.insert(
                row[8].to_string().trim().to_string(),
                row[8].to_string().trim().to_string(),
            );
        }
    }
    log_debug!(log, "Cannot get total fd data for {:?} ", total_fd_data);

    //FCNR Mater file
    let mut fcnr_file = open_workbook_auto(config_param.fcnr_master_file())
        .expect("Error while opening `FCNR MASTER FILE`.");
    let sheet_name = fcnr_file
        .sheet_names()
        .first()
        .expect("excel is empty")
        .to_owned();
    check_sheet_name(config_param.fcnr_master_file().to_string(), &sheet_name.to_string(), &fcnr_file);
    let mut fcnr_data: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = fcnr_file.worksheet_range(&sheet_name) {
        for row in reader.rows() {
            fcnr_data.insert(
                row[0].to_string().trim().to_string(),
                row[2].to_string().trim().to_string(),
            );
        }
    }
    log_debug!(log, "Cannot get fcnr data for {:?} ", fcnr_data);
    let end_read_ref_time = SystemTime::now();
    let total_duration = end_read_ref_time
        .duration_since(start_read_ref_time)
        .expect("Could not calculate total read reference duration.");
    info!(
        diag_log,
        "Reading Reference Total Duration: {:?}", total_duration
    );
    let start_process_time = SystemTime::now();
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{}", error),
    };
    let rec_output_file = match buf_file_wrtr(config_param.rec_output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{}", error),
    };
    let reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);
    let mut recon_writer = BufWriter::new(rec_output_file);
    let mut output_line = String::new();
    let mut recon_map: HashMap<ReconKey, f64> = HashMap::new();
    let mut tot_acc_encntrd = DEFAULT_INT;
    let mut skp_acc = DEFAULT_INT;
    let mut tot_amt = DEFAULT_FLOAT;
    let mut concats: Vec<String> = Vec::new();

    //Add header to output file.
    let header= "account_number|accrued_interest|deposit_type|maturity_date|rat_acct_int|rat_acct_int_var|next_compound_date|next_payment_date|account_start_date|currency_code: i64,|customer_id: i64,|original_balance|origination_date|previous_roll_over_date|description|client_name|tname|as_on_date|bank_num|branch|rate_flag|int_pay_freq: i64,|cost_centre_ftp|institution|new_gl|int_rate|concat|ia_llg|balm_llg|current_book_balance|cost_center|comp_freq: i64,|fin_cost_ftp|cust_category|cod_prod|com_mis_comp_1|rat_prod_var|dat_value_date|alm_concat|amt_initl_dep|bal_principle_lcy|bal_int_accr_lcy|lien_marked|prod_code|acc_open_date|gl_int_comp|division|lcr_classification_code|contr_days|res_days|bal_int_comp_lcy\n";
    output_line.push_str(&header);
    for line in reader.lines() {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
        let mut fields: Vec<&str> = acc_info.split("~#~").collect();

        tot_acc_encntrd += 1;

        if fields.len() != 38 && fields.len() != 39 {
            skp_acc += 1;
            continue;
        }

        if fields[1].parse::<f64>().is_err() {
            continue;
        }

        let bal_principal_lcy: &str = fields[7];
        let rat_acct_int: &str = fields[12];
        let rat_acct_int_var: &str = fields[13];
        let rat_prod_var: &str;
        if fields.len() == 38 {
            rat_prod_var = "0.0";
        } else {
            rat_prod_var = fields[38];
        }
        let bal_int_comp_lcy: &str = fields[25];
        let cod_gl_regular_dep: &str = fields[28];
        let mis1: &str = fields[37];
        let curr: &str = fields[17];
        let cost_cen_ftp: &str = fields[34];
        let comp_freq: &str = fields[30];
        let cip_gl: &str = fields[36];
        let cod_gl: &str = fields[28];

        fields[0] = fields[0].trim();
        output_line.push_str(fields[0]);
        output_line.push('|');
        output_line.push_str(fields[1]);
        output_line.push('|');
        output_line.push_str(fields[5]);
        output_line.push('|');

        let mat_dt = match NaiveDate::parse_from_str(fields[8], "%d-%b-%Y") {
            Ok(dt) => dt.format("%d-%m-%Y").to_string(),
            Err(_) => "".to_string(),
        };
        output_line.push_str(&mat_dt);
        output_line.push_str("|");

        output_line.push_str(fields[12]);
        output_line.push('|');
        output_line.push_str(fields[13]);
        output_line.push('|');

        let nxt_comp_dt = match NaiveDate::parse_from_str(fields[14], "%d-%b-%Y") {
            Ok(dt) => dt.format("%d-%m-%Y").to_string(),
            Err(_) => "".to_string(),
        };
        output_line.push_str(&nxt_comp_dt);
        output_line.push_str("|");

        let nxt_pay_dt = match NaiveDate::parse_from_str(fields[16], "%d-%b-%Y") {
            Ok(dt) => dt.format("%d-%m-%Y").to_string(),
            Err(_) => "".to_string(),
        };
        output_line.push_str(&nxt_pay_dt);
        output_line.push_str("|");

        let acc_st_dt = match NaiveDate::parse_from_str(fields[15], "%d-%b-%Y") {
            Ok(dt) => dt.format("%d-%m-%Y").to_string(),
            Err(_) => "".to_string(),
        };
        output_line.push_str(&acc_st_dt);
        output_line.push_str("|");

        output_line.push_str(fields[17]);
        output_line.push('|');
        output_line.push_str(fields[18]);
        output_line.push('|');
        output_line.push_str(fields[19]);
        output_line.push('|');

        let org_dt = match NaiveDate::parse_from_str(fields[23], "%d-%b-%Y") {
            Ok(dt) => dt.format("%d-%m-%Y").to_string(),
            Err(_) => "".to_string(),
        };
        output_line.push_str(&org_dt);
        output_line.push_str("|");
        let prev_roll_over_dt = match NaiveDate::parse_from_str(fields[23], "%d-%b-%Y") {
            Ok(dt) => dt.format("%d-%m-%Y").to_string(),
            Err(_) => "".to_string(),
        };
        output_line.push_str(&prev_roll_over_dt);
        output_line.push_str("|");

        output_line.push_str(fields[27]);
        output_line.push('|');
        output_line.push_str(fields[31]);
        output_line.push_str("|FWTD|");
        append_as_on_date(&mut output_line, config_param.as_on_date);
        output_line.push_str("000|");
        output_line.push_str(fields[4]);
        output_line.push_str("|F|");
        output_line.push_str(fields[11]);
        output_line.push('|');
        output_line.push_str(fields[34]);
        output_line.push('|');
        let currency = append_currency(&mut output_line, curr, &mut currency_codes);

        let cip_recon_key = ReconKey::new(
            currency.to_string(),
            "CIPGL".to_string(),
            cip_gl.to_string(),
        );
        let cod_recon_key = ReconKey::new(currency, "PRDGL".to_string(), cod_gl.to_string());
        append_gl_acc(&mut output_line, cod_gl_regular_dep);
        append_int_rate(
            &mut output_line,
            rat_acct_int,
            rat_acct_int_var,
            rat_prod_var,
        );
        let cost_center = cost_center(&ref_map3, cod_gl_regular_dep, cost_cen_ftp);
        let gl = cod_gl.to_string() + "#" + &cip_gl.to_string();
        concats.push(append_alm_balm_ia_line(
            &mut output_line,
            &ref_map1,
            &alm_llg,
            &ia_llg,
            &balm_llg,
            cod_gl_regular_dep,
            &cost_center,
            mis1,
            &ref_mis1,
            log,
            fields[0],
            &gl,
        ));
        tot_amt += append_current_book_balance(
            &mut output_line,
            bal_principal_lcy,
            bal_int_comp_lcy,
            cip_recon_key,
            cod_recon_key,
            &mut recon_map,
        );
        output_line.push_str(&cost_center);
        output_line.push('|');
        output_line.push_str(&comp_freq);
        output_line.push('|');
        if mis1.is_empty() {
            output_line.push_str("999");
        } else {
            output_line.push_str(mis1);
        }
        output_line.push('|');
        output_line.push_str("NA"); //Cust category, to match with overseas output
        output_line.push('|');
        output_line.push_str(fields[5]);
        output_line.push('|');
        output_line.push_str(fields[37]);
        output_line.push('|');
        output_line.push_str(fields[38]);
        output_line.push('|');
        let val_date = match NaiveDate::parse_from_str(fields[23], "%d-%b-%Y") {
            Ok(dt) => dt.format("%d-%m-%Y").to_string(),
            Err(_) => "".to_string(),
        };
        output_line.push_str(&val_date);
        output_line.push('|');
        output_line.push_str(&ref_map1.get(fields[28]).unwrap_or(&"NA".to_string()));
        output_line.push('|');
        output_line.push_str(fields[19]);
        output_line.push('|');
        output_line.push_str(fields[7]);
        output_line.push('|');
        output_line.push_str(fields[3]);
        output_line.push('|');
        if total_fd_data
            .get(&fields[0].trim().to_string())
            .unwrap_or(&"NA".to_string())
            == &"NA".to_string()
        {
            output_line.push_str(&"NA".to_string());
        } else {
            output_line.push_str(&"Lien Marked".to_string());
        }
        output_line.push('|');
        output_line.push_str(&fcnr_data.get(fields[5].trim()).unwrap_or(&"NA".to_string()));
        output_line.push('|');
        output_line.push_str(&config_param.as_on_date().format("%d-%m-%Y").to_string()); //account open date, to match with overseas output
        output_line.push('|');
        let gl_int_comp: &str = fields[36];
        output_line.push_str(gl_int_comp);
        output_line.push('|');
        append_division(&mut output_line, &mis1, &mut mis1_desc);
        output_line.push('|');
        output_line.push_str(cust_ids.get(fields[18]).unwrap_or(&"".to_string()));
        output_line.push('|');
        let contr_mat = rbdate::num_days_start_to_end(
            NaiveDate::parse_from_str(&val_date, "%d-%m-%Y").expect("Could  not find value date"),
            NaiveDate::parse_from_str(&mat_dt, "%d-%m-%Y").expect("Could  not find maturity date"),
        );
        output_line.push_str(&contr_mat.to_string());
        output_line.push('|');
        let resi_mat = rbdate::num_days_start_to_end(
            *config_param.as_on_date(),
            NaiveDate::parse_from_str(&mat_dt, "%d-%m-%Y").expect("Could  not find maturity date"),
        );
        output_line.push_str(&resi_mat.to_string());
        output_line.push('|');
        output_line.push_str(fields[25]);
        output_line.push_str("\n");
    }
    let end_process_time = SystemTime::now();
    let duration = end_process_time
        .duration_since(start_process_time)
        .expect("Could not calculate total process duration.");
    info!(diag_log, "Process Total Duration: {:?}.", duration);
    let start_writer_time = SystemTime::now();
    let mut recon_output_line = String::new();
    for (key, value) in recon_map {
        let op = format!(
            "{}|{}|{}|{}|{}|{}",
            config_param.as_on_date().format("%d-%m-%Y"),
            "TD353",
            key.gl_type,
            key.gl_code,
            value,
            key.currency,
        );
        recon_output_line.push_str(&op[..]);
        recon_output_line.push_str("\n");
    }
    match recon_writer.write_all(recon_output_line.as_bytes()) {
        Ok(val) => val,
        Err(error) => {
            panic!("Error writing reconciliation report: {:?}", error);
        }
    }
    match writer.write_all(output_line.as_bytes()) {
        Ok(val) => val,
        Err(error) => {
            panic!("Error writing processed data: {:?}", error);
        }
    }

    let mut concat_lines = String::new();
    let mut concat_writer = match buf_file_wrtr(config_param.concat_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create concat file: `{}` on location `{}` : {}",
            config_param.concat_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    concats.sort();
    concats.dedup();
    for concat in concats.drain(..) {
        concat_lines.push_str(&concat);
        concat_lines.push('\n');
    }
    match concat_writer.write_all(concat_lines.as_bytes()) {
        Ok(_) => println!("Successfully written concats for missing alm lines."),
        Err(error) => panic!(
            "Unable to write concat lines to the file `{}`: {}.",
            config_param.concat_file_path(),
            error,
        ),
    }
    let end_writer_time = SystemTime::now();
    let duration = end_writer_time
        .duration_since(start_writer_time)
        .expect("Could not calculate total write process duration.");
    info!(diag_log, "Write Process Total Duration: {:?}.", duration);

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skp_acc,
        skp_acc,
        tot_amt,
        tot_amt,
        DEFAULT_INT,
    );
    health_report.gen_health_rpt(&config_param.output_file_path());
}
fn check_sheet_name(file_name: String, sheet_name: &String, excel_sheets: &Sheets) {
    if !excel_sheets.sheet_names().contains(&sheet_name.to_string()) {
        panic!(
            "sheet name {} is not present in {} : Available sheet names :{:?}",
            sheet_name,
            file_name,
            excel_sheets.sheet_names()
        )
    }
}
