extern crate clap;
#[macro_use]
extern crate slog;
extern crate chrono;
extern crate health_report;
extern crate math;
extern crate protobuf;
extern crate rbconcurrency;
extern crate rbdate;
extern crate sdb_agg_rules;
extern crate sdb_agg_rules_adj;
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
use sdb_agg_rules::agg_rules::AggRules;
use sdb_agg_rules_adj::agg_rules::AggRules_adj;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use stamp_ftp::amb_file_reader;
use stamp_ftp::bm_reader::{BmKey, IntermediateBmPoints};
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::read_cashflow;
use std::collections::HashMap;
use writer::AccountWithCashflowsWriter;
mod ftp_parameters;
mod proc_ftp;
pub mod stamp_ftp;
mod writer;
use health_report::HealthReport;
use sdb_io::buf_file_wrtr;
use stamp_ftp::ftp_rates_reader;
use stamp_ftp::read_adjustments;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    //Initialization
    let mut ftp_parameters = initialize();

    //Process input records
    let log_output = process_records(&mut ftp_parameters);

    //log the output and exit
    rpt_n_exit(log_output, &ftp_parameters.log);

    let duration = start.elapsed();

    println!(
        "Total time to process securtization investments: {:?}",
        duration
    );
    let log_str = format!(
        "Total time to process securtization investments: {:?}",
        duration
    );
    log_info!(&ftp_parameters.log, "{}", log_str);
}

