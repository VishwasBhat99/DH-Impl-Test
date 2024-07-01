use super::input_account::InputParsedAccount;
use rbdate::NaiveDate;
use statics::*;

#[derive(Debug, Clone)]
pub struct AggregateData {
    pub as_on_month: NaiveDate,
    pub account_id: String,
    pub currency: String,
    pub balance_ccy: f64,
    pub balance_hcy: f64,
    pub int_rate: f64,
    pub int_amt_ccy: f64,
    pub int_amt_hcy: f64,
    pub ftp_method: String,
    pub base_rate_curve_id: String,
    pub rate_flag: String,
    pub adj_code_1: String,
    pub adj_code_2: String,
    pub adj_code_3: String,
    pub adj_code_4: String,
    pub adj_code_5: String,
    pub adj_code_6: String,
    pub val_dt: NaiveDate,
    pub open_dt: NaiveDate,
    pub mat_dt: NaiveDate,
    pub lst_repricing_dt: NaiveDate,
    pub rep_freq: String,
    pub cust_agg_bal: f64,
    pub day_count_basis: String,
    pub base_rate: f64,
    pub adj_rate_1: f64,
    pub adj_rate_2: f64,
    pub adj_rate_3: f64,
    pub adj_rate_4: f64,
    pub adj_rate_5: f64,
    pub adj_rate_6: f64,
    pub ftp_rate: f64,
    pub lock_spread: f64,
    pub ftp_amt_ccy: f64,
    pub ftp_amt_hcy: f64,
    pub a_or_l: String,
    pub dim1: String,
    pub dim2: String,
    pub dim3: String,
    pub dim4: String,
    pub customer_id: String,
    pub rl1: i32,
    pub rl2: String,
    pub rl3: String,
}

impl Default for AggregateData {
    fn default() -> AggregateData {
        let default_date: NaiveDate = NaiveDate::from_ymd(1970, 1, 1);

        AggregateData {
            as_on_month: default_date,
            account_id: String::new(),
            currency: String::new(),
            balance_ccy: DEFAULT_FLOAT,
            balance_hcy: DEFAULT_FLOAT,
            int_rate: DEFAULT_FLOAT,
            int_amt_ccy: DEFAULT_FLOAT,
            int_amt_hcy: DEFAULT_FLOAT,
            ftp_method: String::new(),
            base_rate_curve_id: String::new(),
            rate_flag: String::new(),
            adj_code_1: String::new(),
            adj_code_2: String::new(),
            adj_code_3: String::new(),
            adj_code_4: String::new(),
            adj_code_5: String::new(),
            adj_code_6: String::new(),
            val_dt: default_date,
            open_dt: default_date,
            mat_dt: default_date,
            lst_repricing_dt: default_date,
            rep_freq: String::new(),
            cust_agg_bal: DEFAULT_FLOAT,
            day_count_basis: String::new(),
            base_rate: DEFAULT_FLOAT,
            adj_rate_1: DEFAULT_FLOAT,
            adj_rate_2: DEFAULT_FLOAT,
            adj_rate_3: DEFAULT_FLOAT,
            adj_rate_4: DEFAULT_FLOAT,
            adj_rate_5: DEFAULT_FLOAT,
            adj_rate_6: DEFAULT_FLOAT,
            ftp_rate: DEFAULT_FLOAT,
            lock_spread: DEFAULT_FLOAT,
            ftp_amt_ccy: DEFAULT_FLOAT,
            ftp_amt_hcy: DEFAULT_FLOAT,
            a_or_l: String::new(),
            dim1: String::new(),
            dim2: String::new(),
            dim3: String::new(),
            dim4: String::new(),
            customer_id: String::new(),
            rl1: DEFAULT_INT as i32,
            rl2: String::new(),
            rl3: String::new(),
        }
    }
}

impl AggregateData {
    pub fn add(&mut self, account: &InputParsedAccount) {
        self.add_to_store(account);
    }

