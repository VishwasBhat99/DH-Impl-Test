use sdb_io;
use serde_json;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct AggrKey {
    pub llg_id: String,
    pub currency: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccFields {
    pub currency: String,
    pub bal: String,
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
    pub total_bal_lcy: f64,
    pub total_bal_ccy: f64,
}
impl AggrData {
    pub fn aggr_data(&mut self, acc_aggr_data: &AggrData) {
        self.total_bal_lcy += acc_aggr_data.total_bal_lcy;
        self.total_bal_ccy += acc_aggr_data.total_bal_ccy;
    }
}

#[derive(Debug, Clone)]
pub struct SummaryOutputData {
    pub as_on_date: String,
    pub src_sys_code: String,
    pub ops_claim_id: String,
    pub out_bal_ccy: f64,
    pub out_bal_hcy: f64,
    pub currency: String,
    pub op_rule_id: String,
}
impl Display for SummaryOutputData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{:.2}|{:.2}\n",
            self.as_on_date, self.ops_claim_id, self.currency, self.out_bal_ccy, self.out_bal_hcy
        )
    }
}
