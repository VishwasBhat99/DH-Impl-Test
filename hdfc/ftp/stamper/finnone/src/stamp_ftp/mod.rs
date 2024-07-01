use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use stamp_ftp::CFout::AccountWithCashflows;

pub mod CFout;
pub mod append_output;
pub mod bm_reader;
pub mod calc_ftp;
pub mod cfinput;
pub mod ftp_rates_reader;
pub mod io;
pub mod one_acc_view;
pub mod read_adjustments;
pub mod rule_stamper;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;
pub mod amb_file_reader;

pub fn read_cashflow(
    cfin: &AccountWithCFs,
    input_field_names: &cfinput::AccFieldNames,
) -> AccountWithCashflows {
    let mut cfoutput = AccountWithCashflows::new();

    cfoutput.account_number = match cfin.get_string_for_key(&input_field_names.account_number) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.currency = match cfin.get_string_for_key(&input_field_names.curr_code) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.dept = match cfin.get_string_for_key(&input_field_names.psl) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.int_rate = match cfin.get_f64_for_key(&input_field_names.final_int_rate) {
        Ok(result) => result,
        Err(_) => DEFAULT_FLOAT,
    };

    cfoutput.value_date = match cfin.get_i64_for_key(&input_field_names.value_date) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.maturity_date = match cfin.get_i64_for_key(&input_field_names.mat_date) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.rate_flag = match cfin.get_string_for_key(&input_field_names.rate_flag) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.gl = match cfin.get_string_for_key(&input_field_names.gr_ofs_gl) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.branch = match cfin.get_string_for_key(&input_field_names.branch) {
        Ok(result) => result,
        Err(_) => "NA",
    }
    .to_string();

    cfoutput.product_code = match cfin.get_string_for_key(&input_field_names.product_code) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.org_tenor = match cfin.get_i64_for_key(&input_field_names.org_tenor) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.rep_tenor = match cfin.get_i64_for_key(&input_field_names.rep_tenor) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.lst_rep_date = cfin
        .get_i64_for_key(&input_field_names.last_repricing_date)
        .unwrap();

    cfoutput.nxt_rep_date = match cfin.get_i64_for_key(&input_field_names.next_repricing_date) {
        Ok(result) => result,
        Err(_) => cfoutput.maturity_date,
    };

    cfoutput.orig_bal = match cfin.get_f64_for_key(&input_field_names.orig_bal) {
        Ok(result) => result,
        Err(_) => DEFAULT_FLOAT,
    };
    cfoutput.outstanding_bal = match cfin.get_f64_for_key(&input_field_names.current_book_bal) {
        Ok(result) => result,
        Err(_) => DEFAULT_FLOAT,
    };

    cfoutput.cust_name = match cfin.get_string_for_key(&input_field_names.customer_name) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();
    cfoutput
}
