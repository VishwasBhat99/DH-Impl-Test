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

    cfoutput.account_id = match cfin.get_i64_for_key(&input_field_names.account_id) {
        Ok(result) => result,
        Err(_e) => DEFAULT_INT,
    };

    cfoutput.currency = match cfin.get_string_for_key(&input_field_names.currency) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.outstanding_bal = match cfin.get_f64_for_key(&input_field_names.outstanding_bal) {
        Ok(result) => result,
        Err(_e) => DEFAULT_FLOAT,
    };

    cfoutput.funded = match cfin.get_f64_for_key(&input_field_names.funded) {
        Ok(result) => result,
        Err(_e) => DEFAULT_FLOAT,
    };

    cfoutput.non_funded = match cfin.get_f64_for_key(&input_field_names.non_funded) {
        Ok(result) => result,
        Err(_e) => DEFAULT_FLOAT,
    };

    cfoutput.limit_structure = match cfin.get_string_for_key(&input_field_names.limit_structure) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.customer_loc = match cfin.get_string_for_key(&input_field_names.customer_loc) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.ctp = match cfin.get_string_for_key(&input_field_names.ctp) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.expiry_date = match cfin.get_i64_for_key(&input_field_names.expiry_date) {
        Ok(result) => result,
        Err(_e) => DEFAULT_INT,
    };

    cfoutput.rl1 = llg_finder::get_llg_id(
        cfin,
        &input_field_names.account_id,
        &rules,
        default_rl1,
        &log,
    );

    (cfoutput)
}
