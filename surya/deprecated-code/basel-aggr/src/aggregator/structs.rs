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
    pub int_rate: f64,
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct AccKey {
    pub class_id: String,
    pub cust_id: String,
    pub currency: String,
}

#[derive(Debug, Clone)]
pub struct AggrData {
    pub t1: String,
    pub t2: String,
    pub t3: String,
    pub ca_amt: f64,
    pub ca_wt_int_rate: f64,
    pub sa_amt: f64,
    pub sa_wt_int_rate: f64,
    pub td_wd_amt: f64,
    pub td_wd_wt_int_rate: f64,
    pub td_nwd_amt: f64,
    pub td_nwd_wt_int_rate: f64,
    pub rd_amt: f64,
    pub rd_wt_int_rate: f64,
    pub td_wd_bkts: Vec<f64>,
    pub td_nwd_bkts: Vec<f64>,
    pub rd_wd_bkts: Vec<f64>,
    pub td_rd_wd_bkts: Vec<f64>,
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
            ca_amt: 0.0,
            ca_wt_int_rate: 0.0,
            sa_amt: 0.0,
            sa_wt_int_rate: 0.0,
            td_wd_amt: 0.0,
            td_wd_wt_int_rate: 0.0,
            td_nwd_amt: 0.0,
            td_nwd_wt_int_rate: 0.0,
            rd_amt: 0.0,
            rd_wt_int_rate: 0.0,
            td_wd_bkts: default_data_vec.clone(),
            td_nwd_bkts: default_data_vec.clone(),
            rd_wd_bkts: default_data_vec.clone(),
            td_rd_wd_bkts: default_data_vec.clone(),
        }
    }

    pub fn init_biu_data(&mut self, t1: &String, t2: &String, t3: &String) {
        self.t1 = t1.to_string();
        self.t2 = t2.to_string();
        self.t3 = t3.to_string();
    }

    pub fn aggr_acc_data(&mut self, td_data: &AccData, rd_data: &AccData) {
        if td_data.is_nwd_final == "FALSE" {
            self.td_wd_amt += td_data.amount;
            self.td_wd_bkts[td_data.bucket_id] += td_data.amount;
            self.td_rd_wd_bkts[td_data.bucket_id] += td_data.amount;
            self.td_wd_wt_int_rate += td_data.amount * td_data.int_rate;
        } else {
            self.td_nwd_amt += td_data.amount;
            self.td_nwd_bkts[td_data.bucket_id] += td_data.amount;
            self.td_nwd_wt_int_rate += td_data.amount * td_data.int_rate;
        }
        self.td_rd_wd_bkts[rd_data.bucket_id] += rd_data.amount;
        self.rd_amt += rd_data.amount;
        self.rd_wd_bkts[rd_data.bucket_id] += rd_data.amount;
        self.rd_wt_int_rate += rd_data.amount * rd_data.int_rate;
    }

    pub fn aggr_ca_data(&mut self, data: &AccData) {
        self.ca_amt += data.amount;
        self.ca_wt_int_rate += data.amount * data.int_rate;
    }

    pub fn aggr_sa_data(&mut self, data: &AccData) {
        self.sa_amt += data.amount;
        self.sa_wt_int_rate += data.amount * data.int_rate;
    }

    pub fn aggr_acc_lcy_data(&mut self, td_data: &AccData, rd_data: &AccData) {
        if td_data.is_nwd_final == "FALSE" {
            self.td_wd_amt += td_data.lcy_amount;
            self.td_wd_bkts[td_data.bucket_id] += td_data.lcy_amount;
            self.td_rd_wd_bkts[td_data.bucket_id] += td_data.lcy_amount;
            self.td_wd_wt_int_rate += td_data.lcy_amount * td_data.int_rate;
        } else {
            self.td_nwd_amt += td_data.lcy_amount;
            self.td_nwd_bkts[td_data.bucket_id] += td_data.lcy_amount;
            self.td_nwd_wt_int_rate += td_data.lcy_amount * td_data.int_rate;
        }
        self.td_rd_wd_bkts[rd_data.bucket_id] += rd_data.lcy_amount;
        self.rd_amt += rd_data.lcy_amount;
        self.rd_wd_bkts[rd_data.bucket_id] += rd_data.lcy_amount;
        self.rd_wt_int_rate += rd_data.lcy_amount * rd_data.int_rate;
    }

    pub fn aggr_ca_lcy_data(&mut self, data: &AccData) {
        self.ca_amt += data.lcy_amount;
        self.ca_wt_int_rate += data.lcy_amount * data.int_rate;
    }

    pub fn aggr_sa_lcy_data(&mut self, data: &AccData) {
        self.sa_amt += data.lcy_amount;
        self.sa_wt_int_rate += data.lcy_amount * data.int_rate;
    }

    pub fn add_to_prev_data(&mut self, new_data: &AggrData) {
        self.ca_amt += new_data.ca_amt;
        self.sa_amt += new_data.sa_amt;
        self.td_wd_amt += new_data.td_wd_amt;
        self.td_nwd_amt += new_data.td_nwd_amt;
        self.rd_amt += new_data.rd_amt;
        self.ca_wt_int_rate += new_data.ca_wt_int_rate;
        self.sa_wt_int_rate += new_data.sa_wt_int_rate;
        self.td_wd_wt_int_rate += new_data.td_wd_wt_int_rate;
        self.td_nwd_wt_int_rate += new_data.td_nwd_wt_int_rate;
        self.rd_wt_int_rate += new_data.rd_wt_int_rate;
        // all have same length
        for i in 0..self.td_wd_bkts.len() {
            self.td_wd_bkts[i] += new_data.td_wd_bkts[i];
            self.td_nwd_bkts[i] += new_data.td_nwd_bkts[i];
            self.rd_wd_bkts[i] += new_data.rd_wd_bkts[i];
            self.td_rd_wd_bkts[i] += new_data.td_rd_wd_bkts[i];
        }
    }
}
