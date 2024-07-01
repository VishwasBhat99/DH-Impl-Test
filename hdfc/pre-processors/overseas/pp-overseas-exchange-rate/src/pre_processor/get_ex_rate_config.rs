use chrono::{Datelike, Local, NaiveDate};
use super::{ConfigurationParameters, CurrencyConverter};

pub fn write_config_output(ccy: &CurrencyConverter,config_str:&str,as_on_dt: NaiveDate) -> String{
    let dt = as_on_dt.format("%d-%m-%Y");

    let now = Local::now();
    let crnt_dt = NaiveDate::from_ymd(now.year(), now.month(), now.day())
        .format("%d-%m-%Y")
        .to_string();
        let output_line = format!(
            "{}|{}|{}|{}|Y|dataharmonizer|{}|dataharmonizer|{}\n{}|{}|{}|{}|Y|dataharmonizer|{}|dataharmonizer|{}\n",
            ccy.source,
            config_str,
            ccy.ex_rt,
            &dt.to_string(),
            &crnt_dt,
            &crnt_dt,
            config_str,
            ccy.source,
            1.00/ccy.ex_rt,
            &dt.to_string(),
            &crnt_dt, 
            &crnt_dt,
        );                           
        output_line
}

pub fn write_config_exrt(ccy: &CurrencyConverter,config_str:&str) -> String{
    let mut exrt_line = format!(
        "{}|{}|{}\n{}|{}|{}\n",
        ccy.source,
        config_str,
        ccy.ex_rt,
        config_str,
        ccy.source,
        1.00/ccy.ex_rt,
    );            
    exrt_line
}
