use aggregator::reader::account_with_cfs::AccountWithCFs;
use aggregator::AccFieldNames;
use aggregator::{num_days_start_to_end, NaiveDate};
use chrono::Datelike;

#[derive(Debug, Deserialize)]
pub struct SummaryOp {
    pub cust_id: String,
    pub pdt_code: String,
    pub cust_name: String,
    pub agg_booking: String,
    pub count_of_tds: String,
    pub division: String,
    pub wt_tenor_days: String,
    pub clubbed_tenor: String,
    pub org_wt_rt: String,
    pub club_ten_rate: String,
    pub mis1: String,
    pub ccy: String,
    pub bucket: String,
    pub lcr_cat: String,
    pub avg_bal: String,
}

#[derive(Debug, Deserialize)]
pub struct DrilldownOp {
    pub cust_id: String,
    pub pdt_code: String,
    pub account_no: String,
    pub customer_name: String,
    pub os_amt: String,
    pub division: String,
    pub tenor: String,
    pub wt_tenor_days: String,
    pub org_tenor_desc: String,
    pub clubbed_tenor: String,
    pub org_wt_rt: String,
    pub club_ten_rate: String,
    pub mis1: String,
    pub ccy: String,
    pub bucket: String,
    pub lcr_cat: String,
}

impl SummaryOp {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            self.cust_id,
            self.pdt_code,
            self.cust_name,
            self.agg_booking,
            self.count_of_tds,
            self.division,
            self.wt_tenor_days,
            self.clubbed_tenor,
            self.org_wt_rt,
            self.club_ten_rate,
            self.mis1,
            self.ccy,
            self.bucket,
            self.lcr_cat,
            self.avg_bal,
        )
    }
}

impl DrilldownOp {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            self.cust_id,
            self.pdt_code,
            self.account_no,
            self.customer_name,
            self.os_amt,
            self.division,
            self.tenor,
            self.wt_tenor_days,
            self.org_tenor_desc,
            self.clubbed_tenor,
            self.org_wt_rt,
            self.club_ten_rate,
            self.mis1,
            self.ccy,
            self.bucket,
            self.lcr_cat,
        )
    }
}
#[derive(Debug, PartialEq)]
pub struct AccDetail {
    pub acc_no: String,
    pub os_amt: f64,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Key {
    pub tenor: i64,
    value_mth: String,
    pub pdt_code: String,
}
#[derive(PartialEq, Debug)]
pub struct Value {
    pub customer_name: String,
    pub division: String,
    pub ccy: String,
    pub account_det: Vec<AccDetail>,
    pub agg_booking: f64,
    pub org_tenor: i64,
    pub weighted_int_rt: f64,
    pub weighted_tenor_rt: f64,
}

pub fn get_key(account: &AccountWithCFs, keys: &AccFieldNames) -> Key {
    let mat_dt = naivedate_from_timestamp(account.get_i64_for_key(&keys.mat_dt).unwrap_or(0))
        .format("%d-%m-%Y")
        .to_string();
    let mat_dt =
        NaiveDate::parse_from_str(&mat_dt, "%d-%m-%Y").expect("Could not parse maturity date");
    let value_dt = naivedate_from_timestamp(account.get_i64_for_key(&keys.value_dt).unwrap_or(0))
        .format("%d-%m-%Y")
        .to_string();
    let value_dt =
        NaiveDate::parse_from_str(&value_dt, "%d-%m-%Y").expect("Could not parse value date");
    let value_mth = format!("{}{}", value_dt.month(), value_dt.year());
    let tenor = num_days_start_to_end(value_dt, mat_dt);
    let pdt_code = account
        .get_string_for_key(&keys.pdt_code)
        .unwrap_or(&"NA".to_string())
        .to_string();

    Key {
        tenor,
        value_mth,
        pdt_code,
    }
}
pub fn get_value(
    account: &AccountWithCFs,
    keys: &AccFieldNames,
    acc_no: String,
    os_amt: f64,
    weighted_tenor_rt: f64,
    weighted_int_rt: f64,
    mat_dt: NaiveDate,
    division: String,
) -> Value {
    let cust_name = account
        .get_string_for_key(&keys.cust_name)
        .unwrap_or(&"NA".to_string())
        .to_string();
    let ccy = account
        .get_string_for_key(&keys.ccy)
        .unwrap_or(&"NA".to_string())
        .to_string();

    let acc_open_dt =
        naivedate_from_timestamp(account.get_i64_for_key(&keys.acc_open_dt).unwrap_or(0))
            .format("%d-%m-%Y")
            .to_string();
    let acc_open_dt =
        NaiveDate::parse_from_str(&acc_open_dt, "%d-%m-%Y").expect("Could not parse maturity date");
    let org_tenor = num_days_start_to_end(acc_open_dt, mat_dt);
    let acc_det = AccDetail { acc_no, os_amt };
    Value {
        customer_name: cust_name,
        division,
        ccy,
        account_det: vec![acc_det],
        agg_booking: os_amt,
        org_tenor,
        weighted_int_rt,
        weighted_tenor_rt,
    }
}

pub fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = rbdate::NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}
