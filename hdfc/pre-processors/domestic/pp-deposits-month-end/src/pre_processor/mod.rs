use self::derive_fields::append_alm_balm_ia_line;
use self::derive_fields::append_as_on_date;
use self::manual_handler::remove_comma;
use self::recon::ReconKey;
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::NaiveDate;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use simple_csv::SimpleCsvReader;
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::{prelude::*, BufWriter};
use std::time::SystemTime;
mod derive_fields;
mod manual_handler;
mod recon;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_ref_time = SystemTime::now();
    let mut ref_mis1: String = "".to_string();
    let input_file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(input_file) => input_file,
        Err(error) => panic!("{}", error),
    };

    let mut ref_excel1: Xlsx<_> =
        open_workbook(config_param.ref_file_path_1()).expect("Error while opening `R1` file.");
    let mut ref_map1: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel1.worksheet_range(config_param.r1_sheet_name()) {
        for row in reader.rows() {
            let mut alm_concat = String::new();
            alm_concat.push_str(&row[4].to_string());
            alm_concat.push_str("_");
            alm_concat.push_str(&row[1].to_string());
            alm_concat.push_str("_");
            alm_concat.push_str(&row[5].to_string());
            ref_map1.insert(row[1].to_string(), alm_concat);
            ref_mis1 = row[2].to_string();
        }
    }
    let mut ref_excel2: Xlsx<_> = open_workbook(config_param.ref_file_path_2())
        .expect("Error while opening `ALM Mater File`.");
    let mut alm_llg: HashMap<String, String> = HashMap::new();
    let mut ia_llg: HashMap<String, String> = HashMap::new();
    let mut balm_llg: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel2.worksheet_range(config_param.alm_master_sheet_name()) {
        for row in reader.rows() {
            alm_llg.insert(row[0].to_string(), row[6].to_string());
            ia_llg.insert(row[0].to_string(), row[7].to_string());
            balm_llg.insert(row[0].to_string(), row[9].to_string());
        }
    }
    let mut ref_excel3: Xlsx<_> =
        open_workbook(config_param.ref_file_path_3()).expect("Error while opening `R3`.");
    let mut ref_map3: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel3.worksheet_range("Sheet1") {
        for row in reader.rows() {
            let cost_center = row[1].to_string();
            ref_map3.insert(row[0].to_string(), cost_center);
        }
    }

    let mut ref_excel4: Xlsx<_> = open_workbook(config_param.ref_file_path_4())
        .expect("Error while opening `Ora_GL_Master.xlsx ` file.");
    //Map with alm values to be used in output file.
    let mut alm_map: HashMap<String, String> = HashMap::new();
    let mut mis1_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel4.worksheet_range("Sheet1") {
        for row in reader.rows() {
            mis1_map.insert(row[0].to_string(), row[2].to_string());
            let mut alm_concat = String::new();
            alm_concat.push_str(&row[2].to_string());
            alm_concat.push_str("_");
            alm_concat.push_str(&row[4].to_string());
            alm_concat.push_str("_");
            alm_concat.push_str(&row[1].to_string());
            alm_concat.push_str("_");
            alm_concat.push_str(&row[5].to_string());
            alm_map.insert(row[0].to_string(), alm_concat);
        }
    }

    let mut ref_excel5: Xlsx<_> = open_workbook(config_param.ref_file_path_5())
        .expect("Error while opening `MIS1_Desc.xlsx` file.");
    let mut div_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel5.worksheet_range("Sheet1") {
        for row in reader.rows() {
            div_map.insert(row[0].to_string(), row[2].to_string());
        }
    }

    let end_read_ref_time = SystemTime::now();
    let total_duration = end_read_ref_time
        .duration_since(start_read_ref_time)
        .expect("Could not calculate total read reference duration.");
    debug!(
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
    let mut writer = BufWriter::new(output_file);
    let mut recon_writer = BufWriter::new(rec_output_file);
    let mut output_line = String::new();
    let mut recon_map: HashMap<ReconKey, f64> = HashMap::new();
    let mut tot_acc = DEFAULT_INT;
    let mut skp_acc = DEFAULT_INT;
    let mut concats: Vec<String> = Vec::new();
    //Add header to output file.
    let header="account_number|accrued_interest|deposit_type|maturity_date|rat_acct_int|rat_acct_int_var|next_compound_date|next_payment_date|account_start_date|currency_code: i64,|customer_id: i64,|original_balance|origination_date|previous_roll_over_date|description|client_name|tname|as_on_date|bank_num|branch|rate_flag|int_pay_freq: i64,|cost_centre_ftp|institution|new_gl|int_rate|concat|ia_llg|balm_llg|current_book_balance|cost_center|comp_freq: i64,|fin_cost_ftp|cust_category|cod_prod|com_mis_comp_1|rat_prod_var|dat_value_date|alm_concat|amt_initl_dep|bal_principle_lcy|bal_int_accr_lcy|lien_marked|prod_code|acc_open_date|gl_int_comp|division\n";
    output_line.push_str(&header);
    for (line_num, lines) in SimpleCsvReader::new(input_file).enumerate() {
        let fields = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_file_path(),
                line_num + 1,
                error
            ),
        };
        tot_acc += 1;

        if fields.len() != 22 {
            skp_acc += 1;
            continue;
        }

        if fields[1].parse::<i64>().is_err() {
            skp_acc += 1;
            continue;
        }

        let mis1: &str = "";

        output_line.push_str(&fields[2][..]);
        output_line.push_str("||");
        output_line.push_str(&fields[1][..]);
        output_line.push('|');

        let mat_dt = match NaiveDate::parse_from_str(&fields[6][..], "%d-%b-%y") {
            Ok(dt) => dt.format("%d-%m-%Y").to_string(),
            Err(_) => "".to_string(),
        };
        output_line.push_str(&mat_dt);
        output_line.push_str("|");

        output_line.push_str(&fields[8][..]);
        output_line.push('|');
        output_line.push_str(&fields[9][..]);
        output_line.push_str("|||");

        let acc_st_dt = match NaiveDate::parse_from_str(&fields[5][..], "%d-%b-%y") {
            Ok(dt) => dt.format("%d-%m-%Y").to_string(),
            Err(_) => "".to_string(),
        };
        output_line.push_str(&acc_st_dt);
        output_line.push_str("|1|");

        output_line.push_str(&fields[12][..]);
        output_line.push_str("||");
        output_line.push_str(&acc_st_dt);
        output_line.push('|');
        output_line.push_str(&acc_st_dt);
        output_line.push_str("||");
        output_line.push_str(&fields[13][..]);
        output_line.push_str("|FWTD|");
        append_as_on_date(&mut output_line, config_param.as_on_date);
        output_line.push_str("000|");
        output_line.push_str(&fields[0][..]);
        output_line.push_str("|F|0||");

        let ccy = "INR".to_string();
        output_line.push_str(&ccy);
        output_line.push('|');

        let gl = fields[20].to_string();
        output_line.push_str(&gl);
        output_line.push('|');

        output_line.push_str(&fields[15]);
        output_line.push('|');

        let recon_key = ReconKey::new(ccy, "TDMthEnd".to_string(), gl.to_string());

        let cost_center = "";
        concats.push(append_alm_balm_ia_line(
            &mut output_line,
            &ref_map1,
            &alm_llg,
            &ia_llg,
            &balm_llg,
            &gl,
            &cost_center,
            mis1,
            &ref_mis1,
            log,
            &fields[2][..],
        ));

        let book_bal: f64 = remove_comma(&fields[10]).parse().unwrap_or(DEFAULT_FLOAT);
        recon_map
            .entry(recon_key)
            .and_modify(|val| *val += book_bal)
            .or_insert(book_bal);

        output_line.push_str(&book_bal.to_string());
        output_line.push('|');
        output_line.push_str(&cost_center);
        output_line.push_str("|0|||");
        output_line.push_str(&fields[1]);
        output_line.push('|');
        let com_mis_comp_1 = match mis1_map.get(&fields[19]) {
            Some(val) => val,
            None => {
                log_error!(
                    log,
                    "Could not get mis1 value for source gl: {}",
                    &fields[19]
                );
                "NA"
            }
        };
        output_line.push_str(com_mis_comp_1);
        output_line.push_str("||");
        output_line.push_str(&fields[5]);
        output_line.push('|');
        let alm_concat = match alm_map.get(&fields[19]) {
            Some(val) => val,
            None => {
                log_error!(
                    log,
                    "Could not get alm_concat value for source gl: {}",
                    &fields[19]
                );
                "NA"
            }
        };
        output_line.push_str(&alm_concat);
        output_line.push('|');
        output_line.push_str(&fields[10]);
        output_line.push_str("||");
        output_line.push_str(&fields[18]);
        output_line.push_str("|||");
        output_line.push_str(&acc_st_dt);
        output_line.push_str("||");
        let division = match div_map.get(com_mis_comp_1) {
            Some(val) => val,
            None => {
                log_error!(
                    log,
                    "Cannot get division for mis1 value: {}",
                    &com_mis_comp_1
                );
                "NA"
            }
        };
        output_line.push_str(division);
        output_line.push('\n');
    }
    let end_process_time = SystemTime::now();
    let duration = end_process_time
        .duration_since(start_process_time)
        .expect("Could not calculate total process duration.");
    debug!(diag_log, "Process Total Duration: {:?}.", duration);
    let start_writer_time = SystemTime::now();
    let mut recon_output_line = String::new();
    for (key, value) in recon_map {
        let op = format!(
            "{}|{}|{}|{}|{}|{}",
            config_param.as_on_date().format("%d-%m-%Y"),
            "TD353-Month-End",
            key.gl_type,
            key.gl_code,
            value,
            key.currency,
        );
        recon_output_line.push_str(&op[..]);
        recon_output_line.push('\n');
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
    debug!(diag_log, "Write Process Total Duration: {:?}.", duration);

    let report_string = format!(
        "Accounts encountered: {}\n\
         Accounts proccessed suceessfully: {}\n\
         Accounts failed to process: {}",
        tot_acc,
        tot_acc - skp_acc,
        skp_acc,
    );
    info!(log, "{}", report_string);
    println!("{}", report_string);
}
