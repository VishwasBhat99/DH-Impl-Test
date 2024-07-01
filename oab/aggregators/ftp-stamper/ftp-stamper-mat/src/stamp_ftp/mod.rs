mod account_with_cashflows;
mod account_with_cashflows_writer;
mod avg_bal_reader;
mod bm_reader;
mod calc_ftp;
mod cashflow_appender;
mod currency;
mod format_output;
mod ftp_rates_reader;
mod io;
mod process_ftp;
mod read_adjustments;
mod required_fields;
mod rule_stamper;

use self::avg_bal_reader::read_avg_bal;
use self::format_output::get_output_string;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_agg_rules_adj::agg_rules::AggRules_adj;
use sdb_dyn_proto_rdr::reader;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use stamp_ftp::account_with_cashflows_writer::AccountWithCashflowsWriter;
use stamp_ftp::cashflow_appender::append_cashflow;
use stamp_ftp::required_fields::RequiredFields;
use statics::*;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;

pub fn process_records(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut total_acc_encountered = DEFAULT_INT;
    let mut total_acc_skipped = DEFAULT_INT;
    let mut total_amt_output = DEFAULT_FLOAT;
    let mut total_acc_with_cf = DEFAULT_INT;
    let start_generator_timer = SystemTime::now();
    let mut output: String = String::new();

    let required_fields = RequiredFields::new_from_path(config_params.req_fields_file_path());

    let mut input_data = reader::Reader::new_at_path(
        &config_params.meta_data_file_path(),
        &config_params.input_file_path(),
    );
    let avg_bal = read_avg_bal(config_params.avg_bal_file(), log);
    let mut total_amt_input = DEFAULT_FLOAT;
    for amount in avg_bal.values() {
        total_amt_input += amount;
    }

    let m_rules = AggRules::new_from_path(&config_params.m_rule_file_path(), &input_data);
    let bc_rules = AggRules::new_from_path(&config_params.bc_rule_file_path(), &input_data);
    let adj_rules = AggRules_adj::new_from_path(&config_params.adj_rule_file_path(), &input_data);
    let ftp_rate_lock = ftp_rates_reader::read_ftp_rates(&config_params.ftp_rates_file_path());

    let mut output_writer =
        AccountWithCashflowsWriter::new(&config_params.output_file_path(), &log);

    for mut account_input in input_data.iter() {
        total_acc_encountered += 1;

        //Read cashflow
        let cf_input = append_cashflow(&mut account_input, &required_fields, log);
        total_acc_with_cf += 1;

        //calculate FTP
        let ftp_output = process_ftp::calculate_ftp(
            &mut account_input,
            cf_input,
            &m_rules,
            &bc_rules,
            &adj_rules,
            &config_params,
            &required_fields,
            log,
            diag_log,
            &ftp_rate_lock,
            &avg_bal,
        );
        total_amt_output += ftp_output.balance_ccy;

        //Generate Output String
        let out_str = get_output_string(&ftp_output);
        if out_str.is_empty() {
            total_acc_skipped += 1;
        }
        output.push_str(&out_str);

        //write output
        output_writer.write(ftp_output);
    }

    let out_path = format!("{}.txt", &config_params.output_file_path());
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
        Ok(_) => println!("Successfully processed all accounts."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`.{}",
            out_path, error
        ),
    }
    let end_generator_timer = SystemTime::now();
    let total_duration = end_generator_timer
        .duration_since(start_generator_timer)
        .expect("Could not calculate total duration.");
    let report_string = format!(
        "Accounts Encountered: {}\n\
         Total Duration: {:?}\n\
         Total amount in input: {:?}\n\
         Total amount in output: {:?}",
        total_acc_encountered, total_duration, total_amt_input, total_amt_output
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);

    let health_report = HealthReport::new(
        total_acc_encountered,
        total_acc_encountered - total_acc_skipped,
        total_acc_skipped,
        total_amt_input,
        total_amt_output,
        total_acc_with_cf,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}
