use derive::account::Account;
use rbdate::{timestamp, NaiveDate};
use statics::*;

pub fn create_account_without_cashflows(
    fields: &[&str],
    balm_llg: String,
    ia_llg: String,
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
    account.balm_llg = balm_llg;
    account.ia_llg = ia_llg;
    account
}
