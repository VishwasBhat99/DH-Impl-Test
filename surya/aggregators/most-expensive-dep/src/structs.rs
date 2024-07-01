use sdb_io;
use serde_json;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io::Read;

#[derive(Debug)]
pub struct MatSlab {
    pub mat_id: i64,
    pub mat_name: String,
    pub from_days: i64,
    pub to_days: i64,
    pub threshold_ir: f64,
}

#[derive(Debug, Clone)]
pub struct OutputData {
    pub as_on_date: String,
    pub account_id: String,
    pub cust_id: String,
    pub ccy: String,
    pub mat_bkt_id: i64,
    pub mat_bkt_name: String,
    pub cust_name: String,
    pub branch_code: String,
    pub bal_ccy: f64,
    pub bal_lcy: f64,
    pub start_date: String,
    pub mat_date: String,
    pub int_rate: f64,
}
impl Display for OutputData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{:.2}\n",
            self.as_on_date,
            self.account_id,
            self.cust_id,
            self.ccy,
            self.mat_bkt_id,
            self.mat_bkt_name,
            self.cust_name,
            self.branch_code,
            self.bal_ccy,
            self.bal_lcy,
            self.start_date,
            self.mat_date,
            self.int_rate
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccFields {
    pub account_id: String,
    pub cust_id: String,
    pub currency: String,
    pub cust_name: String,
    pub branch_code: String,
    pub bal_lcy: String,
    pub start_date: String,
    pub mat_date: String,
    pub int_rate: String,
    pub exchange_rate: String,
}
impl AccFields {
    pub fn new_from_path(_path: &str) -> AccFields {
        let mut file =
            sdb_io::open_file_read(_path).expect("Cannot open the account metadata file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: AccFields = serde_json::from_str(&buf[..])
            .expect("Account metadata json file was not well-formatted");
        req_fields
    }
}

#[derive(Debug, Clone)]
pub struct AggrData {
    pub acc_count: f64,
    pub max_ir: f64,
    pub min_ir: f64,
    pub tot_ir: f64,
    pub total_bal_lcy: f64,
    pub mat_bkt_name: String,
    pub threshold_ir: f64,
}
impl AggrData {
    pub fn aggr_data(&mut self, acc_aggr_data: &AggrData) {
        if self.max_ir < acc_aggr_data.max_ir {
            self.max_ir = acc_aggr_data.max_ir;
        }
        if self.min_ir > acc_aggr_data.min_ir {
            self.min_ir = acc_aggr_data.min_ir
        }
        self.tot_ir += acc_aggr_data.tot_ir;
        self.total_bal_lcy += acc_aggr_data.total_bal_lcy;
    }
}

#[derive(Debug, Clone)]
pub struct SummaryOutputData {
    pub as_on_date: String,
    pub mat_bkt_id: i64,
    pub mat_bkt_name: String,
    pub max_ir: f64,
    pub min_ir: f64,
    pub avg_ir: f64,
    pub threshold_ir: f64,
    pub total_bal_lcy: f64,
}
impl Display for SummaryOutputData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{:.2}|{:.2}|{:.2}|{:.2}|{}\n",
            self.as_on_date,
            self.mat_bkt_id,
            self.mat_bkt_name,
            self.max_ir,
            self.min_ir,
            self.avg_ir,
            self.threshold_ir,
            self.total_bal_lcy
        )
    }
}
