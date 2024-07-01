use self::derive_fields::{get_gl, get_op_line};
use self::input_account::InputAccount;
use self::reconcilation::ReconKey;
use self::structs::{MasterLLGFields, OraFields};
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use csv::ReaderBuilder;
use health_report::HealthReport;
use macros;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::default::Default;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;

mod derive_fields;
mod input_account;
mod reconcilation;
mod structs;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let st_tm_read = SystemTime::now();
    let mut op_line: String = String::new();
    //adding header to output
    let header = "deal_no|bond_issuance|isin|issuance_dt|branch_entity|desk|portfolio_type|category|security_type|slrnon_slr|short_name|secured_unsecured|rt|nxt_call_dt|nxt_put_dt|agency|rating|agency_of_current_rating|listed_unlisted|mat_dt|conversion_rt_lcy|ccy|bv_after_amortisation|wap|laf_and_msf_ost_fv|laf_and_msf_ost_bv|reverse_laf_ost_fv|reverse_repo_ost_fv|collateral_placed_fv|encumbered_fv|encumbered_bv|ytm|basis|issue_country|domicile_country|category1|category2|category3|category4|industry_code|taxability|air_till_dt|modified_duration|int_coupontype|nxt_rep_dt|sec_grp|sec_typ|sec_issuer|sec_guaranteed|mrkt|idx_label|bd_cat|bd_typ|lstd|npa|cf_dt|cf_int_amt|cf_prin_amt|fwd_outright_buy_fv|fwd_outright_buy_bv|fwd_outright_sale_fv|fwd_outright_sale_bv|as_on_dt|alm_concat|alm_line|ia_line|face_val|book_val|market_val|mtm_amt\n";
    op_line.push_str(&header);
    let mut concat_line: String = String::new();
    let mut tot_rec = DEFAULT_INT;
    let mut skp_rec = DEFAULT_INT;

    let mut ref_excel1: Xlsx<_> = open_workbook(config_param.murex_inv_master())
        .expect("Unable to open Murex_Inv_Master.xlsx.xlsx.");
    let mut fv_gl: HashMap<String, String> = HashMap::new();
    let mut prem_gl: HashMap<String, String> = HashMap::new();
    let mut prem_amt: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel1.worksheet_range(config_param.murex_inv_sheet_name()) {
        for row in reader.rows() {
            let concat = row[5].to_string().as_str().replace("\u{a0}", " ");
            fv_gl.insert(concat.to_string(), row[6].to_string());
            prem_gl.insert(concat.to_string(), row[7].to_string());
            prem_amt.insert(
                concat.to_string(),
                row[9]
                    .to_string()
                    .to_uppercase()
                    .as_str()
                    .replace("\u{a0}", " "),
            );
        }
    }
    let mut ref_excel2: Xlsx<_> =
        open_workbook(config_param.ora_gl()).expect("Unable to open Ora_GL.xlsx.");
    let mut ora_fields_map: HashMap<String, OraFields> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel2.worksheet_range(config_param.ora_gl_sheet_name()) {
        for row in reader.rows() {
            ora_fields_map.insert(
                row[0].to_string(),
                OraFields {
                    ora_mis1: row[2].to_string(),
                    ora_prod: row[4].to_string(),
                    ora_gl: row[1].to_string(),
                    ora_category: row[5].to_string(),
                },
            );
        }
    }

    let mut ref_excel3: Xlsx<_> =
        open_workbook(config_param.master_llg()).expect("Unable to open Master_LLG.xlsx.");
    let mut master_llg_fields_map: HashMap<String, MasterLLGFields> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel3.worksheet_range(config_param.master_llg_sheet_name()) {
        for row in reader.rows() {
            master_llg_fields_map.insert(
                row[0].to_string(),
                MasterLLGFields {
                    alm_line: row[6].to_string(),
                    ia_line: row[7].to_string(),
                },
            );
        }
    }

    let mut recon: HashMap<ReconKey, f64> = HashMap::new();
    let mut reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(config_param.input_file_path())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut tot_amt = 0.0;
    for (line_num, lines) in reader.deserialize().enumerate() {
        let input_account: InputAccount = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.input_file_path(),
                    line_num + 1,
                    error
                );
                Default::default()
            }
        };
        tot_rec += 1;

        if input_account.branch_entity != config_param.entity() {
            skp_rec += 1;
            continue;
        }
        let amt = input_account.cf_prin_amt.parse::<f64>().unwrap_or(0.0);
        tot_amt += amt;
        let gls = get_gl(&input_account, &mut fv_gl, &mut prem_gl, &mut prem_amt);
        let mut amt: f64 = input_account.fv.parse().unwrap_or(0.0);
        let alm_concat_fields = match ora_fields_map.get(&gls.fv_gl) {
            Some(value) => value.to_owned(),
            None => OraFields {
                ..Default::default()
            },
        };
        let alm_concat = alm_concat_fields.ora_mis1
            + "_"
            + &alm_concat_fields.ora_prod
            + "_"
            + &alm_concat_fields.ora_gl
            + "_"
            + &alm_concat_fields.ora_category;
        let product_concat = &gls.product_concat;
        let master_llg_value = match master_llg_fields_map.get(&alm_concat) {
            Some(value) => value.to_owned(),
            None => MasterLLGFields {
                ..Default::default()
            },
        };
        let gl_concat = gls.fv_gl.to_string() + "#" + &gls.prem_gl.to_string();
        op_line.push_str(&get_op_line(
            &input_account,
            *config_param.as_on_date(),
            &master_llg_value,
            &alm_concat,
            &product_concat,
            &mut concat_line,
            &gl_concat,
        ));
        let mut recon_key = ReconKey::new(
            input_account.ccy.to_string(),
            "face_value_gl".to_string(),
            gls.fv_gl.to_string(),
        );
        recon
            .entry(recon_key)
            .and_modify(|val| *val += amt)
            .or_insert(amt);
        amt = if gls.prem_amt_field == "BVBEFOREAMORT-FV" {
            input_account.bv_before_amort.parse().unwrap_or(0.0) - amt
        } else if gls.prem_amt_field == "BVAFTERAMORT-FV" {
            input_account.bv_after_amortisation.parse().unwrap_or(0.0) - amt
        } else {
            log_error!(
                log,
                "Invalid field for premium amount selection: `{}`.",
                gls.prem_amt_field
            );
            DEFAULT_FLOAT
        };

        recon_key = ReconKey::new(
            input_account.ccy.to_string(),
            "premium_gl".to_string(),
            gls.prem_gl.to_string(),
        );
        recon
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
    let mut op_writer = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            config_param.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match op_writer.write_all(op_line.as_bytes()) {
        Ok(_) => println!("Successfully processed all accounts."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}` : {}",
            config_param.output_file_path(),
            error
        ),
    }

    let mut recon_writer = match buf_file_wrtr(config_param.rec_output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create reconcilation file `{}` on location `{}` : {}",
            config_param.rec_output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };
    let mut concat_writer = match buf_file_wrtr(config_param.concat(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create concat file `{}` on location `{}` : {}",
            config_param.concat(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };
    match concat_writer.write_all(concat_line.as_bytes()) {
        Ok(_) => println!("Successfully written concat file."),
        Err(error) => panic!(
            "Unable to write concat lines to file `{}`: {}.",
            config_param.rec_output_file_path(),
            error
        ),
    };
    let mut recon_op_line = String::new();
    for (key, value) in recon {
        let op = format!(
            "{}|{}|{}|{}|{}|{}",
            config_param.as_on_date().format("%d-%m-%Y"),
            "Security_Closing_Stock",
            key.gl_type,
            key.gl_code,
            value,
            key.currency,
        );
        recon_op_line.push_str(&op[..]);
        recon_op_line.push_str("\n");
    }
    match recon_writer.write_all(recon_op_line.as_bytes()) {
        Ok(_) => println!("Successfully written reconcilation file."),
        Err(error) => panic!(
            "Unable to write reconcilation lines to file `{}`: {}.",
            config_param.rec_output_file_path(),
            error
        ),
    };
    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_param.output_file_path());

    let ed_tm_writer = SystemTime::now();
    let duration = ed_tm_writer
        .duration_since(st_tm_writer)
        .expect("Could not calculate total duration for write process.");
    debug!(
        diag_log,
        "Writing Murex Security Closing Stock, Total Duration: {:?}.", duration
    );
}
