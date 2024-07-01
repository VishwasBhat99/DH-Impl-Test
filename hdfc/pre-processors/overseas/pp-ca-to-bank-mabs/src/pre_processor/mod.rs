use self::derive_casa_fields::get_casa_op_line;
use self::reconcilation::ReconKey;
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use statics::DEFAULT_INT;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;

mod derive_casa_fields;
mod reconcilation;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let mut ref_excel1: Xlsx<_> =
        open_workbook(config_param.ref_file_path_1()).expect("Unable to open `Ora_PROD.xlsx`.");
    let mut t_ora_prod: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel1.worksheet_range("Sheet1") {
        for row in reader.rows() {
            t_ora_prod.insert(row[1].to_string(), row[0].to_string());
        }
    }

    let mut ref_excel2: Xlsx<_> = open_workbook(config_param.ref_file_path_2())
        .expect("Error while opening `Ora_GL.xlsx` file.");
    let mut t_ora_gl: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel2.worksheet_range("Sheet1") {
        for row in reader.rows() {
            t_ora_gl.insert(row[0].to_string(), row[1].to_string());
        }
    }

    let mut ref_excel3: Xlsx<_> = open_workbook(config_param.ref_file_path_3())
        .expect("Error while opening `ALM_Line_Master.xlsx` file.");
    let mut alm_line: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel3.worksheet_range(config_param.alm_master_sheet_name()) {
        for row in reader.rows() {
            alm_line.insert(row[1].to_string(), row[6].to_string());
        }
    }

    let mut ref_excel4: Xlsx<_> = open_workbook(config_param.ref_file_path_4())
        .expect("Error while opening `ALM_COA_Master.xlsx` file.");
    let mut t_bdp_coa: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel4.worksheet_range("Sheet1") {
        for row in reader.rows() {
            t_bdp_coa.insert(row[0].to_string(), row[5].to_string());
        }
    }

    let mut ref_excel5: Xlsx<_> = open_workbook(config_param.ref_file_path_5())
        .expect("Error while opening `MIS1_Desc.xlsx` file.");
    let mut div: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel5.worksheet_range("Sheet1") {
        for row in reader.rows() {
            div.insert(row[1].to_string(), row[2].to_string());
        }
    }

    let inp_file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(inp_file) => inp_file,
        Err(error) => panic!(
            "Could not found inp file: `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let end_read_timer = SystemTime::now();
    let duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration read timer.");
    debug!(
        diag_log,
        "Reading Reference Files, Total Duration: {:?}.", duration
    );

    let start_derive_timer = SystemTime::now();
    let mut counter = 1;
    let mut op_line_td: String = String::new();
    let mut op_line_casa: String = String::new();
    //adding header to output
    let header = "casa_acc_no|casa_prod_cd|acc_stats|acc_br_cd|book_bal|avail_bal|flex_cube_cust_id|tot_od_lmt|acc_open_dt|cust_shrt_name|asset_bal_gl|liability_bal_gl|int_acrd_base_cd|cbr_num_1|cbr_num_2|cbr_num_3|cr_rt|dr_rt|act_typ|prod_name|int_rt|component|rt_flg|inst|crnt_book_bal|acrl_basis|div|alm_line|ia_llg|balm_llg|int_index_cd|int_index_name|od_variance|npa_flg|gl|cust_cat\n";
    op_line_casa.push_str(&header);
    let mut recon_casa: HashMap<ReconKey, f64> = HashMap::new();
    let mut recon_td: HashMap<ReconKey, f64> = HashMap::new();
    let mut tot_casa_acc: i64 = DEFAULT_INT;
    let mut tot_td_acc: i64 = DEFAULT_INT;
    let mut tot_amt: f64 = 0.0;
    let mut skip_acc: i64 = 0;

    for (line_num, lines) in inp_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_file_path(),
                line_num + 1,
                error
            ),
        };

        let mut row: Vec<&str> = line.split('|').collect();

        if counter < 3 {
            counter += 1;
            if row[3].to_string().parse::<i64>().is_err() {
                skip_acc += 1;
                continue;
            }
        }
        if row[0].to_string().is_empty() {
            skip_acc += 1;
            continue;
        }
        if row[8].to_string().is_empty() {
            op_line_casa.push_str(&get_casa_op_line(
                &mut row,
                &mut t_ora_prod,
                &mut t_ora_gl,
                &mut t_bdp_coa,
                &mut div,
                &mut alm_line,
                &log,
            ));

            let recon_key = ReconKey::new(
                row[6].to_string(),
                config_param.gl_type().to_string(),
                row[16].to_string(),
            );
            let amt: f64 = remove_comma(row[20].to_string());
            recon_casa
                .entry(recon_key)
                .and_modify(|val| *val += amt)
                .or_insert(amt);

            tot_amt += amt;
            tot_casa_acc += 1;
        } else {
            continue;
        }
    }

    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);

    let start_write_timer = SystemTime::now();

    let mut casa_writer = match buf_file_wrtr(config_param.output_file_path_casa(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create casa output file: `{}` on location `{}` : {}",
            config_param.output_file_path_casa(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match casa_writer.write_all(op_line_casa.as_bytes()) {
        Ok(_) => println!("Successfully processed all CASA accounts."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_param.output_file_path_casa(),
            error,
        ),
    }

    let casa_file_path = &get_full_file_path(config_param.rec_output_file_path(), "CASA");
    let td_file_path = &get_full_file_path(config_param.rec_output_file_path(), "TD");
    let mut recon_writer_casa = match buf_file_wrtr(casa_file_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create reconcilation file for CASA `{}` on location `{}` : {}",
            td_file_path,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };
    let mut recon_writer_td = match buf_file_wrtr(td_file_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create reconcilation file for TD`{}` on location `{}` : {}",
            td_file_path,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    let mut recon_op_line_casa = String::new();
    for (key, value) in recon_casa {
        let op = format!(
            "{}|{}|{}|{}|{}|{}",
            config_param.as_on_date().format("%d-%m-%Y"),
            config_param.input_file_name(),
            key.gl_type,
            key.gl_code,
            value,
            key.currency,
        );
        recon_op_line_casa.push_str(&op[..]);
        recon_op_line_casa.push_str("\n");
    }
    match recon_writer_casa.write_all(recon_op_line_casa.as_bytes()) {
        Ok(_) => println!("Successfully written reconcilation file."),
        Err(error) => panic!(
            "Unable to write reconcilation lines to file `{}`: {}.",
            config_param.rec_output_file_path(),
            error,
        ),
    };

    let mut recon_op_line_td = String::new();
    for (key, value) in recon_td {
        let op = format!(
            "{}|{}|{}|{}|{}|{}",
            config_param.as_on_date().format("%d-%m-%Y"),
            config_param.input_file_name(),
            key.gl_type,
            key.gl_code,
            value,
            key.currency,
        );
        recon_op_line_td.push_str(&op[..]);
        recon_op_line_td.push_str("\n");
    }
    match recon_writer_td.write_all(recon_op_line_td.as_bytes()) {
        Ok(_) => println!("Successfully written reconcilation file."),
        Err(error) => panic!(
            "Unable to write reconcilation lines to file `{}`: {}.",
            config_param.rec_output_file_path(),
            error,
        ),
    };
    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing pre-processed output and reconcilation files.");
    debug!(
        diag_log,
        "Writing TD & CASA Records and Reconcilation File, Total Duration: {:?}.", duration
    );
    let health_report = HealthReport::new(
        tot_td_acc,
        tot_td_acc - skip_acc,
        skip_acc,
        tot_amt,
        tot_amt,
        0,
    );

    let report_string = format!(
        "Total accounts encountered: {}\n\
         CASA accounts encountered: {}",
        tot_casa_acc + tot_td_acc,
        tot_casa_acc,
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
}

pub fn remove_comma(amt: String) -> f64 {
    let mut val: String = amt.to_string();
    val.retain(|dig| dig != ',');
    val.parse().unwrap_or(0.0)
}

fn get_full_file_path(file_path: &str, suffix: &str) -> String {
    let mut full_file_path = String::new();
    full_file_path.push_str(file_path);
    full_file_path.push_str(suffix);
    full_file_path.push_str(".txt");
    full_file_path
}
