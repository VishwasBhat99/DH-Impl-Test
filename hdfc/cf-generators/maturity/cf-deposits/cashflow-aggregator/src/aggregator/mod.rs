use std::time::SystemTime;
use aggregator::aggregate_collector::get_717_aggregates;
use rbdate::NaiveDate;
use slog::Logger;
use macros;
use self::duration_extensions::DurationAsNano;
use aggregator::reports::input_report::InputReport;
use aggregator::reports::llgs_report::LLGsReport;
use aggregator::reports::AggregationReport;
use aggregator::grouped_aggregates_store::CashflowOrganizer;
use self::cf_grouping::cashflows_grouped_by_day;
use sdb_dyn_proto_rdr::reader;
use aggregator::account_field_names::AccFieldNames;
use sdb_agg_rules::agg_rules::AggRules;

mod aggregator_fn;
mod aggregate_collector;
mod structs;
mod dates;
mod implementation;
mod cf_grouping;
pub mod writer;
mod grouped_aggregates_store;
mod duration_extensions;
mod llg_key;
mod currency;
mod reports;
mod account_field_names;

pub fn aggregate_cashflows(
    cf_file_path: &str,
    as_on_date: &NaiveDate,
    output_path: &str,
    base_currency: &str,
    currency_conversion_file_path: &str,
    known_fields_file_path: &str,
    account_metadata_file_path: &str,
    rules_file_path: &str,
    logger: &Logger,
    diag_logger: &Logger
) -> AggregationReport {

    // Prepare data we will require for processing.
    let start_time = SystemTime::now();
    let agg_as_on_date = &as_on_date.succ();
    let mut input_report = InputReport::new();
    let mut llgs_report = LLGsReport::new();

    let aggregation_date_limit = dates::get_aggregation_date_limit(agg_as_on_date);

    let currency_converter = currency::create_currency_converter(
        base_currency,
        currency_conversion_file_path
    );
    let mut cashflow_organizer = CashflowOrganizer::new(
        currency_converter
    );
    let keys = AccFieldNames::new_from_path(known_fields_file_path);
    // Read Cashflows and organise them.
    let mut account_reader = reader::Reader::new_at_path (
        account_metadata_file_path,
        cf_file_path
    );
    let rules = AggRules::new_from_path(rules_file_path, &account_reader);
    for account_with_cfs in account_reader.iter() {
        let llg = log_measurements!(
            diag_logger,
            [format!("Type: GetLLG, Identifier: {:?}", account_with_cfs.get_string_for_key(&keys.account_number).expect("fail"))],
            implementation::llg_for_account(&account_with_cfs, &keys, &rules, logger)
        );

        let grouped_cashflows = log_measurements!(
            diag_logger,
            [format!("Type: GroupCFs, Identifier: {:?}", account_with_cfs.get_string_for_key(&keys.account_number).expect("fail"))],
            cashflows_grouped_by_day(account_with_cfs, &keys, as_on_date, aggregation_date_limit)
        );

        input_report.add_account_totals(
            grouped_cashflows.account_amounts_report
        );
        llgs_report.add_account_totals_for_llg(
            &llg,
            grouped_cashflows.account_amounts_report
        );

        log_measurements!(
            diag_logger,
            [format!("Type: OrganiseCFsInLLG, Identifier: {:?}", llg)],
            cashflow_organizer.build_with(llg, grouped_cashflows.date_grouped_cashflows)
        );

    }

    // Compute aggregates and flush to file.
    let mut writer = writer::Writer::new(
        output_path,
        agg_as_on_date,
        diag_logger.clone()
    );
    for (llg, cfs_grouped_by_day) in cashflow_organizer.drain() {

        let aggregates = log_measurements!(
            diag_logger,
            [format!("Type: GetAggregatesForLLG, Identifier: {:?}", llg)],
            get_717_aggregates(
                agg_as_on_date,
                cfs_grouped_by_day,
            )
        );

        writer.flush(llg, aggregates);
    }

    // Wind down:
    // 1. Close the writer (this will write the summary records).
    // 2. Print and log the time taken.
    // 3. Prepare the report, and return it back to the caller.
    let (llg_summaries_report, writer_report) = writer.close();
    let output_records_written_report = llgs_report
        .add_aggregate_writer_report(writer_report);

    let aggregation_report = AggregationReport::new(
        input_report,
        llgs_report,
        llg_summaries_report,
        output_records_written_report
    );

    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);

    return aggregation_report;
}