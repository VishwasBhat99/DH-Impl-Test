use super::get_pass_throughs;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PassThroughs {
    pub acc_mt_dt: String,
    pub orginal_dep_amt_lcy: String,
    pub current_outstanding_amt: String,
    pub current_outstanding_amt_lcy: String,
    pub res_tenor: String,
    pub cont_tenor: String,
    pub rep_tenor: String,
    pub comp_freq: String,
    pub cust_cons_code: String,
    pub industry: String,
    pub division: String,
    pub cust_initial_dep_total_amount: String,
    pub cust_total_deposit_amount: String,
    pub is_withdrawable: String,
    pub is_custody_ac: String,
    pub is_clearing_ac: String,
    pub is_cash_managment: String,
    pub is_tax_saving: String,
    pub is_under_lien: String,
    pub is_wealth_mang: String,
    pub pta1: String,
    pub pta2: String,
    pub pta3: String,
    pub pta4: String,
    pub pta5: String,
}

impl Default for PassThroughs {
    fn default() -> Self {
        PassThroughs {
            acc_mt_dt: String::from("N"),
            orginal_dep_amt_lcy: String::from("N"),
            current_outstanding_amt: String::from("N"),
            current_outstanding_amt_lcy: String::from("N"),
            res_tenor: String::from("N"),
            cont_tenor: String::from("N"),
            rep_tenor: String::from("N"),
            comp_freq: String::from("N"),
            cust_cons_code: String::from("N"),
            industry: String::from("N"),
            division: String::from("N"),
            cust_initial_dep_total_amount: String::from("N"),
            cust_total_deposit_amount: String::from("N"),
            is_withdrawable: String::from("N"),
            is_custody_ac: String::from("N"),
            is_clearing_ac: String::from("N"),
            is_cash_managment: String::from("N"),
            is_tax_saving: String::from("N"),
            is_under_lien: String::from("N"),
            is_wealth_mang: String::from("N"),
            pta1: String::from("N"),
            pta2: String::from("N"),
            pta3: String::from("N"),
            pta4: String::from("N"),
            pta5: String::from("N"),
        }
    }
}

impl PassThroughs {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            get_pass_throughs(&self.acc_mt_dt),
            get_pass_throughs(&self.orginal_dep_amt_lcy),
            get_pass_throughs(&self.current_outstanding_amt),
            get_pass_throughs(&self.current_outstanding_amt_lcy),
            get_pass_throughs(&self.res_tenor),
            get_pass_throughs(&self.cont_tenor),
            get_pass_throughs(&self.rep_tenor),
            get_pass_throughs(&self.comp_freq),
            get_pass_throughs(&self.cust_cons_code),
            get_pass_throughs(&self.industry),
            get_pass_throughs(&self.division),
            get_pass_throughs(&self.cust_initial_dep_total_amount),
            get_pass_throughs(&self.cust_total_deposit_amount),
            get_pass_throughs(&self.is_withdrawable),
            get_pass_throughs(&self.is_custody_ac),
            get_pass_throughs(&self.is_clearing_ac),
            get_pass_throughs(&self.is_cash_managment),
            get_pass_throughs(&self.is_tax_saving),
            get_pass_throughs(&self.is_under_lien),
            get_pass_throughs(&self.is_wealth_mang),
            get_pass_throughs(&self.pta1),
            get_pass_throughs(&self.pta2),
            get_pass_throughs(&self.pta3),
            get_pass_throughs(&self.pta4),
            get_pass_throughs(&self.pta5),
        )
    }
    pub fn new() -> Self {
        PassThroughs {
            ..Default::default()
        }
    }

    pub fn insert(&mut self, pass_throughs: PassThroughs) {
        self.acc_mt_dt = pass_throughs.acc_mt_dt;
        self.orginal_dep_amt_lcy = pass_throughs.orginal_dep_amt_lcy;
        self.current_outstanding_amt = pass_throughs.current_outstanding_amt;
        self.current_outstanding_amt_lcy = pass_throughs.current_outstanding_amt_lcy;
        self.res_tenor = pass_throughs.res_tenor;
        self.cont_tenor = pass_throughs.cont_tenor;
        self.rep_tenor = pass_throughs.rep_tenor;
        self.comp_freq = pass_throughs.comp_freq;
        self.cust_cons_code = pass_throughs.cust_cons_code;
        self.industry = pass_throughs.industry;
        self.division = pass_throughs.division;
        self.cust_initial_dep_total_amount = pass_throughs.cust_initial_dep_total_amount;
        self.cust_total_deposit_amount = pass_throughs.cust_total_deposit_amount;
        self.is_withdrawable = pass_throughs.is_withdrawable;
        self.is_custody_ac = pass_throughs.is_custody_ac;
        self.is_clearing_ac = pass_throughs.is_clearing_ac;
        self.is_cash_managment = pass_throughs.is_cash_managment;
        self.is_tax_saving = pass_throughs.is_tax_saving;
        self.is_under_lien = pass_throughs.is_under_lien;
        self.is_wealth_mang = pass_throughs.is_wealth_mang;
        self.pta1 = pass_throughs.pta1;
        self.pta2 = pass_throughs.pta2;
        self.pta3 = pass_throughs.pta3;
        self.pta4 = pass_throughs.pta4;
        self.pta5 = pass_throughs.pta5;
    }
}
