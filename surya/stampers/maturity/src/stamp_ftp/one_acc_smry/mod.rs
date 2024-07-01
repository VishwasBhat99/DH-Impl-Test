mod accounts;
mod appender;
mod convertion;

pub use self::accounts::*;
pub use self::appender::*;
pub use self::convertion::*;
use super::*;
use stamp_ftp::{req_fields::*, AccountWithCFs};
use statics::*;

impl OneAccountView {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\
                {}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\
                    {}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            date_from_timestamp(self.as_on_month).format("%d-%m-%Y"),
            self.account_id,
            self.currency,
            self.eop_balance_ccy,
            self.eop_balance_hcy,
            self.avg_balance_ccy,
            self.avg_balance_hcy,
            self.int_rate,
            self.acr_int_amt_ccy,
            self.acr_int_amt_hcy,
            self.ftp_method,
            self.base_rate_curve_id,
            self.rate_flag,
            self.adj_codes[0],
            self.adj_codes[1],
            self.adj_codes[2],
            self.adj_codes[3],
            self.adj_codes[4],
            self.adj_codes[5],
            date_from_timestamp(self.val_dt).format("%d-%m-%Y"),
            date_from_timestamp(self.open_dt).format("%d-%m-%Y"),
            date_from_timestamp(self.mat_dt).format("%d-%m-%Y"),
            date_from_timestamp(self.lst_repricing_dt).format("%d-%m-%Y"),
            self.rep_freq,
            self.cust_agg_bal,
            self.day_count_basis,
            self.base_rate,
            self.adj_rates[0],
            self.adj_rates[1],
            self.adj_rates[2],
            self.adj_rates[3],
            self.adj_rates[4],
            self.adj_rates[5],
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
            date_from_timestamp(self.bc_as_on_rule).format("%d-%m-%Y"),
            date_from_timestamp(self.tenor_start_date_rule).format("%d-%m-%Y"),
            date_from_timestamp(self.tenor_end_date_rule).format("%d-%m-%Y"),
            date_from_timestamp(self.bc_as_on_applied).format("%d-%m-%Y"),
            date_from_timestamp(self.tenor_start_date_applied).format("%d-%m-%Y"),
            date_from_timestamp(self.tenor_end_date_applied).format("%d-%m-%Y"),
            self.gl_code,
            self.prod_code,
            self.div_code,
            self.mis_code_1,
            self.mis_code_2,
            self.mis_code_3,
        )
    }

    pub fn print_spread(&self, method_id: i32) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            self.account_id,
            self.ftp_rate,
            self.base_rate_curve_id,
            self.base_rate,
            self.adj_codes[0],
            self.adj_rates[0],
            self.adj_codes[1],
            self.adj_rates[1],
            self.adj_codes[2],
            self.adj_rates[2],
            self.adj_codes[3],
            self.adj_rates[3],
            self.adj_codes[4],
            self.adj_rates[4],
            self.adj_codes[5],
            self.adj_rates[5],
            self.int_rate,
            self.lock_spread,
            method_id,
        )
    }

    pub fn print_rep_spread(&self, method_id: i32, prev_spread: f32) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            self.account_id,
            self.ftp_rate,
            self.base_rate_curve_id,
            self.base_rate,
            self.adj_codes[0],
            self.adj_rates[0],
            self.adj_codes[1],
            self.adj_rates[1],
            self.adj_codes[2],
            self.adj_rates[2],
            self.adj_codes[3],
            self.adj_rates[3],
            self.adj_codes[4],
            self.adj_rates[4],
            self.adj_codes[5],
            self.adj_rates[5],
            self.int_rate,
            prev_spread,
            method_id,
        )
    }

    pub fn ccy_val_mult_by(&mut self, multiplier: f64) {
        self.eop_balance_ccy *= multiplier;
        self.avg_balance_ccy *= multiplier;
        self.ftp_amt_ccy *= multiplier;
    }

    pub fn hcy_val_mult_by(&mut self, multiplier: f64) {
        self.eop_balance_hcy *= multiplier;
        self.avg_balance_hcy *= multiplier;
        self.ftp_amt_hcy *= multiplier;
    }

    pub fn act_int_ccy_val_mult_by(&mut self, multiplier: f64, is_amt_consolidated: bool) {
        if is_amt_consolidated {
            self.acr_int_amt_ccy *= multiplier;
        }
    }

    pub fn act_int_hcy_val_mult_by(&mut self, multiplier: f64, is_amt_consolidated: bool) {
        if !is_amt_consolidated {
            self.acr_int_amt_hcy *= multiplier;
        }
    }

    pub fn rounded(&mut self, rate_prec: i8, bal_prec: i8) {
        self.eop_balance_ccy = rounded_f64(self.eop_balance_ccy, bal_prec);
        self.eop_balance_hcy = rounded_f64(self.eop_balance_hcy, bal_prec);
        self.avg_balance_ccy = rounded_f64(self.avg_balance_ccy, bal_prec);
        self.avg_balance_hcy = rounded_f64(self.avg_balance_hcy, bal_prec);
        self.int_rate = rounded_f64(self.int_rate, rate_prec);
        self.acr_int_amt_ccy = rounded_f64(self.acr_int_amt_ccy, bal_prec);
        self.acr_int_amt_hcy = rounded_f64(self.acr_int_amt_hcy, bal_prec);
        self.cust_agg_bal = rounded_f64(self.cust_agg_bal, bal_prec);
        self.base_rate = rounded_f64(self.base_rate, rate_prec);

        for index in 0..6 {
            self.adj_rates[index] = rounded_f64(self.adj_rates[index], rate_prec);
        }

        self.ftp_rate = rounded_f64(self.ftp_rate, rate_prec);
        self.ftp_amt_ccy = rounded_f64(self.ftp_amt_ccy, bal_prec);
        self.ftp_amt_hcy = rounded_f64(self.ftp_amt_hcy, bal_prec);
    }

    pub fn copied(&mut self) {
        self.calc_ftp_rate = self.ftp_rate;
        self.calc_lock_spread = self.lock_spread;
    }
}
