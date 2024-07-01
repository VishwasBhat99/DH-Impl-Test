use aggregator::readers::*;
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct SecurityKey {
    pub isin_no: String,
    pub book_category: String,
}

#[derive(Debug, Clone)]
pub struct SecurityValue {
    pub actual_face_value: f64,
    pub actual_book_value: f64,
    pub actual_market_value: f64,
    pub pledged_face_value: f64,
    pub pledged_book_value: f64,
    pub pledged_market_value: f64,
    pub maturity_date: NaiveDate,
    pub coupon: f64,
    pub appr_depr: f64,
    pub out_face_value: f64,
    pub out_book_value: f64,
    pub out_market_value: f64,
    pub repo_mat_date: NaiveDate,
}

impl SecurityValue {
    pub fn add_value(&mut self, sec_val: &[&str]) {
        self.pledged_face_value += sec_val[3].replace(',', "").parse::<f64>().unwrap_or(0.0);
        self.pledged_book_value += sec_val[4].replace(',', "").parse::<f64>().unwrap_or(0.0);
        self.pledged_market_value += sec_val[5].replace(',', "").parse::<f64>().unwrap_or(0.0);
    }
    pub fn add_key(&mut self, fields: &[&str]) {
        self.actual_face_value += fields[3].replace(',', "").parse::<f64>().unwrap_or(0.0);
        self.actual_book_value += fields[4].replace(',', "").parse::<f64>().unwrap_or(0.0);
        self.actual_market_value += fields[5].replace(',', "").parse::<f64>().unwrap_or(0.0);
    }
}
pub fn add_key_to_hashmap(
    hashmap: &mut HashMap<SecurityKey, SecurityValue>,
    fields: Vec<&str>,
    config_params: &ConfigurationParameters,
    _logger: &Logger,
) {
    //checking mat, repo and repo_mat_date from input:
    let mat_dt_fmt = get_valid_format(config_params, fields[6]).unwrap_or("%d-%m-%Y".to_string());
    let repo_dt_fmt = get_valid_format(config_params, fields[7]).unwrap_or("%d-%m-%Y".to_string());

    let dt = if fields.len() >= 10 {
        NaiveDate::parse_from_str(fields[9], &repo_dt_fmt).expect("Cannot parse repo date.")
    } else {
        NaiveDate::from_ymd_opt(1900, 1, 1).expect("Cannot get default date.")
    };
    let sec_key = SecurityKey {
        isin_no: fields[1].to_string(),
        book_category: fields[2].to_string(),
    };
    hashmap
        .entry(sec_key)
        .and_modify(|x| x.add_key(&fields))
        .or_insert(SecurityValue {
            actual_face_value: fields[3].replace(',', "").parse::<f64>().unwrap_or(0.0),
            actual_book_value: fields[4].replace(',', "").parse::<f64>().unwrap_or(0.0),
            actual_market_value: fields[5].replace(',', "").parse::<f64>().unwrap_or(0.0),
            pledged_face_value: 0.0,
            pledged_book_value: 0.0,
            pledged_market_value: 0.0,
            maturity_date: NaiveDate::parse_from_str(fields[6], &mat_dt_fmt)
                .expect("Cannot parse maturity date."),
            coupon: fields[7].replace(',', "").parse::<f64>().unwrap_or(0.0),
            appr_depr: 0.0,
            out_face_value: 0.0,
            out_book_value: 0.0,
            out_market_value: 0.0,
            repo_mat_date: dt,
        });
}

pub fn calculate_values(map: &mut HashMap<SecurityKey, SecurityValue>) {
    for (key, mut value) in map.to_owned() {
        value.out_book_value =
            value.actual_book_value.to_owned() - value.pledged_book_value.to_owned();
        value.out_face_value = value.actual_face_value - value.pledged_face_value;
        value.out_market_value = value.actual_market_value - value.pledged_market_value;
        value.appr_depr =
            ((value.out_market_value - value.out_book_value) / value.out_face_value) * 100.0;

        map.insert(key, value);
    }
}

#[derive(Debug, Clone)]
pub struct ManualData {
    pub slrgsec_maintained: f64,
    pub slrrequired: f64,
    pub lending_to_nbfchfc: f64,
    pub fallcrceiling: f64,
}

impl ManualData {
    pub fn new(slr_gsec_main: f64, slr_req: f64, lend_nbfchc: f64, fallcrceil: f64) -> ManualData {
        ManualData {
            slrgsec_maintained: slr_gsec_main,
            slrrequired: slr_req,
            lending_to_nbfchfc: lend_nbfchc,
            fallcrceiling: fallcrceil,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HqlaData {
    pub req_slrperc: f64,
    pub req_msfperc: f64,
    pub req_fallcrperc: f64,
}

impl HqlaData {
    pub fn new(req_slr: f64, req_msf: f64, req_fallcr: f64) -> HqlaData {
        HqlaData {
            req_slrperc: req_slr,
            req_msfperc: req_msf,
            req_fallcrperc: req_fallcr,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct AppDate {
    pub app_start_date: NaiveDate,
    pub app_end_date: NaiveDate,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReqPcnt {
    pub req_slr_perc: f64,
    pub req_msf_perc: f64,
}
