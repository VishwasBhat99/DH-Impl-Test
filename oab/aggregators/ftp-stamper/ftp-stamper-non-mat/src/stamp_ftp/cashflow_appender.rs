use macros;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use stamp_ftp::account_with_cashflows::{AccountWithCashflows, Cashflow};
use stamp_ftp::required_fields::RequiredFields;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};

pub fn append_cashflow(
    cfin: &mut AccountWithCFs,
    required_fields: &RequiredFields,
    log: &Logger,
) -> AccountWithCashflows {
    let mut cfoutput = AccountWithCashflows::new();

    cfoutput.account_id = match cfin.get_string_for_key(&required_fields.account_number) {
        Ok(result) => result,
        Err(err) => {
            log_error!(log, "Error in reading account_number. Error = {:?}", err);
            ""
        }
    }
    .to_string();

    cfoutput.currency = match cfin.get_string_for_key(&required_fields.currency) {
        Ok(result) => result,
        Err(err) => {
            log_error!(log, "Error in reading currency. Error = {:?}", err);
            ""
        }
    }
    .to_string();

    cfoutput.balance_ccy = match cfin.get_f64_for_key(&required_fields.amount) {
        Ok(result) => result,
        Err(err) => {
            log_error!(log, "Error in reading amount. Error = {:?}", err);
            DEFAULT_FLOAT
        }
    };

    cfoutput.int_rate = match cfin.get_f64_for_key(&required_fields.int_rate) {
        Ok(result) => result,
        Err(err) => {
            log_error!(log, "Error in reading int_rate. Error = {:?}", err);
            DEFAULT_FLOAT
        }
    };

    cfoutput.rate_flag = match cfin.get_string_for_key(&required_fields.rate_flag) {
        Ok(result) => result,
        Err(err) => {
            log_error!(log, "Error in reading rate_flag. Error = {:?}", err);
            ""
        }
    }
    .to_string();

    cfoutput.val_dt = match cfin.get_i64_for_key(&required_fields.value_date) {
        Ok(result) => result,
        Err(err) => {
            log_error!(log, "Error in reading value_date. Error = {:?}", err);
            DEFAULT_INT
        }
    };

    cfoutput.open_dt = match cfin.get_i64_for_key(&required_fields.open_date) {
        Ok(result) => result,
        Err(err) => {
            log_error!(log, "Error in reading open_date. Error = {:?}", err);
            DEFAULT_INT
        }
    };

    cfoutput.mat_dt = match cfin.get_i64_for_key(&required_fields.mat_date) {
        Ok(result) => result,
        Err(err) => {
            log_error!(log, "Error in reading mat_date. Error = {:?}", err);
            DEFAULT_INT
        }
    };

    cfoutput.lst_repricing_dt = match cfin.get_i64_for_key(&required_fields.lst_reprice_date) {
        Ok(result) => result,
        Err(err) => {
            log_error!(log, "Error in reading lst_reprice_date. Error = {:?}", err);
            DEFAULT_INT
        }
    };

    cfoutput.rep_freq = match cfin.get_string_for_key(&required_fields.rep_freq) {
        Ok(result) => result,
        Err(err) => {
            log_error!(log, "Error in reading rep_freq. Error = {:?}", err);
            ""
        }
    }
    .to_string();

    cfoutput.a_or_l = match cfin.get_string_for_key(&required_fields.a_or_l) {
        Ok(result) => result,
        Err(err) => {
            log_error!(log, "Error in reading a_or_l. Error = {:?}", err);
            ""
        }
    }
    .to_string();

    cfoutput.dim1 = match cfin.get_string_for_key(&required_fields.dim1) {
        Ok(result) => result,
        Err(err) => {
            log_error!(log, "Error in reading dim1. Error = {:?}", err);
            ""
        }
    }
    .to_string();

    cfoutput.dim2 = match cfin.get_string_for_key(&required_fields.dim2) {
        Ok(result) => result,
        Err(err) => {
            log_error!(log, "Error in reading dim2. Error = {:?}", err);
            ""
        }
    }
    .to_string();

    cfoutput.dim3 = match cfin.get_string_for_key(&required_fields.dim3) {
        Ok(result) => result,
        Err(err) => {
            log_error!(log, "Error in reading dim3. Error = {:?}", err);
            ""
        }
    }
    .to_string();

    cfoutput.dim4 = match cfin.get_string_for_key(&required_fields.dim4) {
        Ok(result) => result,
        Err(err) => {
            log_error!(log, "Error in reading dim4. Error = {:?}", err);
            ""
        }
    }
    .to_string();

    cfoutput.customer_id = match cfin.get_string_for_key(&required_fields.customer_id) {
        Ok(result) => result,
        Err(err) => {
            log_error!(log, "Error in reading customer_id. Error = {:?}", err);
            ""
        }
    }
    .to_string();

    cfoutput.rl1 = match cfin.get_i32_for_key(&required_fields.rl1) {
        Ok(result) => result,
        Err(err) => {
            log_error!(log, "Error in reading rl1. Error = {:?}", err);
            DEFAULT_INT as i32
        }
    };

    cfoutput.rl2 = match cfin.get_i32_for_key(&required_fields.rl2) {
        Ok(result) => result,
        Err(err) => {
            log_error!(log, "Error in reading rl2. Error = {:?}", err);
            DEFAULT_INT as i32
        }
    };

    cfoutput.rl3 = match cfin.get_i32_for_key(&required_fields.rl3) {
        Ok(result) => result,
        Err(err) => {
            log_error!(log, "Error in reading rl3. Error = {:?}", err);
            DEFAULT_INT as i32
        }
    };

    cfoutput.gl_code = cfin
        .get_string_for_key(&required_fields.gl_code)
        .unwrap_or(&String::default())
        .to_string();

    cfoutput.prod_code = cfin
        .get_string_for_key(&required_fields.prod_code)
        .unwrap_or(&String::default())
        .to_string();

    cfoutput.div_code = cfin
        .get_string_for_key(&required_fields.div_code)
        .unwrap_or(&String::default())
        .to_string();

    cfoutput.mis_code_1 = cfin
        .get_string_for_key(&required_fields.mis_code_1)
        .unwrap_or(&String::default())
        .to_string();

    cfoutput.mis_code_2 = cfin
        .get_string_for_key(&required_fields.mis_code_2)
        .unwrap_or(&String::default())
        .to_string();

    cfoutput.mis_code_3 = cfin
        .get_string_for_key(&required_fields.mis_code_3)
        .unwrap_or(&String::default())
        .to_string();

    cfoutput.eop_balance_ccy = match cfin.get_f64_for_key(&required_fields.eop_bal_ccy) {
        Ok(result) => result,
        Err(err) => {
            log_error!(log, "Error in reading eop_bal_ccy. Error = {:?}", err);
            DEFAULT_FLOAT
        }
    };

    // TODO: We need to be able to read and write into Cashflow without having to iterate through cashflow values
    let cashflow_data: Vec<Cashflow> =
        match &mut cfin.remove_cfs_for_key(&required_fields.cashflows) {
            Ok(result) => {
                let mut cashflows: Vec<Cashflow> = Vec::new();
                for index in 0..result.len() {
                    let mut cf: Cashflow = Cashflow::new();
                    cf.interest_amount = result[index].interest_amount;
                    cf.principal_amount = result[index].principal_amount;
                    cf.date = result[index].date;
                    cashflows.push(cf);
                }
                cashflows
            }
            Err(err) => {
                log_error!(log, "Error in appending cashflows. Error = {:?}", err);
                Vec::new()
            }
        };
    cfoutput.cashflows = protobuf::RepeatedField::from_vec(cashflow_data);

    cfoutput
}
