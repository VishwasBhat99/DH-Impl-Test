extern crate serde;
extern crate serde_derive;
use self::serde_derive::Deserialize;
use self::serde_derive::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// The structure in which the Aggr-Input File is read and written back
pub struct AggrData {
    pub account_id: String,
    pub out_bal: f64,
    pub wt_int_rate: f64,
    pub tot_int_inc: f64,
    pub prod_code: String,
    pub cap_freq: String,
}

impl AggrData {
    pub fn def() -> AggrData {
        ::std::default::Default::default()
    }
}

pub fn format_aggr_output(aggr_data: &AggrData) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}\n",
        aggr_data.account_id,
        aggr_data.out_bal,
        aggr_data.wt_int_rate,
        aggr_data.tot_int_inc,
        aggr_data.prod_code,
        aggr_data.cap_freq,
    )
}

//Common-Cap File written in the same format in which it was read
pub fn format_commoncap_output(commoncap_data: &[&str]) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        commoncap_data[0],
        commoncap_data[1],
        commoncap_data[2],
        commoncap_data[3],
        commoncap_data[4],
        commoncap_data[5],
        commoncap_data[6],
        commoncap_data[7],
        commoncap_data[8],
        commoncap_data[9],
        commoncap_data[10],
        commoncap_data[11],
        commoncap_data[12],
        commoncap_data[13],
        commoncap_data[14],
        commoncap_data[15],
    )
}
