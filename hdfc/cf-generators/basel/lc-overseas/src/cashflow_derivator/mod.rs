use slog::Logger;
mod account_appender;
mod account_reader;
mod currency;
mod account_without_cashflows;
mod account_writer;

use self::account_reader::InputAccountReader;
use self::account_writer::AccountWithoutCashflows;
use calamine::{open_workbook, Reader, Xlsx};
use cashflow_derivator::account_appender::create_account_without_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use statics::*;
use std::collections::HashMap;
use std::time::SystemTime;

pub fn generate(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut total_accounts_encountered: i64 = DEFAULT_INT;
    let mut total_amount_in_input = 0.0;
    let mut total_amount_in_output = 0.0;
    let mut errorneous_account = 0;
    let start_generator_timer = SystemTime::now();
    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut reader_iterator = reader.into_iter();
    let currency_converter = currency::create_currency_converter(
        config_params.base_currency(),
        config_params.currency_conversion_file_path(),
    );
    let mut fin_map_excel: Xlsx<_> = open_workbook(config_params.fin_map_ref_path())
        .expect("Error while opening `fin_map_excel`.");
    let mut fin_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = fin_map_excel.worksheet_range(config_params.fin_map_sheet_name()) {
        for row in reader.rows() {
            let fin_val = row[2].to_string();
            fin_map.insert(row[0].to_string(), fin_val);
        }
    }
    let mut interelimination_excel: Xlsx<_> =
        open_workbook(config_params.interelimination_ref_path())
            .expect("Error while opening `interelimination excel`.");
    let mut interelimination_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) =
        interelimination_excel.worksheet_range(config_params.interelimination_sheet_name())
    {
        for row in reader.rows() {
            let elemination_value = row[3].to_string();
            interelimination_map.insert(row[0].to_string(), elemination_value);
        }
    }
    let mut ora_gl_map_excel: Xlsx<_> = open_workbook(config_params.ora_gl_map_ref_path())
        .expect("Error while opening `ora_gl_map_excel`.");
    let mut ora_gl_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) =
        ora_gl_map_excel.worksheet_range(config_params.ora_gl_map_sheet_name())
    {
        for row in reader.rows() {
            let ora_gl_val = row[1].to_string();
            let ora_gl_key = row[0].to_string();
            ora_gl_map.insert(ora_gl_key, ora_gl_val);
        }
    }
    loop {
        let account_opt = log_measurements!(
            diag_log,
            [format!(
                "Type: ReadParseInputAccount, Identifier: {}",
                total_accounts_encountered
            )],
            reader_iterator.next()
        );

        if account_opt.is_none() {
            break;
        }
        let input_account = account_opt.expect("Unexpected error occured.");
        // Get rid of this total_accounts_encountered
        total_accounts_encountered += 1;
        total_amount_in_input += input_account.amt;
        if input_account.exp_dt.is_none() {
            errorneous_account += 1;
            continue;
        }
        let exp_date = &input_account.exp_dt.expect("Unexpected Error.");
        let tenor = if exp_date > config_params.as_on_date() {
            rbdate::num_days_start_to_end(*config_params.as_on_date(), *exp_date)
        } else {
            rbdate::num_days_start_to_end(*exp_date, *config_params.as_on_date()) * -1
        };
        let natural_gl: String = match ora_gl_map.get(&input_account.natural_acc) {
            Some(val) => val.trim().to_string(),
            None => "NONE".to_string(),
        };
        let acc_fin_val = match fin_map.get(&natural_gl) {
            Some(val) => val,
            None => "NONE",
        };
        let interelemination_flg = match interelimination_map.get(&input_account.ref_no) {
            Some(_val) => "Y",
            None => "N",
        };
        let consol_amt =
            currency_converter.get_consol_data(config_params.input_currency(), &input_account.amt);

        let cf_data = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: {}",
                input_account.ref_no
            )],
            create_account_without_cashflows(
                input_account,
                *exp_date,
                tenor,
                acc_fin_val,
                consol_amt,
                interelemination_flg,
                log
            )
        );
        total_amount_in_output += cf_data.amt;
        writer.write(cf_data);
    }

    writer.close();

    let end_generator_timer = SystemTime::now();
    let total_duration = end_generator_timer
        .duration_since(start_generator_timer)
        .expect("Could not calculate total duration.");
    let report_string = format!(
        "Accounts Encountered: {}\n\
         Errorneous Accounts: {}\n\
         Total Duration: {:?}\n\
         Total outstanding amount in input: {:?}\n\
         Total outstanding amount in output: {:?}",
        total_accounts_encountered,
        errorneous_account,
        total_duration,
        total_amount_in_input,
        total_amount_in_output
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);

    let health_stat = HealthReport::new(
        total_accounts_encountered,
        total_accounts_encountered,
        0,
        total_amount_in_input,
        total_amount_in_output,
        0,
    );

    health_stat.gen_health_rpt(config_params.output_file_path())
}

fn create_io_workers(
    input_path: &str,
    output_path: &str,
    log: &Logger,
) -> (InputAccountReader, AccountWithoutCashflows) {
    let reader = InputAccountReader::new(input_path, log);
    let writer = AccountWithoutCashflows::new(output_path, log);

    (reader, writer)
}
