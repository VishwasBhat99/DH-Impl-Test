extern crate serde;
extern crate serde_derive;
use self::serde_derive::Deserialize;
use self::serde_derive::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NpaLiveAccount {
    pub finnacle_cust_id: String,
    pub seg_class: String,
    pub cust_hlth_cd: String,
    pub cust_const: String,
    pub npa_date: String,
    pub v_ucif_code: String,
    pub reporting_date: String,
}

impl NpaLiveAccount {
    pub fn new() -> NpaLiveAccount {
        NpaLiveAccount {
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NpaDataAccount {
    pub npa_class: String,
    pub seg_cd: String,
    pub amount: String,
}

impl NpaDataAccount {
    pub fn new() -> NpaDataAccount {
        NpaDataAccount {
            ..Default::default()
        }
    }
}
