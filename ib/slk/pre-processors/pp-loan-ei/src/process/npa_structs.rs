use calamine::DataType;

#[derive(Debug, Clone)]
pub struct NPAData {
    pub npa_code: String,
    pub npa_cbs_bal: String,
    pub npa_int_unc_prev: String,
    pub npa_claim_recd: String,
    pub npa_sd_held: String,
    pub npa_int_real: String,
    pub npa_tot_cf: String,
    pub npa_exc_prov: String,
    pub npa_prov_sec: String,
    pub npa_prov_unsec: String,
    pub npa_tot_prov: String,
    pub npa_auc: String,
    pub npa_auc_cur: String,
    pub npa_status: String,
    pub gnpa: String,
    pub npa_date: String,
    pub npa_cif: String,
    pub npa_acct_no: String,
}

#[derive(Debug, Clone, Default)]
pub struct RepData {
    pub gl_code: String,
    pub gl_desc: String,
    pub currency: String,
    pub benchmark: String,
    pub fix_float: String,
    pub repricing_date: String,
    pub repricing_bucket: String,
}
impl RepData {
    pub fn new_from_xlsx(rep_data: &[DataType]) -> RepData {
        RepData {
            gl_code: Self::get_str_from_xlsx(rep_data, 0),
            gl_desc: Self::get_str_from_xlsx(rep_data, 1),
            currency: Self::get_str_from_xlsx(rep_data, 2),
            benchmark: Self::get_str_from_xlsx(rep_data, 3),
            fix_float: Self::get_str_from_xlsx(rep_data, 4),
            repricing_date: Self::get_str_from_xlsx(rep_data, 5),
            repricing_bucket: Self::get_str_from_xlsx(rep_data, 6),
        }
    }
    pub fn get_str_from_xlsx(data: &[DataType], index: usize) -> String {
        data.get(index)
            .unwrap_or_else(|| {
                panic!(
                    "Could not get data at column-no: `{}` for row: `{:?}`",
                    index + 1,
                    data
                )
            })
            .to_string()
            .trim()
            .to_string()
    }
}
