use derive::account::Account;
use rbdate::{timestamp, NaiveDate};
use statics::*;
use super::AlmMaster;

pub fn create_account_without_cashflows(
    fields: &[&str],
    alm_master: &mut AlmMaster,
) -> Account {
    let mut account = Account::new();

    let as_on_dt = NaiveDate::parse_from_str(fields[0], "%d-%m-%Y")
        .expect("Error while getting `as_on_date`.");
    account.as_on_dt = timestamp(as_on_dt);
    account.src_file_name = fields[1].to_string();
    account.src_gl_cd = fields[2].parse().unwrap_or(DEFAULT_INT);
    account.gl_typ = fields[3].to_string();
    account.src_ccy = fields[4].to_string();
    account.src_gl_bal = fields[5].parse().unwrap_or(DEFAULT_FLOAT);
    account.ora_gl_cd = fields[6].parse().unwrap_or(DEFAULT_INT);
    account.ora_gl_bal = fields[7].parse().unwrap_or(DEFAULT_FLOAT);
    account.ora_ccy = fields[8].to_string();
    account.gl_diff_amt = fields[9].parse().unwrap_or(DEFAULT_FLOAT);
    account.w4b_cd = alm_master.w4b_cd.to_string();
    account.balm_llg = alm_master.balm_llg.to_string();
    account.care_llg = alm_master.care_llg.to_string();
    account.ba_llg = alm_master.ba_llg.to_string();
    account
}
