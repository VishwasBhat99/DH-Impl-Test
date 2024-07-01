use sdb_io;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub FlowID: String,
    pub GrpID: String,
    pub LLGID: String,
    pub Amount: String,
    pub CcyID: String,
    pub IntrRate: String,
    pub RepriceFreq: String,
    pub RepriceDt: String,
    pub MatuDt: String,
    pub AcctNum: String,
    pub StrtDt: String,
    pub IntrCalFreq: String,
    pub IsFlotRate: String,
    pub FlotRateBM: String,
    pub BUID: String,
    pub CustID: String,
    pub CustName: String,
    pub Sprd: String,
    pub SchmCode: String,
    pub MinIR: String,
    pub MaxIR: String,
    pub DepAmount: String,
    pub MatuAmt: String,
    pub ExchRate: String,
    pub CustCtryCode: String,
    pub CustCrdtRtng: String,
    pub CustSectCode: String,
    pub CustIndtCode: String,
    pub Custom1: String,
    pub Custom2: String,
    pub cashflows: String,
}

impl AccFieldNames {
    pub fn new_from_path(_path: &str) -> AccFieldNames {
        let mut file =
            sdb_io::open_file_read(_path).expect("Cannot open the account metadata file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: AccFieldNames = serde_json::from_str(&buf[..])
            .expect("Account metadata json file was not well-formatted");
        req_fields
    }
}
