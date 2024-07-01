use classifier::account_field_names::AccFieldNames;
use macros;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;

pub fn add_std_fields(
    op: &mut String,
    account: &AccountWithCFs,
    keys: &AccFieldNames,
    logger: &Logger,
) {
    for key in &keys.std_fields {
        match account.get_string_for_key(key) {
            Ok(val) => {
                op.push_str(val);
                op.push('|');
            }
            Err(_err) => match account.get_i64_for_key(key) {
                Ok(val) => {
                    op.push_str(&val.to_string());
                    op.push('|');
                }
                Err(_err) => match account.get_f64_for_key(key) {
                    Ok(val) => {
                        op.push_str(&val.to_string());
                        op.push('|');
                    }
                    Err(err) => {
                        log_error!(logger, "Error reading pass_through: {:?}", err);
                    }
                },
            },
        }
    }
}

pub fn add_pass_through(
    op: &mut String,
    account: &AccountWithCFs,
    keys: &AccFieldNames,
    logger: &Logger,
) {
    for key in &keys.pass_through {
        match account.get_string_for_key(key) {
            Ok(val) => {
                op.push_str(val);
                op.push('|');
            }
            Err(_err) => match account.get_i64_for_key(key) {
                Ok(val) => {
                    op.push_str(&val.to_string());
                    op.push('|');
                }
                Err(_err) => match account.get_f64_for_key(key) {
                    Ok(val) => {
                        op.push_str(&val.to_string());
                        op.push('|');
                    }
                    Err(err) => {
                        log_error!(logger, "Error reading pass_through: {:?}", err);
                    }
                },
            },
        }
    }
}

pub fn add_derived_fields(
    op: &mut String,
    cust_type: &String,
    amount: String,
    lcy_amount: String,
    res_days: String,
    bucket_id: &String,
    is_nwd: &str,
    is_nwd_final: &str,
    mat_date: String,
) {
    op.push_str(&mat_date);
    op.push('|');
    op.push_str(&amount);
    op.push('|');
    op.push_str(&lcy_amount);
    op.push('|');
    op.push_str(&cust_type);
    op.push('|');
    op.push_str(&res_days);
    op.push('|');
    op.push_str(is_nwd);
    op.push('|');
    op.push_str(is_nwd_final);
    op.push('|');
    op.push_str(&bucket_id);
    op.push('|');
}
