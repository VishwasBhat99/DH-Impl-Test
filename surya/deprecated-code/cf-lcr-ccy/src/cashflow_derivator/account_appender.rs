use super::structs::*;
use cashflow_derivator::account_without_cashflows::OutputAccount;
use slog::Logger;

pub fn create_account_without_cashflows(key: AggrKey, data: LCR, _log: &Logger) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.file_id = key.file_id.to_string();
    out_acc.currency = key.currency;
    out_acc.ca = data.ca;
    out_acc.sa = data.sa;
    out_acc.td_wd = data.td_wd;
    out_acc.td_nwd = data.td_nwd;
    out_acc.rd = data.rd;
    out_acc.tot_stable = data.tot_stable;
    out_acc.tot_less_stable = data.tot_less_stable;
    out_acc.ca_stable = data.ca_stable;
    out_acc.ca_less_stable = data.ca_less_stable;
    out_acc.sa_stable = data.sa_stable;
    out_acc.sa_less_stable = data.sa_less_stable;
    out_acc.casa_stable = data.casa_stable;
    out_acc.casa_less_stable = data.casa_less_stable;
    out_acc.stable_b1 = data.stable_b1;
    out_acc.stable_b2 = data.stable_b2;
    out_acc.stable_b3 = data.stable_b3;
    out_acc.less_stable_b1 = data.less_stable_b1;
    out_acc.less_stable_b2 = data.less_stable_b2;
    out_acc.less_stable_b3 = data.less_stable_b3;
    out_acc.nwd_b1 = data.nwd_b1;
    out_acc.nwd_b2 = data.nwd_b2;
    out_acc.nwd_b3 = data.nwd_b3;

    out_acc
}
