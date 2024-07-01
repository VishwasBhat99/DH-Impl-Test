pub mod input_account;
use self::input_account::InputAccount;

use super::InputAccountAdditional;
#[derive(Debug, Default)]
pub struct AverageValues {
    pub average_balance: f64,
    pub final_ftp_amount: f64,
    pub base_rate_1: f64,
    pub final_ftp_rate: f64,
    pub base_rate_2: f64,
    pub adj1: f64,
    pub adj2: f64,
    pub adj3: f64,
    pub adj4: f64,
    pub adj5: f64,
    pub adj6: f64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AverageValuesAdd {
    pub average_balance: f64,
    pub accr_int_income: f64,
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
    pub base_tpr_amt: f64,
    pub final_ftp_amt: f64,
    pub ftp_amt_without_psl: f64,
    pub psl_amt: f64,
    pub tot_lp_amt: f64,
    pub tot_psl_amt_without_ews_and_smf: f64,
    pub tot_ews_amt: f64,
    pub tot_smf_amt: f64,
    pub margin_amt: f64,
}

impl AverageValues {
    pub fn new(acc: &InputAccount) -> Self {
        AverageValues {
            average_balance: acc.average_balance,
            final_ftp_amount: acc.final_ftp_amount,
            base_rate_1: acc.base_rate_1,
            final_ftp_rate: acc.final_ftp_rate,
            base_rate_2: acc.base_rate_2,
            adj1: acc.adj1,
            adj2: acc.adj2,
            adj3: acc.adj3,
            adj4: acc.adj4,
            adj5: acc.adj5,
            adj6: acc.adj6,
        }
    }

    pub fn weighted(&mut self, other: &InputAccount) {
        self.base_rate_1 = calc_wtd_avg_rt(
            self.average_balance,
            self.base_rate_1,
            other.average_balance,
            other.base_rate_1,
        );
        self.final_ftp_rate = calc_wtd_avg_rt(
            self.average_balance,
            self.final_ftp_rate,
            other.average_balance,
            other.final_ftp_rate,
        );
        self.base_rate_2 = calc_wtd_avg_rt(
            self.average_balance,
            self.base_rate_2,
            other.average_balance,
            other.base_rate_2,
        );
        self.adj1 = calc_wtd_avg_rt(
            self.average_balance,
            self.adj1,
            other.average_balance,
            other.adj1,
        );
        self.adj2 = calc_wtd_avg_rt(
            self.average_balance,
            self.adj2,
            other.average_balance,
            other.adj2,
        );
        self.adj3 = calc_wtd_avg_rt(
            self.average_balance,
            self.adj3,
            other.average_balance,
            other.adj3,
        );
        self.adj4 = calc_wtd_avg_rt(
            self.average_balance,
            self.adj4,
            other.average_balance,
            other.adj4,
        );
        self.adj5 = calc_wtd_avg_rt(
            self.average_balance,
            self.adj5,
            other.average_balance,
            other.adj5,
        );
        self.adj6 = calc_wtd_avg_rt(
            self.average_balance,
            self.adj6,
            other.average_balance,
            other.adj6,
        );
        self.average_balance = calc_tot_amt(self.average_balance, other.average_balance);
        self.final_ftp_amount = calc_tot_amt(self.final_ftp_amount, other.final_ftp_amount);
    }
}

impl AverageValuesAdd {
    pub fn new(acc: &InputAccountAdditional) -> Self {
        AverageValuesAdd {
            average_balance: acc.avg_bal,
            final_ftp_rate: acc.final_ftp_rate,
            accr_int_income: acc.accr_int,
            accr_int_rate: acc.accr_int_rate,
            adj1: acc.adj1,
            adj2: acc.adj2,
            adj3: acc.adj3,
            adj4: acc.adj4,
            adj5: acc.adj5,
            adj6: acc.adj6,
            adj7: acc.adj7,
            adj8: acc.adj8,
            adj9: acc.adj9,
            adj10: acc.adj10,
            ftp_rate_without_psl: acc.ftp_rate_without_psl,
            margin_rate: acc.margin_rate,
            yld_to_call: acc.yld_to_call,
            int_rate: acc.int_rate,
            base_rate: acc.base_rate,
            base_tpr_amt: acc.base_tpr_amount,
            final_ftp_amt: acc.final_ftp_amount,
            ftp_amt_without_psl: acc.ftp_amt_without_psl,
            psl_amt: acc.psl_amount,
            tot_lp_amt: acc.total_lp_amount,
            tot_psl_amt_without_ews_and_smf: acc.total_psl_amount_without_ews_smf,
            tot_ews_amt: acc.total_ews_amount,
            tot_smf_amt: acc.total_smf_amount,
            margin_amt: acc.margin_amount,
        }
    }

