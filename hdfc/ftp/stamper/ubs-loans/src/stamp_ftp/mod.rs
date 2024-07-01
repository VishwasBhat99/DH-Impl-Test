use sdb_dyn_proto_rdr::reader::account_with_cfs::{get_field_value, AccountWithCFs};
use sdb_dyn_proto_rdr::reader::Reader;
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
use math::round::half_away_from_zero;

use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

pub fn read_cashflow(
    input_reader: &Reader,
    cfin: &AccountWithCFs,
    input_field_names: &cfinput::AccFieldNames,
    rate_precision: i8,
    bal_precision: i8,
) -> AccountWithCashflows {
    let mut cfoutput = AccountWithCashflows::new();

    cfoutput.reference = match cfin.get_string_for_key(&input_field_names.reference) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.cust_no = match cfin.get_string_for_key(&input_field_names.cust_no) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.curr = match cfin.get_string_for_key(&input_field_names.curr) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.norm_int_rt = half_away_from_zero(
        match cfin.get_f64_for_key(&input_field_names.norm_int_rt) {
            Ok(result) => result,
            Err(_) => DEFAULT_FLOAT,
        },
        rate_precision,
    );

    cfoutput.val_dt = match cfin.get_i64_for_key(&input_field_names.val_dt) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.mat_dt = match cfin.get_i64_for_key(&input_field_names.mat_dt) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.gl = match cfin.get_string_for_key(&input_field_names.gl) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.rt_flag_new = match cfin.get_string_for_key(&input_field_names.rt_flag_new) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.prod_cd = match cfin.get_string_for_key(&input_field_names.prod_cd) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.branch_cd = match cfin.get_string_for_key(&input_field_names.branch_cd) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.cust_name = match cfin.get_string_for_key(&input_field_names.cust_name) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.alm_line = match cfin.get_string_for_key(&input_field_names.alm_line) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.lst_repricing_dt = match cfin.get_i64_for_key(&input_field_names.lst_repricing_dt) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.nxt_repricing_dt = match cfin.get_i64_for_key(&input_field_names.nxt_repricing_dt) {
        Ok(result) => result,
        Err(_) => cfoutput.mat_dt,
    };

    cfoutput.resid_tenor = match cfin.get_i64_for_key(&input_field_names.resid_tenor) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.org_tenor = match cfin.get_i64_for_key(&input_field_names.org_tenor) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.compmis2 = match cfin.get_i64_for_key(&input_field_names.compmis2) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };
    cfoutput.compmis1 = match cfin.get_i64_for_key(&input_field_names.compmis1) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.prin_ost_bal = half_away_from_zero(
        match cfin.get_f64_for_key(&input_field_names.prin_ost_bal) {
            Ok(result) => result,
            Err(_) => DEFAULT_FLOAT,
        },
        bal_precision,
    );

    cfoutput.total_principal_amount = half_away_from_zero(
        match cfin.get_f64_for_key(&input_field_names.total_principal_amount) {
            Ok(result) => result,
            Err(_) => DEFAULT_FLOAT,
        },
        bal_precision,
    );

    cfoutput.npa_typ =
        match get_field_value(&cfin, &input_reader, input_field_names.npa_typ.to_string()) {
            Ok(result) => result.to_string(),
            Err(_) => "".to_string(),
        };

    cfoutput.bmid = match get_field_value(&cfin, &input_reader, input_field_names.bmid.to_string())
    {
        Ok(result) => result.to_string(),
        Err(_) => "".to_string(),
    };

    cfoutput
}
