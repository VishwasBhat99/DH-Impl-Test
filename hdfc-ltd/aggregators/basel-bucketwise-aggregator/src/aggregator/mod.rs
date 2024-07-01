use self::bucket::bucket_op;
use self::cf_grouping::cashflows_grouped_by_day;
use self::overdue_llg::ResidualPeriod;
use aggregator::account_field_names::AccFieldNames;
use aggregator::grouped_aggregates_store::CashflowOrganizer;
use aggregator::input_report::InputReport;
use chrono::{Duration, NaiveDate};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::io::Write;
use std::time::SystemTime;
use std::{env, fs};

mod account_field_names;
mod bucket;
mod cf_grouping;
mod currency;
mod dates;
mod grouped_aggregates_store;
mod implementation;
mod input_report;
mod llg_key;
mod overdue_llg;
mod structs;

pub fn aggregate_cashflows(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    let aggregation_date_limit = dates::get_aggregation_date_limit(config_params.as_on_date());
    let keys = AccFieldNames::new_from_path(config_params.req_fields_file_path());
    let mut input_report = InputReport::new();

    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );

    let currency_converter = currency::create_currency_converter(
        config_params.base_currency(),
        config_params.currency_conversion_file_path(),
    );
    let mut cashflow_organizer = CashflowOrganizer::new(currency_converter);

    let rules = AggRules::new_from_path(config_params.rules_file_path(), &account_reader);
    let mut output_file = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(create) => create,
        Err(error) => {
            panic!(
                "Could not create output file: `{}` on location `{}`: {}.",
                config_params.output_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };

    let mut overdue_llg_map: HashMap<ResidualPeriod, i32> = HashMap::new();

    if config_params.default_overdue_llg_path() != "NA" {
        let overdue_llg_reader = fs::read_to_string(&config_params.default_overdue_llg_path())
            .expect("Could Not Read Overdue_llg_config_file.");

        for line in overdue_llg_reader.lines() {
            let fields: Vec<&str> = line.split('|').collect();
            let residual_period = ResidualPeriod::new(fields[0].to_string(), fields[1].to_string());

            overdue_llg_map.insert(
                residual_period,
                fields[2]
                    .parse::<i32>()
                    .expect("Cannot parse overdue llg as integer from overdue llg file"),
            );
        }
    }

    let as_on_date = config_params.as_on_date();
    for account_with_cfs in account_reader.iter() {
        let llg = log_measurements!(
            diag_logger,
            [format!(
                "Type: GetLLG, Identifier: {:?}",
                account_with_cfs
                    .get_string_for_key(&keys.account_number)
                    .unwrap_or(&"NA".to_string())
            )],
            implementation::llg_for_account(
                &account_with_cfs,
                &keys,
                &rules,
                config_params,
                logger
            )
        );

        let grouped_cashflows = log_measurements!(
            diag_logger,
            [format!(
                "Type: GroupCFs, Identifier: {:?}",
                account_with_cfs
                    .get_string_for_key(&keys.account_number)
                    .unwrap_or(&"NA".to_string())
            )],
            cashflows_grouped_by_day(
                account_with_cfs,
                &keys,
                config_params,
                aggregation_date_limit,
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
                config_params,
                logger,
                &overdue_llg_map.clone()
            )
        );
    }

    for (llg_key, data) in cashflow_organizer.drain() {
        //Aggregation:
        //1. Write to output amounts of each day upto 184 days.
        //2. From 185th day onwards, sum values for each month upto 24 months and write to output.
        //3. Write values after 24 months to single bucket labeled as after 2 years.
        let op_line = format!(
            "{}|{}|{}|{}|",
            as_on_date,
            config_params.country(),
            llg_key.currency,
            llg_key.category
        );
        let mut bucket_dt: NaiveDate = *config_params.as_on_date();

        if overdue_llg_map
            .values()
            .any(|&value| value == llg_key.category)
        {
            writeln!(
                output_file,
                "{}",
                bucket_op(
                    data,
                    *as_on_date,
                    op_line,
                    config_params.is_edate_req(),
                    config_params.is_custom_bucket_req(),
                    &mut bucket_dt
                )
            )
            .expect("Unable to generate aggregated summary file.");
        } else {
            bucket_dt = *as_on_date + Duration::days(1);
            writeln!(
                output_file,
                "{}",
                bucket_op(
                    data,
                    *as_on_date,
                    op_line,
                    config_params.is_edate_req(),
                    config_params.is_custom_bucket_req(),
                    &mut bucket_dt
                )
            )
            .expect("Unable to generate aggregated summary file.");
        }
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
