extern crate serde;
extern crate serde_derive;
use self::serde_derive::Deserialize;
use self::serde_derive::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LienData {
    pub key_1: String,
    pub acct_no: String,
    pub loan_acct_no: String,
    pub tdrm_amt: f64,
    pub tdrv_amt: f64,
    pub mat_date: String,

    //Derived Fields (Not Read from Input)
    pub cust_class: String,
    pub tenor_flag: String,
    pub curr: String,
}

impl LienData {
    pub fn new(inp_acc: Vec<&str>) -> LienData {
        LienData {
            key_1: inp_acc[0].to_string(),
            acct_no: inp_acc[1].to_string(),
            loan_acct_no: inp_acc[2].to_string(),
            tdrm_amt: inp_acc[3].to_string().parse::<f64>().unwrap_or(0.0),
            tdrv_amt: inp_acc[4].to_string().parse::<f64>().unwrap_or(0.0),
            mat_date: inp_acc[5].to_string(),
            cust_class: "N".to_string(),
            tenor_flag: "NA".to_string(),
            curr: "INR".to_string(),
        }
    }
}

pub fn format_output(output_rec: &LienData) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        output_rec.key_1,
        output_rec.acct_no,
        output_rec.loan_acct_no,
        output_rec.tdrm_amt,
        output_rec.tdrv_amt,
        output_rec.mat_date,
        output_rec.cust_class,
        output_rec.tenor_flag,
        output_rec.curr,
    )
}

#[derive(Debug, Clone, Default, Hash)]
pub struct TDVal {
    pub cust_class: String,
    pub currency: String,
}

impl TDVal {
    pub fn new(td_acc: Vec<&str>) -> TDVal {
        TDVal {
            cust_class: td_acc[41].to_string(),
            currency: td_acc[6].to_string(),
        }
    }
    pub fn def() -> TDVal {
        TDVal {
            cust_class: "NA".to_string(),
            currency: "INR".to_string(),
        }
    }
}
