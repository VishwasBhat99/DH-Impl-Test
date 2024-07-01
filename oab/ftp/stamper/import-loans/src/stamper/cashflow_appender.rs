use macros;
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
    stamp_field: &str,
    default_stamp_code: i32,
    log: &Logger,
) -> AccountWithCashflows {
    let mut cfoutput = AccountWithCashflows::new();

    cfoutput.account_id = match cfin.get_string_for_key(&input_field_names.account_id) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.currency = match cfin.get_string_for_key(&input_field_names.currency) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.int_rate = match cfin.get_f64_for_key(&input_field_names.int_rate) {
        Ok(result) => result,
        Err(_e) => DEFAULT_FLOAT,
    };

    cfoutput.outstanding_bal = match cfin.get_f64_for_key(&input_field_names.outstanding_bal) {
        Ok(result) => result,
        Err(_e) => DEFAULT_FLOAT,
    };

    cfoutput.gl = match cfin.get_i64_for_key(&input_field_names.gl) {
        Ok(result) => result,
        Err(_e) => DEFAULT_INT,
    };

    cfoutput.start_date = match cfin.get_i64_for_key(&input_field_names.start_date) {
        Ok(result) => result,
        Err(_e) => DEFAULT_INT,
    };

    cfoutput.maturity_date = match cfin.get_i64_for_key(&input_field_names.maturity_date) {
        Ok(result) => result,
        Err(_e) => DEFAULT_INT,
    };

    cfoutput.customer_id = match cfin.get_string_for_key(&input_field_names.customer_id) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.customer_type = match cfin.get_string_for_key(&input_field_names.customer_type) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.lcy_amount = match cfin.get_f64_for_key(&input_field_names.lcy_amount) {
        Ok(result) => result,
        Err(_e) => DEFAULT_FLOAT,
    };

    cfoutput.reference = match cfin.get_string_for_key(&input_field_names.reference) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.npa_flag = match cfin.get_string_for_key(&input_field_names.npa_flag) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.npa_type = match cfin.get_string_for_key(&input_field_names.npa_type) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.interest_type = match cfin.get_string_for_key(&input_field_names.interest_type) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.int_repayment_frequency =
        match cfin.get_i64_for_key(&input_field_names.int_repayment_frequency) {
            Ok(result) => result,
            Err(_e) => DEFAULT_INT,
        };

    cfoutput.last_repr_date = match cfin.get_i64_for_key(&input_field_names.last_repr_date) {
        Ok(result) => result,
        Err(_e) => DEFAULT_INT,
    };

    cfoutput.next_repr_date = match cfin.get_i64_for_key(&input_field_names.next_repr_date) {
        Ok(result) => result,
        Err(_e) => DEFAULT_INT,
    };

    cfoutput.cust_constitution_code =
        match cfin.get_i64_for_key(&input_field_names.cust_constitution_code) {
            Ok(result) => result,
            Err(_e) => DEFAULT_INT,
        };

    cfoutput.rate_flag = match cfin.get_string_for_key(&input_field_names.rate_flag) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.customer_name = match cfin.get_string_for_key(&input_field_names.customer_name) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.product_code = match cfin.get_string_for_key(&input_field_names.product_code) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.account_type = match cfin.get_string_for_key(&input_field_names.account_type) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.branch = match cfin.get_string_for_key(&input_field_names.branch) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.rm = match cfin.get_string_for_key(&input_field_names.rm) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.group_code = match cfin.get_string_for_key(&input_field_names.group_code) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.monthly_avg_bal = match cfin.get_string_for_key(&input_field_names.monthly_avg_bal) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.aorl = match cfin.get_string_for_key(&input_field_names.aorl) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.rl1 = match cfin.get_i32_for_key(&input_field_names.rl1) {
        Ok(result) => result,
        Err(_e) => DEFAULT_INT as i32,
    };

    cfoutput.rl2 = match cfin.get_i32_for_key(&input_field_names.rl2) {
        Ok(result) => result,
        Err(_e) => DEFAULT_INT as i32,
    };

    cfoutput.rl3 = match cfin.get_i32_for_key(&input_field_names.rl3) {
        Ok(result) => result,
        Err(_e) => DEFAULT_INT as i32,
    };

    cfoutput.customer_rating = match cfin.get_string_for_key(&input_field_names.customer_rating) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.p2 = match cfin.get_string_for_key(&input_field_names.p2) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.waiver_flag = match cfin.get_string_for_key(&input_field_names.waiver_flag) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.accrued_int_amt = match cfin.get_string_for_key(&input_field_names.accrued_int_amt) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.string1 = match cfin.get_string_for_key(&input_field_names.string1) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.string2 = match cfin.get_string_for_key(&input_field_names.string2) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.string3 = match cfin.get_string_for_key(&input_field_names.string3) {
        Ok(result) => result,
        Err(_e) => "",
    }
    .to_string();

    cfoutput.number1 = match cfin.get_i64_for_key(&input_field_names.number1) {
        Ok(result) => result,
        Err(_e) => DEFAULT_INT,
    };

    cfoutput.number2 = match cfin.get_i64_for_key(&input_field_names.number2) {
        Ok(result) => result,
        Err(_e) => DEFAULT_INT,
    };

    cfoutput.number3 = match cfin.get_i64_for_key(&input_field_names.number3) {
        Ok(result) => result,
        Err(_e) => DEFAULT_INT,
    };

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
            let mut cashflows: Vec<Cashflow> = Vec::new();
            for index in 0..result.len() {
                let mut cf: Cashflow = Cashflow::new();
                cf.int_amt = result[index].interest_amount;
                cf.prin_amt = result[index].principal_amount;
                cf.date = result[index].date;
                cashflows.push(cf);
            }
            cashflows
        }
        Err(error) => {
            log_error!(
                log,
                "Error in appending cashflow: {:?} for account {}",
                error,
                cfoutput.account_id
            );
            Vec::new()
        }
    };
    cfoutput.cashflows = protobuf::RepeatedField::from_vec(cashflow_data);

    let llg_id = llg_finder::get_llg_id(
        cfin,
        &input_field_names.account_id,
        &rules,
        default_stamp_code,
        &log,
    );

    match stamp_field.to_lowercase().as_str() {
        "aorl" => cfoutput.aorl = get_aorl(llg_id, log),
        "rl1" => cfoutput.rl1 = llg_id,
        "rl2" => cfoutput.rl2 = llg_id,
        "rl3" => cfoutput.rl3 = llg_id,
        _ => {
            log_error!(log, "Undefined Stamper Field : {}", stamp_field);
        }
    }

    cfoutput
}

fn get_aorl(llg_id: i32, log: &Logger) -> String {
    match llg_id {
        0 => "L",
        1 => "A",
        _ => {
            log_error!(log, "Invalid Asset or Liability Code {}", llg_id);
            ""
        }
    }
    .to_string()
}
