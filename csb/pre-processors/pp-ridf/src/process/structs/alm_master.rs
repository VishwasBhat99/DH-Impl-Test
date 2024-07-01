use super::get_master_data;
use std::collections::HashMap;
#[derive(Debug)]
pub struct AlmMaster {
    pub gl_desc: String,
    pub w4b_cd: String,
    pub w4b_desc: String,
    pub balm_llg: String,
    pub care_llg: String,
    pub ba_llg: String,
}

impl Default for AlmMaster {
    fn default() -> Self {
        AlmMaster {
            gl_desc: String::from("NA"),
            w4b_cd: String::from("NA"),
            w4b_desc: String::from("NA"),
            balm_llg: String::from("NONE"),
            care_llg: String::from("NA"),
            ba_llg: String::from("NA"),
        }
    }
}

impl AlmMaster {
    pub fn new() -> Self {
        AlmMaster {
            ..Default::default()
        }
    }
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}",
            get_master_data(&self.gl_desc),
            get_master_data(&self.w4b_cd),
            get_master_data(&self.w4b_desc),
            get_master_data(&self.balm_llg),
            get_master_data(&self.care_llg),
            get_master_data(&self.ba_llg),
        )
    }
}

#[derive(Debug, Default)]
pub struct AlmMasterMap {
    pub store: HashMap<String, AlmMaster>,
}

impl AlmMasterMap {
    pub fn new() -> Self {
        AlmMasterMap {
            store: HashMap::new(),
        }
    }
}
