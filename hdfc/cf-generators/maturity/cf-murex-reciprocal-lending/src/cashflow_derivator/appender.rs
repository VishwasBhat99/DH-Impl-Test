use cashflow_derivator::account_reader::input_account::InputAccount;
use rbdate::date_from_timestamp;

pub fn get_op_line(account: InputAccount, cf_type: String, cf_dt: i64, cf_amt: f64) -> String {
    let mut op_line = String::new();
    op_line.push_str(&account.cf_sub_type);
    op_line.push('|');
    op_line.push_str(&account.c_party);
    op_line.push('|');
    op_line.push_str(&account.ccy);
    op_line.push('|');
    op_line.push_str(&account.typ);
    op_line.push('|');
    op_line.push_str(&account.sanc_amt.to_string());
    op_line.push('|');
    if let Some(dt) = account.st_dt {
        op_line.push_str(&dt.format("%d-%m-%Y").to_string())
    };
    op_line.push('|');
    if let Some(dt) = account.ed_dt {
        op_line.push_str(&dt.format("%d-%m-%Y").to_string())
    };
    op_line.push('|');
    op_line.push_str(&account.country);
    op_line.push('|');
    op_line.push_str(&account.util_amt.to_string());
    op_line.push('|');
    op_line.push_str(&cf_type);
    op_line.push('|');
    op_line.push_str(&date_from_timestamp(cf_dt).format("%d-%m-%Y").to_string());
    op_line.push('|');
    op_line.push_str(&cf_amt.to_string());
    op_line.push('\n');

    op_line
}
