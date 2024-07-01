use super::*;

#[derive(Debug, Default)]
pub struct TotalBalances {
    pub ttl_prin_amt: f64,
    pub ttl_int_amt: f64,
    pub ttl_base_rate_prod: f64,
    pub ttl_end_rate_bal: f64,
    pub ttl_org_tenor_bal: f64,
    pub ttl_ftp_amt: f64,
    pub run_duration: i64,
    pub residual_days: i64,
    pub max_days_in_year: i64,
    pub fix_adj: f64,
    pub base_rate: f64,
    pub ftp_rate: f64,
    pub var_adj: f64,
}

impl TotalBalances {
    pub fn new(fix_adj: f64, var_adj: f64, static_params: &StaticParams) -> Self {
        Self {
            ttl_prin_amt: 0.0,
            ttl_int_amt: 0.0,
            ttl_base_rate_prod: 0.0,
            ttl_end_rate_bal: 0.0,
            ttl_org_tenor_bal: 0.0,
            ttl_ftp_amt: 0.0,
            run_duration: num_days_start_to_end(
                *static_params.config_params.from_date(),
                *static_params.config_params.to_date(),
            ) + 1,
            residual_days: 0,
            max_days_in_year: num_days_start_to_end(
                *static_params.config_params.to_date(),
                increment_date_by_months(*static_params.config_params.to_date(), 12_u16),
            ),
            fix_adj,
            var_adj,
            base_rate: 0.0,
            ftp_rate: 0.0,
        }
    }

    pub fn derive_rates(&mut self) {
        if self.ttl_org_tenor_bal != 0.0 {
            self.base_rate = self.ttl_base_rate_prod / self.ttl_org_tenor_bal;
            self.ftp_rate = self.ttl_end_rate_bal / self.ttl_org_tenor_bal;
        }
    }
}
