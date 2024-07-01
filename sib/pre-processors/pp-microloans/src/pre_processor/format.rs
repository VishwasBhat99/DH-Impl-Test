use super::CashFlow;
use rbdate::NaiveDate;

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
    account: &[&str],
    cf: &CashFlow,
    component: &str,
    as_on_date: NaiveDate,
    cf_amt: f64,
    npa_fields: &NPAFields,
) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}||{}|{}|{}|{}|{}|{}|{}||{}|{}||{}|{}|{}||{}|{}|25285|\n",
        account[0],
        account[0],
        account[1],
        account[2],
        account[11],
        account[9],
        NaiveDate::parse_from_str(account[5],"%d-%m-%Y").unwrap_or(NaiveDate::from_ymd(1900,1,1)).format("%d-%m-%Y"),
        NaiveDate::parse_from_str(account[5],"%d-%m-%Y").unwrap_or(NaiveDate::from_ymd(1900,1,1)).format("%d-%m-%Y"),
        NaiveDate::parse_from_str(account[8],"%d-%m-%Y").unwrap_or(NaiveDate::from_ymd(1900,1,1)).format("%d-%m-%Y"),
        cf.cf_date.format("%d-%m-%Y"),
        npa_fields.npa_classification,
        account[4],
        account[17],
        account[3],
        account[6],
        component,
        account[6],
        account[12],
        cf_amt,
        account[35],
        account[13],
        account[14],
        npa_fields.provision_amt,
        account[11],
        account[9],
        account[16],
        NaiveDate::parse_from_str(account[7],"%d-%m-%Y").unwrap_or(NaiveDate::from_ymd(1900,1,1)).format("%d-%m-%Y"),
        NaiveDate::parse_from_str(account[5],"%d-%m-%Y").unwrap_or(NaiveDate::from_ymd(1900,1,1)).format("%d-%m-%Y"),
        account[15],
        account[29],
        account[25],
        account[30],
        account[31],
        account[28],
        NaiveDate::parse_from_str(account[27],"%d-%m-%Y").unwrap_or(NaiveDate::from_ymd(1900,1,1)).format("%d-%m-%Y"),
        NaiveDate::parse_from_str(account[26],"%d-%m-%Y").unwrap_or(NaiveDate::from_ymd(1900,1,1)).format("%d-%m-%Y"),
        as_on_date.format("%d-%m-%Y"),
        account[10],
        account[24],
        account[32],
        npa_fields.claim_amount,
        npa_fields.writeoff,
        npa_fields.gnpa,
        account[33],
        account[34],
    )
}
