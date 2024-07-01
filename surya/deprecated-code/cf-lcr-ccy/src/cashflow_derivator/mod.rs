use slog::Logger;
mod account_appender;
mod account_reader;
mod account_without_cashflows;
mod account_writer;
mod structs;

use self::account_reader::InputAccountReader;
use self::account_writer::AccountWithoutCashflows;
use self::structs::{AggrKey, LCR};
use cashflow_derivator::account_appender::create_account_without_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use std::collections::HashMap;
use std::time::SystemTime;

pub fn generate(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_generator_timer = SystemTime::now();
    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut reader_iterator = reader.into_iter();
    let mut summary_data: HashMap<AggrKey, LCR> = HashMap::new();
    let mut tot_rec = 0;
    let skp_rec = 0;
    let tot_amt = 0.0;
    loop {
        let account_opt = log_measurements!(
            diag_log,
            [format!(
                "Type: ReadParseInputAccount, Identifier: {}",
                tot_rec
            )],
            reader_iterator.next()
        );

        if account_opt.is_none() {
            break;
        }
        let input_account = account_opt.expect("Unexpected error occured.");
        tot_rec += 1;
        let grp_key = AggrKey {
            file_id: input_account.file_id,
            currency: input_account.ccy.to_string(),
        };
        let acc_data = LCR {
            ca: input_account.ca,
            sa: input_account.sa,
            td_wd: input_account.td_wd,
            td_nwd: input_account.td_nwd,
            rd: input_account.rd,
            tot_stable: input_account.tot_stable,
            tot_less_stable: input_account.tot_less_stable,
            ca_stable: input_account.ca_stable,
            ca_less_stable: input_account.ca_less_stable,
            sa_stable: input_account.sa_stable,
            sa_less_stable: input_account.sa_less_stable,
            casa_stable: input_account.casa_stable,
            casa_less_stable: input_account.casa_less_stable,
            stable_b1: input_account.stable_b1,
            stable_b2: input_account.stable_b2,
            stable_b3: input_account.stable_b3,
            less_stable_b1: input_account.less_stable_b1,
            less_stable_b2: input_account.less_stable_b2,
            less_stable_b3: input_account.less_stable_b3,
            nwd_b1: input_account.nwd_b1,
            nwd_b2: input_account.nwd_b2,
            nwd_b3: input_account.nwd_b3,
        };
        summary_data
            .entry(grp_key)
            .and_modify(|data| {
                data.ca += input_account.ca;
                data.sa += input_account.sa;
                data.td_wd += input_account.td_wd;
                data.td_nwd += input_account.td_nwd;
                data.rd += input_account.rd;
                data.tot_stable += input_account.tot_stable;
                data.tot_less_stable += input_account.tot_less_stable;
                data.ca_stable += input_account.ca_stable;
                data.sa_stable += input_account.sa_stable;
                data.ca_less_stable += input_account.ca_less_stable;
                data.sa_less_stable += input_account.sa_less_stable;
                data.casa_stable += input_account.casa_stable;
                data.casa_less_stable += input_account.casa_less_stable;
                data.stable_b1 += input_account.stable_b1;
                data.stable_b2 += input_account.stable_b2;
                data.stable_b3 += input_account.stable_b3;
                data.less_stable_b1 += input_account.less_stable_b1;
                data.less_stable_b2 += input_account.less_stable_b2;
                data.less_stable_b3 += input_account.less_stable_b3;
                data.nwd_b1 += input_account.nwd_b1;
                data.nwd_b2 += input_account.nwd_b2;
                data.nwd_b3 += input_account.nwd_b3;
            })
            .or_insert(acc_data);
    }
    for (key, data) in summary_data.drain() {
        log_debug!(log, "Data --> Key: {:#?} Value: {:#?}", key, data);
        if !config_params.filter_ccy().contains(&key.currency)
            && config_params.filter_ccy() != "NONE"
        {
            continue;
        }
        let cf_data = create_account_without_cashflows(key, data, log);
        writer.write(cf_data);
    }
    writer.close();

    let end_generator_timer = SystemTime::now();
    let total_duration = end_generator_timer
        .duration_since(start_generator_timer)
        .expect("Could not calculate total duration.");
    println!("Total Account Encountered: {}", tot_rec);
    println!("Total Time Taken to Generate CF: {:#?}", total_duration);

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    health_report.gen_health_rpt(&config_params.output_file_path());
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
