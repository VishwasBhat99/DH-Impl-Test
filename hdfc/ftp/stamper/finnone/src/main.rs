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
#[macro_use]
extern crate serde_derive;

use ftp_parameters::FtpParameters;
use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_agg_rules_adj::agg_rules::AggRules_adj;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use stamp_ftp::amb_file_reader;
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::read_cashflow;
use writer::AccountWithCashflowsWriter;
mod ftp_parameters;
mod proc_ftp;
pub mod stamp_ftp;
mod writer;
use amb_file_reader::AverageBalance;
use sdb_io::buf_file_wrtr;
use stamp_ftp::bm_reader::{BmKey, IntermediateBmPoints};
use stamp_ftp::ftp_rates_reader;
use stamp_ftp::read_adjustments;
use std::collections::HashMap;
use std::collections::HashSet;
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

    println!("Total time to process Finnone Loans: {:?}", duration);
    let log_str = format!("Total time to process Finnone Loans: {:?}", duration);
    log_info!(&ftp_parameters.log, "{}", log_str);
}

//Initialize all command line parameters , Read input cashflow file and rule files.
fn initialize() -> FtpParameters {
    let app_name = "ftp-stamper-finnone-loans";

    //Initializing all configuration parameters

    let cp = cp::get_cp(app_name);

    let (log, diag_log) = log::setup_loggers(&cp.log_file_path(), &cp.diagnostics_file_path());
    cp.log_parameters(&log);

    let input_field_names = AccFieldNames::get_input_fields_names(&cp.req_fields_file());
    let input_data = reader::Reader::new_at_path(&cp.meta_data_file_path(), &cp.input_file_path());
    let input_reader =
        reader::Reader::new_at_path(&cp.meta_data_file_path(), &cp.input_file_path());

    let m_rules = AggRules::new_from_path(&cp.m_rule_file_path(), &input_data);
    let bc_rules = AggRules::new_from_path(&cp.bc_rule_file_path(), &input_data);
    let fix_adj_rules = AggRules_adj::new_from_path(&cp.fix_adj_rule_file_path(), &input_data);
    let var_adj_rules = AggRules_adj::new_from_path(&cp.var_adj_rule_file_path(), &input_data);
    let (ftp_rates, lock_adjs) = ftp_rates_reader::read_ftp_rates(&cp.ftp_rates_file_path());

    let adj_rates = read_adjustments::read_adj_rates(&cp.adj_rate_file_path());
    // Todo: Problem point
    let avg_bal = amb_file_reader::read_avg_bal(&cp.amb_file_path());

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
        input_reader,
        input_field_names,
        ftp_rates,
        lock_adjs,
        adj_rates,
        avg_bal,
        spread_writer,
    };
}

