use derive::account::Account;

pub fn create_account_without_cashflows(
    fields: &[&str],
    dr_bal: f64,
    cr_bal: f64,
    c_typ: &str,
    is_acc_gl: &str,
    ccy: &str,
    net_bal: f64,
    alm_line: &str,
    code_desc: &String,
    group_2: &String,
    group_3: &String,
    line: &String,
    prefix: &String,
) -> Account {
    let mut account = Account::new();
    account.set_gl_cd(fields[0].to_string());
    account.set_branch_cd(fields[4].to_string());
    account.set_dr_bal(dr_bal);
    account.set_cr_bal(cr_bal);
    account.set_net_bal(net_bal);
    account.set_cf_type(c_typ.to_string());
    account.set_curr(ccy.to_string());
    account.set_is_gl(is_acc_gl.to_string());
    account.set_alm_line(alm_line.to_string());
    account.set_code_desc(code_desc.to_string());
    account.set_group_2(group_2.to_string());
    account.set_group_3(group_3.to_string());
    account.set_line(line.to_string());
    account.set_prefix(prefix.to_string());

    account
}
