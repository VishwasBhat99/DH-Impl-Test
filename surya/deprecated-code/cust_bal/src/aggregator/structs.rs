pub struct Schema {
    pub from_bkt: i64,
    pub to_bkt: i64,
    pub id: i64,
}

#[derive(Debug, Clone)]
pub struct AccData {
    pub amount: f64,
    pub lcy_amount: f64,
    pub is_nwd_final: String,
    pub bucket_id: usize,
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct AccKey {
    pub class_id: String,
    pub cust_id: String,
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct CustKey {
    pub class_id: String,
    pub cust_id: String,
}

#[derive(Debug, Clone)]
pub struct AggrData {
    pub t1: String,
    pub t2: String,
    pub t3: String,
    pub casa_lcy_amt: f64,
    pub td_wd_lcy_amt: f64,
    pub td_nwd_lcy_amt: f64,
    pub rd_lcy_amt: f64,
    pub td_wd_lcy_bkts: Vec<f64>,
    pub td_nwd_lcy_bkts: Vec<f64>,
    pub rd_wd_lcy_bkts: Vec<f64>,
    pub td_rd_wd_lcy_bkts: Vec<f64>,
}

impl AggrData {
    pub fn new(num_of_bkts: usize) -> AggrData {
        let mut default_data_vec: Vec<f64> = Vec::with_capacity(num_of_bkts);
        let mut i = 0;
        while i <= num_of_bkts {
            default_data_vec.push(0.0);
            i += 1;
        }
        AggrData {
            t1: "".to_string(),
            t2: "".to_string(),
            t3: "".to_string(),
            casa_lcy_amt: 0.0,
            td_wd_lcy_amt: 0.0,
            td_nwd_lcy_amt: 0.0,
            rd_lcy_amt: 0.0,
            td_wd_lcy_bkts: default_data_vec.clone(),
            td_nwd_lcy_bkts: default_data_vec.clone(),
            rd_wd_lcy_bkts: default_data_vec.clone(),
            td_rd_wd_lcy_bkts: default_data_vec.clone(),
        }
    }

    pub fn init_biu_data(&mut self, t1: &String, t2: &String, t3: &String) {
        self.t1 = t1.to_string();
        self.t2 = t2.to_string();
        self.t3 = t3.to_string();
    }

    pub fn aggr_acc_data(&mut self, td_data: &AccData, rd_data: &AccData) {
        if td_data.is_nwd_final == "FALSE" {
            self.td_wd_lcy_amt += td_data.lcy_amount;
            self.td_wd_lcy_bkts[td_data.bucket_id] += td_data.lcy_amount;
            self.td_rd_wd_lcy_bkts[td_data.bucket_id] += td_data.lcy_amount;
        } else {
            self.td_nwd_lcy_amt += td_data.lcy_amount;
            self.td_nwd_lcy_bkts[td_data.bucket_id] += td_data.lcy_amount;
        }
        self.td_rd_wd_lcy_bkts[rd_data.bucket_id] += rd_data.lcy_amount;
        self.rd_lcy_amt += rd_data.lcy_amount;
        self.rd_wd_lcy_bkts[rd_data.bucket_id] += rd_data.lcy_amount;
    }

    pub fn add_data(&mut self, data: &AccData) {
        self.casa_lcy_amt += data.lcy_amount;
    }
}
