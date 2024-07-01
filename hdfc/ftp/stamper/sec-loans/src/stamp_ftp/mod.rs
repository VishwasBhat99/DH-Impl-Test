use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use stamp_ftp::CFout::AccountWithCashflows;

pub mod CFout;
pub mod amb_file_reader;
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

pub fn read_cashflow(
    cfin: &AccountWithCFs,
    input_field_names: &cfinput::AccFieldNames,
) -> AccountWithCashflows {
    let mut cfoutput = AccountWithCashflows::new();

    cfoutput.acc_id = match cfin.get_string_for_key(&input_field_names.acc_id) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.cust_name = match cfin.get_string_for_key(&input_field_names.deal_name) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.ccy = match cfin.get_string_for_key(&input_field_names.ccy) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.int_rt = match cfin.get_f64_for_key(&input_field_names.int_rt) {
        Ok(result) => result,
        Err(_) => DEFAULT_FLOAT,
    };

    cfoutput.st_dt = match cfin.get_i64_for_key(&input_field_names.st_dt) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.c_dt = match cfin.get_i64_for_key(&input_field_names.c_dt) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.nxt_rep_dt = match cfin.get_i64_for_key(&input_field_names.nxt_rep_dt) {
        Ok(result) => result,
        Err(_) => cfoutput.c_dt,
    };

    cfoutput.gl_cd = match cfin.get_string_for_key(&input_field_names.gl_cd) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.alm_line = match cfin.get_string_for_key(&input_field_names.alm_line) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.yeild = match cfin.get_f64_for_key(&input_field_names.yeild) {
        Ok(result) => result,
        Err(_) => DEFAULT_FLOAT,
    };

    cfoutput.originator = match cfin.get_string_for_key(&input_field_names.originator) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.int_typ = match cfin.get_string_for_key(&input_field_names.int_typ) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.total_principal_amount =
        match cfin.get_f64_for_key(&input_field_names.total_principal_amount) {
            Ok(result) => result,
            Err(_) => DEFAULT_FLOAT,
        };

    cfoutput.rt_flag = match cfin.get_string_for_key(&input_field_names.rt_flag) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.mis2 = match cfin.get_i64_for_key(&input_field_names.mis2) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };
    cfoutput.pout_bal = match cfin.get_f64_for_key(&input_field_names.pout_bal) {
        Ok(result) => result,
        Err(_) => DEFAULT_FLOAT,
    };
    cfoutput.ftp_runid = match cfin.get_i64_for_key(&input_field_names.mis1) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput
}
