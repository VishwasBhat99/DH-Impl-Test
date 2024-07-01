use cashflow_derivator::account_reader::InputAccountReader;
use cashflow_derivator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use sdb_io::new_buf_rdr;
use slog::Logger;
use rbdate::{DateParser, NaiveDate};

mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_appender;
mod der_cashflows;

use cashflow_derivator::cashflow_appender::create_account_with_cashflows;
use cashflow_derivator::der_cashflows::derive_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use statics::*;
use std::collections::HashMap;
use std::io::BufRead;
use std::time::SystemTime;

#[derive(Clone,Debug)]
pub struct DealData{
    deal_dt:NaiveDate,
    buy_quantity:f64,
    buy_amount:f64
}

pub fn derive(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
    let mut tot_acc_encntrd: i64 = 0;
    let mut tot_acc_with_cfs: i64 = 0;
    let mut tot_amt_inp= 0.0;
    let mut tot_amt_out=0.0;
    let mut tot_cfs: usize = 0;
    let mut skpd_acc = DEFAULT_INT;
    let start_time = SystemTime::now();

    //read deal_listing file:
    let deal_list_file = match new_buf_rdr(config_params.deal_listing_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found deal_listing_file: `{}`.error:{}",
            config_params.deal_listing_file_path(),
            error
        ),
    };
    let mut deal_list_map: HashMap<String, Vec<DealData>> = HashMap::new();
    for (line_num, lines) in deal_list_file.lines().enumerate().skip(1) {
        let deal_lines = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.deal_listing_file_path(),
                line_num + 1,
                error
            ),
        };
        let deal_fields = deal_lines
            .split("|")
            .collect::<Vec<&str>>();

        let portfolio = deal_fields[7].replace("_INV","");
        let sec_num = deal_fields[10].replace("ISIN - ","");
        let deal_key = format!("{}-{}",sec_num,portfolio);
        let deal_dt = date_parser.parse(deal_fields[2]);
        let buy_sell_flag = deal_fields[5].trim().to_string();
        let buy_quantity = deal_fields[12].parse::<f64>().unwrap_or(0.0);
        let buy_amount = deal_fields[15].parse::<f64>().unwrap_or(0.0);
        let curr_deal_data = DealData{
            deal_dt:deal_dt,
            buy_quantity:buy_quantity,
            buy_amount:buy_amount,
        };
        if buy_sell_flag == "BUY" {
            deal_list_map.entry(deal_key)
            .and_modify(|val| val.push(curr_deal_data.clone()))
            .or_insert_with(|| vec![curr_deal_data]);
        }
    }
    //read main input file:
    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );

    let mut reader_iterator = reader.into_iter();

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
            skpd_acc += 1;
            break;
        }
        tot_acc_encntrd += 1;
        let mut input_account = account_opt.expect("Unable to parse InputAccount struct.");
        if config_params.is_perf_diagnostics_enabled() {
            info!(diag_log, "found input account: {:?}", input_account.concat_deal_id);
        }
        let cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: DeriveCFs, Identifier: {}",
                input_account.concat_deal_id
            )],
            derive_cashflows(&mut input_account, *config_params.as_on_date(), log, &deal_list_map)
        );
        tot_acc_with_cfs += 1;
        tot_cfs += cashflows.len();
        let a_w_cf = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: {}",
                input_account.concat_deal_id
            )],
            create_account_with_cashflows(input_account.clone(), cashflows)
        );

        if config_params.is_perf_diagnostics_enabled() {
            info!(diag_log, "generated account with cash-flow:{:?}", a_w_cf.concat_deal_id);
        }

        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                a_w_cf.concat_deal_id
            )],
            writer.write(a_w_cf)
        );
    }
    writer.close();

    let end_time = SystemTime::now();
    let tot_duration = end_time
        .duration_since(start_time)
        .expect("Could not calculate total duration.");
    let report_string = format!(
        "Accounts Encountered: {}\n\
         Accounts Processed: {}\n\
         Accounts Skipped: {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total Duration: {:?}\n",
        tot_acc_encntrd,
        tot_acc_encntrd - skpd_acc,
        skpd_acc,
        tot_acc_with_cfs,
        tot_cfs,
        tot_duration
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
    let health_report: HealthReport = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skpd_acc,
        skpd_acc,
        tot_amt_inp,
        tot_amt_out,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}

pub fn create_io_workers(
    input_path: &str,
    output_path: &str,
    log: &Logger,
) -> (InputAccountReader, AccountWithCashflowsWriter) {
    let reader = InputAccountReader::new(input_path, log);
    let writer = AccountWithCashflowsWriter::new(output_path, log);

    (reader, writer)
}
