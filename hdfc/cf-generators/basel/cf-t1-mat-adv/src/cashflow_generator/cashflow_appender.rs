use super::account_as_cashflows::Account;
use super::CustFields;
use cashflow_generator::account_field_names::AccFieldNames;
use cashflow_generator::RangeSlab;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{NaiveDate, NaiveDateTime};
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};
use std::collections::HashMap;

pub fn append_data<'a>(
    mut account: AccountWithCFs,
    keys: &AccFieldNames,
    prd_slabs: &Vec<RangeSlab>,
    cust_master: &HashMap<String, String>,
    config_params: &ConfigurationParameters,
    cust_details: &HashMap<String, CustFields>,
    diag_log: &Logger,
) -> Account {
    let mut amt_buckets = vec![0.0, 0.0, 0.0, 0.0];

    for cf in account
        .remove_cfs_for_key(&keys.cashflows)
        .expect("Error while removing cashflow from the pool of cashflows.")
        .iter_mut()
    {
        let cf_date = naivedate_from_timestamp(cf.get_date());
        let prin_amt = match config_params.amount_type().to_uppercase().as_str() {
            "INT" => cf.get_interest_amount(),
            _=> cf.get_principal_amount(),
        };
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
    let default_string = "".to_string();
    let mut out_acc = Account::new();

    let cust_id = account
        .get_string_for_key(&keys.cust_id)
        .expect("Error while reading cust id.");
    let cust_type = match cust_master.get(cust_id) {
        Some(val) => val.trim().to_string(),
        None => {
            log_debug!(diag_log, "Unmapped Cust ID: --{}--", cust_id);
            "NA".to_string()
        }
    };
    let acc_id = account
        .get_string_for_key(&keys.acc_id)
        .expect("Error while reading account id.");
    let prod_code = account
        .get_string_for_key(&keys.prod_code)
        .expect("Error while reading prod code.");
    let int_rate = account
        .get_f64_for_key(&keys.int_rate)
        .expect("Error while reading int rate.");
    let str1 = account
        .get_string_for_key(&keys.str1)
        .unwrap_or(&default_string);
    let str2 = account
        .get_string_for_key(&keys.str2)
        .unwrap_or(&default_string);
    let str3 = account
        .get_string_for_key(&keys.str3)
        .unwrap_or(&default_string);
    let str4 = account
        .get_string_for_key(&keys.str4)
        .unwrap_or(&default_string);
    let str5 = account
        .get_string_for_key(&keys.str5)
        .unwrap_or(&default_string);
    let int1 = account.get_i64_for_key(&keys.int1).unwrap_or(DEFAULT_INT);
    let int2 = account.get_i64_for_key(&keys.int2).unwrap_or(DEFAULT_INT);
    let int3 = account.get_i64_for_key(&keys.int3).unwrap_or(DEFAULT_INT);
    let int4 = account.get_i64_for_key(&keys.int4).unwrap_or(DEFAULT_INT);
    let int5 = account.get_i64_for_key(&keys.int5).unwrap_or(DEFAULT_INT);
    let float1 = account
        .get_f64_for_key(&keys.float1)
        .unwrap_or(DEFAULT_FLOAT);
    let float2 = account
        .get_f64_for_key(&keys.float2)
        .unwrap_or(DEFAULT_FLOAT);
    let float3 = account
        .get_f64_for_key(&keys.float3)
        .unwrap_or(DEFAULT_FLOAT);
    let float4 = account
        .get_f64_for_key(&keys.float4)
        .unwrap_or(DEFAULT_FLOAT);
    let float5 = account
        .get_f64_for_key(&keys.float5)
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

    out_acc.str1 = str1.to_string();
    out_acc.str2 = str2.to_string();
    out_acc.str3 = str3.to_string();
    out_acc.str4 = str4.to_string();
    out_acc.str5 = str5.to_string();
    out_acc.int1 = int1;
    out_acc.int2 = int2;
    out_acc.int3 = int3;
    out_acc.int4 = int4;
    out_acc.int5 = int5;
    out_acc.float1 = float1;
    out_acc.float2 = float2;
    out_acc.float3 = float3;
    out_acc.float4 = float4;
    out_acc.float5 = float5;
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
