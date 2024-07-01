use super::{AverageValues, AverageValuesAdd};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InputAccount {
    pub account_number: String,
    pub cust_name: String,
    pub average_balance: f64,
    pub accr_int: String,
    pub yld_to_call: String,
    pub interest_rate: String,
    pub base_rate_1: f64,
    pub final_ftp_rate: f64,
    pub value_date: String,
    pub maturity_date: String,
    pub next_reprice_date: String,
    pub last_reprice_date: String,
    pub mis1: String,
    pub mis2: String,
    pub psl_code: String,
    pub prod_code_type: String,
    pub rate_flag: String,
    pub blank_1: String,
    pub source_file_name: String,
    pub currency: String,
    pub gl: String,
    pub cust_id: String,
    pub final_ftp_amount: f64,
    pub alm_line: String,
    pub blank_2: String,
    pub initial_dep_amt_td: String,
    pub current_outstanding_td: String,
    pub base_rate_2: f64,
    pub adj1: f64,
    pub adj2: f64,
    pub adj3: f64,
    pub adj4: f64,
    pub adj5: f64,
    pub adj6: f64,
    pub input_benchmark: String,
    pub pdo: String,
    pub npa: String,
    pub ftp_method: String,
    pub ftp_rate_curve: String,
    pub org_tenor: String,
    pub repricing_tenor: String,
    pub fixed_spread: String,
    pub variable_spread: String,
    pub first_month_ftp: String,
    pub bc_as_on_rule: String,
    pub tenor_start_date_rule: String,
    pub tenor_end_date_rule: String,
    pub bc_as_on_applied: String,
    pub tenor_start_date_applied: String,
    pub tenor_end_date_applied: String,
    pub alm_concat: String,
    pub two_point_concat: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InputAccountAdditional {
    pub ftp_month: String,
    pub acc_num: String,
    pub cust_name: String,
    pub avg_bal: f64,
    pub accr_int: f64,
    pub accr_int_rate: f64,
    pub yld_to_call: f64,
    pub int_rate: f64,
    pub base_rate: f64,
    pub adj1: f64,
    pub adj2: f64,
    pub adj3: f64,
    pub adj4: f64,
    pub adj5: f64,
    pub adj6: f64,
    pub adj7: f64,
    pub adj8: f64,
    pub adj9: f64,
    pub adj10: f64,
    pub final_ftp_rate: f64,
    pub ftp_rate_without_psl: f64,
    pub margin_rate: f64,
    pub base_tpr_amount: f64,
    pub final_ftp_amount: f64,
    pub ftp_amt_without_psl: f64,
    pub psl_amount: f64,
    pub total_lp_amount: f64,
    pub total_psl_amount_without_ews_smf: f64,
    pub total_ews_amount: f64,
    pub total_smf_amount: f64,
    pub margin_amount: f64,
    pub value_date: String,
    pub maturity_date: String,
    pub last_reprice_date: String,
    pub next_reprice_date: String,
    pub mis1: String,
    pub mis2: String,
    pub psl_code: String,
    pub prod_code: String,
    pub rate_flag: String,
    pub branch: String,
    pub source_file_name: String,
    pub currency: String,
    pub gl_code: String,
    pub cust_id: String,
    pub alm_line: String,
    pub trade_date: String,
    pub initial_dep_amt: f64,
    pub current_outstanding: f64,
    pub input_benchmark: String,
    pub pdo: String,
    pub npa: String,
    pub ftp_method: String,
    pub ftp_rate_curve: String,
    pub org_tenor: String,
    pub repricing_tenor: String,
    pub fixed_spread: f64,
    pub variable_spread: f64,
    pub first_month_ftp: f64,
    pub bc_as_on_rule: String,
    pub tenor_start_date_rule: String,
    pub tenor_end_rate_rule: String,
    pub bc_as_on_applied: String,
    pub tenor_start_date_applied: String,
    pub tenor_end_date_applied: String,
    pub concat_4_point: String,
    pub concat_2_point: String,
    pub ews_flag: String,
    pub bdp_division: String,
    pub bdp_coa: String,
    pub adj_id_1: String,
    pub adj_id_2: String,
    pub adj_id_3: String,
    pub adj_id_4: String,
    pub adj_id_5: String,
    pub adj_id_6: String,
    pub adj_id_7: String,
    pub adj_id_8: String,
    pub adj_id_9: String,
    pub adj_id_10: String,
}

impl InputAccount {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\
                {}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            self.account_number,
            self.cust_name,
            self.average_balance,
            self.accr_int,
            self.yld_to_call,
            self.interest_rate,
            self.base_rate_1,
            self.final_ftp_rate,
            self.value_date,
            self.maturity_date,
            self.next_reprice_date,
            self.last_reprice_date,
            self.mis1,
            self.mis2,
            self.psl_code,
            self.prod_code_type,
            self.rate_flag,
            self.blank_1,
            self.source_file_name,
            self.currency,
            self.gl,
            self.cust_id,
            self.final_ftp_amount,
            self.alm_line,
            self.blank_2,
            self.initial_dep_amt_td,
            self.current_outstanding_td,
            self.base_rate_2,
            self.adj1,
            self.adj2,
            self.adj3,
            self.adj4,
            self.adj5,
            self.adj6,
            self.input_benchmark,
            self.pdo,
            self.npa,
            self.ftp_method,
            self.ftp_rate_curve,
            self.org_tenor,
            self.repricing_tenor,
            self.fixed_spread,
            self.variable_spread,
            self.first_month_ftp,
            self.bc_as_on_rule,
            self.tenor_start_date_rule,
            self.tenor_end_date_rule,
            self.bc_as_on_applied,
            self.tenor_start_date_applied,
            self.tenor_end_date_applied,
            self.alm_concat,
            self.two_point_concat
        )
    }

    pub fn print_weighted(&self, avg_wt: &AverageValues) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\
                {}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            self.account_number,
            self.cust_name,
            avg_wt.average_balance,
            self.accr_int,
            self.yld_to_call,
            self.interest_rate,
            avg_wt.base_rate_1,
            avg_wt.final_ftp_rate,
            self.value_date,
            self.maturity_date,
            self.next_reprice_date,
            self.last_reprice_date,
            self.mis1,
            self.mis2,
            self.psl_code,
            self.prod_code_type,
            self.rate_flag,
            self.blank_1,
            self.source_file_name,
            self.currency,
            self.gl,
            self.cust_id,
            avg_wt.final_ftp_amount,
            self.alm_line,
            self.blank_2,
            self.initial_dep_amt_td,
            self.current_outstanding_td,
            avg_wt.base_rate_2,
            avg_wt.adj1,
            avg_wt.adj2,
            avg_wt.adj3,
            avg_wt.adj4,
            avg_wt.adj5,
            avg_wt.adj6,
            self.input_benchmark,
            self.pdo,
            self.npa,
            self.ftp_method,
            self.ftp_rate_curve,
            self.org_tenor,
            self.repricing_tenor,
            self.fixed_spread,
            self.variable_spread,
            self.first_month_ftp,
            self.bc_as_on_rule,
            self.tenor_start_date_rule,
            self.tenor_end_date_rule,
            self.bc_as_on_applied,
            self.tenor_start_date_applied,
            self.tenor_end_date_applied,
            self.alm_concat,
            self.two_point_concat
        )
    }
}

