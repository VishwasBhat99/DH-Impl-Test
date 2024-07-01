use self::llg_key::LLGKey;
use self::organize::Cashflow;
use aggregator::required_fields::RequiredFields;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use configuration_parameters::ConfigurationParameters;

mod currency;
mod llg_key;
mod organize;
mod required_fields;
mod writer;

pub fn aggregate_cashflows(
    config_params :ConfigurationParameters,
    logger: &Logger,
) {
    let mut curr_code_read: String = "".to_string();
    let mut accounts = reader::Reader::new_at_path(config_params.account_metadata_file_path(), config_params.input_file_path());
    let rules = AggRules::new_from_path(config_params.rules_file_path(), &accounts);
    let currency_map =
        currency::get_exchange_rate(config_params.source_local_currency(), config_params.currency_conversion_file_path());
    let required_fields_file = RequiredFields::new_from_path(config_params.req_fields_file_path());
    let mut total_read_time = Duration::new(0, 0);
    let mut total_process_time = Duration::new(0, 0);
    let mut read_start_time = SystemTime::now();
    let spread = 0;
    let mut given_ex: f64 = 1.0;
    let mut consol_ex: f64 = 1.0;
    let mut map_to_write: HashMap<LLGKey, Cashflow> = HashMap::new();
    for each_account in accounts.iter() {
        let read_end_time = SystemTime::now();
        let read_elapsed_time = read_end_time.duration_since(read_start_time).unwrap();
        total_read_time += read_elapsed_time;
        let mut llg_id_read = match rules.llg_for_acc(&each_account) {
            Some(c) => c.llg,
            None => config_params.default_llg_code(),
        };
        let flow = llg_id_read / 10000;
        llg_id_read = llg_id_read % 10000;
        curr_code_read = each_account
            .get_string_for_key(&required_fields_file.curr_code)
            .unwrap_or(&"NONE".to_string())
            .to_string();
        if !currency_map.contains_key(&curr_code_read) {
            //log error for XYZ currency
            log_error!(
                logger,
                "Exchange rate for Currency: `{}` not found",
                curr_code_read
            );
            continue;
        }
        let ex_rt = currency_map
            .get(&curr_code_read)
            .expect("Cannot read exchange rate");
        let rep_freq = 14;
        let intt_rate_read = each_account
            .get_f64_for_key(&required_fields_file.intt_rate)
            .expect("interest read json error");

        let bm_id_read = "FIXED".to_string();

        let next_repr_date_read: i64 = 4102358400;
        let tenor = config_params.def_tenor();
        let mut amount = each_account
            .get_f64_for_key(&required_fields_file.prin_amt)
            .unwrap_or(0.0);
        let llg_key_consol_curr = llg_key::create_key(
            next_repr_date_read,
            llg_id_read,
            &config_params.consolidated_currency().to_string(),
            &bm_id_read,
            rep_freq,
            &tenor,
        );
        if flow == 0 {
            amount = amount * -1.0;
        }
        let process_start_time = SystemTime::now();
        if curr_code_read == config_params.source_local_currency().to_string() {
            let llg_key_disp_local_curr = llg_key::create_key(
                next_repr_date_read,
                llg_id_read,
                &config_params.display_local_currency().to_string(),
                &bm_id_read,
                rep_freq,
                &tenor,
            );
            if map_to_write.contains_key(&llg_key_consol_curr) {
                let map_value = map_to_write
                    .get(&llg_key_consol_curr)
                    .expect("llg key given curr");
                let new_value =
                    organize::aggregate_existing(amount, map_value.clone(), ex_rt, intt_rate_read);
                map_to_write.insert(llg_key_consol_curr.clone(), new_value);
            } else {
                let new_value = organize::aggregate_new(ex_rt, amount, intt_rate_read);
                map_to_write.insert(llg_key_consol_curr.clone(), new_value);
            }
            if map_to_write.contains_key(&llg_key_disp_local_curr) {
                let map_value = map_to_write
                    .get(&llg_key_disp_local_curr)
                    .expect("llg key given curr");
                let new_value =
                    organize::aggregate_existing(amount, map_value.clone(), ex_rt, intt_rate_read);
                map_to_write.insert(llg_key_disp_local_curr.clone(), new_value);
            } else {
                let new_value = organize::aggregate_new(ex_rt, amount, intt_rate_read);
                map_to_write.insert(llg_key_disp_local_curr.clone(), new_value);
            }
        } else {
            let llg_key_curr_code_read = llg_key::create_key(
                next_repr_date_read,
                llg_id_read,
                &curr_code_read.to_string(),
                &bm_id_read,
                rep_freq,
                &tenor,
            );
            let llg_key_foreign_con_curr = llg_key::create_key(
                next_repr_date_read,
                llg_id_read,
                &config_params.foreign_consolidated_currency().to_string(),
                &bm_id_read,
                rep_freq,
                &tenor,
            );
            if ex_rt <= &0.0 {
                given_ex = 0.0;
                consol_ex = 0.0;
                log_error!(
                    logger,
                    "Exchange rate for Currency: `{}` is less than 0",
                    curr_code_read
                );
            } else if config_params.is_consolidated() {
                given_ex = 1.0 / ex_rt;
                consol_ex = 1.0;
            } else {
                given_ex = 1.0;
                consol_ex = ex_rt.clone();
            }
            if map_to_write.contains_key(&llg_key_curr_code_read) {
                let map_value = map_to_write
                    .get(&llg_key_curr_code_read)
                    .expect("llg key given curr");
                let new_value = organize::aggregate_existing(
                    amount,
                    map_value.clone(),
                    &given_ex,
                    intt_rate_read,
                );
                map_to_write.insert(llg_key_curr_code_read.clone(), new_value);
            } else {
                let new_value = organize::aggregate_new(&given_ex, amount, intt_rate_read);
                map_to_write.insert(llg_key_curr_code_read.clone(), new_value);
            }
            if map_to_write.contains_key(&llg_key_foreign_con_curr) {
                let map_value = map_to_write
                    .get(&llg_key_foreign_con_curr)
                    .expect("llg key given curr");
                let new_value = organize::aggregate_existing(
                    amount,
                    map_value.clone(),
                    &consol_ex,
                    intt_rate_read,
                );
                map_to_write.insert(llg_key_foreign_con_curr.clone(), new_value);
            } else {
                let new_value = organize::aggregate_new(&consol_ex, amount, intt_rate_read);
                map_to_write.insert(llg_key_foreign_con_curr.clone(), new_value);
            }
            if map_to_write.contains_key(&llg_key_consol_curr) {
                let map_value = map_to_write
                    .get(&llg_key_consol_curr)
                    .expect("llg key given curr");
                let new_value = organize::aggregate_existing(
                    amount,
                    map_value.clone(),
                    &consol_ex,
                    intt_rate_read,
                );
                map_to_write.insert(llg_key_consol_curr.clone(), new_value);
            } else {
                let new_value = organize::aggregate_new(&consol_ex, amount, intt_rate_read);
                map_to_write.insert(llg_key_consol_curr.clone(), new_value);
            }
        }
        let process_end_time = SystemTime::now();
        let process_elapsed_time = process_end_time.duration_since(process_start_time).unwrap();
        total_process_time += process_elapsed_time;
        read_start_time = SystemTime::now();
    }
    let writer = writer::create_writer_for_path(config_params.output_file_path());
    writer.set_len(0).expect("writer smry set_len(0)");
    for (k, v) in map_to_write.iter() {
        let curr = k.curr_code.clone();
        let mut ex_rt: f64 = 1.0;
        if curr != config_params.source_local_currency() {
            ex_rt = *currency_map
                .get(&curr)
                .expect("could not find exchange rate");
        }
        if config_params.is_consolidated() {
            ex_rt = 1.0 / ex_rt;
        }
        writer::write_to_file(config_params.as_on_date(), &writer, k, spread, v.clone(), ex_rt)
    }
}
