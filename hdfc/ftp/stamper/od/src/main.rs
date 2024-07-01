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
#[macro_use]
extern crate serde_derive;

use ftp_parameters::FtpParameters;
use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_agg_rules_adj::agg_rules::AggRules_adj;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use stamp_ftp::cfinput::AccFieldNames;
mod ftp_parameters;
mod proc_ftp;
pub mod stamp_ftp;
mod writer;
use health_report::HealthReport;
use sdb_io::buf_file_wrtr;
use stamp_ftp::bm_reader::{BmKey, IntermediateBmPoints};
use stamp_ftp::read_adjustments;
use stamp_ftp::restructured_op::additional_struct::AmbData;
use stamp_ftp::restructured_op::calc_restructured_ftp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env::current_dir;
use std::fs;
use std::io::prelude::*;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    //Initialization
    let mut ftp_parameters = initialize();

    //Process input records
    let log_output = process_records(&mut ftp_parameters);

    //log the output and exit
    rpt_n_exit(ftp_parameters.cp, log_output, &ftp_parameters.log);

    let duration = start.elapsed();

    println!("Total time to process in: {:?}", duration);
    let log_str = format!("Total time to process OD: {:?}", duration);
    log_info!(&ftp_parameters.log, "{}", log_str);
}

//Initialize all command line parameters , Read input cashflow file and rule files.
fn initialize() -> FtpParameters {
    let app_name = "ftp-stamper-od";

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

    let adj_rates = read_adjustments::read_adj_rates(&cp.adj_rate_file_path());

    //Reading Amb file
    let mut amb_map: HashMap<String, AmbData> = HashMap::new();
    let od_amb_file = match sdb_io::new_buf_rdr(cp.amb_file_path()) {
        Ok(r) => r,
        Err(e) => panic!(format!(
            "Cannot read file at path: '{}', Error: '{}'",
            cp.amb_file_path(),
            e
        )),
    };

    for (line_num, line) in od_amb_file.lines().enumerate() {
        let data = match line {
            Ok(curr_info) => curr_info,
            Err(error) => {
                continue;
            }
        };
        let fields: Vec<&str> = data.split("|").collect();
        let mut amb_data: AmbData = AmbData::new(cp.amb_file_path(), &fields, line_num + 1);
        let vf_casa_scct_num = amb_data.vf_casa_acct_number.clone();
        amb_map.insert(vf_casa_scct_num, amb_data);
    }

    //Reading Config file
    let mut config_map: HashMap<String, HashSet<String>> = HashMap::new();
    let config_file = match sdb_io::new_buf_rdr(cp.adjustment_config_file_path()) {
        Ok(r) => r,
        Err(e) => panic!(
            "Cannot read file at path: '{}', Error: '{}'",
            cp.adjustment_config_file_path(),
            e
        ),
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

    //TODO:: read Adjustment rules

    LOG_PARAMS.set_once_diagnostic_level(cp.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(cp.is_perf_diagnostics_enabled());

    return FtpParameters {
        cp,
        log,
        diag_log,
        input_data,
        input_reader,
        input_field_names,
        m_rules,
        bc_rules,
        fix_adj_rules,
        var_adj_rules,
        adj_rates,
        amb_map,
        config_map,
    };
}

fn process_records(ftp_parameters: &mut FtpParameters) -> String {
    let mut total_account_with_cf = 0;
    let total_bal_ip = 0.0;
    let mut output: String = String::new();
    let mut saved_bm_rates: HashMap<BmKey, Vec<IntermediateBmPoints>> = HashMap::new();
    //Header for output
    let op_hdr = "account_number|cust_name|average_balance|accr_int|yld_to_call|int_rate|base_rate|final_ftp_rate|value_date|maturity_date|lst_rep_dt|nxt_rep_dt|mis1|mis2|psl_code|prod_type|rate_flag|branch|source_file_name|ccy|gl|cust_id|final_ftp_amt|alm_line|trade_dt|orig_bal|outstanding_bal|base_rate|adj1|adj2|adj3|adj4|adj5|adj6|input_benchmark|pdo|npa|method|rate_curve|org_tenor|rep_tenor|fx_spread|var_spread|first_ftp|bc_as_on_rule|tenor_start_date_rule|tenor_end_date_rule|bc_as_on_applied|tenor_start_date_applied|tenor_end_date_applied|alm_concat|two_point_concat\n";
    output.push_str(&op_hdr);

    let mut additional_op: String = String::new();
    let add_op_hdr = "FTP MONTH|AccNum|CustName|AvgBal|Accr_Int/Interest income|Accr_Int_rate|YldToCall|IntRate|BaseRate |ADJ LP |ADJ TP|ADJ PCFC|ADJ PSL|ADJ EWS|ADJ SMF|ADJ MO|ADJ 8|ADJ 9|ADJ 10|FinalFTPRate|FTP rate without PSL|Margin Rate|Base­_tpr_amount|FinalFTPAmount|FTP Amt without PSL |PSL Amount |Total LP Amount|Total PSL Amount without EWS & SMF|Total EWS Amount|Total SMF Amount|Margin Amount|ValueDate|MaturityDate|LastRepriceDate|NextRepriceDate|MIS1|MIS2|PSLCode|ProdCode|RateFlag|branch|SourceFileName|Currency|GLCode|CustId|AlmLine|TradeDate|InitialDepAmt|CurrentOutstanding|InputBenchmark|PDO|NPA|FTPMethod|FTPRateCurve|OrgTenor|RepricingTenor|FixedSpread|VariableSpread|FirstMonthFTP|bc_as_on_rule|tenor_start_date_rule|tenor_end_rate_rule|bc_as_on_applied|tenor_start_date_applied|tenor_end_date_applied|Concat 4 point|Concat 2 point|EWS Flag|BDP Division |BDP COA|ADJ ID LP|ADJ ID TP|ADJ ID PCFC|ADJ ID PSL|ADJ ID EWS|ADJ ID SMF|ADJ ID MO|ADJ ID 8|ADJ ID 9|ADJ ID10\n";
    additional_op.push_str(&add_op_hdr);

    for mut account_input in ftp_parameters.input_data.iter() {
        total_account_with_cf += 1;

        let ews_flag: String = account_input
            .get_string_for_key(&ftp_parameters.input_field_names.ews_flag)
            .unwrap_or(&String::default())
            .to_string();

        let bdp_division = account_input
            .get_string_for_key(&ftp_parameters.input_field_names.bdp_division)
            .unwrap_or(&String::default())
            .to_string();

        let bdp_coa = account_input
            .get_string_for_key(&ftp_parameters.input_field_names.bdp_coa)
            .unwrap_or(&String::default())
            .to_string();
        //calculate FTP
        let (out_str, adj_str) = proc_ftp::calculate_ftp(
            &ftp_parameters.input_reader,
            &mut account_input,
            &ftp_parameters.input_field_names,
            &ftp_parameters.log,
            &ftp_parameters.diag_log,
            ftp_parameters.cp.from_date(),
            ftp_parameters.cp.to_date(),
            &ftp_parameters.m_rules,
            &ftp_parameters.bc_rules,
            &ftp_parameters.fix_adj_rules,
            &ftp_parameters.var_adj_rules,
            ftp_parameters.cp.bc_file_path().to_string(),
            ftp_parameters.cp.default_method(),
            ftp_parameters.cp.default_basecurve(),
            ftp_parameters.cp.fixed_adj_count(),
            ftp_parameters.cp.var_adj_count(),
            &mut saved_bm_rates,
            ftp_parameters.cp.rate_precision(),
            ftp_parameters.cp.bal_precision(),
            &ftp_parameters.adj_rates,
            ftp_parameters.cp.is_closed(),
            &ftp_parameters.amb_map,
        );

        //calculate restructured output

        let add_op_str = calc_restructured_ftp(
            out_str.clone(),
            &ftp_parameters.amb_map,
            &ftp_parameters.config_map,
            &ftp_parameters.cp,
            &ftp_parameters.adj_rates,
            ews_flag,
            bdp_division,
            bdp_coa,
            adj_str,
            ftp_parameters.cp.from_date(),
            ftp_parameters.cp.to_date(),
        );
        output.push_str(&out_str);
        additional_op.push_str(&add_op_str);
    }
    //Footer for output
    let mut op_ftr = format!(
        "FTR|{}|{}\n",
        ftp_parameters.cp.from_date(),
        total_account_with_cf
    );
    output.push_str(&op_ftr);
    let out_path = format!("{}.txt", &ftp_parameters.cp.output_file_path());
    let addtional_op_path = format!("{}_additional.txt", &ftp_parameters.cp.output_file_path());

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
        Ok(_) => println!("Successfully processed all OD accounts ."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`.{}",
            out_path, error
        ),
    }

    let mut add_out_writer = match buf_file_wrtr(&addtional_op_path, None) {
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

    match add_out_writer.write_all(additional_op.as_bytes()) {
        Ok(_) => println!("Successfully processed all OD accounts ."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`.{}",
            out_path, error
        ),
    }

    let report_string = format!("total Accounts: {}", total_account_with_cf);

    let health_report = HealthReport::new(
        total_account_with_cf,
        total_account_with_cf,
        0,
        total_bal_ip,
        total_bal_ip,
        0,
    );
    health_report.gen_health_rpt(&ftp_parameters.cp.output_file_path());

    return report_string;
}

fn rpt_n_exit(_: cp::CP, out_log: String, log: &Logger) {
    log_info!(log, "{}", out_log);
    println!("{}", out_log);
}
