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
use math::round::half_away_from_zero;
use statics::*;

pub fn read_cashflow(
    cfin: &AccountWithCFs,
    input_field_names: &cfinput::AccFieldNames,
    rate_precision: i8,
    bal_precision: i8,
) -> (AccountWithCashflows) {
    let mut cfoutput = AccountWithCashflows::new();

    cfoutput.acc_no = match cfin.get_string_for_key(&input_field_names.acc_no) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.cust_typ = match cfin.get_string_for_key(&input_field_names.cust_typ) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.ccy = match cfin.get_string_for_key(&input_field_names.ccy) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.int_rt = half_away_from_zero(
        match cfin.get_f64_for_key(&input_field_names.int_rt) {
            Ok(result) => result,
            Err(e) => DEFAULT_FLOAT,
        },
        rate_precision,
    );

    cfoutput.st_dt = match cfin.get_i64_for_key(&input_field_names.st_dt) {
        Ok(result) => result,
        Err(e) => DEFAULT_INT,
    };

    cfoutput.mat_dt = match cfin.get_i64_for_key(&input_field_names.mat_dt) {
        Ok(result) => result,
        Err(e) => DEFAULT_INT,
    };

    cfoutput.gl_no = match cfin.get_i64_for_key(&input_field_names.gl_no) {
        Ok(result) => result,
        Err(e) => DEFAULT_INT,
    };

    cfoutput.div = match cfin.get_string_for_key(&input_field_names.div) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.counter_party = match cfin.get_string_for_key(&input_field_names.counter_party) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.alm_line = match cfin.get_string_for_key(&input_field_names.alm_line) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.tot_prin_amt = half_away_from_zero(
        match cfin.get_f64_for_key(&input_field_names.tot_prin_amt) {
            Ok(result) => result,
            Err(e) => DEFAULT_FLOAT,
        },
        bal_precision,
    );

    (cfoutput)
}
