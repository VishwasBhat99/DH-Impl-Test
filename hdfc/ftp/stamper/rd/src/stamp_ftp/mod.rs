use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use stamp_ftp::CFout::AccountWithCashflows;

pub mod CFout;
pub mod aggr_key;
pub mod amb_file_reader;
pub mod calc_ftp;
pub mod cfinput;
pub mod ftp_rates_reader;
pub mod io;
pub mod rule_stamper;

pub fn read_cashflow(
    cfin: &AccountWithCFs,
    input_field_names: &cfinput::AccFieldNames,
) -> (AccountWithCashflows) {
    let mut cfoutput = AccountWithCashflows::new();

    cfoutput.acc_no = cfin
        .get_string_for_key(&input_field_names.acc_no)
        .unwrap_or(&String::default())
        .to_string();
    cfoutput.cod_mis_comp1 = cfin
        .get_string_for_key(&input_field_names.cod_mis_comp1)
        .unwrap_or(&String::default())
        .to_string();
    cfoutput.mat_dt = cfin.get_i64_for_key(&input_field_names.mat_dt).unwrap_or(0);
    cfoutput.st_dt = cfin.get_i64_for_key(&input_field_names.st_dt).unwrap_or(0);
    cfoutput.amt = cfin.get_f64_for_key(&input_field_names.amt).unwrap_or(0.0);

    cfoutput.int_rt = cfin
        .get_f64_for_key(&input_field_names.int_rt)
        .unwrap_or(0.0);

    (cfoutput)
}
