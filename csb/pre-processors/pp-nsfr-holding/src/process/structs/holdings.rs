use super::get_data;
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InputData {
    pub date: String,
    pub segment: String,
    pub sub_segment: String,
    pub member_id: String,
    pub member_name: String,
    pub isin: String,
    pub security_desc: String,
    pub mat_date: String,
    pub face_value: String,
    pub face_val_treps: String,
    pub balance: String,
    pub isin_cred_lend: String,
    pub security_des: String,
    pub face_val_rec: String,
}

impl InputData {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            get_data(&self.date),
            get_data(&self.segment),
            get_data(&self.sub_segment),
            get_data(&self.member_id),
            get_data(&self.member_name),
            get_data(&self.isin),
            get_data(&self.security_desc),
            get_data(&self.mat_date),
            get_data(&self.face_value),
            get_data(&self.face_val_treps),
            get_data(&self.balance),
            get_data(&self.isin_cred_lend),
            get_data(&self.security_des),
            get_data(&self.face_val_rec),
        )
    }

    pub fn new() -> Self {
        InputData {
            ..Default::default()
        }
    }
}
