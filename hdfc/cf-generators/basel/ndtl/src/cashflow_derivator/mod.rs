use slog::Logger;
mod account_appender;
mod account_without_cashflows;
mod account_writer;
mod msf;

use self::account_writer::AccountWithoutCashflows;
use crate::cashflow_derivator::msf::*;
use cashflow_derivator::account_appender::create_account_without_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;

pub fn generate(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_generator_timer = SystemTime::now();
    let mut msf_map: HashMap<Msf, String> = HashMap::new();
    let msf_reader =
        fs::read_to_string(&config_params.msf_file_path()).expect("Failed to read MSF file!");
    for line in msf_reader.lines() {
        let msf_fields = line.split("|").collect::<Vec<&str>>();
        let msf_inst = Msf::new(msf_fields[0].to_string(), msf_fields[1].to_string());
        msf_map.insert(msf_inst, msf_fields[2].to_string());
    }
    let op_path = config_params.output_file_path().to_string();
    let mut writer = get_writer(&op_path, log);
    let msf_pct = get_msf_desc(*config_params.as_on_date(), &msf_map);
    let account_without_cashflows = create_account_without_cashflows(&config_params, &msf_pct);
    let tot_rec = 1;
    let skp_rec = 0;
    let tot_amt = account_without_cashflows.ndtl_val;

    let as_on_date = &config_params.as_on_date().format("%d-%m-%Y").to_string();
    log_info!(
        log,
        "The MSF_percentage used as_on {:?} is: {:?}",
        &as_on_date,
        &msf_pct.to_string().parse::<f32>().unwrap()
    );

    log_measurements!(
        diag_log,
        [format!(
            "Type: WriteAccWithCFs, Identifier: {}",
            account_without_cashflows.as_on_date
        )],
        writer.write(account_without_cashflows)
    );
    writer.close();

    let end_generator_timer = SystemTime::now();
    let total_duration = end_generator_timer
        .duration_since(start_generator_timer)
        .expect("Could not calculate total duration.");
    log_info!(log, "Total time take: {:?}", total_duration);
    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&op_path);
}

fn get_writer(output_path: &str, log: &Logger) -> AccountWithoutCashflows {
    let writer = AccountWithoutCashflows::new(output_path, log);
    writer
}
