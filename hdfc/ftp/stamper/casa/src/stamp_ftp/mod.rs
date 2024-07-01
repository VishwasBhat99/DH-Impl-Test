use math::round::half_away_from_zero;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use stamp_ftp::CFout::AccountWithCashflows;
use statics::*;

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

pub fn read_cashflow(
    cfin: &AccountWithCFs,
    input_field_names: &cfinput::AccFieldNames,
    rate_precision: i8,
    bal_precision: i8,
) -> (AccountWithCashflows) {
    let mut cfoutput = AccountWithCashflows::new();

    cfoutput.account_no = match cfin.get_string_for_key(&input_field_names.account_no) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.seg_1 = match cfin.get_string_for_key(&input_field_names.seg_1) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.seg_3 = match cfin.get_string_for_key(&input_field_names.seg_3) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.seg_4 = match cfin.get_string_for_key(&input_field_names.seg_4) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.seg_5 = match cfin.get_string_for_key(&input_field_names.seg_5) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.seg_6 = match cfin.get_string_for_key(&input_field_names.seg_6) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.seg_8 = match cfin.get_string_for_key(&input_field_names.seg_8) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.concat = match cfin.get_string_for_key(&input_field_names.concat) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.int_rate = half_away_from_zero(
        match cfin.get_f64_for_key(&input_field_names.int_rate) {
            Ok(result) => result,
            Err(e) => DEFAULT_FLOAT,
        },
        rate_precision,
    );

    cfoutput.alm_line = match cfin.get_string_for_key(&input_field_names.alm_line) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.cf_type = match cfin.get_string_for_key(&input_field_names.cf_type) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.bal_total = half_away_from_zero(
        match cfin.get_f64_for_key(&input_field_names.bal_total) {
            Ok(result) => result,
            Err(e) => DEFAULT_FLOAT,
        },
        bal_precision,
    );

    (cfoutput)
}
