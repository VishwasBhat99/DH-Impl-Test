use std::collections::HashMap;

use rbdate::NaiveDate;

#[derive(Debug, Clone, Copy)]
pub struct CashflowData {
    pub tenure: i64,
    pub instl_id: i64,
    pub cf_date: NaiveDate,
    pub payment: f64,
    pub principal_payment: f64,
    pub interest_payment: f64,
    pub int_rate: f64,
    pub card_number: i64,
}

pub fn get_cashflow_data(fields: Vec<&str>) -> CashflowData {
    CashflowData {
        tenure: fields[2].parse::<i64>().unwrap_or(0),
        instl_id: fields[3].parse::<i64>().unwrap_or(0),
        cf_date: NaiveDate::parse_from_str(fields[4], "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd(1900, 1, 1)),
        payment: fields[5].parse::<f64>().unwrap_or(0.0),
        interest_payment: fields[6].parse::<f64>().unwrap_or(0.0),
        principal_payment: fields[7].parse::<f64>().unwrap_or(0.0),
        int_rate: fields[8].parse::<f64>().unwrap_or(0.0),
        card_number: fields[9].parse::<i64>().unwrap_or(0),
    }
}

#[derive(Clone, Debug, Default)]
pub struct NPAFields {
    pub npa_classification: String,
    pub provision_amt: f64,
    pub claim_amount: f64,
    pub writeoff: String,
    pub gnpa: String,
}

impl NPAFields {
    pub fn new() -> NPAFields {
        NPAFields {
            npa_classification: "NA".to_string(),
            provision_amt: 0.0,
            claim_amount: 0.0,
            writeoff: "NA".to_string(),
            gnpa: "NA".to_string(),
        }
    }
    pub fn get_npa_fields(fields: Vec<&str>) -> NPAFields {
        NPAFields {
            npa_classification: fields[2].to_string(),
            provision_amt: fields[3].parse::<f64>().unwrap_or(0.0),
            claim_amount: fields[4].parse::<f64>().unwrap_or(0.0),
            writeoff: fields[5].to_string(),
            gnpa: fields[6].to_string(),
        }
    }
}

pub fn get_op_line(
    acc_id_concat: &String,
    account: &[&str],
    cf: &CashflowData,
    component: &str,
    as_on_date: NaiveDate,
    cf_amt: f64,
    npa_fields: &NPAFields,
) -> String {
    let spread =
        account[18].parse::<f64>().unwrap_or(0.0) - account[14].parse::<f64>().unwrap_or(0.0);

    format!(
        "{}|{}|{}||{}||{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|||{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}||{}|{}|{}|{}|{}|{}|{}|{}|\n",
        account[4],
        acc_id_concat,
        account[3],
        account[6].parse::<f64>().unwrap_or(0.0),
        NaiveDate::parse_from_str(account[23],"%d-%m-%Y").unwrap_or(NaiveDate::from_ymd(1900,1,1)).format("%d-%m-%Y"),
        NaiveDate::parse_from_str(account[28],"%d-%m-%Y").unwrap_or(NaiveDate::from_ymd(1900,1,1)).format("%d-%m-%Y"),
        NaiveDate::parse_from_str(account[25],"%d-%m-%Y").unwrap_or(NaiveDate::from_ymd(1900,1,1)).format("%d-%m-%Y"),
        cf.cf_date.format("%d-%m-%Y"),
        account[2],
        account[7],
        account[8].parse::<i64>().unwrap_or(0),
        account[5],
        account[14].parse::<f64>().unwrap_or(0.0),
        component,
        account[18].parse::<f64>().unwrap_or(0.0),
        cf.payment,
        cf_amt,
        spread,
        account[11].parse::<f64>().unwrap_or(0.0),
        account[19].parse::<f64>().unwrap_or(0.0),
        account[22].parse::<f64>().unwrap_or(0.0),
        cf.tenure,
        cf.instl_id,
        npa_fields.npa_classification,
        npa_fields.provision_amt,
        account[20],
        account[0],
        account[1],
        account[21],
        npa_fields.npa_classification,
        account[24],
        NaiveDate::parse_from_str(account[27],"%d-%m-%Y").unwrap_or(NaiveDate::from_ymd(1900,1,1)).format("%d-%m-%Y"),
        NaiveDate::parse_from_str(account[26],"%d-%m-%Y").unwrap_or(NaiveDate::from_ymd(1900,1,1)).format("%d-%m-%Y"),
        as_on_date.format("%d-%m-%Y"),
        account[12],
        account[13],
        npa_fields.claim_amount,
        npa_fields.writeoff,
        account[15],
        account[16],
        account[17],
        npa_fields.gnpa,
        cf.int_rate,
        cf.card_number,
        account[10],
    )
}
