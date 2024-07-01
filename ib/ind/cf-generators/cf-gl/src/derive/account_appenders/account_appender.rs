use derive::account::Account;
use statics::DEFAULT_FLOAT;

pub fn create_account_without_cashflows(
    fields: &[&str],
    dr_bal: f64,
    cr_bal: f64,
    temp_concat: &str,
    alm_line: &str,
    c_typ: &str,
    is_acc_gl: &str,
    ccy: &str,
    net_bal: f64,
    ia_line: &str,
) -> Account {
    let mut account = Account::new();
    account.set_seg_1(fields[0].to_string());
    account.set_account_no(fields[1].to_string());
    account.set_seg_5(fields[2].to_string());
    account.set_seg_6(fields[3].to_string());
    account.set_seg_3(fields[4].to_string());
    account.set_seg_8(ccy.to_string());
    account.set_seg_4(fields[6].to_string());
    account.set_dr_bal(dr_bal);
    account.set_cr_bal(cr_bal);
    account.set_bal_total(net_bal);
    account.set_concat(temp_concat.to_string());
    account.set_alm_line(alm_line.to_string());
    account.set_int_rate(DEFAULT_FLOAT);
    account.set_cf_type(c_typ.to_string());
    account.set_gl_prefix(fields[0].chars().next().unwrap_or('0').to_string());
    account.set_is_acct_gl(is_acc_gl.to_string());
    account.set_ia_line(ia_line.to_string());

    account
}
