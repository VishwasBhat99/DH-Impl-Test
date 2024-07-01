use classifier::account_field_names::AccFieldNames;
use classifier::date_utils::naivedate_from_timestamp;
use macros;
use rbdate::NaiveDate;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
// asumed to be integer
pub fn get_cust_id(account: &AccountWithCFs, keys: &AccFieldNames, logger: &Logger) -> String {
    match account.get_i64_for_key(&keys.cust_id) {
        Ok(val) => val.to_string(),
        Err(_err) => match account.get_string_for_key(&keys.cust_id) {
            Ok(val) => val.to_string(),
            Err(err) => {
                log_error!(
                    logger,
                    "Error reading cust_id as I64 or String: {:?}, Default value 0 taken.",
                    err
                );
                0.to_string()
            }
        },
    }
}

pub fn get_amount(account: &AccountWithCFs, keys: &AccFieldNames, logger: &Logger) -> f64 {
    match account.get_f64_for_key(&keys.amount) {
        Ok(val) => val,
        Err(_err) => match account.get_string_for_key(&keys.cust_id) {
            Ok(val) => val.parse::<f64>().unwrap_or(0.0),
            Err(err) => {
                log_error!(
                    logger,
                    "Error reading amount as F64 or String: {:?}, Default value 0.0 taken.",
                    err
                );
                0.0
            }
        },
    }
}

pub fn get_mat_date(account: &AccountWithCFs, keys: &AccFieldNames, logger: &Logger) -> NaiveDate {
    let date_timestamp = match account.get_i64_for_key(&keys.mat_date) {
        Ok(val) => val,
        Err(_err) => match account.get_string_for_key(&keys.mat_date) {
            Ok(val) => val.parse::<i64>().unwrap_or(0),
            Err(err) => {
                log_error!(
                    logger,
                    "Error reading mat_date as I64 or String: {:?}, Default value 0 taken.",
                    err
                );
                0
            }
        },
    };
    naivedate_from_timestamp(date_timestamp)
}
// prod code assumed to be integer
pub fn get_prod_code(account: &AccountWithCFs, keys: &AccFieldNames, logger: &Logger) -> String {
    if account.get_i64_for_key(&keys.prod_code).is_err() {
        match account.get_string_for_key(&keys.prod_code) {
            Ok(val) => val.to_string(),
            Err(err) => {
                log_error!(
                    logger,
                    "Error reading prod_code as I64 or String: {:?}, Default value 0 taken.",
                    err
                );
                0.to_string()
            }
        }
    } else {
        account
            .get_i64_for_key(&keys.prod_code)
            .unwrap_or(0)
            .to_string()
    }
}

pub fn get_currency(account: &AccountWithCFs, keys: &AccFieldNames, logger: &Logger) -> String {
    match account.get_string_for_key(&keys.currency) {
        Ok(val) => val.to_string(),
        Err(err) => {
            log_error!(
                logger,
                "Error reading currency or String: {:?}, Default value NONE taken.",
                err
            );
            "NONE".to_string()
        }
    }
}

pub fn get_cust_type(account: &AccountWithCFs, key: &str, logger: &Logger) -> String {
    match account.get_string_for_key(&key.to_string()) {
        Ok(val) => val.to_string(),
        Err(err) => {
            log_error!(
                logger,
                "Error reading cust type or String: {:?}, Default value NONE taken.",
                err
            );
            "NONE".to_string()
        }
    }
}
