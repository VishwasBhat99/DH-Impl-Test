use self::cashflows::get_amount;
use self::llg_key::LLGKey;
use self::organize::Cashflow;
use self::spread::get_spread;
use aggregator::required_fields::RequiredFields;
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{timestamp, DateParser, NaiveDateTime};
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use std::collections::HashMap;
use std::fs;
use std::time::{Duration, SystemTime};

mod cashflows;
mod currency;
mod llg_key;
mod organize;
mod required_fields;
mod spread;
mod tenor;
mod writer;

pub fn aggregate_cashflows(config_params: ConfigurationParameters, logger: &Logger) {
    let mut curr_code_read: String = "".to_string();
    let mut _ex_rt: f64 = 1.0;
    let mut accounts = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
    let def_repr_date = date_parser.parse("31-12-2099");
    let default_repr_date = timestamp(def_repr_date);
    let rules = AggRules::new_from_path(config_params.rules_file_path(), &accounts);
    let currency_map = currency::get_exchange_rate(
        config_params.src_local_ccy(),
        config_params.currency_conversion_file_path(),
    );
    let tenor_file =
        fs::read_to_string(config_params.tenor_file_path()).expect("unable to read tenor");
    let mut map_to_write: HashMap<LLGKey, Cashflow> = HashMap::new();
    let mut ref_excel1: Xlsx<_> =
        open_workbook(config_params.ref_file()).expect("Unable to open `spread_ref_1.xlsx`.");
    let mut bm_rt_map: HashMap<String, Vec<f64>> = HashMap::new();
    let mut row_count = 0;
    let mut date_vec: Vec<String> = Vec::new();
    if let Some(Ok(reader)) = ref_excel1.worksheet_range("BM Rates") {
        for row in reader.rows() {
            let mut row_index = 1;
            let mut bm_rt_vec: Vec<f64> = Vec::new();
            let row_len = row.len();
            while row_index < row_len {
                if row_count == 0 {
                    date_vec.push(row[row_index].to_string().as_str().replace("\u{a0}", " "));
                } else {
                    bm_rt_vec.push(
                        row[row_index]
                            .to_string()
                            .as_str()
                            .replace("\u{a0}", " ")
                            .parse::<f64>()
                            .unwrap_or(0.0),
                    );
                }
                row_index += 1;
            }
            if row_count != 0 {
                bm_rt_map.insert(
                    row[0].to_string().as_str().replace("\u{a0}", " "),
                    bm_rt_vec.clone(),
                );
            }
            bm_rt_vec.clear();
            row_count += 1;
        }
    }
    let mut spread: f64 = 0.0;
    let mut given_ex: f64 = 1.0;
    let mut consol_ex: f64 = 1.0;
    let mut is_npa_acc: bool;
    let required_fields_file = RequiredFields::new_from_path(config_params.req_fields_file_path());
    let npa_flag_values = config_params.npa_flag_values().to_uppercase();
    let mut npa_flag_values_vec: Vec<&str> = npa_flag_values.split(",").collect();
    for i in 0..npa_flag_values_vec.len() {
        npa_flag_values_vec[i] = npa_flag_values_vec[i].trim();
    }
    let mut total_read_time = Duration::new(0, 0);
    let mut total_process_time = Duration::new(0, 0);
    let mut read_start_time = SystemTime::now();
    for each_account in accounts.iter() {
        let read_end_time = SystemTime::now();
        let read_elapsed_time = read_end_time.duration_since(read_start_time).unwrap();
        total_read_time += read_elapsed_time;
        let bmid_default_string = "FIXED".to_string();

        let llg_id_read = match rules.llg_for_acc(&each_account) {
            Some(val) => val.llg,
            None => config_params.default_llg_code(),
        };
        let mat_date_read = each_account
            .get_i64_for_key(&required_fields_file.mat_date)
            .unwrap_or(0);
        let mut npa_flag = each_account
            .get_string_for_key(&required_fields_file.npa_flag)
            .expect("npa flag json error")
            .to_uppercase()
            .to_string();
        let mut bm_id_read = each_account
            .get_string_for_key(&required_fields_file.bm_id)
            .unwrap_or(&bmid_default_string)
            .to_string();
        let rep_freq = each_account
            .get_string_for_key(&required_fields_file.rep_freq)
            .expect("repr frequency json error")
            .to_uppercase();
        let mut rep_freq_read = match rep_freq.as_str() {
            "MONTHLY" => 1,
            "BI MONTHLY" => 2,
            "QUARTERLY" => 3,
            "HALF YEARLY" => 6,
            "ANNUAL" => 12,
            _ => 0,
        };

        if rep_freq_read == 0 {
            rep_freq_read = 14;
        }

        is_npa_acc = if npa_flag_values_vec.contains(&npa_flag.to_uppercase().trim()) {
            bm_id_read = bmid_default_string.clone();
            rep_freq_read = 14;
            true
        } else {
            false
        };
        let acc_open_date_read = each_account
            .get_i64_for_key(&required_fields_file.acc_open_date)
            .unwrap_or(0);
        let mut tenor: String = config_params.def_tenor().to_string();

        if acc_open_date_read != 0 && mat_date_read != 0 && config_params.tenor_flag() {
            let start_date = NaiveDateTime::from_timestamp(acc_open_date_read, 0);

            let matt_date = NaiveDateTime::from_timestamp(mat_date_read, 0);
            tenor = tenor::get_tenor(&tenor_file, start_date, matt_date);
        }

        curr_code_read = each_account
            .get_string_for_key(&required_fields_file.curr_code)
            .expect("currency code json error")
            .to_string();

        let mut intt_rate_read = each_account
            .get_f64_for_key(&required_fields_file.intt_rate)
            .expect("interest read json error");

        let mut next_repr_date_read = each_account
            .get_i64_for_key(&required_fields_file.next_repr_date)
            .unwrap_or(default_repr_date);
        if next_repr_date_read == 0 || next_repr_date_read <= timestamp(*config_params.as_on_date())
        {
            next_repr_date_read = default_repr_date;
        }
        if bm_id_read == "FIXED" {
            next_repr_date_read = default_repr_date;
        }

        let mut amount = get_amount(each_account, &required_fields_file);
        amount = amount.abs();
        let llg_key_consol_curr = llg_key::create_key(
            next_repr_date_read,
            llg_id_read,
            &config_params.consol_ccy().to_string(),
            &bm_id_read,
            rep_freq_read as f64,
            &tenor,
        );
        let ex_rt = currency_map
            .get(&curr_code_read)
            .expect("Cannot read exchange rate");
        if is_npa_acc {
            intt_rate_read = 0.0;
            spread = 0.0;
        } else {
            if rep_freq_read != 14 && next_repr_date_read != default_repr_date {
                spread = get_spread(
                    next_repr_date_read,
                    &bm_id_read,
                    intt_rate_read,
                    rep_freq_read,
                    &bm_rt_map,
                    &date_vec,
                );
            } else {
                spread = 0.0;
            }
        }
        let process_start_time = SystemTime::now();
        if curr_code_read == config_params.src_local_ccy().to_string() {
            let llg_key_disp_local_curr = llg_key::create_key(
                next_repr_date_read,
                llg_id_read,
                &config_params.display_local_ccy().to_string(),
                &bm_id_read,
                rep_freq_read as f64,
                &tenor,
            );
            if map_to_write.contains_key(&llg_key_consol_curr) {
                let map_value = map_to_write
                    .get(&llg_key_consol_curr)
                    .expect("llg key given curr");
                let value = organize::aggregate_existing(
                    map_value.clone(),
                    &amount,
                    ex_rt,
                    intt_rate_read,
                    &spread,
                );
                map_to_write.insert(llg_key_consol_curr.clone(), value);
            } else {
                let value = organize::aggregate_new(ex_rt, &amount, intt_rate_read, &spread);
                map_to_write.insert(llg_key_consol_curr.clone(), value.clone());
            }
            if map_to_write.contains_key(&llg_key_disp_local_curr) {
                let map_value = map_to_write
                    .get(&llg_key_disp_local_curr)
                    .expect("llg key local curr");
                let value = organize::aggregate_existing(
                    map_value.clone(),
                    &amount,
                    ex_rt,
                    intt_rate_read,
                    &spread,
                );
                map_to_write.insert(llg_key_disp_local_curr.clone(), value.clone());
            } else {
                let value = organize::aggregate_new(ex_rt, &amount, intt_rate_read, &spread);
                map_to_write.insert(llg_key_disp_local_curr.clone(), value.clone());
            }
        } else {
            let llg_key_curr_code_read = llg_key::create_key(
                next_repr_date_read,
                llg_id_read,
                &curr_code_read.to_string(),
                &bm_id_read,
                rep_freq_read as f64,
                &tenor,
            );

            let llg_key_foreign_con_curr = llg_key::create_key(
                next_repr_date_read,
                llg_id_read,
                &config_params.foreign_consol_ccy().to_string(),
                &bm_id_read,
                rep_freq_read as f64,
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
                let value = organize::aggregate_existing(
                    map_value.clone(),
                    &amount,
                    &given_ex,
                    intt_rate_read,
                    &spread,
                );
                map_to_write.insert(llg_key_curr_code_read.clone(), value.clone());
            } else {
                let value = organize::aggregate_new(&given_ex, &amount, intt_rate_read, &spread);
                map_to_write.insert(llg_key_curr_code_read.clone(), value.clone());
            }
            if map_to_write.contains_key(&llg_key_foreign_con_curr) {
                let map_value = map_to_write
                    .get(&llg_key_foreign_con_curr)
                    .expect("llg key cons curr");
                let value = organize::aggregate_existing(
                    map_value.clone(),
                    &amount,
                    &consol_ex,
                    intt_rate_read,
                    &spread,
                );
                map_to_write.insert(llg_key_foreign_con_curr.clone(), value.clone());
            } else {
                let value = organize::aggregate_new(&consol_ex, &amount, intt_rate_read, &spread);
                map_to_write.insert(llg_key_foreign_con_curr.clone(), value.clone());
            }
            if map_to_write.contains_key(&llg_key_consol_curr) {
                let map_value = map_to_write
                    .get(&llg_key_consol_curr)
                    .expect("llg key fcy curr");
                let value = organize::aggregate_existing(
                    map_value.clone(),
                    &amount,
                    &consol_ex,
                    intt_rate_read,
                    &spread,
                );
                map_to_write.insert(llg_key_consol_curr.clone(), value.clone());
            } else {
                let value = organize::aggregate_new(&consol_ex, &amount, intt_rate_read, &spread);
                map_to_write.insert(llg_key_consol_curr.clone(), value.clone());
            }
        }
        let process_end_time = SystemTime::now();
        let process_elapsed_time = process_end_time.duration_since(process_start_time).unwrap();
        total_process_time += process_elapsed_time;
        read_start_time = SystemTime::now();
    }

    let writer = writer::create_writer_for_path(config_params.output_file_path());
    writer.set_len(0).expect("writer smry set_len(0)");
    for (llg_key, value) in map_to_write.iter() {
        let curr = llg_key.curr_code.clone();
        let mut ex_rt: f64 = 1.0;
        if curr != config_params.src_local_ccy() {
            ex_rt = *currency_map
                .get(&curr)
                .expect("could not find exchange rate");
        }
        if config_params.is_consolidated() {
            ex_rt = 1.0 / ex_rt;
        }
        writer::write_to_file(
            config_params.as_on_date(),
            &writer,
            llg_key,
            value.clone(),
            &ex_rt,
        )
    }
}
