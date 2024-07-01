use calamine::DataType;
use gen_crbalaccdata::{str_to_flt, str_to_int};
pub struct Account {
    pub moc_id: String,
    pub claim_id: i64,
    pub mod_desc: String,
    pub out_stand_bal_hcy: f64,
    pub ccyid: String,
    pub glcd1: String,
    pub rw_perc: f64,
    pub prov_amt_hcy: f64,
    pub crm_amt_hcy: f64,
    pub credit_equi_hcy: f64,
    pub final_rw_amt_hcy: f64,
    pub exchnage_rt: f64,
}
impl Account {
    pub fn get_from_line(row: &[DataType]) -> Account {
        Account {
            moc_id: row[0].to_string(),
            claim_id: str_to_int(row[1].to_string().as_str()),
            mod_desc: row[2].to_string(),
            out_stand_bal_hcy: str_to_flt(row[3].to_string().as_str()),
            ccyid: row[4].to_string(),
            glcd1: row[5].to_string(),
            rw_perc: str_to_flt(row[6].to_string().as_str()),
            prov_amt_hcy: str_to_flt(row[7].to_string().as_str()),
            crm_amt_hcy: str_to_flt(row[8].to_string().as_str()),
            credit_equi_hcy: str_to_flt(row[9].to_string().as_str()),
            final_rw_amt_hcy: str_to_flt(row[10].to_string().as_str()),
            exchnage_rt: 1.0,
        }
    }
}
