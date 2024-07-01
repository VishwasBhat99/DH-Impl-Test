use super::account_as_cashflows::Account;
use super::structs::LCBGMasterFields;
use configuration_parameters::ConfigurationParameters;
use rbdate::{
    datevalue_to_naive_date, incr_dt_by_mon_presrv_eom, num_days_start_to_end, DateParser,
    NaiveDate,
};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct RangeSlab {
    from: f64,
    to: f64,
}
pub fn append_data<'a>(
    account: &[calamine::DataType],
    master_values: &LCBGMasterFields,
    as_on_date: &NaiveDate,
    config_params: &ConfigurationParameters,
) -> Account {
    let mut out_acc = Account::new();
    let td_tenor: i64;
    let mut td_expiry_dt = NaiveDate::from_ymd(1900, 1, 1);
    let date_parser = DateParser::new("%d-%b-%Y".to_string(), false);
    let mut deal_typ = "";
    let prd_slabs: Vec<RangeSlab> =
        get_prd_slabs(config_params.slab_file_path(), config_params.as_on_date());

    if master_values.pay_on_demand.to_uppercase() == "Y"
        && master_values.backed_by_td.to_uppercase() == "Y"
        && master_values.trade_non_trade.to_uppercase() == "Y"
    {
        if date_parser.parse_opt(&master_values.td_exp_dt).is_none() {
            td_expiry_dt = datevalue_to_naive_date(&master_values.td_exp_dt)
                .expect("Cound not convert to date");
        } else {
            td_expiry_dt = date_parser.parse(&master_values.td_exp_dt);
        }
        td_tenor = num_days_start_to_end(as_on_date.clone(), td_expiry_dt);
    } else {
        td_tenor = 0;
    }
    for val in prd_slabs {
        if td_tenor >= val.from as i64 && td_tenor <= val.to as i64 || td_tenor == 0 {
            deal_typ = "NONLMRDeal"
        } else {
            deal_typ = "LMRDeal"
        }
    }

    out_acc.branch = account[0].to_string();
    out_acc.product = account[1].to_string();
    out_acc.reference = account[2].to_string();
    out_acc.ussr_ref_no = account[3].to_string();
    out_acc.applicant = account[4].to_string();
    out_acc.beneficiary = account[5].to_string();
    out_acc.ccy = account[6].to_string();
    out_acc.contract_amt = account[7].to_string().parse::<f64>().unwrap();
    out_acc.amt = account[8].to_string().parse::<f64>().unwrap();
    out_acc.curr_avail = account[9].to_string().parse::<f64>().unwrap();
    out_acc.laib_oust_amt = account[10].to_string().parse::<f64>().unwrap();
    out_acc.issue_dt = account[11].to_string();
    out_acc.exp_dt = account[12].to_string();
    out_acc.closure = account[13].to_string();
    out_acc.tenor = account[14].to_string();
    out_acc.credit_line = account[15].to_string();
    out_acc.status = account[16].to_string();
    out_acc.fwd_cover = account[17].to_string();
    out_acc.customer = account[18].to_string();
    out_acc.last_res_dt = account[19].to_string();
    out_acc.lc_code = account[20].to_string();
    out_acc.lg_code = account[21].to_string();
    out_acc.comm_earned = account[22].to_string();
    out_acc.cust_no = account[23].to_string();
    out_acc.cust_name = account[24].to_string();
    out_acc.sec_unsec = account[25].to_string();
    out_acc.bank_non_bank = account[26].to_string();
    out_acc.rpt_dt = account[27].to_string();
    out_acc.pay_on_demand = master_values.pay_on_demand.to_string();
    out_acc.backed_by_td = master_values.backed_by_td.to_string();
    out_acc.trade_non_trade = master_values.trade_non_trade.to_string();
    out_acc.td_exp_dt = td_expiry_dt.to_string();
    out_acc.td_tenor = td_tenor.to_string().parse::<i64>().unwrap();
    out_acc.as_on_dt = as_on_date.to_string();
    out_acc.deal_type = deal_typ.to_string();
    out_acc
}

pub fn get_prd_slabs(path: &str, as_on_date: &NaiveDate) -> Vec<RangeSlab> {
    let mut slabs: Vec<RangeSlab> = Vec::new();
    let input_file = match File::open(path) {
        Ok(input_file) => input_file,
        Err(error) => panic!("{}", error),
    };
    let reader = BufReader::new(input_file);
    for line in reader.lines() {
        match line {
            Ok(slab_info) => {
                let info: Vec<&str> = slab_info.split('|').collect();
                let from_days = if info[0].contains("-") {
                    -1.0 * get_days(info[0].replace("-", "").as_str(), as_on_date)
                } else {
                    get_days(info[0], as_on_date)
                };
                let to_days = if info[1].contains("-") {
                    -1.0 * get_days(info[1].replace("-", "").as_str(), as_on_date)
                } else {
                    get_days(info[1], as_on_date)
                };
                let new_slab = RangeSlab {
                    from: from_days,
                    to: to_days,
                };
                slabs.push(new_slab)
            }
            Err(error) => {
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
    }
    slabs
}

fn get_days(info: &str, as_on_date: &NaiveDate) -> f64 {
    let mut alpha_code: Vec<&str> = info.split(|c: char| c.is_numeric()).collect();
    alpha_code.retain(|&x| x != "");
    let mut num_code: Vec<&str> = info.split(|c: char| c.is_alphabetic()).collect();
    num_code.retain(|&x| x != "");
    let mut days = 0.0;
    for (i, num_val) in num_code.iter().enumerate() {
        let period = num_val.to_string() + alpha_code[i];
        days += num_days(&period, as_on_date);
    }
    days
}
fn num_days(info: &str, as_on_date: &NaiveDate) -> f64 {
    if info.contains("D") {
        let period: i64 = info
            .trim_matches('D')
            .parse::<i64>()
            .expect("Invalid from day format");
        return period as f64;
    } else if info.contains("M") {
        let period: usize = info
            .trim_matches('M')
            .parse::<usize>()
            .expect("Invalid from month format");
        let new_date = incr_dt_by_mon_presrv_eom(*as_on_date, period)
            .expect("Cannot add month to as on date as per prd slab config");
        return rbdate::num_days_start_to_end(*as_on_date, new_date) as f64;
    } else if info.contains("Y") {
        let period: usize = info
            .trim_matches('Y')
            .parse::<usize>()
            .expect("Invalid from year format");
        let new_date = incr_dt_by_mon_presrv_eom(*as_on_date, period * 12)
            .expect("Cannot add month to as on date as per prd slab config");
        return rbdate::num_days_start_to_end(*as_on_date, new_date) as f64;
    } else {
        panic!("Invalid period type in prd config file.");
    }
}
