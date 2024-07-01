use self::derive_casaod_fields::get_casaod_op_line;
use self::derive_cust_master::create_cust_master;
use self::derive_td_fields::get_td_op_line;
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
use std::os;
use std::time::SystemTime;

mod derive_casaod_fields;
mod derive_cust_master;
mod derive_td_fields;
mod reconcilation;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let instance = config_param.instance_name();
    let mut ref_excel1: Xlsx<_> =
        open_workbook(config_param.ref_file_path_1()).expect("Unable to open `Ora_PROD.xlsx`.");
    let mut t_ora_prod: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel1.worksheet_range(config_param.ref1_sheet_name()) {
        for row in reader.rows() {
            t_ora_prod.insert(row[1].to_string(), row[0].to_string());
        }
    }

    let mut ref_excel2: Xlsx<_> = open_workbook(config_param.ref_file_path_2())
        .expect("Error while opening `Ora_GL.xlsx` file.");
    let mut t_ora_gl: HashMap<String, Vec<String>> = HashMap::new();
    let mut prod_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel2.worksheet_range(config_param.ref2_sheet_name()) {
        for row in reader.rows() {
            let mut t_ora_gl_fields: Vec<String> = Vec::new();
            t_ora_gl_fields.push(row[1].to_string());
            t_ora_gl_fields.push(row[4].to_string());
            t_ora_gl_fields.push(row[2].to_string());
            t_ora_gl.insert(row[0].to_string(), t_ora_gl_fields);
            prod_map.insert(row[0].to_string(), row[4].to_string());
        }
    }

    let mut ref_excel3: Xlsx<_> = open_workbook(config_param.ref_file_path_3())
        .expect("Error while opening `Master_LLG_Updated.xlsx` file.");
    let mut alm_line: HashMap<String, String> = HashMap::new();
    let mut ia_llg_map: HashMap<String, String> = HashMap::new();
    let mut balm_llg_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel3.worksheet_range(config_param.alm_master_sheet_name()) {
        for row in reader.rows() {
            alm_line.insert(row[1].to_string(), row[6].to_string());
            ia_llg_map.insert(row[1].to_string(), row[7].to_string());
            balm_llg_map.insert(row[1].to_string(), row[9].to_string());
        }
    }

    let mut ref_excel4: Xlsx<_> = open_workbook(config_param.ref_file_path_4())
        .expect("Error while opening `ALM_COA_Master.xlsx` file.");
    let mut t_bdp_coa: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel4.worksheet_range(config_param.ref4_sheet_name()) {
        for row in reader.rows() {
            t_bdp_coa.insert(row[0].to_string(), row[5].to_string());
        }
    }

    let mut ref_excel5: Xlsx<_> = open_workbook(config_param.ref_file_path_5())
        .expect("Error while opening `MIS1_Desc.xlsx` file.");
    let mut div: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel5.worksheet_range(config_param.ref5_sheet_name()) {
        for row in reader.rows() {
            div.insert(row[1].to_string(), row[2].to_string());
        }
    }
    let end_read_timer = SystemTime::now();
    let duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration read timer.");
    debug!(
        diag_log,
        "Reading Reference Files, Total Duration: {:?}.", duration
    );

    //read mf master file
    let mut mf_master: Xlsx<_> = open_workbook(config_param.mf_master_file())
        .expect("Error while opening `TD_MF_Master.xlsx` file.");
    let sheet_name = mf_master
        .sheet_names()
        .first()
        .expect("excel is empty")
        .to_owned();
    let mut mf_master_fields: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = mf_master.worksheet_range(&sheet_name) {
        for row in reader.rows() {
            let mut custno_entity = String::new();
            custno_entity = format!("{}_{}", row[3].to_string(), row[1].to_string());
            mf_master_fields.insert(custno_entity, row[4].to_string());
        }
    }

    let start_derive_timer = SystemTime::now();
    let mut op_line_td: String = String::new();
    let mut op_line_od: String = String::new();
    let mut op_line_casa: String = String::new();
    let mut op_concat_line_td: String = String::new();
    let mut op_concat_line_od: String = String::new();
    let mut op_concat_line_casa: String = String::new();
    let mut recon_casa: HashMap<ReconKey, f64> = HashMap::new();
    let mut recon_td: HashMap<ReconKey, f64> = HashMap::new();
    let mut recon_od: HashMap<ReconKey, f64> = HashMap::new();
    let mut tot_casa_acc: i64 = DEFAULT_INT;
    let mut tot_td_acc: i64 = DEFAULT_INT;
    let mut tot_od_acc: i64 = DEFAULT_INT;
    let mut tot_amt: f64 = 0.0;
    let mut skip_acc: i64 = 0;

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
    let mut line_count = 0;
    //Add header to output.
    let header= "account_number|accrued_interest|deposit_type|maturity_date|rat_acct_int|rat_acct_int_var|next_compound_date|next_payment_date|account_start_date|currency_code: i64,|customer_id: i64,|original_balance|origination_date|previous_roll_over_date|description|client_name|tname|as_on_date|bank_num|branch|rate_flag|int_pay_freq: i64,|cost_centre_ftp|institution|new_gl|int_rate|concat|ia_llg|balm_llg|current_book_balance|cost_center|comp_freq: i64,|fin_cost_ftp|cust_category|cod_prod|com_mis_comp_1|rat_prod_var|dat_value_date|alm_concat|amt_initl_dep|bal_principle_lcy|bal_int_accr_lcy|lien_marked|prod_code|acc_open_date|gl_int_comp|division|lcr_classification_code|contr_days|res_days\n";
    op_line_td.push_str(header);
    for (line_num, lines) in file.lines().enumerate() {
        // Skip 1 line - header
        if line_count < 1 {
            line_count += 1;
            continue;
        }
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_file_path(),
                line_num + 1,
                error
            ),
        };
        let info: Vec<&str> = line.split('|').collect();
        let mut row: Vec<&str> = Vec::new();
        for data in info {
            row.push(data.trim());
        }

        if row.len() < 21 {
            let report_string = format!(
                "Less no of fields present in line no:{}, line is:{}",
                line_num, line
            );
            log_info!(log, "{}", report_string);
            continue;
        }

        if row[0].to_string().is_empty() {
            skip_acc += 1;
            continue;
        }
        let acc_class = row[5].to_string().parse().unwrap_or(0);
        let os_bal_lcy = row[19].parse().unwrap_or(0.0);
        if acc_class < 500 && os_bal_lcy >= 0.0 {
            let casa = get_casaod_op_line(
                &mut row,
                &mut t_ora_prod,
                &mut t_ora_gl,
                &mut t_bdp_coa,
                &mut div,
                &mut alm_line,
                &log,
                &mf_master_fields,
                &instance,
            );
            op_line_casa.push_str(&casa[1]);
            op_concat_line_casa.push_str(&casa[0]);

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
        } else if acc_class >= 500 {
            let td = &get_td_op_line(
                &mut row,
                &mut t_ora_prod,
                &mut t_ora_gl,
                &mut t_bdp_coa,
                &mut prod_map,
                &mut ia_llg_map,
                &mut balm_llg_map,
                &mut div,
                &mut alm_line,
                *config_param.as_on_date(),
                &log,
                &mf_master_fields,
                &instance,
            );
            op_line_td.push_str(&td[1]);
            op_concat_line_td.push_str(&td[0]);

            let recon_key = ReconKey::new(
                row[6].to_string(),
                config_param.gl_type().to_string(),
                row[16].to_string(),
            );
            let amt: f64 = remove_comma(row[20].to_string());
            recon_td
                .entry(recon_key)
                .and_modify(|val| *val += amt)
                .or_insert(amt);

            tot_amt += amt;
            tot_td_acc += 1;
        } else if acc_class < 500 && os_bal_lcy < 0.0 {
            let od = get_casaod_op_line(
                &mut row,
                &mut t_ora_prod,
                &mut t_ora_gl,
                &mut t_bdp_coa,
                &mut div,
                &mut alm_line,
                &log,
                &mf_master_fields,
                &instance,
            );
            op_line_od.push_str(&od[1]);
            op_concat_line_od.push_str(&od[0]);

            let recon_key = ReconKey::new(
                row[6].to_string(),
                config_param.gl_type().to_string(),
                row[16].to_string(),
            );
            let amt: f64 = remove_comma(row[20].to_string());
            recon_od
                .entry(recon_key)
                .and_modify(|val| *val += amt)
                .or_insert(amt);

            tot_amt += amt;
            tot_od_acc += 1;
        }
    }
    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);

    //creates cust master
    create_cust_master(&config_param, log);
    let start_write_timer = SystemTime::now();

    let mut td_writer = match buf_file_wrtr(config_param.output_file_path_td(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create td ouput file: `{}` on location `{}` : {}",
            config_param.output_file_path_td(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match td_writer.write_all(op_line_td.as_bytes()) {
        Ok(_) => println!("Successfully processed all TD accounts."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_param.output_file_path_td(),
            error,
        ),
    }

    let mut od_writer = match buf_file_wrtr(config_param.output_file_path_od(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create od ouput file: `{}` on location `{}` : {}",
            config_param.output_file_path_od(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match od_writer.write_all(op_line_od.as_bytes()) {
        Ok(_) => println!("Successfully processed all OD accounts."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_param.output_file_path_od(),
            error,
        ),
    }

    let mut concat_td_writer = match buf_file_wrtr(config_param.output_concat_file_path_td(), None)
    {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create td concat output file: `{}` on location `{}` : {}",
            config_param.output_concat_file_path_td(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    let mut concat_od_writer = match buf_file_wrtr(config_param.output_concat_file_path_od(), None)
    {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create od concat ouput file: `{}` on location `{}` : {}",
            config_param.output_concat_file_path_od(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match concat_td_writer.write_all(op_concat_line_td.as_bytes()) {
        Ok(_) => println!("Successfully processed all TD Concat accounts."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_param.output_concat_file_path_td(),
            error,
        ),
    }

    match concat_od_writer.write_all(op_concat_line_od.as_bytes()) {
        Ok(_) => println!("Successfully processed all OD Concat accounts."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_param.output_concat_file_path_td(),
            error,
        ),
    }

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

    let mut concat_casa_writer =
        match buf_file_wrtr(config_param.output_concat_file_path_casa(), None) {
            Ok(file) => file,
            Err(error) => panic!(
                "Unable to create casa concat output file: `{}` on location `{}` : {}",
                config_param.output_concat_file_path_casa(),
                current_dir()
                    .expect("Unable to get current directory path.")
                    .display(),
                error
            ),
        };

    match concat_casa_writer.write_all(op_concat_line_casa.as_bytes()) {
        Ok(_) => println!("Successfully processed all CASA Concat accounts."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_param.output_concat_file_path_casa(),
            error,
        ),
    }

    let casa_file_path = &get_full_file_path(config_param.rec_output_file_path(), "CASA");
    let td_file_path = &get_full_file_path(config_param.rec_output_file_path(), "TD");
    let od_file_path = &get_full_file_path(config_param.rec_output_file_path(), "OD");
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
    let mut recon_writer_od = match buf_file_wrtr(od_file_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create reconcilation file for OD`{}` on location `{}` : {}",
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
        Ok(_) => println!("Successfully written CASA reconcilation file."),
        Err(error) => panic!(
            "Unable to write CASA reconcilation lines to file `{}`: {}.",
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
        Ok(_) => println!("Successfully written TD reconcilation file."),
        Err(error) => panic!(
            "Unable to write TD reconcilation lines to file `{}`: {}.",
            config_param.rec_output_file_path(),
            error,
        ),
    };
    let mut recon_op_line_od = String::new();
    for (key, value) in recon_od {
        let op = format!(
            "{}|{}|{}|{}|{}|{}",
            config_param.as_on_date().format("%d-%m-%Y"),
            config_param.input_file_name(),
            key.gl_type,
            key.gl_code,
            value,
            key.currency,
        );
        recon_op_line_od.push_str(&op[..]);
        recon_op_line_od.push_str("\n");
    }
    match recon_writer_od.write_all(recon_op_line_od.as_bytes()) {
        Ok(_) => println!("Successfully written OD reconcilation file."),
        Err(error) => panic!(
            "Unable to write OD reconcilation lines to file `{}`: {}.",
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
    health_report.gen_health_rpt(&config_param.output_file_path_td());
    let report_string = format!(
        "Total accounts encountered: {}\n\
         CASA accounts encountered: {}\n\
         OD accounts encountered: {}\n\
         TD accounts encountered: {}",
        tot_casa_acc + tot_td_acc,
        tot_casa_acc,
        tot_od_acc,
        tot_td_acc,
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
