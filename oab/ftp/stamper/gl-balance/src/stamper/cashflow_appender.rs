use sdb_agg_rules::agg_rules::AggRules;
use slog::Logger;
use stamper::account_reader;
use stamper::account_without_cashflows::OutputAccount;
use stamper::llg_finder;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};

pub fn append_cashflow(
    cfin: &mut sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs<'_>,
    input_field_names: &account_reader::AccFieldNames,
    rules: &AggRules,
    default_rl1: i32,
    log: &Logger,
) -> (OutputAccount) {
    let mut cfoutput = OutputAccount::new();

    cfoutput.gl_item = match cfin.get_i64_for_key(&input_field_names.gl_item) {
        Ok(result) => result,
        Err(_e) => DEFAULT_INT,
    };

    cfoutput.branch = match cfin.get_i64_for_key(&input_field_names.branch) {
        Ok(result) => result,
        Err(_e) => DEFAULT_INT,
    };

    cfoutput.basic = match cfin.get_i64_for_key(&input_field_names.basic) {
        Ok(result) => result,
        Err(_e) => DEFAULT_INT,
    };

    cfoutput.suffix = match cfin.get_i64_for_key(&input_field_names.suffix) {
        Ok(result) => result,
        Err(_e) => DEFAULT_INT,
    };

    cfoutput.currency = match cfin.get_string_for_key(&input_field_names.currency) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.cf_amount = match cfin.get_f64_for_key(&input_field_names.cf_amount) {
        Ok(result) => result,
        Err(_e) => DEFAULT_FLOAT,
    };

    cfoutput.balance_in_omr = match cfin.get_f64_for_key(&input_field_names.balance_in_omr) {
        Ok(result) => result,
        Err(_e) => DEFAULT_FLOAT,
    };

    cfoutput.rl1 =
        llg_finder::get_llg_id(cfin, &input_field_names.gl_item, &rules, default_rl1, &log);

    (cfoutput)
}
