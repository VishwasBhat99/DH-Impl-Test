#[derive(Debug, Clone, Default)]
pub struct RepDateData {
    pub bm: String,
    pub bm_name: String,
    pub v_or_f_flag: String,
    pub rep_freq: String,
    pub reset_day_of_month: u32,
    pub reset_month: u32,
    pub override_cbs_reset_date: String,
}

impl RepDateData {
    pub fn new(rep_data: &[calamine::DataType]) -> RepDateData {
        RepDateData {
            bm: rep_data[0].to_string(),
            bm_name: rep_data[1].to_string(),
            v_or_f_flag: rep_data[2].to_string(),
            rep_freq: rep_data[3].to_string(),
            reset_day_of_month: rep_data[4].to_string().parse::<u32>().unwrap_or(0),
            reset_month: rep_data[5].to_string().parse::<u32>().unwrap_or(0),
            override_cbs_reset_date: rep_data[6].to_string(),
        }
    }
    pub fn def() -> Vec<RepDateData> {
        ::std::default::Default::default()
    }
}
