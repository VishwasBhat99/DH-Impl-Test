use super::account_as_cashflows::Account;
use super::structs::LMRBond;
use rbdate::{datevalue_to_naive_date, num_days_start_to_end, DateParser, NaiveDate};
use std::collections::HashMap;

pub fn append_data_xl<'a>(
    account: &[calamine::DataType],
    master_map: &HashMap<String, LMRBond>,
    as_on_date: &NaiveDate,
    ccy: &str,
) -> Account {
    let mut out_acc = Account::new();
    let tenor: i64;
    let mat_dt: NaiveDate;
    let date_parser = DateParser::new("%d %b %Y".to_string(), false);
    let sys_identifier: String;
    let class_1: String;
    let class_2: String;
    let class_3: String;
    let tenure_class: String;

    if master_map.contains_key(&account[1].to_string()) {
        let master_values = master_map.get(&account[1].to_string()).unwrap();
        class_1 = master_values.class_1.to_string();
        class_2 = master_values.class_2.to_string();
        class_3 = master_values.class_3.to_string();
        tenure_class = master_values.tenure_classification.to_string();
        sys_identifier = master_values.sys_identifier.to_string();

        let mtm_amt = account[19].to_string().parse::<f64>().unwrap_or(0.0);

        if date_parser.parse_opt(&account[16].to_string()).is_none() {
            mat_dt = datevalue_to_naive_date(&account[16].to_string())
                .expect("Cound not convert to date");
        } else {
            mat_dt = date_parser.parse(&account[16].to_string());
        }
        tenor = num_days_start_to_end(as_on_date.clone(), mat_dt);
        out_acc.date = as_on_date.to_string();
        out_acc.isin = account[1].to_string();
        out_acc.sec_dep = account[8].to_string();
        out_acc.book_val = account[14].to_string().parse::<f64>().unwrap_or(0.0);
        out_acc.mat_dt = mat_dt.to_string();
        out_acc.mtm_in_usd = account[19].to_string().parse::<f64>().unwrap_or(0.0);
        out_acc.class_1 = class_1.to_string();
        out_acc.class_2 = class_2.to_string();
        out_acc.class_3 = class_3.to_string();
        out_acc.tenure_class = tenure_class.to_string();
        out_acc.sys_identifier = sys_identifier.to_string();
        out_acc.mtm_amt = mtm_amt.to_string().parse::<f64>().unwrap_or(0.0);
        out_acc.tenor = tenor.to_string().parse::<i64>().unwrap_or(0);
        out_acc.ccy = ccy.to_string();
        out_acc.rating_identifier = if account[40].to_string().contains("A") {
            String::from("Above")
        } else {
            String::from("Below")
        };
    }
    out_acc
}

pub fn append_data_txt<'a>(
    account: Vec<&str>,
    master_map: &HashMap<String, LMRBond>,
    as_on_date: &NaiveDate,
    ccy: &str,
) -> Account {
    let mut out_acc = Account::new();
    let tenor: i64;
    let mat_dt: NaiveDate;
    let date_parser = DateParser::new("%d %b %Y".to_string(), false);
    let sys_identifier: String;
    let class_1: String;
    let class_2: String;
    let class_3: String;
    let tenure_class: String;

    if master_map.contains_key(&account[1].to_string()) {
        let master_values = master_map.get(&account[1].to_string()).unwrap();
        class_1 = master_values.class_1.to_string();
        class_2 = master_values.class_2.to_string();
        class_3 = master_values.class_3.to_string();
        tenure_class = master_values.tenure_classification.to_string();
        sys_identifier = master_values.sys_identifier.to_string();

        let mtm_amt = account[19].to_string().parse::<f64>().unwrap_or(0.0);

        if date_parser.parse_opt(&account[16].to_string()).is_none() {
            mat_dt = datevalue_to_naive_date(&account[16].to_string())
                .expect("Cound not convert to date");
        } else {
            mat_dt = date_parser.parse(&account[16].to_string());
        }
        tenor = num_days_start_to_end(as_on_date.clone(), mat_dt);
        out_acc.date = as_on_date.to_string();
        out_acc.isin = account[1].to_string();
        out_acc.sec_dep = account[8].to_string();
        out_acc.book_val = account[14].to_string().parse::<f64>().unwrap_or(0.0);
        out_acc.mat_dt = mat_dt.to_string();
        out_acc.mtm_in_usd = account[19].to_string().parse::<f64>().unwrap_or(0.0);
        out_acc.class_1 = class_1.to_string();
        out_acc.class_2 = class_2.to_string();
        out_acc.class_3 = class_3.to_string();
        out_acc.tenure_class = tenure_class.to_string();
        out_acc.sys_identifier = sys_identifier.to_string();
        out_acc.mtm_amt = mtm_amt.to_string().parse::<f64>().unwrap_or(0.0);
        out_acc.tenor = tenor.to_string().parse::<i64>().unwrap_or(0);
        out_acc.ccy = ccy.to_string();
        out_acc.rating_identifier = if account[40].to_string().contains("A") {
            String::from("Above")
        } else {
            String::from("Below")
        };
    }
    out_acc
}
