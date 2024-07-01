use std::default::Default;
#[derive(Debug, Clone, Default)]
pub struct LCBGMasterFields {
    pub pay_on_demand: String,
    pub backed_by_td: String,
    pub trade_non_trade: String,
    pub td_exp_dt: String,
}
