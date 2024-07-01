use crate::configuration_parameters::ConfigurationParameters;
use calamine::DataType;
use sdb_io::buf_file_wrtr;
use std::collections::HashMap;
use std::io::BufWriter;
use std::{env::current_dir, fs::File};

#[derive(Debug, Clone, Default)]
pub struct Account {
    pub bookdt: String,
    pub gl_code: String,
    pub gl_desc: String,
    pub sol_lineid: String,
    pub div_lineid: String,
    pub prd_lineid: String,
    pub ad_line1: String,
    pub ad_line2: String,
    pub cycle: String,
    pub ccy: String,
    pub cost_amount_hcy: f64,
}

impl Account {
    pub fn new_from_txt(input_acc: &[&str]) -> Account {
        Account {
            bookdt: get_str_from_txt(input_acc, 0),
            gl_code: get_str_from_txt(input_acc, 1),
            gl_desc: get_str_from_txt(input_acc, 2),
            sol_lineid: get_str_from_txt(input_acc, 3),
            div_lineid: get_str_from_txt(input_acc, 4),
            prd_lineid: get_str_from_txt(input_acc, 5),
            ad_line1: get_str_from_txt(input_acc, 6),
            ad_line2: get_str_from_txt(input_acc, 7),
            cycle: get_str_from_txt(input_acc, 8),
            ccy: get_str_from_txt(input_acc, 9),
            cost_amount_hcy: get_str_from_txt(input_acc, 10).parse::<f64>().unwrap(),
        }
    }
    pub fn new_from_xlsx(input_acc: &[DataType]) -> Account {
        Account {
            bookdt: get_str_from_xlsx(input_acc, 0),
            gl_code: get_str_from_xlsx(input_acc, 1),
            gl_desc: get_str_from_xlsx(input_acc, 2),
            sol_lineid: get_str_from_xlsx(input_acc, 3),
            div_lineid: get_str_from_xlsx(input_acc, 4),
            prd_lineid: get_str_from_xlsx(input_acc, 5),
            ad_line1: get_str_from_xlsx(input_acc, 6),
            ad_line2: get_str_from_xlsx(input_acc, 7),
            cycle: get_str_from_xlsx(input_acc, 8),
            ccy: get_str_from_xlsx(input_acc, 9),
            cost_amount_hcy: get_str_from_xlsx(input_acc, 10).parse::<f64>().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SolDimData {
    pub sollineid: String,
    pub sol_name: String,
    pub sol_type: String,
    pub sol_cat1: String,
    pub sol_cat2: String,
    pub sol_cat3: String,
    pub sol_cat4: String,
    pub sol_cat5: String,
    pub hl_ho: String,
    pub hl_ro: String,
    pub hl_ad1: String,
    pub hl_ad2: String,
    pub hl_ad3: String,
}

impl SolDimData {
    pub fn def() -> SolDimData {
        SolDimData {
            sollineid: "NA".to_string(),
            sol_name: "NA".to_string(),
            sol_type: "NA".to_string(),
            sol_cat1: "NA".to_string(),
            sol_cat2: "NA".to_string(),
            sol_cat3: "NA".to_string(),
            sol_cat4: "NA".to_string(),
            sol_cat5: "NA".to_string(),
            hl_ho: "NA".to_string(),
            hl_ro: "NA".to_string(),
            hl_ad1: "NA".to_string(),
            hl_ad2: "NA".to_string(),
            hl_ad3: "NA".to_string(),
        }
    }
}

pub fn format_output(
    output_rec: &Account,
    cost_amt_hcy: f64,
    soldim_hashmap: &HashMap<String, (String, String)>,
    gl_code: String,
) -> String {
    let default_tuple = ("NA".to_string(), "NA".to_string());
    let adline1 = &soldim_hashmap.get(&gl_code).unwrap_or(&default_tuple).1;
    let adline2 = &soldim_hashmap.get(&gl_code).unwrap_or(&default_tuple).0;
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        output_rec.bookdt,
        output_rec.gl_code,
        adline2,
        output_rec.sol_lineid,
        output_rec.div_lineid,
        output_rec.prd_lineid,
        adline1,
        adline2,
        output_rec.cycle,
        output_rec.ccy,
        cost_amt_hcy,
    )
}

pub fn get_writer(file_path: &str) -> BufWriter<File> {
    match buf_file_wrtr(file_path, None) {
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

pub fn get_op_data(
    input_account: &Account,
    config_params: &ConfigurationParameters,
    cost_amt_hcy: f64,
) -> Account {
    let data = Account {
        bookdt: rbdate::datevalue_to_naive_date(&input_account.bookdt.to_string())
            .unwrap_or(*config_params.as_on_date())
            .format("%d-%m-%Y")
            .to_string(),
        gl_code: input_account.gl_code.to_string(),
        gl_desc: input_account.gl_desc.to_string(),
        sol_lineid: input_account.sol_lineid.to_string(),
        div_lineid: input_account.div_lineid.to_string(),
        prd_lineid: input_account.prd_lineid.to_string(),
        ad_line1: input_account.ad_line1.to_string(),
        ad_line2: input_account.ad_line2.to_string(),
        cycle: input_account.cycle.to_string(),
        ccy: input_account.ccy.to_string(),
        cost_amount_hcy: cost_amt_hcy,
    };
    data
}

pub fn get_str_from_txt(data: &[&str], index: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` for row: `{:?}`",
                index + 1,
                data
            )
        })
        .trim()
        .to_string()
}

pub fn get_str_from_xlsx(data: &[DataType], index: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` for row: `{:?}`",
                index + 1,
                data
            )
        })
        .to_string()
        .trim()
        .to_string()
}
