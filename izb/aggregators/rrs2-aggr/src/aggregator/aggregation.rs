use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

pub struct AccData {
    pub amount: f64,
    pub amount_lcy: f64,
    pub int_rate: f64,
    pub num_dim1: f64,
    pub num_dim1_lcy: f64,
    pub num_dim2: f64,
    pub num_dim2_lcy: f64,
    pub num_dim3: f64,
    pub num_dim3_lcy: f64,
    pub num_dim4: f64,
    pub num_dim4_lcy: f64,
    pub num_dim5: f64,
    pub num_dim5_lcy: f64,
}

impl AccData {
    pub fn add_data(&mut self, data: &AccData) {
        self.amount += data.amount;
        self.amount_lcy += data.amount_lcy;
        self.int_rate += data.int_rate * data.amount_lcy;
        self.num_dim1 += data.num_dim1;
        self.num_dim1_lcy += data.num_dim1_lcy;
        self.num_dim2 += data.num_dim2;
        self.num_dim2_lcy += data.num_dim2_lcy;
        self.num_dim3 += data.num_dim3;
        self.num_dim3_lcy += data.num_dim3_lcy;
        self.num_dim4 += data.num_dim4;
        self.num_dim4_lcy += data.num_dim4_lcy;
        self.num_dim5 += data.num_dim5;
        self.num_dim5_lcy += data.num_dim5_lcy;
    }
}

impl Display for AccData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let int_rt;
        if self.amount_lcy == 0.0 {
            int_rt = 0.0;
        } else {
            int_rt = self.int_rate / self.amount_lcy;
        }
        write!(
            f,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            self.amount,
            self.amount_lcy,
            int_rt,
            self.num_dim1,
            self.num_dim1_lcy,
            self.num_dim2,
            self.num_dim2_lcy,
            self.num_dim3,
            self.num_dim3_lcy,
            self.num_dim4,
            self.num_dim4_lcy,
            self.num_dim5,
            self.num_dim5_lcy
        )
    }
}
