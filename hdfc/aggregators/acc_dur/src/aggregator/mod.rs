use self::acc_level_data::get_acc_level_data;
use self::structs::{Currency, LLGKey, LLGVal, RequiredFields};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

mod acc_level_data;
mod organize;
mod structs;
mod writer;

pub fn aggregate(config_params: ConfigurationParameters, logger: &Logger) {
    let mut accounts = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let rules = AggRules::new_from_path(config_params.rules_file_path(), &accounts);
    let currency_map = Currency::new(config_params.exchange_rate_file());
    let required_fields_file = RequiredFields::new_from_path(config_params.req_fields_file_path());
    let mut total_read_time = Duration::new(0, 0);
    let mut total_process_time = Duration::new(0, 0);
    let mut read_start_time = SystemTime::now();
    let mut map_to_write: HashMap<LLGKey, LLGVal> = HashMap::new();
    let mut acc_level_op: String = String::new();
    let mut tot_acc_encntrd = 0;
    let mut tot_amt = 0.0;
    let acc_skip = 0;
    for each_account in accounts.iter() {
        tot_acc_encntrd += 1;
        let read_end_time = SystemTime::now();
        let read_elapsed_time = read_end_time.duration_since(read_start_time).unwrap();
        total_read_time += read_elapsed_time;
        let llg = match rules.llg_for_acc(&each_account) {
            Some(val) => val.llg,
            None => config_params.default_llg_code(),
        };
        let ccy = each_account
            .get_string_for_key(&required_fields_file.ccy)
            .unwrap_or(&"NONE".to_string())
            .to_string();
        let exrt_key = Currency::get_key(&ccy, config_params.base_currency());
        if !currency_map.contains_key(&exrt_key) {
            log_error!(
                logger,
                "Exchange rate for conversion: '{}' to  `{}` not found",
                ccy,
                config_params.base_currency()
            );
            continue;
        }
        let ex_rt = currency_map
            .get(&exrt_key)
            .expect("Cannot read exchange rate");
        let acc_no = each_account
            .get_string_for_key(&required_fields_file.acc_no)
            .expect("Could not fetch account number");
        let ccy = each_account
            .get_string_for_key(&required_fields_file.ccy)
            .expect(&format!(
                "Could not fetch currency for account number: {}",
                acc_no
            ));
        let ccy_amt = each_account
            .get_f64_for_key(&required_fields_file.amt_ccy)
            .expect(&format!(
                "Could not fetch amount for account number: {}",
                acc_no
            ));
        tot_amt += ccy_amt;
        let mut duration = each_account
            .get_f64_for_key(&required_fields_file.duration)
            .expect(&format!(
                "Could not fetch duration for account number: {}",
                acc_no
            ));
        let actual_exrt: f64 = if config_params.is_consolidated() {
            1.0 / ex_rt
        } else {
            *ex_rt
        };
        let hcy_amt = ccy_amt * actual_exrt;
        get_acc_level_data(
            &mut acc_level_op,
            config_params.as_on_date(),
            llg.to_string(),
            acc_no.to_string(),
            ccy.to_string(),
            &ccy_amt,
            &hcy_amt,
            &duration,
        );
        duration = duration * ccy_amt;
        let llg_key = LLGKey::get_key(llg, ccy);
        let llg_val = LLGVal::get_val(&ccy_amt, &hcy_amt, &duration);
        let process_start_time = SystemTime::now();
        organize::aggregate(&mut map_to_write, llg_key.clone(), llg_val.clone());
        if ccy == config_params.base_currency(){
            get_acc_level_data(
                &mut acc_level_op,
                config_params.as_on_date(),
                llg.to_string(),
                acc_no.to_string(),
                config_params.local_ccy().to_string(),
                &ccy_amt,
                &hcy_amt,
                &duration,
            );
            let llg_key = LLGKey::get_key(llg, config_params.local_ccy());
            let llg_val = LLGVal::get_val(&ccy_amt, &hcy_amt, &duration);
            organize::aggregate(&mut map_to_write, llg_key.clone(), llg_val.clone());
        }
        let process_end_time = SystemTime::now();
        let process_elapsed_time = process_end_time.duration_since(process_start_time).unwrap();
        total_process_time += process_elapsed_time;
        read_start_time = SystemTime::now();
    }

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - acc_skip,
        acc_skip,
        tot_amt,
        tot_amt,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());

    writer::write_to_file(
        acc_level_op,
        map_to_write,
        config_params.output_file_path(),
        config_params.as_on_date(),
    );
}