    pub fn weighted(&mut self, other: &InputAccountAdditional) {
        self.base_tpr_amt = calc_tot_amt(self.base_tpr_amt, other.base_tpr_amount);
        self.final_ftp_amt = calc_tot_amt(self.final_ftp_amt, other.final_ftp_amount);
        self.ftp_amt_without_psl =
            calc_tot_amt(self.ftp_amt_without_psl, other.ftp_amt_without_psl);
        self.psl_amt = calc_tot_amt(self.psl_amt, other.psl_amount);
        self.tot_lp_amt = calc_tot_amt(self.tot_lp_amt, other.total_lp_amount);
        self.tot_psl_amt_without_ews_and_smf = calc_tot_amt(
            self.tot_psl_amt_without_ews_and_smf,
            other.total_psl_amount_without_ews_smf,
        );
        self.tot_ews_amt = calc_tot_amt(self.tot_ews_amt, other.total_ews_amount);
        self.tot_smf_amt = calc_tot_amt(self.tot_smf_amt, other.total_smf_amount);
        self.margin_amt = calc_tot_amt(self.margin_amt, other.margin_amount);
        self.accr_int_income = calc_wtd_avg_rt(
            self.average_balance,
            self.accr_int_income,
            other.avg_bal,
            other.accr_int,
        );
        self.accr_int_rate = calc_wtd_avg_rt(
            self.average_balance,
            self.accr_int_rate,
            other.avg_bal,
            other.accr_int_rate,
        );
        self.yld_to_call = calc_wtd_avg_rt(
            self.average_balance,
            self.yld_to_call,
            other.avg_bal,
            other.yld_to_call,
        );
        self.int_rate = calc_wtd_avg_rt(
            self.average_balance,
            self.int_rate,
            other.avg_bal,
            other.int_rate,
        );
        self.final_ftp_rate = calc_wtd_avg_rt(
            self.average_balance,
            self.final_ftp_rate,
            other.avg_bal,
            other.final_ftp_rate,
        );
        self.base_rate = calc_wtd_avg_rt(
            self.average_balance,
            self.base_rate,
            other.avg_bal,
            other.base_rate,
        );
        self.adj1 = calc_wtd_avg_rt(self.average_balance, self.adj1, other.avg_bal, other.adj1);

        self.adj2 = calc_wtd_avg_rt(self.average_balance, self.adj2, other.avg_bal, other.adj2);
        self.adj3 = calc_wtd_avg_rt(self.average_balance, self.adj3, other.avg_bal, other.adj3);
        self.adj4 = calc_wtd_avg_rt(self.average_balance, self.adj4, other.avg_bal, other.adj4);
        self.adj5 = calc_wtd_avg_rt(self.average_balance, self.adj5, other.avg_bal, other.adj5);
        self.adj6 = calc_wtd_avg_rt(self.average_balance, self.adj6, other.avg_bal, other.adj6);
        self.adj7 = calc_wtd_avg_rt(self.average_balance, self.adj7, other.avg_bal, other.adj7);
        self.adj8 = calc_wtd_avg_rt(self.average_balance, self.adj8, other.avg_bal, other.adj8);
        self.adj9 = calc_wtd_avg_rt(self.average_balance, self.adj9, other.avg_bal, other.adj9);
        self.adj10 = calc_wtd_avg_rt(self.average_balance, self.adj10, other.avg_bal, other.adj10);
        self.ftp_rate_without_psl = calc_wtd_avg_rt(
            self.average_balance,
            self.ftp_rate_without_psl,
            other.avg_bal,
            other.ftp_rate_without_psl,
        );
        self.margin_rate = calc_wtd_avg_rt(
            self.average_balance,
            self.margin_rate,
            other.avg_bal,
            other.margin_rate,
        );
        self.average_balance = calc_tot_amt(self.average_balance, other.avg_bal);
    }
}

fn calc_tot_amt(amt1: f64, amt2: f64) -> f64 {
    amt1 + amt2
}

fn calc_wtd_avg_rt(amt1: f64, rt1: f64, amt2: f64, rt2: f64) -> f64 {
    ((amt1 * rt1) + (amt2 * rt2)) / calc_tot_amt(amt1, amt2)
}
