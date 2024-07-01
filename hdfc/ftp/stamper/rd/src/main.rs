extern crate clap;
#[macro_use]
extern crate slog;
extern crate chrono;
extern crate health_report;
extern crate protobuf;
extern crate rbconcurrency;
extern crate rbdate;
extern crate sdb_agg_rules;
extern crate sdb_dyn_proto_rdr;
extern crate sdb_io;
extern crate serde;
extern crate serde_json;
extern crate slog_async;
extern crate slog_term;

mod statics;
#[macro_use]
mod macros;
mod cp;
mod log;

use ftp_parameters::FtpParameters;
use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use stamp_ftp::cfinput::AccFieldNames;
mod ftp_parameters;
pub mod stamp_ftp;
mod writer;
use health_report::HealthReport;
use sdb_io::buf_file_wrtr;
use stamp_ftp::aggr_key::read_aggr_file::read_file;
use stamp_ftp::amb_file_reader;
use stamp_ftp::calc_ftp::calc_ftp;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    //Initialization
    let mut ftp_parameters = initialize();

    //Process input records
    let (log_output, output_less_than_two, output_two_to_five, output_greater_than_five) =
        process_records(&mut ftp_parameters);

    //log the output and exit
    rpt_n_exit(
        ftp_parameters.cp,
        log_output,
        output_less_than_two,
        output_two_to_five,
        output_greater_than_five,
        &ftp_parameters.log,
    );

    let duration = start.elapsed();

    println!("Total time to process RD accounts: {:?}", duration);
    let log_str = format!("Total time to process RD accounts: {:?}", duration);
    log_info!(&ftp_parameters.log, "{}", log_str);
}

