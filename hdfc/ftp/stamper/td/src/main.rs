extern crate clap;
#[macro_use]
extern crate slog;
extern crate chrono;
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
use stamp_ftp::cfinput::AccFieldNames;
mod ftp_parameters;
mod proc_ftp;
pub mod stamp_ftp;
use sdb_io::buf_file_wrtr;
use stamp_ftp::aggr_key::read_aggr_file::read_file;
use stamp_ftp::bm_reader::{BmKey, IntermediateBmPoints};
use stamp_ftp::calc_ftp;
use stamp_ftp::ftp_rates_reader;
use stamp_ftp::read_adjustments;
use stamp_ftp::rule_stamper;
use statics::DEFAULT_FLOAT;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let start_init_timer = Instant::now();
    //Initialization
    let mut ftp_parameters = initialize();
    let init_time = start_init_timer.elapsed();
    //  print!("Total Init Time: {:#?}", init_time);
    //Process input records
    let start_proc_timer = Instant::now();
    let log_output = process_records(&mut ftp_parameters);
    let proc_time = start_proc_timer.elapsed();
    // print!("Total Proc Time: {:#?}", proc_time);
    //log the output and exit
    rpt_n_exit(ftp_parameters.cp, log_output, &ftp_parameters.log);

    let duration = start.elapsed();

    println!("Total time to process TD: {:?}", duration);
    let log_str = format!("Total time to process TD: {:?}", duration);
    log_info!(&ftp_parameters.log, "{}", log_str);
}

