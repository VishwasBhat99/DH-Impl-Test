use crate::configuration_parameters::ConfigurationParameters;
use calamine::DataType;
use rbdate::NaiveDate;
use std::io::BufWriter;
use std::{env::current_dir, fs::File};

#[derive(Debug, Clone, Default)]
pub struct Input1 {
    pub co_code: String,
    pub txn_id: String,
    pub bp_id: String,
    pub asset_liability: String,
    pub class_id: String,
    pub prd_typ: String,
    pub txn_type: String,
    pub series: String,
    pub portfolio: String,
    pub gl_code: String,
    pub status: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub issue_date: NaiveDate,
    pub principal_amt: f64,
    pub principal_ost: f64,
    pub book_val: f64,
    pub rate_type: String,
    pub create_date: NaiveDate,
    pub update_date: NaiveDate,
}

impl Input1 {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        input_acc: &[&str],
        row: usize,
    ) -> Input1 {
        Input1 {
            co_code: get_str_from_txt(input_file, input_acc, 0, row),
            txn_id: get_str_from_txt(input_file, input_acc, 1, row),
            bp_id: get_str_from_txt(input_file, input_acc, 2, row),
            asset_liability: get_str_from_txt(input_file, input_acc, 3, row),
            class_id: get_str_from_txt(input_file, input_acc, 4, row),
            prd_typ: get_str_from_txt(input_file, input_acc, 5, row),
            txn_type: get_str_from_txt(input_file, input_acc, 6, row),
            series: get_str_from_txt(input_file, input_acc, 7, row),
            portfolio: get_str_from_txt(input_file, input_acc, 8, row),
            gl_code: get_str_from_txt(input_file, input_acc, 9, row),
            status: get_str_from_txt(input_file, input_acc, 10, row),
            start_date: get_date(config_params, input_file, input_acc, 11, row),
            end_date: get_date(config_params, input_file, input_acc, 12, row),
            issue_date: get_date(config_params, input_file, input_acc, 13, row),
            principal_amt: get_str_from_txt(input_file, input_acc, 14, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            principal_ost: get_str_from_txt(input_file, input_acc, 15, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            book_val: get_str_from_txt(input_file, input_acc, 16, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            rate_type: get_str_from_txt(input_file, input_acc, 17, row),
            create_date: get_date(config_params, input_file, input_acc, 18, row),
            update_date: get_date(config_params, input_file, input_acc, 19, row),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Input2 {
    pub co_code: String,
    pub txn_id: String,
    pub update_type: String,
    pub payment_date: NaiveDate,
    pub class_id: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub amount: f64,
    pub direction: String,
    pub isin_no: String,
    pub br_id: String,
}

impl Input2 {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        input_acc: &[&str],
        row: usize,
        isin: String,
    ) -> Input2 {
        Input2 {
            co_code: get_str_from_txt(input_file, input_acc, 0, row),
            txn_id: get_str_from_txt(input_file, input_acc, 1, row),
            update_type: get_str_from_txt(input_file, input_acc, 2, row),
            payment_date: get_date(config_params, input_file, input_acc, 3, row),
            class_id: get_str_from_txt(input_file, input_acc, 4, row),
            start_date: get_date(config_params, input_file, input_acc, 5, row),
            end_date: get_date(config_params, input_file, input_acc, 6, row),
            amount: get_str_from_txt(input_file, input_acc, 7, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            direction: get_str_from_txt(input_file, input_acc, 8, row),
            isin_no: isin,
            br_id: get_str_from_txt(input_file, input_acc, 10, row),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Input3 {
    pub base_rate_id: String,
    pub reset_date: NaiveDate,
    pub coupon: String,
}

impl Input3 {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        input_acc: &[&str],
        row: usize,
    ) -> Input3 {
        Input3 {
            base_rate_id: get_str_from_txt(input_file, input_acc, 0, row),
            reset_date: get_date(config_params, input_file, input_acc, 1, row),
            coupon: get_str_from_txt(input_file, input_acc, 2, row),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Input4 {
    pub bp_id: String,
    pub bp_name: String,
    pub address_1: String,
    pub address_2: String,
    pub address_3: String,
    pub city: String,
    pub pincode: String,
    pub telephone: String,
    pub fax: String,
    pub email: String,
    pub country: String,
    pub industry: String,
    pub create_date: NaiveDate,
    pub update_date: NaiveDate,
}

impl Input4 {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        input_acc: &[&str],
        row: usize,
    ) -> Input4 {
        Input4 {
            bp_id: get_str_from_txt(input_file, input_acc, 0, row),
            bp_name: get_str_from_txt(input_file, input_acc, 1, row),
            address_1: get_str_from_txt(input_file, input_acc, 2, row),
            address_2: get_str_from_txt(input_file, input_acc, 3, row),
            address_3: get_str_from_txt(input_file, input_acc, 4, row),
            city: get_str_from_txt(input_file, input_acc, 5, row),
            pincode: get_str_from_txt(input_file, input_acc, 6, row),
            telephone: get_str_from_txt(input_file, input_acc, 7, row),
            fax: get_str_from_txt(input_file, input_acc, 8, row),
            email: get_str_from_txt(input_file, input_acc, 9, row),
            country: get_str_from_txt(input_file, input_acc, 10, row),
            industry: get_str_from_txt(input_file, input_acc, 11, row),
            create_date: get_date(config_params, input_file, input_acc, 12, row),
            update_date: get_date(config_params, input_file, input_acc, 13, row),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Input5 {
    pub co_code: String,
    pub txn_id: String,
    pub opt_type: String,
    pub reset_date: String,
    pub class_id: String,
    pub description: String,
    pub create_date: NaiveDate,
    pub update_date: NaiveDate,
}

impl Input5 {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        input_acc: &[&str],
        row: usize,
    ) -> Input5 {
        Input5 {
            co_code: get_str_from_txt(input_file, input_acc, 0, row),
            txn_id: get_str_from_txt(input_file, input_acc, 1, row),
            opt_type: get_str_from_txt(input_file, input_acc, 2, row),
            reset_date: get_str_from_txt(input_file, input_acc, 3, row),
            class_id: get_str_from_txt(input_file, input_acc, 4, row),
            description: get_str_from_txt(input_file, input_acc, 5, row),
            create_date: get_date(config_params, input_file, input_acc, 6, row),
            update_date: get_date(config_params, input_file, input_acc, 7, row),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Input6 {
    pub co_code: String,
    pub txn_id: String,
    pub cond_type: String,
    pub effect_date: NaiveDate,
    pub calc_start_date: NaiveDate,
    pub calc_end_date: NaiveDate,
    pub calc_month_end: String,
    pub calc_end_incl: String,
    pub calc_end_holiday: String,
    pub due_date: NaiveDate,
    pub due_month_end: String,
    pub due_end_incl: String,
    pub due_end_holiday: String,
    pub base_rate_id: String,
    pub spread: String,
    pub int_rate: f64,
    pub amount: f64,
    pub calc_method: String,
    pub frequency: String,
}

impl Input6 {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        input_acc: &[&str],
        row: usize,
    ) -> Input6 {
        Input6 {
            co_code: get_str_from_txt(input_file, input_acc, 0, row),
            txn_id: get_str_from_txt(input_file, input_acc, 1, row),
            cond_type: get_str_from_txt(input_file, input_acc, 2, row),
            effect_date: get_date(config_params, input_file, input_acc, 3, row),
            calc_start_date: get_date(config_params, input_file, input_acc, 4, row),
            calc_end_date: get_date(config_params, input_file, input_acc, 5, row),
            calc_month_end: get_str_from_txt(input_file, input_acc, 6, row),
            calc_end_incl: get_str_from_txt(input_file, input_acc, 7, row),
            calc_end_holiday: get_str_from_txt(input_file, input_acc, 8, row),
            due_date: get_date(config_params, input_file, input_acc, 9, row),
            due_month_end: get_str_from_txt(input_file, input_acc, 10, row),
            due_end_incl: get_str_from_txt(input_file, input_acc, 11, row),
            due_end_holiday: get_str_from_txt(input_file, input_acc, 12, row),
            base_rate_id: get_str_from_txt(input_file, input_acc, 13, row),
            spread: get_str_from_txt(input_file, input_acc, 14, row),
            int_rate: get_str_from_txt(input_file, input_acc, 15, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            amount: get_str_from_txt(input_file, input_acc, 16, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            calc_method: get_str_from_txt(input_file, input_acc, 17, row),
            frequency: get_str_from_txt(input_file, input_acc, 18, row),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input7 {
    pub prod_type: String,
    pub desc: String,
    pub borr_inv_fx: String,
}

impl Input7 {
    pub fn new(input_file: &str, input_acc: &[DataType], row: usize) -> Input7 {
        Input7 {
            prod_type: get_str_from_xlsx(input_file, input_acc, 0, row),
            desc: get_str_from_xlsx(input_file, input_acc, 1, row),
            borr_inv_fx: get_str_from_xlsx(input_file, input_acc, 2, row),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input8 {
    pub update_type: String,
    pub update_type_text: String,
}

impl Input8 {
    pub fn new(input_file: &str, input_acc: &[DataType], row: usize) -> Input8 {
        Input8 {
            update_type: get_str_from_xlsx(input_file, input_acc, 0, row),
            update_type_text: get_str_from_xlsx(input_file, input_acc, 1, row),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input9 {
    pub businessdate: NaiveDate,
    pub entity: String,
    pub serialno: String,
    pub isin_no: String,
    pub series: String,
    pub first_holder_name: String,
    pub first_holder_pan: String,
    pub category: String,
    pub product: String,
    pub amount: f64,
    pub date_of_issue: NaiveDate,
    pub maturity_date: NaiveDate,
    pub coupon_rate: f64,
    pub nextrepaymentdate: NaiveDate,
}

impl Input9 {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        input_acc: &[DataType],
        row: usize,
    ) -> Input9 {
        Input9 {
            businessdate: get_date_for_xlsx(config_params, input_file, input_acc, 0, row),
            entity: get_str_from_xlsx(input_file, input_acc, 1, row),
            serialno: get_str_from_xlsx(input_file, input_acc, 2, row),
            isin_no: get_str_from_xlsx(input_file, input_acc, 3, row),
            series: get_str_from_xlsx(input_file, input_acc, 4, row),
            first_holder_name: get_str_from_xlsx(input_file, input_acc, 5, row),
            first_holder_pan: get_str_from_xlsx(input_file, input_acc, 6, row),
            category: get_str_from_xlsx(input_file, input_acc, 7, row),
            product: get_str_from_xlsx(input_file, input_acc, 8, row),
            amount: get_str_from_xlsx(input_file, input_acc, 9, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            date_of_issue: get_date_for_xlsx(config_params, input_file, input_acc, 10, row),
            maturity_date: get_date_for_xlsx(config_params, input_file, input_acc, 11, row),
            coupon_rate: get_str_from_xlsx(input_file, input_acc, 12, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            nextrepaymentdate: get_date_for_xlsx(config_params, input_file, input_acc, 13, row),
        }
    }
    pub fn default() -> Input9 {
        Input9 {
            businessdate: NaiveDate::parse_from_str("01-01-1900", "%d-%m-%Y").unwrap(),
            entity: "NA".to_string(),
            serialno: "NA".to_string(),
            isin_no: "NA".to_string(),
            series: "NA".to_string(),
            first_holder_name: "NA".to_string(),
            first_holder_pan: "NA".to_string(),
            category: "NA".to_string(),
            product: "NA".to_string(),
            amount: 0.0,
            date_of_issue: NaiveDate::parse_from_str("01-01-1900", "%d-%m-%Y").unwrap(),
            maturity_date: NaiveDate::parse_from_str("01-01-1900", "%d-%m-%Y").unwrap(),
            coupon_rate: 0.0,
            nextrepaymentdate: NaiveDate::parse_from_str("01-01-1900", "%d-%m-%Y").unwrap(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Input10 {
    pub portfolio: String,
    pub protfolio_name: String,
}
impl Input10 {
    pub fn new(input_file: &str, input_acc: &[DataType], row: usize) -> Input10 {
        Input10 {
            portfolio: get_str_from_xlsx(input_file, input_acc, 0, row),
            protfolio_name: get_str_from_xlsx(input_file, input_acc, 1, row),
        }
    }
    pub fn default() -> Input10 {
        Input10 {
            portfolio: "NA".to_string(),
            protfolio_name: "NA".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input11 {
    pub businessdate: NaiveDate,
    pub companycode: String,
    pub transactionclassid: String,
    pub accruedinterest: f64,
    pub category_type: String,
    pub category_accruedinterest: f64,
    pub nextintdate: NaiveDate,
}
impl Input11 {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        input_acc: &[&str],
        row: usize,
    ) -> Input11 {
        Input11 {
            businessdate: get_date(config_params, input_file, input_acc, 0, row),
            companycode: get_str_from_txt(input_file, input_acc, 1, row),
            transactionclassid: get_str_from_txt(input_file, input_acc, 2, row),
            accruedinterest: get_str_from_txt(input_file, input_acc, 3, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            category_type: get_str_from_txt(input_file, input_acc, 4, row),
            category_accruedinterest: get_str_from_txt(input_file, input_acc, 5, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            nextintdate: get_date(config_params, input_file, input_acc, 6, row),
        }
    }
    pub fn default() -> Input11 {
        Input11 {
            businessdate: NaiveDate::parse_from_str("01-01-1900", "%d-%m-%Y").unwrap(),
            companycode: "NA".to_string(),
            transactionclassid: "NA".to_string(),
            accruedinterest: 0.0,
            category_type: "NA".to_string(),
            category_accruedinterest: 0.0,
            nextintdate: NaiveDate::parse_from_str("01-01-1900", "%d-%m-%Y").unwrap(),
        }
    }
}
pub fn get_str_from_txt(input_file: &str, data: &[&str], index: usize, row: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` in row-no: `{:?}` from File: {}",
                index + 1,
                row,
                input_file,
            )
        })
        .trim()
        .trim_matches(|pat| pat == ' ' || pat == '"')
        .to_string()
}

pub fn get_str_from_xlsx(input_file: &str, data: &[DataType], index: usize, row: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` in row-no: `{:?}` from File: {}",
                index + 1,
                row,
                input_file,
            )
        })
        .to_string()
        .trim()
        .trim_matches(|pat| pat == ' ' || pat == '"')
        .to_string()
}

pub fn get_date(
    config_params: &ConfigurationParameters,
    input_file: &str,
    data: &[&str],
    index: usize,
    row: usize,
) -> NaiveDate {
    let date_parser = rbdate::DateParser::new(config_params.input_date_format().to_string(), false);
    date_parser
        .parse_opt(
            &data
                .get(index)
                .unwrap_or_else(|| {
                    panic!(
                        "Could not get data at column-no: `{}` in row-no: `{:?}` from File: {}",
                        index + 1,
                        row,
                        input_file,
                    )
                })
                .replace('.', ""),
        )
        .unwrap_or(*config_params.as_on_date())
}

pub fn get_date_for_xlsx(
    config_params: &ConfigurationParameters,
    input_file: &str,
    data: &[DataType],
    index: usize,
    row: usize,
) -> NaiveDate {
    let date_parser = rbdate::DateParser::new(config_params.input_date_format().to_string(), false);
    date_parser
        .parse_opt(
            &data
                .get(index)
                .unwrap_or_else(|| {
                    panic!(
                        "Could not get data at column-no: `{}` in row-no: `{:?}` from File: {}",
                        index + 1,
                        row,
                        input_file,
                    )
                })
                .to_string()
                .replace('.', ""),
        )
        .unwrap_or(*config_params.as_on_date())
}

pub fn get_writer(file_path: &str) -> BufWriter<File> {
    match sdb_io::buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            file_path,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    }
}
