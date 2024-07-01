use calamine::DataType;
use derive::account::Account;
use statics::DEFAULT_FLOAT;

pub fn create_account_without_cashflows_moc(
    fields: &[DataType],
    dr_bal: f64,
    cr_bal: f64,
    ccy: &str,
    c_typ: &str,
    is_acc_gl: &str,
    net_bal: f64,
    ia_line: &str,
    nsfr: &str,
) -> Account {
    let mut account = Account::new();
    account.set_seg_1(fields[0].to_string());
    account.set_account_no("source".to_string());
    account.set_seg_5("product_code".to_string());
    account.set_seg_6("category".to_string());
    account.set_seg_3("mis1".to_string());
    account.set_seg_8(ccy.to_string());
    account.set_seg_4("seg_4".to_string());
    account.set_dr_bal(dr_bal);
    account.set_cr_bal(cr_bal);
    account.set_bal_total(net_bal);
    account.set_concat("concat".to_string());
    account.set_alm_line(fields[6].to_string());
    account.set_int_rate(DEFAULT_FLOAT);
    account.set_cf_type(c_typ.to_string());
    account.set_gl_prefix(
        fields[0]
            .to_string()
            .chars()
            .next()
            .unwrap_or('0')
            .to_string(),
    );
    account.set_is_acct_gl(is_acc_gl.to_string());
    account.set_ia_line(ia_line.to_string());
    account.set_nsfr(nsfr.to_string());

    account
}
