extern crate serde;
extern crate serde_derive;
use self::serde_derive::Deserialize;
use self::serde_derive::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InputAccount {
    pub acc_no: String,
    pub bill_id: String,
    pub branch_cd: String,
    pub cust_no: String,
    pub ucc_id: String,
    pub ccy: String,
    pub gl_cd: String,
    pub prod_cd: String,
    pub acc_open_dt: String,
    pub curr_out_bal: String,
    pub curr_out_bal_lcy: String,
    pub original_bill_amt: String,
    pub mat_amt: String,
    pub int_rt: String,
    pub mat_dt: String,
    pub nego_strt_dt: String,
    pub int_accured: String,
    pub int_realised: String,
    pub ext_rt_agency_id: String,
    pub cust_rt_ext: String,
    pub cust_rt_int: String,
    pub npa_class: String,
    pub prov_amt: String,
    pub prov_dt: String,
    pub cust_const_cd: String,
    pub a1: String,
    pub a2: String,
    pub a3: String,
    pub a4: String,
    pub a5: String,
    pub a6: String,
    pub a7: String,
    pub a8: String,
    pub a9: String,
    pub a10: String,
}

impl InputAccount {
    pub fn new() -> InputAccount {
        InputAccount {
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MasterSheetAccount {
    pub gl_acc_no: String,
    pub description: String,
    pub classification: String,
    pub group: String,
    pub llg: String,
    pub other_llg_classification: String,
    pub logic: String,
}

impl MasterSheetAccount {
    pub fn new() -> MasterSheetAccount {
        MasterSheetAccount {
            ..Default::default()
        }
    }
}