//Initialize all command line parameters , Read input cashflow file and rule files.
fn initialize() -> FtpParameters {
    let app_name = "FTP-Stamper-Sec-Investments";

    //Initializing all configuration parameters

    let cp = cp::get_cp(app_name);

    let (log, diag_log) = log::setup_loggers(&cp.log_file_path(), &cp.diagnostics_file_path());
    cp.log_parameters(&log);

    let input_field_names = AccFieldNames::get_input_fields_names();

    let input_data1 = reader::Reader::new_at_path(&cp.meta_data_file_path(), &cp.input_file_path());
    let input_data2 = reader::Reader::new_at_path(&cp.meta_data_file_path(), &cp.input_file_path());

    let m_rules1 = AggRules::new_from_path(&cp.m_rule_file_path(), &input_data1);
    let bc_rules1 = AggRules::new_from_path(&cp.bc_rule_file_path(), &input_data1);
    let fix_adj_rules1 = AggRules_adj::new_from_path(&cp.fix_adj_rule_file_path(), &input_data1);
    let var_adj_rules1 = AggRules_adj::new_from_path(&cp.var_adj_rule_file_path(), &input_data1);
    let (ftp_rates1, lock_adjs1) = ftp_rates_reader::read_ftp_rates(&cp.ftp_rates_file_path());

    let m_rules2 = AggRules::new_from_path(&cp.m_rule_file_path(), &input_data2);
    let bc_rules2 = AggRules::new_from_path(&cp.bc_rule_file_path(), &input_data2);
    let fix_adj_rules2 = AggRules_adj::new_from_path(&cp.fix_adj_rule_file_path(), &input_data2);
    let var_adj_rules2 = AggRules_adj::new_from_path(&cp.var_adj_rule_file_path(), &input_data2);
    let (ftp_rates2, lock_adjs2) = ftp_rates_reader::read_ftp_rates(&cp.ftp_rates_file_path());

    let adj_rates = read_adjustments::read_adj_rates(&cp.adj_rate_file_path());
    let avg_bal = amb_file_reader::read_avg_bal(&cp.amb_file_path());

    //TODO:: read Adjustment rules

    LOG_PARAMS.set_once_diagnostic_level(cp.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(cp.is_perf_diagnostics_enabled());

    let out_path = format!("{}_spread.txt", cp.output_file_path());
    let mut spread_writer = match buf_file_wrtr(&out_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            out_path,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    return FtpParameters {
        cp,
        log,
        diag_log,
        m_rules1,
        m_rules2,
        bc_rules1,
        bc_rules2,
        fix_adj_rules1,
        fix_adj_rules2,
        var_adj_rules1,
        var_adj_rules2,
        input_data1,
        input_data2,
        input_field_names,
        ftp_rates1,
        ftp_rates2,
        lock_adjs1,
        lock_adjs2,
        adj_rates,
        avg_bal,
        spread_writer,
    };
}

fn process_records(ftp_parameters: &mut FtpParameters) -> (String) {
    let mut total_account_with_cf = 0;
    let mut total_bal = 0.0;
    let mut output: String = String::new();
    let mut cashflow_out: String = String::new();
    let mut output_writer =
        create_io_workers(&ftp_parameters.cp.output_file_path(), &ftp_parameters.log);

    let mut saved_bm_rates: HashMap<BmKey, Vec<IntermediateBmPoints>> = HashMap::new();
    //Header for output
    let mut op_hdr = "account_number|ubs_acc_num|cust_name|average_balance|accr_int|yld_to_call|int_rate|base_rate|final_ftp_rate|value_date|maturity_date|nxt_rep_dt|lst_rep_dt|mis1|mis2|psl_code|prod_type|rate_flag|branch|source_file_name|ccy|gl|cust_id|final_ftp_amt|alm_line|trade_dt|orig_bal|outstanding_bal|base_rate|adj1|adj2|adj3|adj4|adj5|adj6|input_benchmark|pdo|npa|method|rate_curve|org_tenor|rep_tenor|fx_spread|var_spread|first_ftp|bc_as_on_rule|tenor_start_date_rule|tenor_end_date_rule|bc_as_on_applied|tenor_start_date_applied|tenor_end_date_applied\n";
    output.push_str(&op_hdr);
    cashflow_out.push_str(&op_hdr);

    // Reading .cf file for old account numbers
    for mut account_input in ftp_parameters.input_data1.iter().skip(1) {
        total_account_with_cf += 1;

        //Read cashflow
        let mut cf_input = read_cashflow(&account_input, &ftp_parameters.input_field_names);

        total_bal += cf_input.total_principal_amount;

        cf_input.acc_no = cf_input.fc_ubs_acc.to_string();
        if !cf_input.acc_no.is_empty() {
            //calculate FTP
            let (out_str, cf_out) = proc_ftp::calculate_ftp(
                &mut account_input,
                cf_input,
                &ftp_parameters.m_rules1,
                &ftp_parameters.bc_rules1,
                &ftp_parameters.fix_adj_rules1,
                &ftp_parameters.var_adj_rules1,
                ftp_parameters.cp.bc_file_path().to_string(),
                &ftp_parameters.input_field_names,
                &ftp_parameters.log,
                &ftp_parameters.diag_log,
                ftp_parameters.cp.ftprunid(),
                ftp_parameters.cp.from_date(),
                ftp_parameters.cp.to_date(),
                &ftp_parameters.cp.output_file_path(),
                &mut ftp_parameters.ftp_rates1,
                &ftp_parameters.lock_adjs1,
                &ftp_parameters.adj_rates,
                &ftp_parameters.avg_bal,
                ftp_parameters.cp.default_method(),
                ftp_parameters.cp.default_basecurve(),
                ftp_parameters.cp.fixed_adj_count(),
                ftp_parameters.cp.var_adj_count(),
                ftp_parameters.cp.is_closed(),
                &mut saved_bm_rates,
                &mut ftp_parameters.spread_writer,
                ftp_parameters.cp.rate_precision(),
                ftp_parameters.cp.bal_precision(),
            );

            output.push_str(&out_str);
            cashflow_out.push_str(&cf_out);
        }
    }

    //Footer for output
    let mut op_ftr = format!(
        "FTR|{}|{}\n",
        ftp_parameters.cp.from_date(),
        total_account_with_cf
    );
    output.push_str(&op_ftr);
    cashflow_out.push_str(&op_ftr);

    let out_path = format!("{}.txt", &ftp_parameters.cp.output_file_path());
    let cf_out_path = format!("{}_cf.txt", &ftp_parameters.cp.output_file_path());

    let mut out_writer = match buf_file_wrtr(&out_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            out_path,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match out_writer.write_all(output.as_bytes()) {
        Ok(_) => println!("Successfully processed all securtization investments accounts ."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`.{}",
            out_path, error
        ),
    }

    let mut out_writer = match buf_file_wrtr(&cf_out_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            cf_out_path,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match out_writer.write_all(cashflow_out.as_bytes()) {
        Ok(_) => println!("Successfully processed all securtization investments accounts ."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`.{}",
            out_path, error
        ),
    }

    let report_string = format!("Accounts With Cashflows: {}", total_account_with_cf);

    let health_report = HealthReport::new(
        total_account_with_cf,
        total_account_with_cf,
        0,
        total_bal,
        total_bal,
        0,
    );
    health_report.gen_health_rpt(&ftp_parameters.cp.output_file_path());

    return report_string;
}

fn rpt_n_exit(out_log: String, log: &Logger) {
    log_debug!(log, "{}", out_log);
}

fn create_io_workers(output_path: &str, log: &Logger) -> (AccountWithCashflowsWriter) {
    let writer = AccountWithCashflowsWriter::new(output_path, log);

    return writer;
}
