use slog::Logger;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_appender;
mod gen_cashflows;

use self::account_reader::InputAccountReader;
use cashflow_generator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use cashflow_generator::cashflow_appender::create_account_with_cashflows;
use cashflow_generator::gen_cashflows::{generate_cashflows, generate_cashflows_securitised};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use statics::*;
use std::time::SystemTime;
use math::round::half_away_from_zero;

pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut total_accounts_encountered: i64 = DEFAULT_INT;
    let mut total_accounts_with_cashflows: i64 = DEFAULT_INT;
    let mut total_cfs: usize = 0;
    let mut tot_prin_in_in = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let mut tot_int_in_op = DEFAULT_FLOAT;
    let mut tot_prin_in_op_non_hdfc = DEFAULT_FLOAT;
    let mut tot_int_in_op_non_hdfc = DEFAULT_FLOAT;

    let start_generate_timer = SystemTime::now();
    let (reader, mut writer, mut writer_non_hdfc) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut reader_iterator = reader.into_iter();
    loop {
        let account_opt = log_measurements!(
            diag_log,
            [format!(
                "Type: ReadParseInputAccount, Identifier: `{}`",
                total_accounts_encountered
            )],
            reader_iterator.next()
        );

        if account_opt.is_none() {
            break;
        }

        let mut input_account = account_opt.expect("Unable to parse `record`.");
        total_accounts_encountered += 1;
        tot_prin_in_in += input_account.os_loan_bal_lcy
            + input_account.pre_ei_bal_lcy
            + input_account.os_p_bal_due_local_ccy
            + input_account.os_i_bal_due_local_ccy
            + input_account.ei_amt_paid_adv_lcy;
        let cashflows_for_account_result = log_measurements!(
            diag_log,
            [format!(
                "Type: GenCFs, Identifier: `{}`",
                input_account.acc_no
            )],
            generate_cashflows(&mut input_account, config_params, log)
        );

        // for securitized portion
        let cashflows_for_non_hdfc_account_result = log_measurements!(
            diag_log,
            [format!(
                "Type: GenCFs, Identifier: `{}`",
                input_account.acc_no
            )],
            generate_cashflows_securitised(&mut input_account, config_params, log)
        );

        if cashflows_for_account_result.is_err() {
            log_error!(
                log,
                "Cashflows not generated for account: `{}`. Error: {}",
                input_account.acc_no,
                cashflows_for_account_result.expect_err("Unable to unwrap error.");
            );
            continue;
        }

        // for securitised portion
        if cashflows_for_non_hdfc_account_result.is_err() {
            log_error!(
                log,
                "Non hdfc Cashflows not generated for account: `{}`. Error: {}",
                input_account.acc_no,
                cashflows_for_non_hdfc_account_result.expect_err("Unable to unwrap error.");
            );
            continue;
        }
        let cashflows = cashflows_for_account_result.expect("Unable to generate cashflows.");
        let cashflows_non_hdfc = cashflows_for_non_hdfc_account_result
            .expect("Unable to generate cashflows for securitized part.");

        total_accounts_with_cashflows += 1;
        total_cfs += cashflows.len();

        let account_with_cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: `{}`",
                input_account.acc_no
            )],
            create_account_with_cashflows(&input_account, cashflows, *config_params.as_on_date())
        );
        let cfs = &account_with_cashflows.cashflows;
        let mut tot_prin_cf_amt = 0.0;
        for cf in cfs {
            tot_prin_cf_amt += cf.prin_amt;
        }
        let mut outstanding_amount: f64 = ((input_account.os_loan_bal_lcy
            + input_account.pre_ei_bal_lcy)
            * input_account.hdfc_ltd_percent)
            / 100.0;
        outstanding_amount = half_away_from_zero(outstanding_amount, 0);
        if tot_prin_cf_amt != outstanding_amount && tot_prin_cf_amt != 0.0 {
            log_debug!(
                log,
                "acc_no = {}, total_cf_prin_amt = {}, outstanding_amt = {}, Difference = {}",
                input_account.acc_no,
                tot_prin_cf_amt,
                outstanding_amount,
                tot_prin_cf_amt - outstanding_amount
            );
        }

        let account_with_cashflows_non_hdfc = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: `{}`",
                input_account.acc_no
            )],
            create_account_with_cashflows(
                &input_account,
                cashflows_non_hdfc,
                *config_params.as_on_date()
            )
        );

        tot_prin_in_op += account_with_cashflows.tot_prin_amt;
        tot_int_in_op += account_with_cashflows.tot_int_amt;
        tot_prin_in_op_non_hdfc += account_with_cashflows_non_hdfc.tot_prin_amt;
        tot_int_in_op_non_hdfc += account_with_cashflows_non_hdfc.tot_int_amt;

        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                account_with_cashflows.acc_no
            )],
            writer.write(account_with_cashflows)
        );

        log_measurements!(
            diag_log,
            [format!(
                "Type: SecWriteAccWithCFs, Identifier: {}",
                account_with_cashflows_non_hdfc.acc_no
            )],
            writer_non_hdfc.write(account_with_cashflows_non_hdfc)
        );
    }

    writer.close();
    writer_non_hdfc.close();

    let end_generate_timer = SystemTime::now();
    let total_duration = end_generate_timer
        .duration_since(start_generate_timer)
        .expect("Could not calculate total duration for generate timer.");
    let report_string = format!(
        "Accounts Encountered: {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total Duration: {:.2?}\n\
         Total outstanding amount in input: {:.2} \n\
         Total outstanding amount in output: {:.2}\n\
         Total interest amount in output: {:.2}\n\
         Total non-hdfc outstanding amount in output: {:.2}\n\
         Total non-hdfc interest amount in output: {:.2}\n",
        total_accounts_encountered,
        total_accounts_with_cashflows,
        total_cfs,
        total_duration,
        tot_prin_in_in,
        tot_prin_in_op,
        tot_int_in_op,
        tot_prin_in_op_non_hdfc,
        tot_int_in_op_non_hdfc
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);

    let health_stat = HealthReport::new(
        total_accounts_encountered,
        total_accounts_with_cashflows,
        0,
        tot_prin_in_in,
        tot_prin_in_op,
        total_cfs as i64,
    );
    health_stat.gen_health_rpt(config_params.output_file_path())
}

fn create_io_workers(
    input_path: &str,
    output_path: &str,
    log: &Logger,
) -> (
    InputAccountReader,
    AccountWithCashflowsWriter,
    AccountWithCashflowsWriter,
) {
    let reader = InputAccountReader::new(input_path, log);
    let writer = AccountWithCashflowsWriter::new(output_path, log);
    let op_path_securitised = format!("{}_securitized", output_path);
    let writer_non_hdfc = AccountWithCashflowsWriter::new(&op_path_securitised, log);

    (reader, writer, writer_non_hdfc)
}
