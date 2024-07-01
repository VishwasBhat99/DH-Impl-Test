use self::cf_grouping::cashflows_grouped_by_day;
use self::check_npa::check_npa;
use aggregator::account_field_names::AccFieldNames;
use aggregator::aggregate_collector::get_717_aggregates;
use aggregator::grouped_aggregates_store::CashflowOrganizer;
use aggregator::reports::input_report::InputReport;
use aggregator::reports::llgs_report::LLGsReport;
use aggregator::reports::AggregationReport;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use std::time::SystemTime;

mod account_field_names;
mod aggregate_collector;
mod aggregator_fn;
mod cf_grouping;
mod check_npa;
mod config;
mod currency;
mod dates;
mod duration_extensions;
mod grouped_aggregates_store;
mod implementation;
mod llg_key;
mod reports;
mod structs;
pub mod writer;

pub fn aggregate_cashflows(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) -> AggregationReport {
    // Prepare data we will require for processing.
    let start_time = SystemTime::now();
    // Read Files Configuration
    let files_config = config::get_files(config_params.config_file_path());
    let agg_as_on_date = &config_params.as_on_date().succ();
    let mut input_report = InputReport::new();
    let mut llgs_report = LLGsReport::new();

    let aggregation_date_limit = dates::get_aggregation_date_limit(config_params.as_on_date());

    let mut ex_rt: f64 = 1.0;
    let currency_converter = currency::create_currency_converter(
        config_params.src_local_ccy(),
        config_params.currency_conversion_file_path(),
    );
    let mut cashflow_organizer = CashflowOrganizer::new(currency_converter);
    for file in files_config.files {
        let keys = AccFieldNames::new_from_path(&file.req_fields_file_path);
        // Read Cashflows and organise them.
        let mut account_reader =
            reader::Reader::new_at_path(&file.metadata_file_path, &file.input_file_path);
        let rules = AggRules::new_from_path(&file.rules_file_path, &account_reader);
        if file.is_non_maturity {
            log_debug!(
                logger,
                "Is non maturity flag set to true for input file:{}",
                file.input_file_path
            );
        }
        let default_overdue_llg_code = &file.default_overdue_llg_code;
        for account_with_cfs in account_reader.iter() {
            let is_npa = check_npa(&file.is_npa, &file.npa_values, &account_with_cfs);
            let llg = log_measurements!(
                diag_logger,
                [format!(
                    "Type: GetLLG, Identifier: {:?}",
                    account_with_cfs
                        .get_string_for_key(&keys.account_number)
                        .expect("Error getting `account_number`.")
                )],
                implementation::llg_for_account(
                    &account_with_cfs,
                    &keys,
                    &rules,
                    config_params,
                    logger
                )
            );

            if file.is_account_level_exchange_rate {
                ex_rt = account_with_cfs
                    .get_f64_for_key(&keys.exchange_rate)
                    .expect("Cannot get exchange rate from account.");
                if ex_rt == 0.0 {
                    log_error!(
                        logger,
                        "Exchange rate for Account: `{}` with llg: `{}` is 0.0.",
                        account_with_cfs
                            .get_string_for_key(&keys.account_number)
                            .expect("Error getting `account_number`."),
                        llg.category
                    );
                    continue;
                }
            }

            let grouped_cashflows = log_measurements!(
                diag_logger,
                [format!(
                    "Type: GroupCFs, Identifier: {:?}",
                    account_with_cfs
                        .get_string_for_key(&keys.account_number)
                        .expect("Error while reading account number.")
                )],
                cashflows_grouped_by_day(
                    account_with_cfs,
                    &keys,
                    config_params.as_on_date(),
                    aggregation_date_limit,
                    *default_overdue_llg_code,
                    file.is_rep_mandatory,
                    file.is_non_maturity,
                    is_npa,
                    logger
                )
            );
            input_report.add_account_totals(grouped_cashflows.account_amounts_report);

            log_measurements!(
                diag_logger,
                [format!("Type: OrganiseCFsInLLG, Identifier: {:?}", llg)],
                cashflow_organizer.organize(
                    &llg,
                    grouped_cashflows.date_grouped_cashflows,
                    grouped_cashflows.account_amounts_report,
                    grouped_cashflows.overdue_cashflows,
                    grouped_cashflows.account_overdue_amount_report,
                    config_params,
                    *default_overdue_llg_code,
                    file.is_consolidated,
                    file.is_account_level_exchange_rate,
                    &mut llgs_report,
                    ex_rt,
                    logger
                )
            );
        }
    }

    // Compute aggregates and flush to file.
    let mut writer = writer::Writer::new(
        config_params.output_file_path(),
        &agg_as_on_date.pred(),
        diag_logger.clone(),
    );
    for (llg, cfs_grouped_by_day) in cashflow_organizer.drain() {
        let aggregates = log_measurements!(
            diag_logger,
            [format!("Type: GetAggregatesForLLG, Identifier: {:?}", llg)],
            get_717_aggregates(agg_as_on_date, cfs_grouped_by_day,)
        );
        // overdue grouped cfs
        if !aggregates.1.is_empty() {
            let mut overdue_aggregate_records = Vec::new();
            for grped_cf in aggregates.1.values() {
                let overdue_aggregate = Some(grped_cf.to_cf_aggregated());
                overdue_aggregate_records.push(overdue_aggregate);
            }
            writer.flush_overdue(llg.clone(), overdue_aggregate_records);
        } else {
            writer.flush(llg, aggregates.0);
        }
        let health_report = HealthReport::new(
            input_report.accounts_count as i64,
            input_report.accounts_count as i64,
            0,
            input_report.total_principal_amount,
            input_report.total_principal_amount,
            input_report.cashflows_count as i64,
        );
        health_report.gen_health_rpt(config_params.output_file_path());
    }

    // Wind down:
    // 1. Close the writer (this will write the summary records).
    // 2. Print and log the time taken.
    // 3. Prepare the report, and return it back to the caller.
    let (llg_summaries_report, writer_report) = writer.close();
    let output_records_written_report = llgs_report.add_aggregate_writer_report(writer_report);

    let aggregation_report = AggregationReport::new(
        input_report,
        llgs_report,
        llg_summaries_report,
        output_records_written_report,
    );

    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);

    return aggregation_report;
}
