use configuration_parameters::ConfigurationParameters;
use currency;
use health_report::HealthReport;
use macros;
use rbdate::{NaiveDate, NaiveDateTime};
use sdb_dyn_proto_rdr::reader::Reader;
use sdb_io::buf_file_wrtr;
use slab::get_mat_bkt_id;
use slab::get_mat_slabs;
use slog::Logger;
use std::collections::HashMap;
use std::io::Write;
use structs::{AccFields, AggrData, OutputData, SummaryOutputData};

pub fn process_name(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    let summary_op_file = config_params.output_file().replace(".txt", "") + "-summary.txt";
    // Create Output writer
    let mut output_file = match buf_file_wrtr(config_params.output_file(), None) {
        Ok(create) => create,
        Err(error) => {
            panic!(
                "Could not create output file: `{}`: {}.",
                config_params.output_file(),
                error
            );
        }
    };
    // Create Summary Output writer
    let mut summary_output_file = match buf_file_wrtr(&summary_op_file, None) {
        Ok(create) => create,
        Err(error) => {
            panic!(
                "Could not create output file: `{}`: {}.",
                config_params.output_file(),
                error
            );
        }
    };
    // Read Req Account fields file
    let keys = AccFields::new_from_path(config_params.req_field_file());
    // Read currency exchange rate file
    let currency_converter = currency::create_currency_converter(
        config_params.base_currency(),
        config_params.exchange_rate_file(),
    );
    // Read cashflow file
    let mut cf_rec_reader: Reader =
        Reader::new_at_path(config_params.metadata_file(), config_params.input_file());
    let mut acc_aggr: HashMap<i64, AggrData> = HashMap::new();
    let mut tot_acc_enc = 0;
    let mut skipped_rec = 0;
    let mut total_bal = 0.0;
    for account in cf_rec_reader.iter() {
        tot_acc_enc += 1;
        let default_value = "".to_string();
        let base_currency = config_params.base_currency().to_string();
        let account_id = account
            .get_string_for_key(&keys.account_id)
            .unwrap_or(&default_value);
        let cust_id = account
            .get_string_for_key(&keys.cust_id)
            .unwrap_or(&default_value);
        let cust_name = account
            .get_string_for_key(&keys.cust_name)
            .unwrap_or(&default_value);
        let branch_code = account
            .get_string_for_key(&keys.branch_code)
            .unwrap_or(&default_value);
        let currency = account
            .get_string_for_key(&keys.currency)
            .unwrap_or(&base_currency);
        let bal_1 = account.get_f64_for_key(&keys.bal_lcy).unwrap_or(0.0);
        let bal_2 = currency_converter.convert(
            &account,
            &currency,
            bal_1,
            config_params.is_consolidated(),
            config_params.is_account_level_exchange_rate(),
            &keys.exchange_rate,
            logger,
        );
        let bal_ccy;
        let bal_lcy;
        if config_params.is_consolidated() {
            bal_ccy = bal_2;
            bal_lcy = bal_1;
        } else {
            bal_ccy = bal_1;
            bal_lcy = bal_2;
        }
        total_bal += bal_lcy;
        let start_date =
            naivedate_from_timestamp(account.get_i64_for_key(&keys.start_date).unwrap_or(0));
        let mat_date =
            naivedate_from_timestamp(account.get_i64_for_key(&keys.mat_date).unwrap_or(0));
        let int_rate = account.get_f64_for_key(&keys.int_rate).unwrap_or(0.0);
        let mat_tenor = rbdate::num_days_start_to_end(start_date, mat_date);
        // Read Maturity bucket config file
        let mat_slabs = get_mat_slabs(&config_params, start_date);
        let (mat_bkt_id, mat_bkt_name, threshold_ir) = get_mat_bkt_id(&mat_slabs, mat_tenor);
        if mat_bkt_id == 0 {
            log_warn!(
                logger,
                "Maturity Slab Config Not Found For Account No: {}",
                account_id
            );
        }
        if int_rate < threshold_ir {
            log_debug!(
                logger,
                "Account No: {} skipped: Interest Rate is Less than Threshold Rate",
                account_id
            );
            continue;
        }
        let op: OutputData = OutputData {
            as_on_date: config_params.as_on_date().format("%d-%m-%Y").to_string(),
            account_id: account_id.to_string(),
            cust_id: cust_id.to_string(),
            ccy: currency.to_string(),
            mat_bkt_id: mat_bkt_id,
            mat_bkt_name: mat_bkt_name.to_string(),
            cust_name: cust_name.to_string(),
            branch_code: branch_code.to_string(),
            bal_ccy: bal_ccy,
            bal_lcy: bal_lcy,
            start_date: start_date.format("%d-%m-%Y").to_string(),
            mat_date: mat_date.format("%d-%m-%Y").to_string(),
            int_rate: int_rate,
        };
        let acc_aggr_data = AggrData {
            acc_count: 1.0,
            max_ir: int_rate,
            min_ir: int_rate,
            tot_ir: (int_rate * bal_lcy),
            total_bal_lcy: bal_lcy,
            mat_bkt_name: mat_bkt_name,
            threshold_ir: threshold_ir,
        };
        acc_aggr
            .entry(mat_bkt_id)
            .and_modify(|data| data.aggr_data(&acc_aggr_data))
            .or_insert(acc_aggr_data);
        match output_file.write(op.to_string().as_bytes()) {
            Ok(val) => {
                log_debug!(diag_logger, "Writing data to output file: {}", val);
            }
            Err(error) => {
                skipped_rec += 1;
                log_debug!(diag_logger, "Cannot write data to output file: {}", error);
            }
        }
    }

    for (bkt_id, aggr_data) in acc_aggr.drain() {
        let op = SummaryOutputData {
            as_on_date: config_params.as_on_date().format("%d-%m-%Y").to_string(),
            mat_bkt_id: bkt_id,
            mat_bkt_name: aggr_data.mat_bkt_name,
            max_ir: aggr_data.max_ir,
            min_ir: aggr_data.min_ir,
            avg_ir: aggr_data.tot_ir / aggr_data.total_bal_lcy,
            threshold_ir: aggr_data.threshold_ir,
            total_bal_lcy: aggr_data.total_bal_lcy,
        };
        match summary_output_file.write(op.to_string().as_bytes()) {
            Ok(val) => {
                log_debug!(diag_logger, "Writing summary data to output file: {}", val);
            }
            Err(error) => {
                log_debug!(
                    diag_logger,
                    "Cannot write summary data to output file: {}",
                    error
                );
            }
        }
    }

    let health_report = HealthReport::new(
        tot_acc_enc,
        tot_acc_enc - skipped_rec,
        skipped_rec,
        total_bal,
        total_bal,
        0,
    );
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file());
}

pub fn naivedate_from_timestamp(t: i64) -> rbdate::NaiveDate {
    if t == 0 {
        rbdate::NaiveDate::from_ymd(1900, 1, 1)
    } else {
        let naive_date_time = rbdate::NaiveDateTime::from_timestamp(t, 0);
        naive_date_time.date()
    }
}