    fn add_to_store(&mut self, account: &InputParsedAccount) {
        self.as_on_month = account.as_on_month;
        self.account_id = account.account_id.to_string();
        self.currency = account.currency.to_string();
        self.balance_ccy += account.balance_ccy;
        self.balance_hcy += account.balance_hcy;
        self.int_rate += account.balance_ccy * account.int_rate;
        self.int_amt_ccy += account.int_amt_ccy;
        self.int_amt_hcy += account.int_amt_hcy;
        self.ftp_method = account.ftp_method.to_string();
        self.base_rate_curve_id = account.base_rate_curve_id.to_string();
        self.rate_flag = account.rate_flag.to_string();
        self.adj_code_1 = account.adj_code_1.to_string();
        self.adj_code_2 = account.adj_code_2.to_string();
        self.adj_code_3 = account.adj_code_3.to_string();
        self.adj_code_4 = account.adj_code_4.to_string();
        self.adj_code_5 = account.adj_code_5.to_string();
        self.adj_code_6 = account.adj_code_6.to_string();
        self.val_dt = account.val_dt;
        self.open_dt = account.open_dt;
        self.mat_dt = account.mat_dt;
        self.lst_repricing_dt = account.lst_repricing_dt;
        self.rep_freq = account.rep_freq.to_string();
        self.cust_agg_bal = account.cust_agg_bal;
        self.day_count_basis = account.day_count_basis.to_string();
        self.base_rate += account.balance_ccy * account.base_rate;
        self.adj_rate_1 += account.balance_ccy * account.adj_rate_1;
        self.adj_rate_2 += account.balance_ccy * account.adj_rate_2;
        self.adj_rate_3 += account.balance_ccy * account.adj_rate_3;
        self.adj_rate_4 += account.balance_ccy * account.adj_rate_4;
        self.adj_rate_5 += account.balance_ccy * account.adj_rate_5;
        self.adj_rate_6 += account.balance_ccy * account.adj_rate_6;
        self.ftp_rate += account.balance_ccy * account.ftp_rate;
        self.lock_spread = account.lock_spread;
        self.ftp_amt_ccy += account.ftp_amt_ccy;
        self.ftp_amt_hcy += account.ftp_amt_hcy;
        self.a_or_l = account.a_or_l.to_string();
        self.dim1 = account.dim1.to_string();
        self.dim2 = account.dim2.to_string();
        self.dim3 = account.dim3.to_string();
        self.dim4 = account.dim4.to_string();
        self.customer_id = account.customer_id.to_string();
        self.rl1 = account.rl1;
        self.rl2 = account.rl2.to_string();
        self.rl3 = account.rl3.to_string();
    }

    pub fn average(&mut self, no_of_days: f64) {
        let total_balance_ccy = self.balance_ccy;
        self.balance_ccy /= no_of_days;
        self.balance_hcy /= no_of_days;
        self.int_rate /= total_balance_ccy;
        self.int_amt_ccy /= no_of_days;
        self.int_amt_hcy /= no_of_days;
        self.base_rate /= total_balance_ccy;
        self.adj_rate_1 /= total_balance_ccy;
        self.adj_rate_2 /= total_balance_ccy;
        self.adj_rate_3 /= total_balance_ccy;
        self.adj_rate_4 /= total_balance_ccy;
        self.adj_rate_5 /= total_balance_ccy;
        self.adj_rate_6 /= total_balance_ccy;
        self.ftp_rate /= total_balance_ccy;
        self.ftp_amt_ccy /= no_of_days;
        self.ftp_amt_hcy /= no_of_days;
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}|{}|{}|{:.4}|{:.4}|{:.4}|{:.4}|{:.4}|{}|{}|\
             {}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\
             {}|{}|{:.4}|{}|{:.4}|{:.4}|{:.4}|{:.4}|{:.4}|{:.4}|\
             {:.4}|{:.4}|{:.4}|{:.4}|{:.4}|{}|{}|{}|{}|{}|\
             {}|{}|{}|{}|\n",
            self.as_on_month.format("%d-%m-%Y"),
            self.account_id,
            self.currency,
            self.balance_ccy,
            self.balance_hcy,
            self.int_rate,
            self.int_amt_ccy,
            self.int_amt_hcy,
            self.ftp_method,
            self.base_rate_curve_id,
            self.rate_flag,
            self.adj_code_1,
            self.adj_code_2,
            self.adj_code_3,
            self.adj_code_4,
            self.adj_code_5,
            self.adj_code_6,
            self.val_dt.format("%d-%m-%Y"),
            self.open_dt.format("%d-%m-%Y"),
            self.mat_dt.format("%d-%m-%Y"),
            self.lst_repricing_dt.format("%d-%m-%Y"),
            self.rep_freq,
            self.cust_agg_bal,
            self.day_count_basis,
            self.base_rate,
            self.adj_rate_1,
            self.adj_rate_2,
            self.adj_rate_3,
            self.adj_rate_4,
            self.adj_rate_5,
            self.adj_rate_6,
            self.ftp_rate,
            self.lock_spread,
            self.ftp_amt_ccy,
            self.ftp_amt_hcy,
            self.a_or_l,
            self.dim1,
            self.dim2,
            self.dim3,
            self.dim4,
            self.customer_id,
            self.rl1,
            self.rl2,
            self.rl3
        )
    }
}