//Initialize all command line parameters , Read input cashflow file and rule files.
fn initialize() -> FtpParameters {
    let app_name = "ftp-stamper-dep";

    //Initializing all configuration parameters

    let cp = cp::get_cp(app_name);

    let (log, diag_log) = log::setup_loggers(&cp.log_file_path(), &cp.diagnostics_file_path());
    cp.log_parameters(&log);

    let input_field_names = AccFieldNames::get_input_fields_names();
    let input_data = reader::Reader::new_at_path(&cp.meta_data_file_path(), &cp.input_file_path());

    let m_rules = AggRules::new_from_path(&cp.m_rule_file_path(), &input_data);
    let bc_rules = AggRules::new_from_path(&cp.bc_rule_file_path(), &input_data);
    let fix_adj_rules = AggRules_adj::new_from_path(&cp.fix_adj_rule_file_path(), &input_data);
    let var_adj_rules = AggRules_adj::new_from_path(&cp.var_adj_rule_file_path(), &input_data);
    let (ftp_rates, lock_adjs) = ftp_rates_reader::read_ftp_rates(&cp.ftp_rates_file_path());

    let adj_rates = read_adjustments::read_adj_rates(&cp.adj_rate_file_path());
    // Todo: Problem point
    let avg_bal = amb_file_reader::read_avg_bal(&cp.amb_file_path(), &log);
    let aggr_bal = read_file(&cp.aggr_file_path());

    LOG_PARAMS.set_once_diagnostic_level(cp.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(cp.is_perf_diagnostics_enabled());

    let out_path = format!("{}_spread.txt", cp.output_file_path());
    let spread_writer = match buf_file_wrtr(&out_path, None) {
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
        m_rules,
        bc_rules,
        fix_adj_rules,
        var_adj_rules,
        input_data,
        input_field_names,
        ftp_rates,
        lock_adjs,
        adj_rates,
        avg_bal,
        spread_writer,
        aggr_bal,
    };
}

fn process_records(ftp_parameters: &mut FtpParameters) -> (String) {
    let mut total_account_with_cf = 0;

    let mut output_bal_less_than_2cr: String = String::new();
    let mut output_bal_between_2cr_5cr: String = String::new();
    let mut output_bal_greater_5cr: String = String::new();
    //Header for output
    let mut op_hdr = "account_number|cust_name|average_balance|accr_int|yld_to_call|int_rate|base_rate|final_ftp_rate|value_date|maturity_date|nxt_rep_dt|lst_rep_dt|mis1|mis2|psl_code|prod_type|rate_flag|branch|source_file_name|ccy|gl|cust_id|final_ftp_amt|alm_line|trade_dt|orig_bal|outstanding_bal|base_rate|adj1|adj2|adj3|adj4|adj5|adj6|input_benchmark|pdo|npa|method|rate_curve|org_tenor|rep_tenor|fx_spread|var_spread|first_ftp|bc_as_on_rule|tenor_start_date_rule|tenor_end_date_rule|bc_as_on_applied|tenor_start_date_applied|tenor_end_date_applied|two_point_concat|four_point_concat\n";
    output_bal_less_than_2cr.push_str(&op_hdr);
    output_bal_between_2cr_5cr.push_str(&op_hdr);
    output_bal_greater_5cr.push_str(&op_hdr);

    let mut saved_bm_rates: HashMap<BmKey, Vec<IntermediateBmPoints>> = HashMap::new();

    for mut account_input in ftp_parameters.input_data.iter() {
        let cf_read_timer = Instant::now();
        total_account_with_cf += 1;
        //Read cashflow
        let cf_read_time = cf_read_timer.elapsed();

        let outstanding_bal = match account_input
            .get_f64_for_key(&ftp_parameters.input_field_names.current_book_balance)
        {
            Ok(result) => result,
            Err(_) => DEFAULT_FLOAT,
        };

        let calc_ftp_timer = Instant::now();
        //calculate FTP
        let (out_str, out_type) = proc_ftp::calculate_ftp(
            &mut account_input,
            &ftp_parameters.m_rules,
            &ftp_parameters.bc_rules,
            &ftp_parameters.fix_adj_rules,
            &ftp_parameters.var_adj_rules,
            ftp_parameters.cp.bc_file_path().to_string(),
            &ftp_parameters.input_field_names,
            &ftp_parameters.log,
            &ftp_parameters.diag_log,
            ftp_parameters.cp.ftprunid(),
            ftp_parameters.cp.from_date(),
            ftp_parameters.cp.to_date(),
            &ftp_parameters.cp.output_file_path(),
            &mut ftp_parameters.ftp_rates,
            &ftp_parameters.lock_adjs,
            &ftp_parameters.adj_rates,
            &ftp_parameters.avg_bal,
            &ftp_parameters.cp.ftp_rates_file_path(),
            ftp_parameters.cp.default_method(),
            ftp_parameters.cp.default_basecurve(),
            ftp_parameters.cp.fixed_adj_count(),
            ftp_parameters.cp.var_adj_count(),
            &mut saved_bm_rates,
            &mut ftp_parameters.spread_writer,
            ftp_parameters.cp.rate_precision(),
            ftp_parameters.cp.bal_precision(),
            &ftp_parameters.input_field_names,
            &ftp_parameters.aggr_bal,
        );
        let calc_ftp_time = calc_ftp_timer.elapsed();

        let acc_writer_timer = Instant::now();

        match out_type.as_str() {
            "less_than_two" => {
                output_bal_less_than_2cr.push_str(&out_str);
            }
            "two_to_five" => {
                output_bal_between_2cr_5cr.push_str(&out_str);
            }
            _ => {
                output_bal_greater_5cr.push_str(&out_str);
            }
        }

        let acc_writer_time = acc_writer_timer.elapsed();
    }
    //Footer for output
    let mut op_ftr = format!(
        "{}|{}",
        ftp_parameters.cp.from_date(),
        total_account_with_cf
    );

    output_bal_less_than_2cr.push_str(&op_ftr);
    output_bal_between_2cr_5cr.push_str(&op_ftr);
    output_bal_greater_5cr.push_str(&op_ftr);

    let start = Instant::now();

    let out_path_less_than_2cr = format!("{}_lt_2cr.txt", ftp_parameters.cp.output_file_path());
    let out_path_between_2cr_5cr =
        format!("{}_2cr_bt_5cr.txt", ftp_parameters.cp.output_file_path());
    let out_path_gt_5cr = format!("{}_gt_5cr.txt", ftp_parameters.cp.output_file_path());

    let mut out_writer = match buf_file_wrtr(&out_path_less_than_2cr, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            out_path_less_than_2cr,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match out_writer.write_all(output_bal_less_than_2cr.as_bytes()) {
        Ok(_) => println!("Successfully processed all TD accounts ."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`.{}",
            out_path_less_than_2cr, error
        ),
    }

    let mut out_writer = match buf_file_wrtr(&out_path_between_2cr_5cr, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            out_path_between_2cr_5cr,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match out_writer.write_all(output_bal_between_2cr_5cr.as_bytes()) {
        Ok(_) => println!("Successfully processed all TD accounts ."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`.{}",
            out_path_between_2cr_5cr, error
        ),
    }

    let mut out_writer = match buf_file_wrtr(&out_path_gt_5cr, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            out_path_gt_5cr,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match out_writer.write_all(output_bal_greater_5cr.as_bytes()) {
        Ok(_) => println!("Successfully processed all TD accounts ."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`.{}",
            out_path_gt_5cr, error
        ),
    }

    let report_string = format!("Accounts With Cashflows: {}", total_account_with_cf);

    let end = start.elapsed();

    return report_string;
}

fn rpt_n_exit(p: cp::CP, out_log: String, log: &Logger) {
    log_debug!(log, "{}", out_log);
}
