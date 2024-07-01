use super::OldAccountSummary;
use crate::configuration_parameters::ConfigurationParameters;
use crate::statics::*;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NewAccountSummary {
    pub as_on_month: String,
    pub account_id: String,
    pub currency: String,
    pub balance_ccy: String,
    pub balance_hcy: String,
    pub int_rate: String,
    pub acr_int_amt_ccy: String,
    pub acr_int_amt_hcy: String,
    pub ftp_method: String,
    pub base_rate_curve_id: String,
    pub rate_flag: String,
    pub adj_code_1: String,
    pub adj_code_2: String,
    pub adj_code_3: String,
    pub adj_code_4: String,
    pub adj_code_5: String,
    pub adj_code_6: String,
    pub val_dt: String,
    pub open_dt: String,
    pub mat_dt: String,
    pub lst_repricing_dt: String,
    pub rep_freq: String,
    pub cust_agg_bal: String,
    pub day_count_basis: String,
    pub base_rate: String,
    pub adj_rate_1: String,
    pub adj_rate_2: String,
    pub adj_rate_3: String,
    pub adj_rate_4: String,
    pub adj_rate_5: String,
    pub adj_rate_6: String,
    pub ftp_rate: String,
    pub lock_spread: String,
    pub ftp_amt_ccy: String,
    pub ftp_amt_hcy: String,
    pub a_or_l: String,
    pub dim1: String,
    pub dim2: String,
    pub dim3: String,
    pub dim4: String,
    pub customer_id: String,
    pub rl1: String,
    pub rl2: String,
    pub rl3: String,
    pub calc_ftp_rate: String,
    pub calc_lock_spread: String,
    pub bc_as_on_rule: String,
    pub tenor_start_date_rule: String,
    pub tenor_end_date_rule: String,
    pub bc_as_on_applied: String,
    pub tenor_start_date_applied: String,
    pub tenor_end_date_applied: String,
    pub gl_code: String,
    pub prod_code: String,
    pub div_code: String,
    pub mis_code_1: String,
    pub mis_code_2: String,
    pub mis_code_3: String,
}

impl NewAccountSummary {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn mapped(
        &mut self,
        config_params: &ConfigurationParameters,
        old_acc_smry: OldAccountSummary,
    ) {
        self.as_on_month = config_params.as_on_date().format("%d-%m-%Y").to_string();
        self.account_id = old_acc_smry.account_number;
        self.currency = old_acc_smry.currency;
        self.balance_ccy = old_acc_smry.current_outstanding_td.to_string();
        self.balance_hcy = old_acc_smry.current_outstanding_td;
        self.int_rate = old_acc_smry.interest_rate;
        self.acr_int_amt_ccy = old_acc_smry.accr_int.to_string();
        self.acr_int_amt_hcy = old_acc_smry.accr_int;
        self.ftp_method = old_acc_smry.ftp_method;
        self.base_rate_curve_id = old_acc_smry.ftp_rate_curve;
        self.rate_flag = old_acc_smry.rate_flag;
        self.val_dt = old_acc_smry.value_date.to_string();
        self.open_dt = old_acc_smry.value_date;
        self.mat_dt = old_acc_smry.maturity_date;
        self.lst_repricing_dt = old_acc_smry.last_reprice_date;
        self.cust_agg_bal = DEFAULT_FLOAT.to_string();
        self.base_rate = old_acc_smry.base_rate_2;
        self.adj_rate_1 = old_acc_smry.adj1;
        self.adj_rate_2 = old_acc_smry.adj2;
        self.adj_rate_3 = old_acc_smry.adj3;
        self.adj_rate_4 = old_acc_smry.adj4;
        self.adj_rate_5 = old_acc_smry.adj5;
        self.adj_rate_6 = old_acc_smry.adj6;
        self.ftp_rate = old_acc_smry.final_ftp_rate.to_string();
        self.lock_spread = old_acc_smry.fixed_spread.to_string();
        self.ftp_amt_ccy = old_acc_smry.final_ftp_amount.to_string();
        self.ftp_amt_hcy = old_acc_smry.final_ftp_amount;
        self.a_or_l = config_params.a_or_l().to_string();
        self.customer_id = old_acc_smry.cust_id;
        self.calc_ftp_rate = old_acc_smry.final_ftp_rate;
        self.calc_lock_spread = old_acc_smry.fixed_spread;
        self.bc_as_on_rule = old_acc_smry.bc_as_on_rule;
        self.tenor_start_date_rule = old_acc_smry.tenor_start_date_rule;
        self.tenor_end_date_rule = old_acc_smry.tenor_end_date_rule;
        self.bc_as_on_applied = old_acc_smry.bc_as_on_applied;
        self.tenor_start_date_applied = old_acc_smry.tenor_start_date_applied;
        self.tenor_end_date_applied = old_acc_smry.tenor_end_date_applied;
        self.gl_code = old_acc_smry.gl;
        self.prod_code = old_acc_smry.prod_code_type;
        self.mis_code_1 = old_acc_smry.mis1;
        self.mis_code_2 = old_acc_smry.mis2;
        self.mis_code_3 = old_acc_smry.psl_code;
    }

    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\
                {}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\
                    {}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            self.as_on_month,
            self.account_id,
            self.currency,
            self.balance_ccy,
            self.balance_hcy,
            self.int_rate,
            self.acr_int_amt_ccy,
            self.acr_int_amt_hcy,
            self.ftp_method,
            self.base_rate_curve_id,
            self.rate_flag,
            self.adj_code_1,
            self.adj_code_2,
            self.adj_code_3,
            self.adj_code_4,
            self.adj_code_5,
            self.adj_code_6,
            self.val_dt,
            self.open_dt,
            self.mat_dt,
            self.lst_repricing_dt,
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
            self.rl3,
            self.calc_ftp_rate,
            self.calc_lock_spread,
            self.bc_as_on_rule,
            self.tenor_start_date_rule,
            self.tenor_end_date_rule,
            self.bc_as_on_applied,
            self.tenor_start_date_applied,
            self.tenor_end_date_applied,
            self.gl_code,
            self.prod_code,
            self.div_code,
            self.mis_code_1,
            self.mis_code_2,
            self.mis_code_3,
        )
    }
}
