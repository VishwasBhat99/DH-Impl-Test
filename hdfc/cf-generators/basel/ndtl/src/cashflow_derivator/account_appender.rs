use cashflow_derivator::account_without_cashflows::OutputAccount;
use cashflow_derivator::msf::*;
use cashflow_derivator::HashMap;
use configuration_parameters::ConfigurationParameters;

pub fn create_account_without_cashflows(
    config_params: &ConfigurationParameters,
    msf_pct: &String,
) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.as_on_date = config_params.as_on_date().format("%d-%m-%Y").to_string();
    out_acc.ndtl_val = *config_params.ndtl_val();
    out_acc.total_gsec_bv = *config_params.tot_gsec_bv();
    out_acc.total_gsec_mv = *config_params.tot_gsec_mv();
    out_acc.ex_slr_bv = *config_params.excess_slr_val();
    out_acc.msf_bv = *config_params.ndtl_val() * (msf_pct.parse::<f64>().unwrap() / 100.0);
    out_acc.slr_bv = *config_params.ndtl_val() * (*config_params.slr_pct() / 100.0);
    out_acc.ex_slr_pct_gsec = (out_acc.ex_slr_bv / *config_params.tot_gsec_bv()) * 100.0;
    out_acc.msf_pct_gsec = (out_acc.msf_bv / *config_params.tot_gsec_bv()) * 100.0;
    out_acc.slr_pct_gsec = (out_acc.slr_bv / *config_params.tot_gsec_bv()) * 100.0;
    out_acc.ex_slr_mv = *config_params.tot_gsec_mv() * (out_acc.ex_slr_pct_gsec / 100.0);
    out_acc.msf_mv = *config_params.tot_gsec_mv() * (out_acc.msf_pct_gsec / 100.0);
    out_acc.slr_mv = *config_params.tot_gsec_mv() * (out_acc.slr_pct_gsec / 100.0);
    //added new fields for AUSFB
    out_acc.final_excess_slr = if out_acc.ex_slr_bv > out_acc.ex_slr_mv {
        out_acc.ex_slr_mv
    } else {
        out_acc.ex_slr_bv
    };
    out_acc.final_msf = if out_acc.msf_bv > out_acc.msf_mv {
        out_acc.msf_mv
    } else {
        out_acc.msf_bv
    };
    out_acc.final_slr = if out_acc.slr_bv > out_acc.slr_mv {
        out_acc.slr_mv
    } else {
        out_acc.slr_bv
    };

    out_acc
}
