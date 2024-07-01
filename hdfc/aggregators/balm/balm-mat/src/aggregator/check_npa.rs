use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;

pub fn check_npa(is_npa: &str, npa_values: &Vec<String>, account: &AccountWithCFs) -> bool {
    if is_npa != "NA" {
        let acc_npa_val = account
            .get_string_for_key(&is_npa.to_string())
            .expect("Error while reading `acc_npa_val`.")
            .to_uppercase();

        return npa_values.contains(&acc_npa_val);
    }
    false
}