//Initialize all command line parameters , Read input cashflow file and rule files.
fn initialize() -> FtpParameters {
    let app_name = "ftp-stamper-rd";

    //Initializing all configuration parameters

    let cp = cp::get_cp(app_name);

    let (log, diag_log) = log::setup_loggers(&cp.log_file_path(), &cp.diagnostics_file_path());
    cp.log_parameters(&log);

    let input_field_names = AccFieldNames::get_input_fields_names();

    let input_data = reader::Reader::new_at_path(&cp.meta_data_file_path(), &cp.input_file_path());

    let aggr_bal = read_file(&cp.aggr_file_path());
    let avg_bal = amb_file_reader::read_avg_bal(&cp.amb_file_path(), &log);

    //TODO:: read Adjustment rules

    LOG_PARAMS.set_once_diagnostic_level(cp.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(cp.is_perf_diagnostics_enabled());

    return FtpParameters {
        cp,
        log,
        diag_log,
        input_data,
        input_field_names,
        aggr_bal,
        avg_bal,
    };
}

fn process_records(ftp_parameters: &mut FtpParameters) -> (String, String, String, String) {
    let mut total_account_with_cf = 0;
    let mut total_acc_bal = 0.0;
    let mut output_less_than_two: String = String::new();
    let mut output_two_to_five: String = String::new();
    let mut output_less_than_one: String = String::new();
    let mut output_one_to_five: String = String::new();
    let mut output_greater_than_five: String = String::new();
    //Header for output
    let mut op_hdr = "account_number|cust_name|average_balance|accr_int|yld_to_call|int_rate|base_rate|final_ftp_rate|value_date|maturity_date|nxt_rep_dt|lst_rep_dt|mis1|mis2|psl_code|prod_type|rate_flag|branch|source_file_name|ccy|gl|cust_id|final_ftp_amt|alm_line|trade_dt|orig_bal|outstanding_bal|base_rate|adj1|adj2|adj3|adj4|adj5|adj6|input_benchmark|pdo|npa|method|rate_curve|org_tenor|rep_tenor|fx_spread|var_spread|first_ftp|bc_as_on_rule|tenor_start_date_rule|tenor_end_date_rule|bc_as_on_applied|tenor_start_date_applied|tenor_end_date_applied\n";
    output_less_than_two.push_str(&op_hdr);
    output_two_to_five.push_str(&op_hdr);
    output_less_than_one.push_str(&op_hdr);
    output_one_to_five.push_str(&op_hdr);
    output_greater_than_five.push_str(&op_hdr);

    for mut account_input in ftp_parameters.input_data.iter() {
        total_account_with_cf += 1;

        total_acc_bal += match account_input.get_f64_for_key(&ftp_parameters.input_field_names.amt)
        {
            Ok(result) => result,
            Err(e) => 0.0,
        };

        //calculate FTP
        let (out_str, out_type) = calc_ftp(
            &mut account_input,
            &ftp_parameters.input_field_names,
            &ftp_parameters.log,
            &ftp_parameters.diag_log,
            ftp_parameters.cp.ftprunid(),
            ftp_parameters.cp.from_date(),
            ftp_parameters.cp.to_date(),
            &mut ftp_parameters.aggr_bal,
            &ftp_parameters.avg_bal,
        );

        match out_type.as_str() {
            "two_to_five" => {
                output_two_to_five.push_str(&out_str);
            }
            "less_than_two" => {
                output_less_than_two.push_str(&out_str);
            }
            "one_to_five" => {
                output_one_to_five.push_str(&out_str);
            }
            "less_than_one" => {
                output_less_than_one.push_str(&out_str);
            }
            _ => {
                output_greater_than_five.push_str(&out_str);
            }
        }
    }
    //Footer for output
    let mut op_ftr = format!("FTR|{}|{}\n",ftp_parameters.cp.from_date(),total_account_with_cf);
    output_less_than_two.push_str(&op_ftr);
    output_two_to_five.push_str(&op_ftr);
    output_less_than_one.push_str(&op_ftr);
    output_one_to_five.push_str(&op_ftr);
    output_greater_than_five.push_str(&op_ftr);
    
    let report_string = format!("Accounts With Cashflows: {}", total_account_with_cf);

    let health_report = HealthReport::new(
        total_account_with_cf,
        total_account_with_cf,
        0,
        total_acc_bal,
        total_acc_bal,
        0,
    );

    health_report.gen_health_rpt(&ftp_parameters.cp.output_file_path());

    (
        report_string,
        output_less_than_two,
        output_two_to_five,
        output_greater_than_five,
    )
}

fn rpt_n_exit(
    p: cp::CP,
    out_log: String,
    output_less_than_two: String,
    output_two_to_five: String,
    output_greater_than_five: String,
    log: &Logger,
) {
    let out_path_less_than_two = format!("{}_lt_2Cr.txt", p.output_file_path());
    let out_path_two_to_five = format!("{}_2Cr_bt_5Cr.txt", p.output_file_path());
    let out_path_grtr_than_five = format!("{}_gt_5Cr.txt", p.output_file_path());

    let mut out_writer_1 = match buf_file_wrtr(&out_path_less_than_two, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            out_path_less_than_two,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match out_writer_1.write_all(output_less_than_two.as_bytes()) {
        Ok(_) => println!("Successfully processed all less than two crore balance accounts ."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`.{}",
            out_path_less_than_two, error
        ),
    }

    let mut out_writer_2 = match buf_file_wrtr(&out_path_two_to_five, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            out_path_two_to_five,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match out_writer_2.write_all(output_two_to_five.as_bytes()) {
        Ok(_) => println!("Successfully processed all two to five crore balance accounts."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`.{}",
            out_path_two_to_five, error
        ),
    }

    let mut out_writer_3 = match buf_file_wrtr(&out_path_grtr_than_five, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            out_path_grtr_than_five,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match out_writer_3.write_all(output_greater_than_five.as_bytes()) {
        Ok(_) => println!("Successfully processed all greater than five crore balance accounts."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`.{}",
            out_path_grtr_than_five, error
        ),
    }

    log_debug!(log, "{}", out_log);
}
