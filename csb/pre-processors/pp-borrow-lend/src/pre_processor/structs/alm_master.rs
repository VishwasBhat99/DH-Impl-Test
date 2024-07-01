use super::get_master_data;

#[derive(Debug)]
pub struct AlmMaster {
    pub w4b_cd: String,
    pub balm_llg: String,
    pub care_llg: String,
    pub ba_llg: String,
}

impl Default for AlmMaster {
    fn default() -> Self {
        AlmMaster {
            w4b_cd: String::from("NONE"),
            balm_llg: String::from("NONE"),
            care_llg: String::from("NONE"),
            ba_llg: String::from("NONE"),
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
            "|{}|{}|{}|{}|",
            get_master_data(&self.w4b_cd),
            get_master_data(&self.balm_llg),
            get_master_data(&self.care_llg),
            get_master_data(&self.ba_llg),
        )
    }
}

#[derive(Debug, Hash, Default, Eq, PartialEq)]
pub struct AlmMasterKey {
    pub gl_cd: String,
    pub dr_cr: String,
}

impl AlmMasterKey {
    pub fn new() -> Self {
        AlmMasterKey {
            ..Default::default()
        }
    }

    pub fn insert(&mut self, gl_cd: String, dr_cr: String) {
        self.gl_cd = gl_cd;
        self.dr_cr = dr_cr;
    }
}
