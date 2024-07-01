use self::cashflow_organizer::CashflowOrganizer;
use aggregator::account::AccFieldNames;
use aggregator::cashflow_aggregation::get_14_aggregates;
use aggregator::reports::input_report::InputReport;
use aggregator::reports::llgs_report::LLGsReport;
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use std::time::{Duration, SystemTime};

mod account;
mod cashflow_aggregation;
mod cashflow_organizer;
mod currency;
pub mod llg;
mod reports;
mod writer;

pub fn aggregate_cashflows(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    let start_read_timer = SystemTime::now();
    let agg_as_on_date = config_params.as_on_date();
    let input_report = InputReport::new();
    let mut llgs_report = LLGsReport::new();
    let currency_converter = currency::create_currency_converter(
        config_params.consolidated_currency(),
        config_params.currency_conversion_file_path(),
    );
    let keys = AccFieldNames::new_from_path(config_params.req_fields_file_path());
    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let mut ex_rt: f64 = 0.0;
    let rules = AggRules::new_from_path(config_params.rules_file_path(), &account_reader);
    let mut cashflow_organizer = CashflowOrganizer::new(currency_converter);
    let end_read_timer = SystemTime::now();
    let total_duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration for read timer.");
    info!(
        diag_logger,
        "Total Duration for read timer: {:.2?}", total_duration
    );

    let mut tot_llg_dur = Duration::new(0, 0);
    let mut tot_org_dur = Duration::new(0, 0);
    let mut tot_key_read_dur = Duration::new(0, 0);

    for account_with_cfs in account_reader.iter() {
        let start_key_read_timer = SystemTime::now();
        if config_params.is_acc_level_ex_rt() {
            ex_rt = account_with_cfs
                .get_f64_for_key(&keys.ex_rt)
                .expect("Cannot get exchange rate from account.");
        }

        let acc_no = account_with_cfs
            .get_string_for_key(&keys.acc_no)
            .expect("Error getting `account number`.")
            .to_string();
        let end_key_read_timer = SystemTime::now();
        tot_key_read_dur += end_key_read_timer
            .duration_since(start_key_read_timer)
            .expect("Error while calculating total duration for spread, account_nuber and exchange_rate determination.");

        let start_llg_timer = SystemTime::now();
        let mut llg = log_measurements!(
            diag_logger,
            [format!("Type: GetLLG, Identifier: {:?}", acc_no)],
            llg::llg_for_account(
                acc_no,
                &account_with_cfs,
                &keys,
                &rules,
                config_params,
                logger
            )
        );
        let end_llg_timer = SystemTime::now();
        tot_llg_dur += end_llg_timer
            .duration_since(start_llg_timer)
            .expect("Error while calculating total duration for llg determination.");

        let start_org_timer = SystemTime::now();
        log_measurements!(
            diag_logger,
            [format!("Type: OrganiseCFsInLLG, Identifier: {:?}", llg)],
            cashflow_organizer.organize(
                &mut llg,
                account_with_cfs,
                &keys,
                input_report,
                &mut llgs_report,
                config_params.is_acc_level_ex_rt(),
                ex_rt,
            )
        );
        let end_org_timer = SystemTime::now();
        tot_org_dur += end_org_timer
            .duration_since(start_org_timer)
            .expect("Error while calculating total duration for reading cashflows determination.");
    }

    let reporting_string = format!(
        "Reading keys: {:.2?}\n\
         main llg determination: {:.2?}\n\
         reading cashflows: {:.2?}",
        tot_key_read_dur, tot_llg_dur, tot_org_dur
    );
    info!(diag_logger, "{}", reporting_string);

    // Compute aggregates and flush to file.
    let mut writer = writer::Writer::new(
        config_params.output_file_path(),
        agg_as_on_date,
        diag_logger.clone(),
    );

    let start_agg_timer = SystemTime::now();
    for (llg, cfs_grouped_by_day) in cashflow_organizer.drain() {
        let aggregates = log_measurements!(
            logger,
            [format!("Type: GetAggregatesForLLG, Identifier: {:?}", llg)],
            get_14_aggregates(cfs_grouped_by_day)
        );
        writer.flush(llg, aggregates);
    }
    let end_agg_timer = SystemTime::now();
    let total_duration = end_agg_timer
        .duration_since(start_agg_timer)
        .expect("Could not calculate total duration for aggregation timer.");
    debug!(
        diag_logger,
        "Total Duration for aggregate and write records: {:2?}", total_duration
    );

    // Wind down:
    // 1. Close the writer (this will write the summary records).
    // 2. Print and log the time taken.
    // 3. Prepare the report, and return it back to the caller.
    let start_summary_timer = SystemTime::now();
    let (_llg_summaries_report, _writer_report) = writer.close();
    let end_summary_timer = SystemTime::now();
    let total_duration = end_summary_timer
        .duration_since(start_summary_timer)
        .expect("Could not calculate total duration for summary timer.");
    info!(
        diag_logger,
        "Total Duration for summary writing: {:2?}", total_duration
    );
}