impl InputAccountAdditional {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\
                {}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            self.ftp_month,
            self.acc_num,
            self.cust_name,
            self.avg_bal,
            self.accr_int,
            self.accr_int_rate,
            self.yld_to_call,
            self.int_rate,
            self.base_rate,
            self.adj1,
            self.adj2,
            self.adj3,
            self.adj4,
            self.adj5,
            self.adj6,
            self.adj7,
            self.adj8,
            self.adj9,
            self.adj10,
            self.final_ftp_rate,
            self.ftp_rate_without_psl,
            self.margin_rate,
            self.base_tpr_amount,
            self.final_ftp_amount,
            self.ftp_amt_without_psl,
            self.psl_amount,
            self.total_lp_amount,
            self.total_psl_amount_without_ews_smf,
            self.total_ews_amount,
            self.total_smf_amount,
            self.margin_amount,
            self.value_date,
            self.maturity_date,
            self.last_reprice_date,
            self.next_reprice_date,
            self.mis1,
            self.mis2,
            self.psl_code,
            self.prod_code,
            self.rate_flag,
            self.branch,
            self.source_file_name,
            self.source_file_name,
            self.currency,
            self.gl_code,
            self.cust_id,
            self.alm_line,
            self.trade_date,
            self.initial_dep_amt,
            self.current_outstanding,
            self.input_benchmark,
            self.pdo,
            self.npa,
            self.ftp_method,
            self.ftp_rate_curve,
            self.org_tenor,
            self.repricing_tenor,
            self.fixed_spread,
            self.variable_spread,
            self.first_month_ftp,
            self.bc_as_on_rule,
            self.tenor_start_date_rule,
            self.tenor_end_rate_rule,
            self.bc_as_on_applied,
            self.tenor_start_date_applied,
            self.tenor_end_date_applied,
            self.concat_4_point,
            self.concat_2_point,
            self.ews_flag,
            self.bdp_division,
            self.bdp_coa,
            self.adj_id_1,
            self.adj_id_2,
            self.adj_id_3,
            self.adj_id_4,
            self.adj_id_5,
            self.adj_id_6,
            self.adj_id_7,
            self.adj_id_8,
            self.adj_id_9,
            self.adj_id_10,
        )
    }

    pub fn print_weighted(&self, avg_wt: &AverageValuesAdd) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\
            {}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
                self.ftp_month,
                self.acc_num,
                self.cust_name,
                avg_wt.average_balance,
                avg_wt.accr_int_income,
                avg_wt.accr_int_rate,
                avg_wt.yld_to_call,
                avg_wt.int_rate,
                avg_wt.base_rate,
                avg_wt.adj1,
                avg_wt.adj2,
                avg_wt.adj3,
                avg_wt.adj4,
                avg_wt.adj5,
                avg_wt.adj6,
                avg_wt.adj7,
                avg_wt.adj8,
                avg_wt.adj9,
                avg_wt.adj10,
                avg_wt.final_ftp_rate,
                avg_wt.ftp_rate_without_psl,
                avg_wt.margin_rate,
                avg_wt.base_tpr_amt,
                avg_wt.final_ftp_amt,
                avg_wt.ftp_amt_without_psl,
                avg_wt.psl_amt,
                avg_wt.tot_lp_amt,
                avg_wt.tot_psl_amt_without_ews_and_smf,
                avg_wt.tot_ews_amt,
                avg_wt.tot_smf_amt,
                avg_wt.margin_amt,
                self.value_date,
                self.maturity_date,
                self.last_reprice_date,
                self.next_reprice_date,
                self.mis1,
                self.mis2,
                self.psl_code,
                self.prod_code,
                self.rate_flag,
                self.branch,
                self.source_file_name,
                self.source_file_name,
                self.currency,
                self.gl_code,
                self.cust_id,
                self.alm_line,
                self.trade_date,
                self.initial_dep_amt,
                self.current_outstanding,
                self.input_benchmark,
                self.pdo,
                self.npa,
                self.ftp_method,
                self.ftp_rate_curve,
                self.org_tenor,
                self.repricing_tenor,
                self.fixed_spread,
                self.variable_spread,
                self.first_month_ftp,
                self.bc_as_on_rule,
                self.tenor_start_date_rule,
                self.tenor_end_rate_rule,
                self.bc_as_on_applied,
                self.tenor_start_date_applied,
                self.tenor_end_date_applied,
                self.concat_4_point,
                self.concat_2_point,
                self.ews_flag,
                self.bdp_division,
                self.bdp_coa,
                self.adj_id_1,
                self.adj_id_2,
                self.adj_id_3,
                self.adj_id_4,
                self.adj_id_5,
                self.adj_id_6,
                self.adj_id_7,
                self.adj_id_8,
                self.adj_id_9,
                self.adj_id_10,
                
        )
    }
}

