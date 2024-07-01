use crate::configuration_parameters::ConfigurationParameters;

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
    pub class_id: i8,
    pub cust_id: i32,
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

    pub fn aggr_acc_data(
        &mut self,
        td_data: &AccData,
        rd_data: &AccData,
        config_params: &ConfigurationParameters,
    ) {
        let (td_amount, rd_amount) = if config_params.is_lcy_or_ccy() == "LCY" {
            (td_data.lcy_amount, rd_data.lcy_amount)
        } else {
            (td_data.amount, rd_data.amount)
        };
        if td_data.is_nwd_final == "FALSE" {
            self.td_wd_amt += td_amount;
            self.td_wd_bkts[td_data.bucket_id] += td_amount;
            self.td_rd_wd_bkts[td_data.bucket_id] += td_amount;
            self.td_wd_wt_int_rate += td_amount * td_data.int_rate;
        } else {
            self.td_nwd_amt += td_amount;
            self.td_nwd_bkts[td_data.bucket_id] += td_amount;
            self.td_nwd_wt_int_rate += td_amount * td_data.int_rate;
        }
        self.td_rd_wd_bkts[rd_data.bucket_id] += rd_amount;
        self.rd_amt += rd_amount;
        self.rd_wd_bkts[rd_data.bucket_id] += rd_amount;
        self.rd_wt_int_rate += rd_amount * rd_data.int_rate;
    }

    pub fn aggr_ca_data(&mut self, data: &AccData, config_params: &ConfigurationParameters) {
        self.ca_amt += if config_params.is_lcy_or_ccy() == "LCY" {
            data.lcy_amount
        } else {
            data.amount
        };
        self.ca_wt_int_rate += data.lcy_amount * data.int_rate;
    }

    pub fn aggr_sa_data(&mut self, data: &AccData, config_params: &ConfigurationParameters) {
        self.sa_amt += if config_params.is_lcy_or_ccy() == "LCY" {
            data.lcy_amount
        } else {
            data.amount
        };
        self.sa_wt_int_rate += data.lcy_amount * data.int_rate;
    }
}
