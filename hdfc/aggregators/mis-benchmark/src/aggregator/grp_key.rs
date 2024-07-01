use super::account::AccData;
use super::config::File;
use aggregator::account_field_names::AccFieldNames;
use aggregator::llg_key::LLGKey;
use aggregator::tenor::*;
use configuration_parameters::ConfigurationParameters;
use rbdate::{NaiveDate, NaiveDateTime};
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use sdb_dyn_proto_rdr::reader::types::Type;
// use sdb_io::open_file_read;
use slog::Logger;
use std::collections::HashMap;
use std::io::prelude::*;

#[allow(dead_code, unused_imports)]
pub fn fetch_acc_data(
    reader_for_calling_method: &reader::Reader,
    file: &File,
    tenor_map: &HashMap<Tenor, String>,
    exrt: &f64,
    source: String,
    mut account: AccountWithCFs,
    keys: &AccFieldNames,
    num_day_bkts: &Vec<i64>,
    config_params: &ConfigurationParameters,
    _logger: &Logger,
    is_maturity: &bool,
    tot_amt: &f64,
    is_negative: &bool,
) -> AccData {
    let mut acc_data: Vec<f64> = vec![0.0; 51];
    let def_val = "NA".to_string();
    let next_rep_dt_read = account.get_i64_for_key(&keys.next_rep_dt).unwrap_or(0);
    let next_rep_dt = naivedate_from_timestamp(next_rep_dt_read);
    let multiplier = if *is_negative { -1.0 } else { 1.0 };
    // let reader_for_calling_method =
    //     reader::Reader::new_at_path(&file.metadata_file_path, &file.input_file_path);
    // let mut required_fields_file = open_file_read(&file.required_fields_file_path)
    //     .expect("Cannot open the required fields file.");

    // let mut required_fields_buffer = String::new();
    // required_fields_file
    //     .read_to_string(&mut required_fields_buffer)
    //     .expect("Cannot read the required fields file.");

    // let rf: AccFieldNames = serde_json::from_str(&required_fields_buffer[..])
    //     .expect("Unable to parse the required fields file.");
    for cf in account
        .remove_cfs_for_key(&keys.cashflows)
        .expect("Error while removing cashflow from the pool of cashflows.")
        .iter_mut()
    {
        let mut cf_date = naivedate_from_timestamp(cf.get_date());
        if config_params.bkt_scheme_id() == "IRS" {
            if next_rep_dt_read != 0 && next_rep_dt < cf_date {
                cf_date = next_rep_dt;
            }
        }
        let cf_tenor = if &cf_date > config_params.as_on_date() {
            rbdate::num_days_start_to_end(*config_params.as_on_date(), cf_date)
        } else {
            -1
        };
        let mut idx: usize = 1;
        if !is_maturity {
            acc_data[0] = *tot_amt * multiplier;
        } else {
            for day in num_day_bkts {
                if &cf_tenor < day {
                    acc_data[idx] += cf.get_principal_amount() * exrt * multiplier;
                    break;
                } else {
                    idx += 1;
                }
            }
        }
    }
    let ccy = get_required_data(&keys.ccy, &reader_for_calling_method, &mut account);
    let dim_1 = get_required_data(&keys.dim_1, &reader_for_calling_method, &mut account);
    let dim_2 = get_required_data(&keys.dim_2, &reader_for_calling_method, &mut account);
    let dim_3 = get_required_data(&keys.dim_3, &reader_for_calling_method, &mut account);
    let dim_4 = get_required_data(&keys.dim_4, &reader_for_calling_method, &mut account);
    let dim_5 = get_required_data(&keys.dim_5, &reader_for_calling_method, &mut account);
    let dim_6 = get_required_data(&keys.dim_6, &reader_for_calling_method, &mut account);
    let dim_7 = get_required_data(&keys.dim_7, &reader_for_calling_method, &mut account);
    let dim_8 = get_required_data(&keys.dim_8, &reader_for_calling_method, &mut account);
    let dim_9 = get_required_data(&keys.dim_9, &reader_for_calling_method, &mut account);
    let dim_10 = get_required_data(&keys.dim_10, &reader_for_calling_method, &mut account);
    let dim_11 = get_required_data(&keys.dim_11, &reader_for_calling_method, &mut account);
    let dim_12 = get_required_data(&keys.dim_12, &reader_for_calling_method, &mut account);
    let dim_13 = get_required_data(&keys.dim_13, &reader_for_calling_method, &mut account);
    let dim_14 = get_required_data(&keys.dim_14, &reader_for_calling_method, &mut account);
    let dim_15 = get_required_data(&keys.dim_15, &reader_for_calling_method, &mut account);

    let acc_strt_dt = account.get_i64_for_key(&keys.acc_strt_dt).unwrap_or(0);
    let mat_dt = account.get_i64_for_key(&keys.mat_dt).unwrap_or(0);
    let next_rep_dt = account.get_i64_for_key(&keys.next_rep_dt).unwrap_or(0);

    let days = ((mat_dt - acc_strt_dt) / 3600) / 24;
    let mut bkt_scheme = "";
    for (key, value) in tenor_map {
        if days >= key.from_days && days <= key.to_days {
            bkt_scheme = value;
            break;
        }
    }

    let grp_key = LLGKey::new(
        config_params.as_on_date().to_string(),
        source.to_string(),
        config_params.report_id().to_string(),
        config_params.bkt_scheme_id().to_string(),
        ccy.to_string(),
        dim_1.to_string(),
        dim_2.to_string(),
        dim_3.to_string(),
        dim_4.to_string(),
        dim_5.to_string(),
        dim_6.to_string(),
        dim_7.to_string(),
        dim_8.to_string(),
        dim_9.to_string(),
        dim_10.to_string(),
        dim_11.to_string(),
        dim_12.to_string(),
        dim_13.to_string(),
        dim_14.to_string(),
        dim_15.to_string(),
        bkt_scheme.to_string(),
    );

    AccData {
        grp_key: grp_key,
        acc_data: acc_data,
    }
}

pub fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}

pub fn get_required_data(
    field_name: &str,
    reader_for_calling_method: &reader::Reader,
    record: &mut AccountWithCFs,
) -> String {
    let mut req_val = "NA".to_string();
    let def = "NA".to_string();
    if field_name == "" {
        return def;
    }
    match reader_for_calling_method
        .get_field_type(&field_name.to_string())
        .unwrap_or(Type::Cashflows)
    {
        Type::I32 => {
            let val = record.get_i32_for_key(&field_name.to_string()).unwrap_or(0);
            req_val = val.to_string();
        }

        Type::I64 => {
            let val = record.get_i64_for_key(&field_name.to_string()).unwrap_or(0);
            req_val = val.to_string();
        }

        Type::F32 => {
            let val = record
                .get_f32_for_key(&field_name.to_string())
                .unwrap_or(0.0);
            req_val = val.to_string();
        }

        Type::F64 => {
            let val = record
                .get_f64_for_key(&field_name.to_string())
                .unwrap_or(0.0);
            req_val = val.to_string();
        }

        Type::String => {
            let val = record
                .get_string_for_key(&field_name.to_string())
                .unwrap_or(&def);
            req_val = val.to_string();
        }
        Type::Cashflows => {
            req_val = "NA".to_string();
        }
        _ => {
            req_val = "NA".to_string();
        }
    };
    return req_val;
}
