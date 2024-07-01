#[derive(Debug, Clone, Default)]
pub struct CategoryValue {
    pub sub_cat_id: String,
    pub category_id: String,
    pub cat_desc: String,
    pub bucket_desc: String,
    pub os_bal_ccy: f64,
    pub disb_bal_ccy: f64,
    pub lim_bal_ccy: f64,
    pub lim_ccy_int: f64,
    pub disp_order: String,
    pub level_code: String,
    pub is_visible: String,
}

impl CategoryValue {
    pub fn new(input_acc: &[&str]) -> CategoryValue {
        CategoryValue {
            sub_cat_id: input_acc[2].to_string(),
            category_id: input_acc[0].to_string(),
            cat_desc: input_acc[1].to_string(),
            bucket_desc: input_acc[5].to_string(),
            os_bal_ccy: 0.0,
            disb_bal_ccy: 0.0,
            lim_bal_ccy: 0.0,
            lim_ccy_int: 0.0,
            disp_order: input_acc[6].to_string(),
            level_code: input_acc[7].to_string(),
            is_visible: input_acc[8].to_string(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AmtSum {
    pub sum_limbalccy: f64,
    pub sum_limccyint: f64,
}

impl AmtSum {
    pub fn add_sum(&mut self, new_data: Self) {
        self.sum_limbalccy += new_data.sum_limbalccy;
        self.sum_limccyint += new_data.sum_limccyint;
    }
}
