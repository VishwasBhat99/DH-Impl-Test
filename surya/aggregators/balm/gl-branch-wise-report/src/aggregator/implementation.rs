use aggregator::account_field_names::AccFieldNames;
use aggregator::llg_key::LLGKey;
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

#[allow(dead_code, unused_imports)]
pub fn llg_for_account(
    account: &AccountWithCFs,
    k: &AccFieldNames,
    rules: &AggRules,
    default_llg_code: i32,
    logger: &Logger,
    debug_writer: &mut BufWriter<File>,
    config_params: &ConfigurationParameters,
) -> LLGKey {
    let currency = account
        .get_string_for_key(&k.institution)
        .expect("Error while getting currency.");

    let account_num = account
        .get_string_for_key(&k.account_number)
        .expect("Error while getting account number.");
    let def_string = String::from("NA");
    let item_id = account
        .get_string_for_key(&k.item_id)
        .unwrap_or(&def_string);
    let mut gl_debug_fields = String::new();
    for key in &k.pass_through {
        match account.get_string_for_key(key) {
            Ok(val) => {
                gl_debug_fields.push_str(val);
                gl_debug_fields.push('|');
            }
            Err(_err) => match account.get_f64_for_key(key) {
                Ok(val) => {
                    gl_debug_fields.push_str(&val.to_string());
                    gl_debug_fields.push('|');
                }
                Err(_err) => {
                    gl_debug_fields.push_str("0");
                    gl_debug_fields.push('|');
                }
            },
        }
    }
    let category = match rules.llg_for_acc(account) {
        Some(c) => {
            let temp_llg = c.llg % 10000;
            gl_debug_fields.push_str(&temp_llg.to_string());
            gl_debug_fields.push('|');
            gl_debug_fields.push_str(&c.rule_id.to_string());
            gl_debug_fields.push('\n');
            log_debug!(
                logger,
                "Account `{:?}` evaluated to LLGId `{}`, using rule id `{}`",
                account_num,
                c.llg,
                c.rule_id
            );
            c.llg
        }
        None => {
            let temp_default_llg = default_llg_code % 10000;
            gl_debug_fields.push_str(&temp_default_llg.to_string());
            gl_debug_fields.push('|');
            gl_debug_fields.push_str("0");
            gl_debug_fields.push('\n');
            log_debug!(
                logger,
                "Account `{:?}` defaulted to LLGId `{}`",
                account_num,
                default_llg_code
            );
            default_llg_code
        }
    };
    let cf_type = "O";
    if config_params.log_level() == "debug" {
        match debug_writer.write(gl_debug_fields.as_bytes()) {
            Ok(_val) => {}
            Err(err) => println!("Error writing to output file. Error: {}", err),
        }
    }
    LLGKey::new(
        currency.to_string(),
        category,
        cf_type.to_string(),
        config_params.dim_id().to_string(),
        item_id.to_string(),
    )
}
