use crate::configuration_parameters::ConfigurationParameters;
use chrono::{Datelike, NaiveDate};
#[derive(Debug, Clone)]
///Structure which defines InputData
pub struct Account {
    //Input-Fields
    pub card_no: String,
    pub prd_code: String,
    pub acc_no: String,
    pub outstanding_bal: f64,
    pub npa_type: String,
    pub min_due: f64,
    pub unsettled_amt: f64,
    pub cust_id: String,
    pub pan_no: String,

    //Fields to be derived
    pub bgl: String,
    pub cgl: String,
    pub branch_code: String,
    pub int_rate: f64,
    pub due_date: String,
    pub start_date: String,
    pub maturity_date: String,
    pub group: String,
    pub llg: String,
    pub currency: String,
    pub country: String,
}

impl Account {
    pub fn new(input_acc: Vec<&str>, config_params: &ConfigurationParameters) -> Account {
        Account {
            card_no: input_acc[0].to_string(),
            prd_code: input_acc[1].to_string(),
            acc_no: input_acc[2].to_string(),
            outstanding_bal: input_acc[3].parse().unwrap_or(0.0),
            npa_type: input_acc[4].to_string(),
            min_due: input_acc[5].parse().unwrap_or(0.0),
            unsettled_amt: input_acc[6].parse().unwrap_or(0.0),
            cust_id: input_acc[7].to_string(),
            pan_no: input_acc[8].to_string(),
            //TODO: Get bgl and cgl from BGL-CGL Data
            bgl: "NA".to_string(),
            cgl: "NA".to_string(),
            branch_code: config_params.branch_code().to_string(),
            int_rate: *config_params.int_rate(),
            due_date: config_params.as_on_date().format("%d-%m-%Y").to_string(),
            start_date: config_params.as_on_date().format("%d-%m-%Y").to_string(),
            maturity_date: config_params.as_on_date().format("%d-%m-%Y").to_string(),

            //TODO: Get group and llg from Master-Data
            group: "NA".to_string(),
            llg: "NA".to_string(),
            currency: config_params.currency().to_string(),
            country: config_params.country().to_string(),
        }
    }
}

pub fn format_output(output_rec: &Account) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        output_rec.card_no,
        output_rec.prd_code,
        output_rec.acc_no,
        output_rec.outstanding_bal,
        output_rec.npa_type,
        output_rec.min_due,
        output_rec.unsettled_amt,
        output_rec.cust_id,
        output_rec.pan_no,
        output_rec.bgl,
        output_rec.cgl,
        output_rec.branch_code,
        output_rec.int_rate,
        output_rec.due_date,
        output_rec.start_date,
        output_rec.maturity_date,
        output_rec.group,
        output_rec.llg,
        output_rec.currency,
        output_rec.country,
    )
}

#[derive(Debug, Clone, Default)]
///Structure which defines MasterData
pub struct MasterData {
    pub grp: String,
    pub llg: String,
}

impl MasterData {
    pub fn new(grp: String, llg: String) -> Self {
        Self { grp, llg }
    }
    pub fn def() -> MasterData {
        ::std::default::Default::default()
    }
}

pub fn get_due_mat_date(inp_data: &mut Account, config_params: &ConfigurationParameters) {
    let as_on_date = config_params.as_on_date().to_string();
    let days = config_params
        .start_mat_date()
        .split(',')
        .collect::<Vec<&str>>();
    let dd_mm_yyyy = as_on_date.split('-').collect::<Vec<&str>>();
    match dd_mm_yyyy[1] {
        "01" => {
            get_dates(
                inp_data,
                config_params.as_on_date().year() - 1,
                12,
                days[0],
                config_params.as_on_date().year(),
                2,
                days[1],
            );
        }
        "12" => {
            get_dates(
                inp_data,
                config_params.as_on_date().year(),
                11,
                days[0],
                config_params.as_on_date().year() + 1,
                1,
                days[1],
            );
        }
        _ => {
            get_dates(
                inp_data,
                config_params.as_on_date().year(),
                dd_mm_yyyy[1].parse().unwrap_or(13) - 1,
                days[0],
                config_params.as_on_date().year(),
                dd_mm_yyyy[1].parse().unwrap_or(0) + 1,
                days[1],
            );
        }
    }
}

pub fn get_dates(
    inp_data: &mut Account,
    start_year: i32,
    start_month: u32,
    start_date: &str,
    mat_year: i32,
    mat_month: u32,
    mat_date: &str,
) {
    inp_data.start_date = NaiveDate::from_ymd(
        start_year,
        start_month,
        start_date.to_string().parse::<u32>().unwrap_or(0),
    )
    .format("%d-%m-%Y")
    .to_string();
    inp_data.maturity_date = NaiveDate::from_ymd(
        mat_year,
        mat_month,
        mat_date.to_string().parse::<u32>().unwrap_or(0),
    )
    .format("%d-%m-%Y")
    .to_string();
}
