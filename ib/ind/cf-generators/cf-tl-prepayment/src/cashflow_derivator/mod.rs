use slog::Logger;
mod account_reader;
mod account_without_cashflows;
mod account_without_cashflows_writer;
mod append_write_cashflows;
mod bkt_def;
mod cashflow_appender;
use self::append_write_cashflows::{append_cashflows, create_io_workers, write_cashflows};
use self::bkt_def::BktData;
use calamine::{open_workbook_auto, Reader, Sheets};
use cashflow_derivator::account_without_cashflows::AccountWithoutCashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use protobuf::Clear;
use rbdate::date_from_timestamp;
use sdb_io::*;
use statics::*;
use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::time::SystemTime;
pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut tot_acc_encntrd: i64 = DEFAULT_INT;
    let mut tot_acc_with_cfs: i64 = DEFAULT_INT;
    let mut tot_cfs: usize = 0;
    let mut tot_prin_in_ip = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let tot_int_in_op = DEFAULT_FLOAT;
    let tot_int_in_ip = DEFAULT_FLOAT;
    let start_derive_timer = SystemTime::now();
    //Mapping master File reading started
    log_debug!(log, "Mapping master File reading started");
    let mut mapping_master_map: HashMap<String, String> = HashMap::new();
    let mut mapping_master_excel = open_workbook_auto(config_params.mapping_master_file_path())
        .expect("Unable to open Mapping Master File.");
    check_sheet_name(
        config_params.mapping_master_file_path().to_owned(),
        &config_params.mapping_master_sheet_name().to_string(),
        &mapping_master_excel,
    );
    if let Some(Ok(reader)) =
        mapping_master_excel.worksheet_range(config_params.mapping_master_sheet_name())
    {
        for row in reader.rows().skip(0) {
            let gl_acc_no = row[0].to_string();
            let llg = row[4].to_string();
            mapping_master_map.insert(gl_acc_no, llg);
        }
    }
    log_debug!(log, "Mapping Master File Reading Completed");
    log_debug!(log, "Bkt Def File reading started");
    let bkt_def_file = match new_buf_rdr(config_params.bkt_def_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found bkt_def_file: `{}` : {}.",
            config_params.bkt_def_file_path(),
            error
        ),
    };
    let mut bkt_def_vec: Vec<BktData> = Vec::new();
    for (line_num, lines) in bkt_def_file.lines().enumerate() {
        let bkt_def_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.bkt_def_file_path(),
                line_num + 1,
                error
            ),
        };
        let bkt_def_fields = bkt_def_line.split('|').collect::<Vec<&str>>();
        bkt_def_vec.push(BktData {
            from_bkt: bkt_def_fields[0].parse().unwrap_or(0),
            to_bkt: bkt_def_fields[1].parse().unwrap_or(0),
            bkt_id: bkt_def_fields[3].to_string(),
        });
    }
    log_debug!(log, "Bkt Def File Reading is Completed");

    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut recon_writer = match buf_file_wrtr(
        &format!("{}.{}", config_params.output_file_path(), "txt"),
        None,
    ) {
        Ok(recon_output_file) => recon_output_file,
        Err(error) => panic!(
            "Unable to create recon output file: `{}` on location `{}`",
            &format!("{}.{}", config_params.output_file_path(), "txt"),
            error,
        ),
    };
    let mut reader_iterator = reader;
    let mut a_w_cf = AccountWithoutCashflows::new();
    loop {
        let account_opt = log_measurements!(
            diag_log,
            [format!(
                "Type: ReadParseInputAccount, Identifier: {}",
                tot_acc_encntrd
            )],
            reader_iterator.next()
        );

        if account_opt.is_none() {
            break;
        }

        let input_account = account_opt.expect("Unable to parse InputAccount struct.");
        tot_acc_encntrd += 1;
        tot_prin_in_op += input_account.premat_amt;
        tot_prin_in_ip += input_account.premat_amt;
        a_w_cf = append_cashflows(
            diag_log,
            &input_account,
            config_params,
            &mapping_master_map,
            &bkt_def_vec,
        );
        write_cashflows(&mut writer, log, diag_log, &a_w_cf);

        writeln!(
            recon_writer,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            a_w_cf.customer_no,
            a_w_cf.cust_acct_no,
            date_from_timestamp(a_w_cf.apprv_date).format("%d-%m-%Y"),
            date_from_timestamp(a_w_cf.lst_fin_date).format("%d-%m-%Y"),
            date_from_timestamp(a_w_cf.actl_mat_date).format("%d-%m-%Y"),
            a_w_cf.closure_amount,
            a_w_cf.int_rate,
            a_w_cf.gl_class_code,
            a_w_cf.currency_ind,
            a_w_cf.accnt_live_days,
            a_w_cf.preclosure_bkt_id,
            a_w_cf.actual_days_mat,
            a_w_cf.contractual_bkt_id,
            a_w_cf.llg_type,
            a_w_cf.add_dim1,
            a_w_cf.add_dim2,
            a_w_cf.add_dim3,
            a_w_cf.add_dim4,
            a_w_cf.add_dim5,
            a_w_cf.add_dim6,
            a_w_cf.add_dim7,
            a_w_cf.add_dim8
        )
        .expect("recon report can't be written");
        a_w_cf.clear();
    }

    writer.close();

    let end_derive_timer = SystemTime::now();
    let tot_duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total duration for derive timer.");
    let report_string = format!(
        "Accounts Encountered: {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total Duration: {:?}\n\
         Total outstanding amount in input: {:.2} \n\
         Total outstanding amount in output: {:.2}\n\
         Total interest in input: {:.2}\n\
         Total interest in output: {:.2}",
        tot_acc_encntrd,
        tot_acc_with_cfs,
        tot_cfs,
        tot_duration,
        tot_prin_in_ip,
        tot_prin_in_op,
        tot_int_in_ip,
        tot_int_in_op
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
    let health_stat = HealthReport::new(
        tot_acc_with_cfs,
        tot_acc_with_cfs,
        0,
        tot_prin_in_ip,
        tot_prin_in_op,
        tot_cfs as i64,
    );
    health_stat.gen_health_rpt(config_params.output_file_path())
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
