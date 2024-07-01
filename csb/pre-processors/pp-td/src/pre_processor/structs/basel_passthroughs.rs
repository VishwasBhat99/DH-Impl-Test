use super::get_data;

#[derive(Debug)]
pub struct BaselPassThroughs {
    pub res_tenor: String,
    pub cont_tenor: String,
    pub rep_tenor: String,
    pub cust_cons_code: String,
    pub industry: String,
    pub division: String,
    pub cust_initial_dep_total_amount: String,
    pub cust_total_deposit_amount: String,
    pub is_with_drawable: String,
    pub is_custody_ac: String,
    pub is_clearing_ac: String,
    pub is_cash_management: String,
    pub is_tax_savings: String,
    pub is_under_lien: String,
    pub is_wealth_mang: String,
    pub pta_1: String,
    pub pta_2: String,
    pub pta_3: String,
    pub pta_4: String,
    pub pta_5: String,
}

impl Default for BaselPassThroughs {
    fn default() -> Self {
        BaselPassThroughs {
            res_tenor: String::from("N"),
            cont_tenor: String::from("N"),
            rep_tenor: String::from("N"),
            cust_cons_code: String::from("N"),
            industry: String::from("N"),
            division: String::from("N"),
            cust_initial_dep_total_amount: String::from("N"),
            cust_total_deposit_amount: String::from("N"),
            is_with_drawable: String::from("N"),
            is_custody_ac: String::from("N"),
            is_clearing_ac: String::from("N"),
            is_cash_management: String::from("N"),
            is_tax_savings: String::from("N"),
            is_under_lien: String::from("N"),
            is_wealth_mang: String::from("N"),
            pta_1: String::from("N"),
            pta_2: String::from("N"),
            pta_3: String::from("N"),
            pta_4: String::from("N"),
            pta_5: String::from("N"),
        }
    }
}

impl BaselPassThroughs {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            get_data(&self.res_tenor),
            get_data(&self.cont_tenor),
            get_data(&self.rep_tenor),
            get_data(&self.cust_cons_code),
            get_data(&self.industry),
            get_data(&self.division),
            get_data(&self.cust_initial_dep_total_amount),
            get_data(&self.cust_total_deposit_amount),
            get_data(&self.is_with_drawable),
            get_data(&self.is_custody_ac),
            get_data(&self.is_clearing_ac),
            get_data(&self.is_cash_management),
            get_data(&self.is_tax_savings),
            get_data(&self.is_under_lien),
            get_data(&self.is_wealth_mang),
            get_data(&self.pta_1),
            get_data(&self.pta_2),
            get_data(&self.pta_3),
            get_data(&self.pta_4),
            get_data(&self.pta_5),
        )
    }

    pub fn new() -> Self {
        BaselPassThroughs {
            ..Default::default()
        }
    }
}
