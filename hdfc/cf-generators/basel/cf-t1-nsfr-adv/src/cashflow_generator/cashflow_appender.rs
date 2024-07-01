use super::account_as_cashflows::Account;
use super::CustFields;
use cashflow_generator::account_field_names::AccFieldNames;
use cashflow_generator::RFFlag;
use cashflow_generator::RMFlag;
use cashflow_generator::RWData;
use cashflow_generator::RangeSlab;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{NaiveDate, NaiveDateTime};
use sdb_dyn_proto_rdr::reader::account_with_cfs::get_field_value;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use sdb_dyn_proto_rdr::reader::Reader;
use slog::Logger;
use statics::*;
use std::collections::HashMap;

pub fn append_data<'a>(
    acc_id: String,
    mut account: AccountWithCFs,
    method_reader: &Reader,
    keys: &AccFieldNames,
    prd_slabs: &Vec<RangeSlab>,
    cust_master: &HashMap<String, String>,
    rw_master: &HashMap<String, RWData>,
    rf_master: &HashMap<String, RFFlag>,
    rm_master: &HashMap<String, RMFlag>,
    config_params: &ConfigurationParameters,
    cust_details: &HashMap<String, CustFields>,
    ea_master: &HashMap<String, NaiveDate>,
    diag_log: &Logger,
) -> Account {
    let mut amt_buckets = vec![0.0, 0.0, 0.0, 0.0];
    if config_params.has_cashflows() {
        let cfs = match account.remove_cfs_for_key(&keys.cashflows) {
            Ok(cfs) => cfs,
            Err(_) => {
                log_debug!(
                    diag_log,
                    "Account {} - does not have cashflows",
                    acc_id.to_string()
                );
                Vec::new()
            }
        };
        for cf in cfs {
            let cf_date = naivedate_from_timestamp(cf.get_date());
            let prin_amt = cf.get_principal_amount();
            let as_on_date = config_params.as_on_date();
            let cf_res_days: f64 = if as_on_date <= &cf_date {
                rbdate::num_days_start_to_end(*as_on_date, cf_date) as f64
            } else {
                -1.0 * rbdate::num_days_start_to_end(cf_date, *as_on_date) as f64
            };
            for val in prd_slabs {
                if cf_res_days >= val.from && cf_res_days < val.to {
                    let bkt_id = val
                        .id
                        .parse::<usize>()
                        .expect("Invalid bucket id encountered in slabs config file.");
                    amt_buckets[bkt_id] += prin_amt;
                }
            }
        }
    } else {
        let ost_bal = account
            .get_f64_for_key(&keys.ost_bal)
            .expect("Error while reading ost bal.");
        amt_buckets[0] = ost_bal;
    }

    let mut out_acc = Account::new();

    let cust_id = match get_field_value(&account, &method_reader, keys.cust_id.to_string()) {
        Ok(value) => value,
        Err(_error) => panic!("{}", _error),
    };
    let cust_type = match cust_master.get(&cust_id) {
        Some(val) => val.trim().to_string(),
        None => {
            log_debug!(diag_log, "Unmapped Cust ID: --{}--", cust_id);
            "NA".to_string()
        }
    };
    let def_data = RWData { rw_prcnt: 999.9 };
    let rw_data = match rw_master.get(&acc_id) {
        Some(val) => val,
        None => &def_data,
    };

    let rf_flag_data = RFFlag {
        rf_flag: "N".to_string(),
    };
    let rf_data = match rf_master.get(&acc_id) {
        Some(val) => val,
        None => &rf_flag_data,
    };

    let rm_flag_data = RMFlag {
        rm_flag: "N".to_string(),
    };
    let rm_data = match rm_master.get(&acc_id) {
        Some(val) => val,
        None => &rm_flag_data,
    };
    let default_date =
        NaiveDate::parse_from_str("01-01-1900", "%d-%m-%Y").unwrap_or(*config_params.as_on_date());
    let pledge_end_date = match ea_master.get(&acc_id) {
        Some(val) => val,
        None => &default_date,
    };
    let mut encum_flag = "N";
    if *pledge_end_date >= rbdate::incr_dt_by_yrs(*config_params.as_on_date(), 1) {
        encum_flag = "Y";
    }
    out_acc.encum_flag = encum_flag.to_string();
    let prod_code = account
        .get_string_for_key(&keys.prod_code)
        .expect("Error while reading prod code.");
    let int_rate = account
        .get_f64_for_key(&keys.int_rate)
        .expect("Error while reading int rate.");

    out_acc.pt_str_1 = account
        .get_string_for_key(&keys.pt_str_1)
        .unwrap_or(&String::from("NA"))
        .to_string();
    out_acc.pt_str_2 = account
        .get_string_for_key(&keys.pt_str_2)
        .unwrap_or(&String::from("NA"))
        .to_string();
    out_acc.pt_str_3 = account
        .get_string_for_key(&keys.pt_str_3)
        .unwrap_or(&String::from("NA"))
        .to_string();
    out_acc.pt_str_4 = account
        .get_string_for_key(&keys.pt_str_4)
        .unwrap_or(&String::from("NA"))
        .to_string();
    out_acc.pt_str_5 = account
        .get_string_for_key(&keys.pt_str_5)
        .unwrap_or(&String::from("NA"))
        .to_string();
    out_acc.pt_int_1 = account
        .get_i64_for_key(&keys.pt_int_1)
        .unwrap_or(DEFAULT_INT);
    out_acc.pt_int_2 = account
        .get_i64_for_key(&keys.pt_int_2)
        .unwrap_or(DEFAULT_INT);
    out_acc.pt_int_3 = account
        .get_i64_for_key(&keys.pt_int_3)
        .unwrap_or(DEFAULT_INT);
    out_acc.pt_int_4 = account
        .get_i64_for_key(&keys.pt_int_4)
        .unwrap_or(DEFAULT_INT);
    out_acc.pt_int_5 = account
        .get_i64_for_key(&keys.pt_int_5)
        .unwrap_or(DEFAULT_INT);
    out_acc.pt_f64_1 = account
        .get_f64_for_key(&keys.pt_f64_1)
        .unwrap_or(DEFAULT_FLOAT);
    out_acc.pt_f64_2 = account
        .get_f64_for_key(&keys.pt_f64_2)
        .unwrap_or(DEFAULT_FLOAT);
    out_acc.pt_f64_3 = account
        .get_f64_for_key(&keys.pt_f64_3)
        .unwrap_or(DEFAULT_FLOAT);
    out_acc.pt_f64_4 = account
        .get_f64_for_key(&keys.pt_f64_4)
        .unwrap_or(DEFAULT_FLOAT);
    out_acc.pt_f64_5 = account
        .get_f64_for_key(&keys.pt_f64_5)
        .unwrap_or(DEFAULT_FLOAT);

    out_acc.acc_id = acc_id.to_string();
    out_acc.cust_id = cust_id.to_string();
    out_acc.prod_code = prod_code.to_string();
    out_acc.cust_type = cust_type.to_string();
    out_acc.int_rate = int_rate;
    out_acc.currency = account
        .get_string_for_key(&keys.currency)
        .unwrap_or(&"NONE".to_string())
        .to_string();

    let def_cust_fields = CustFields::new();
    let cust_fields = cust_details
        .get(&out_acc.cust_id)
        .unwrap_or(&def_cust_fields);
    out_acc.cust_basel_val_code = cust_fields.cust_basel_val_code.to_string();
    out_acc.div = cust_fields.div.to_string();
    out_acc.nat_of_bus = cust_fields.nat_of_bus.to_string();
    out_acc.txt_desc = if cust_fields.txt_desc {
        String::from("TRUE")
    } else {
        String::from("FALSE")
    };
    out_acc.sme_class = if cust_fields.sme_class {
        String::from("Y")
    } else {
        String::from("N")
    };
    out_acc.rw = rw_data.rw_prcnt;
    out_acc.restr_flag = rf_data.rf_flag.to_string();
    out_acc.resi_flag = rm_data.rm_flag.to_string();
    out_acc.b1 = amt_buckets[0];
    out_acc.b2 = amt_buckets[1];
    out_acc.b3 = amt_buckets[2];
    out_acc.b4 = amt_buckets[3];

    out_acc
}

fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}
