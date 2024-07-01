use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use stamp_ftp::CFout::AccountWithCashflows;

pub mod CFout;
pub mod bm_reader;
pub mod calc_ftp;
pub mod cfinput;
pub mod ftp_rates_reader;
pub mod io;
pub mod read_adjustments;
pub mod rule_stamper;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

pub fn read_cashflow(
    cfin: &AccountWithCFs,
    input_field_names: &cfinput::AccFieldNames,
) -> (AccountWithCashflows) {
    let mut cfoutput = AccountWithCashflows::new();

    cfoutput.deal_id = match cfin.get_string_for_key(&input_field_names.deal_id) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.dealer_name = match cfin.get_string_for_key(&input_field_names.dealer_name) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.currency = match cfin.get_string_for_key(&input_field_names.currency) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.roi = match cfin.get_f64_for_key(&input_field_names.roi) {
        Ok(result) => result,
        Err(e) => DEFAULT_FLOAT,
    };

    cfoutput.val_date = match cfin.get_i64_for_key(&input_field_names.val_date) {
        Ok(result) => result,
        Err(e) => DEFAULT_INT,
    };

    cfoutput.maturity_dt = match cfin.get_i64_for_key(&input_field_names.maturity_dt) {
        Ok(result) => result,
        Err(e) => DEFAULT_INT,
    };

    cfoutput.alm_line = match cfin.get_string_for_key(&input_field_names.alm_line) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    cfoutput.division = match cfin.get_string_for_key(&input_field_names.division) {
        Ok(result) => result,
        Err(e) => "",
    }
    .to_string();

    (cfoutput)
}