fn process_records(ftp_parameters: &mut FtpParameters) -> (String) {
    let mut total_account_with_cf = 0;
    let mut output_bal_less_than_15k: String = String::new();
    let mut output_bal_greater_than_15lac: String = String::new();
    let mut output_bal_between_15k_45k: String = String::new();
    let mut output_bal_between_45k_15lac: String = String::new();
    let mut restructured_output_less_than_15k = String::new();
    let mut restructured_output_greater_than_15lac = String::new();
    let mut restructured_output_between_15k_45k = String::new();
    let mut restructured_output_between_45k_15lac = String::new();

    let mut cashflow_out: String = String::new();
    let mut output_writer =
        create_io_workers(&ftp_parameters.cp.output_file_path(), &ftp_parameters.log);
    let mut saved_bm_rates: HashMap<BmKey, Vec<IntermediateBmPoints>> = HashMap::new();
    // Config File
    let mut config_map: HashMap<String, HashSet<String>> = HashMap::new();
    let config_file = match sdb_io::new_buf_rdr(ftp_parameters.cp.adjustment_config_file_path()) {
        Ok(r) => r,
        Err(e) => panic!(format!(
            "Cannot read file at path: '{}', Error: '{}'",
            ftp_parameters.cp.adjustment_config_file_path(),
            e
        )),
    };
    for (line_num, line) in config_file.lines().enumerate() {
        let data = match line {
            Ok(curr_info) => curr_info,
            Err(error) => {
                continue;
            }
        };
        let fields: Vec<&str> = data.split("|").collect();

        let id_str = fields[1].to_string();
        let ids: Vec<&str> = id_str.split(',').collect();
        let mut id_set: HashSet<String> = HashSet::new();
        for id in ids {
            id_set.insert(id.to_string());
        }
        config_map.insert(fields[0].to_string(), id_set);
    }

    // spread map;
    let mut spread_map: HashMap<String,String> = HashMap::new();
    let ftp_rates_file = match sdb_io::new_buf_rdr(ftp_parameters.cp.ftp_rates_file_path()) {
        Ok(r) => r,
        Err(e) => panic!(format!(
            "Cannot read file at path: '{}', Error: '{}'",
            ftp_parameters.cp.ftp_rates_file_path(),
            e
        )),
    };
    for (line_num, line) in ftp_rates_file.lines().enumerate() {
        let data = match line {
            Ok(curr_info) => curr_info,
            Err(error) => {
                continue;
            }
        };
        let fields: Vec<&str> = data.split("|").collect();
        spread_map.insert(fields[0].to_string(), fields[10].to_string());
    }


    //Header for output
    let add_op_hdr = "FTP MONTH|Scheme ID|BDP|COA|AccNum|CustName|AvgBal|Accr_Int/Interest income|YldToCall|IntRate|BaseRate|FinalFTPRate|GR_OFS_GL_CODE|UI_OFS_GL_CODE|RE_OFS_GL_CODE|IS_OFS_GL_CODE|GR_OFS_GL_CODE Amt|UI_OFS_GL_CODE  Amt|RE_OFS_GL_CODE  Amt|IS_OFS_GL_CODE  Amt|Int income GL|Overdue Int GL|Int on Cancellation GL|W/off GL|Int income GL amt|Overdue Int GL amt|Int on Cancellation GL amt|W/off GL amt|ValueDate|MaturityDate|LastRepriceDate|NextRepriceDate|MIS1|MIS2|PSLCode|ProdCode|RateFlag|SourceFileName|Currency|GLCode|CustId|AlmLine|TradeDate|InitialDepAmt|CurrentOutstanding|InputBenchmark|PDO|NPA|FTPMethod|FTPRateCurve|OrgTenor|RepricingTenor|FixedSpread|VariableSpread|FirstMonthFTP|EWS Flag|ANCHOR SPREAD|ANCHOR MONTH|FTP WITH PSL Amt|PSL Amt|FTP WITHOUT PSL Amt|Margin Amount|ADJ1/ LP|ADJ2/ TP|ADJ3/ PCFC|ADJ4/ PSL|ADJ5/ EWS|ADJ6/ SMF|ADJ7/ MO|ADJ8|ADJ9|ADJ10|ADJ ID1|ADJ ID2|ADJ ID3|ADJ ID4|ADJ ID5|ADJ ID6|ADJ ID7|ADJ ID8|ADJ ID9|ADJ ID10\n";
    let op_hdr = "account_number|cust_name|average_balance|accr_int|yld_to_call|int_rate|base_rate|final_ftp_rate|value_date|maturity_date|nxt_rep_dt|lst_rep_dt|mis1|mis2|psl_code|prod_type|rate_flag|branch|source_file_name|ccy|gl|cust_id|final_ftp_amt|alm_line|trade_dt|orig_bal|outstanding_bal|base_rate|adj1|adj2|adj3|adj4|adj5|adj6|input_benchmark|pdo|npa|method|rate_curve|org_tenor|rep_tenor|fx_spread|var_spread|first_ftp|bc_as_on_rule|tenor_start_date_rule|tenor_end_date_rule|bc_as_on_applied|tenor_start_date_applied|tenor_end_date_applied\n";
    output_bal_less_than_15k.push_str(&op_hdr);
    output_bal_greater_than_15lac.push_str(&op_hdr);
    output_bal_between_15k_45k.push_str(&op_hdr);
    output_bal_between_45k_15lac.push_str(&op_hdr);
    cashflow_out.push_str(&op_hdr);
    restructured_output_less_than_15k.push_str(&add_op_hdr);
    restructured_output_greater_than_15lac.push_str(&add_op_hdr);
    restructured_output_between_15k_45k.push_str(&add_op_hdr);
    restructured_output_between_45k_15lac.push_str(&add_op_hdr);

    let asondate = ftp_parameters.cp.as_on_date();
    for mut account_input in ftp_parameters.input_data.iter() {
        let cf_read_timer = Instant::now();
        total_account_with_cf += 1;
        //Read cashflow
        let cf_input = read_cashflow(&account_input, &ftp_parameters.input_field_names);
        let cf_read_time = cf_read_timer.elapsed();

        let calc_ftp_timer = Instant::now();
        //calculate FTP
        let (out_str, one_acc_op, cf_out, add_output_2) = proc_ftp::calculate_ftp(
            &ftp_parameters.input_reader,
            &mut account_input,
            cf_input,
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
            ftp_parameters.cp.is_cf_req(),
            &mut ftp_parameters.spread_writer,
            ftp_parameters.cp.rate_precision(),
            ftp_parameters.cp.bal_precision(),
            &config_map,
            asondate,
            &spread_map
        );
        let calc_ftp_time = calc_ftp_timer.elapsed();

        log_debug!(
            ftp_parameters.log,
            "acc no {}   balance {}",
            one_acc_op.account_number,
            one_acc_op.outstanding_bal
        );
        let acc_writer_timer = Instant::now();
        if one_acc_op.outstanding_bal <= 15000.00 {
            output_bal_less_than_15k.push_str(&out_str);
            restructured_output_less_than_15k.push_str(&add_output_2);
        } else if one_acc_op.outstanding_bal > 15000.00 && one_acc_op.outstanding_bal <= 45000.00 {
            output_bal_between_15k_45k.push_str(&out_str);
            restructured_output_between_15k_45k.push_str(&add_output_2);
        } else if one_acc_op.outstanding_bal > 45000.00 && one_acc_op.outstanding_bal <= 150000.00 {
            output_bal_between_45k_15lac.push_str(&out_str);
            restructured_output_between_45k_15lac.push_str(&add_output_2);
        } else {
            output_bal_greater_than_15lac.push_str(&out_str);
            restructured_output_greater_than_15lac.push_str(&add_output_2)
        }
        cashflow_out.push_str(&cf_out);
        let acc_writer_time = acc_writer_timer.elapsed();
    }
    //Footer for output
    let op_ftr = format!(
        "{}|{}\n",
        ftp_parameters.cp.from_date(),
        total_account_with_cf
    );

    output_bal_less_than_15k.push_str(&op_ftr);
    output_bal_greater_than_15lac.push_str(&op_ftr);
    output_bal_between_15k_45k.push_str(&op_ftr);
    output_bal_between_45k_15lac.push_str(&op_ftr);    
    restructured_output_less_than_15k.push_str(&op_ftr);
    restructured_output_greater_than_15lac.push_str(&op_ftr);
    restructured_output_between_15k_45k.push_str(&op_ftr);
    restructured_output_between_45k_15lac.push_str(&op_ftr);

    cashflow_out.push_str(&op_ftr);
    let start = Instant::now();
    let cf_out_path = format!("{}_cf.txt", &ftp_parameters.cp.output_file_path());

    let out_path_less_than_15k = format!("{}_lt_15k.txt", ftp_parameters.cp.output_file_path());
    let out_path_between_15k_45k =
        format!("{}_15k_bt_45k.txt", ftp_parameters.cp.output_file_path());
    let out_path_between_45k_15lakh = format!(
        "{}_45k_bt_1.5lakh.txt",
        ftp_parameters.cp.output_file_path()
    );
    let out_path_grtr_than_15lakh =
        format!("{}_gt_1.5lakh.txt", ftp_parameters.cp.output_file_path());

    let restructured_path_less_than_15k = format!("{}_additional_lt_15k.txt",ftp_parameters.cp.output_file_path());
    let restructured_path_greater_than_15lac = format!("{}_additional_gt_1.5lakh.txt",ftp_parameters.cp.output_file_path());
    let restructured_path_between_15k_45k = format!("{}_additional_15k_bt_45k.txt",ftp_parameters.cp.output_file_path());
    let restructured_path_between_45k_15lac = format!("{}_additional_45k_bt_1.5lakh.txt",ftp_parameters.cp.output_file_path());

    let mut out_writer = match buf_file_wrtr(&out_path_less_than_15k, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            out_path_less_than_15k,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match out_writer.write_all(output_bal_less_than_15k.as_bytes()) {
        Ok(_) => println!("Successfully processed all finnone loans accounts ."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`.{}",
            out_path_less_than_15k, error
        ),
    }

    let mut out_writer = match buf_file_wrtr(&out_path_between_15k_45k, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            out_path_between_15k_45k,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match out_writer.write_all(output_bal_between_15k_45k.as_bytes()) {
        Ok(_) => println!("Successfully processed all finnone loans accounts ."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`.{}",
            out_path_between_15k_45k, error
        ),
    }

    let mut out_writer = match buf_file_wrtr(&out_path_between_45k_15lakh, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            out_path_between_45k_15lakh,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match out_writer.write_all(output_bal_between_45k_15lac.as_bytes()) {
        Ok(_) => println!("Successfully processed all finnone loans accounts ."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`.{}",
            out_path_between_45k_15lakh, error
        ),
    }

    let mut out_writer = match buf_file_wrtr(&out_path_grtr_than_15lakh, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            out_path_grtr_than_15lakh,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match out_writer.write_all(output_bal_greater_than_15lac.as_bytes()) {
        Ok(_) => println!("Successfully processed all finnone loans accounts ."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`.{}",
            out_path_grtr_than_15lakh, error
        ),
    }

    //restructured output writer:
    let mut restructured_out_writer = match buf_file_wrtr(&restructured_path_less_than_15k, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            out_path_grtr_than_15lakh,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match restructured_out_writer.write_all(restructured_output_less_than_15k.as_bytes()) {
        Ok(_) => println!("Successfully processed all finnone loans accounts."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`.{}",
            out_path_grtr_than_15lakh, error
        ),
    }

    let mut restructured_out_writer = match buf_file_wrtr(&restructured_path_between_15k_45k, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            out_path_grtr_than_15lakh,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match restructured_out_writer.write_all(restructured_output_between_15k_45k.as_bytes()) {
        Ok(_) => println!("Successfully processed all finnone loans accounts."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`.{}",
            out_path_grtr_than_15lakh, error
        ),
    }

    let mut restructured_out_writer = match buf_file_wrtr(&restructured_path_between_45k_15lac, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            out_path_grtr_than_15lakh,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match restructured_out_writer.write_all(restructured_output_between_45k_15lac.as_bytes()) {
        Ok(_) => println!("Successfully processed all finnone loans accounts."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`.{}",
            out_path_grtr_than_15lakh, error
        ),
    }

    let mut restructured_out_writer = match buf_file_wrtr(&restructured_path_greater_than_15lac, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            out_path_grtr_than_15lakh,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match restructured_out_writer.write_all(restructured_output_greater_than_15lac.as_bytes()) {
        Ok(_) => println!("Successfully processed all finnone loans accounts."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`.{}",
            out_path_grtr_than_15lakh, error
        ),
    }


    let mut cf_out_writer = match buf_file_wrtr(&cf_out_path, None) {
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

    match cf_out_writer.write_all(cashflow_out.as_bytes()) {
        Ok(_) => println!("Successfully processed all finnone loans accounts ."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`.{}",
            cf_out_path, error
        ),
    }

    let report_string = format!("Accounts With Cashflows: {}", total_account_with_cf);

    let end = start.elapsed();
    println!("time to write text file :{:?}", end);

    return report_string;
}

fn rpt_n_exit(p: cp::CP, out_log: String, log: &Logger) {
    log_debug!(log, "{}", out_log);
}

fn create_io_workers(output_path: &str, log: &Logger) -> (AccountWithCashflowsWriter) {
    let writer = AccountWithCashflowsWriter::new(output_path, log);

    return writer;
}
