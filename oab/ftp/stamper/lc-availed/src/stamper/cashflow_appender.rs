use sdb_agg_rules::agg_rules::AggRules;
use slog::Logger;
use stamper::account_reader;
use stamper::account_with_cashflows::AccountWithCashflows;
use stamper::account_with_cashflows::Cashflow;
use stamper::llg_finder;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

pub fn append_cashflow(
    cfin: &mut sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs<'_>,
    input_field_names: &account_reader::AccFieldNames,
    rules: &AggRules,
    default_rl1: i32,
    log: &Logger,
) -> (AccountWithCashflows) {
    let mut cfoutput = AccountWithCashflows::new();

    cfoutput.account_id = match cfin.get_string_for_key(&input_field_names.account_id) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.reference = match cfin.get_string_for_key(&input_field_names.reference) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.start_date = match cfin.get_i64_for_key(&input_field_names.start_date) {
        Ok(result) => result,
        Err(_e) => DEFAULT_INT,
    };

    cfoutput.maturity_date = match cfin.get_i64_for_key(&input_field_names.maturity_date) {
        Ok(result) => result,
        Err(_e) => DEFAULT_INT,
    };

    cfoutput.outstanding_bal = match cfin.get_f64_for_key(&input_field_names.outstanding_bal) {
        Ok(result) => result,
        Err(_e) => DEFAULT_FLOAT,
    };

    cfoutput.currency = match cfin.get_string_for_key(&input_field_names.currency) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.lcy_amount = match cfin.get_f64_for_key(&input_field_names.lcy_amount) {
        Ok(result) => result,
        Err(_e) => DEFAULT_FLOAT,
    };

    cfoutput.customer_type = match cfin.get_string_for_key(&input_field_names.customer_type) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.total_interest_amount =
        match cfin.get_f64_for_key(&input_field_names.total_interest_amount) {
            Ok(result) => result,
            Err(_e) => DEFAULT_FLOAT,
        };

    cfoutput.total_principal_amount =
        match cfin.get_f64_for_key(&input_field_names.total_principal_amount) {
            Ok(result) => result,
            Err(_e) => DEFAULT_FLOAT,
        };

    // TODO: We need to be able to read and write into Cashflow without having to iterate through cashflow values
    let cashflow_data: Vec<Cashflow> = match &cfin.remove_cfs_for_key(&input_field_names.cashflows)
    {
        Ok(result) => {
            let mut cashflow: Vec<Cashflow> = Vec::new();
            for i in 0..result.len() {
                cashflow.push(Cashflow {
                    int_amt: result[i].interest_amount,
                    prin_amt: result[i].principal_amount,
                    date: result[i].date,
                    unknown_fields: Default::default(),
                    cached_size: Default::default(),
                });
            }

            cashflow
        }
        Err(_e) => {
            let cashflow: Vec<Cashflow> = Default::default();

            cashflow
        }
    };
    cfoutput.cashflows = protobuf::RepeatedField::from_vec(cashflow_data);

    cfoutput.rl1 = llg_finder::get_llg_id(
        cfin,
        &input_field_names.account_id,
        &rules,
        default_rl1,
        &log,
    );

    (cfoutput)
}
