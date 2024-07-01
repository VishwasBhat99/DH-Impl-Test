use math::round::half_away_from_zero;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use stamp_ftp::CFout::AccountWithCashflows;

pub mod CFout;
pub mod amb_file_reader;
pub mod bm_reader;
pub mod calc_ftp;
pub mod cfinput;
pub mod ftp_rates_reader;
pub mod io;
pub mod read_adjustments;
pub mod rule_stamper;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;
pub mod append_output;
pub mod one_acc_view;

pub fn read_cashflow(
    cfin: &AccountWithCFs,
    input_field_names: &cfinput::AccFieldNames,
    rate_precision: i8,
    bal_precision: i8,
) -> (AccountWithCashflows, String) {
    let mut cfoutput = AccountWithCashflows::new();
    let mut acc_num: String = String::new();

    cfoutput.reference = match cfin.get_string_for_key(&input_field_names.reference) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    acc_num = match cfin.get_string_for_key(&input_field_names.reference) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.cust = match cfin.get_string_for_key(&input_field_names.cust) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.curr = match cfin.get_string_for_key(&input_field_names.curr) {
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

    cfoutput.val_dt = match cfin.get_i64_for_key(&input_field_names.val_dt) {
        Ok(result) => result,
        Err(e) => DEFAULT_INT,
    };

    cfoutput.mat_dt = match cfin.get_i64_for_key(&input_field_names.mat_dt) {
        Ok(result) => result,
        Err(e) => DEFAULT_INT,
    };

    cfoutput.gl = match cfin.get_i64_for_key(&input_field_names.gl) {
        Ok(result) => result,
        Err(e) => DEFAULT_INT,
    };

    cfoutput.cust_name = match cfin.get_string_for_key(&input_field_names.cust_name) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.alm_line = match cfin.get_string_for_key(&input_field_names.alm_line) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.npa_stats = match cfin.get_string_for_key(&input_field_names.npa_stats) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.loan_type = match cfin.get_string_for_key(&input_field_names.loan_type) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.comp_mis2 = match cfin.get_i64_for_key(&input_field_names.comp_mis2) {
        Ok(result) => result,
        Err(e) => DEFAULT_INT,
    };

    cfoutput.comp_mis1 = match cfin.get_i64_for_key(&input_field_names.comp_mis1) {
        Ok(result) => result,
        Err(e) => DEFAULT_INT,
    };

    cfoutput.int_st_dt = match cfin.get_i64_for_key(&input_field_names.int_st_dt) {
        Ok(result) => result,
        Err(e) => DEFAULT_INT,
    };

    cfoutput.tot_prin_amt = half_away_from_zero(
        match cfin.get_f64_for_key(&input_field_names.tot_prin_amt) {
            Ok(result) => result,
            Err(e) => DEFAULT_FLOAT,
        },
        bal_precision,
    );

    (cfoutput, acc_num)
}
