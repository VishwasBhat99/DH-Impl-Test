#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BLRInput {
    pub srl_no: String,
    pub remarks: String,
}

impl BLRInput {
    pub fn print(&self,as_on_date:String) -> String {
        format!(
            "{}|{}|{}|{}\n",
            as_on_date, 
            self.srl_no, 
            self.remarks,
            self.srl_no,
        )
    }
}
