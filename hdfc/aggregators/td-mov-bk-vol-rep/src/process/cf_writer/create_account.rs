use super::account::Account;

pub fn create_cf_acc(fields: &[&str]) -> Account {
    let mut account = Account::new();
    account.set_as_on(fields[0].to_string());
    account.set_source(fields[1].to_string());
    account.set_acc_no(fields[2].to_string());
    account.set_prod_code(fields[3].to_string());
    account.set_mis1(fields[4].to_string());
    account.set_gl_liab(fields[5].to_string());
    account.set_gl_int_comp(fields[6].to_string());
    account.set_concat(fields[7].to_string());
    account.set_div(fields[8].to_string());
    account.set_alm_line(fields[9].to_string());
    account.set_ia_line(fields[10].to_string());
    account.set_alco(fields[11].to_string());
    account.set_org_tenor(fields[12].to_string());
    account.set_res_tenor(fields[13].to_string());
    account.set_ia_tenor(fields[14].to_string());
    account.set_bucket_val(fields[15].to_string());
    account.set_cat_val(fields[16].to_string());
    account.set_cust_id(fields[17].to_string());
    account.set_wd_nwd_val(fields[18].to_string());
    account.set_bal_lcy(fields[19].to_string());
    account.set_int_comp(fields[20].to_string());
    account.set_rate(fields[21].to_string());
    account.set_rate_var(fields[22].to_string());
    account.set_rate_var2(fields[23].to_string());
    account.set_amt(fields[24].to_string());
    account.set_yld(fields[25].to_string());

    account
}
