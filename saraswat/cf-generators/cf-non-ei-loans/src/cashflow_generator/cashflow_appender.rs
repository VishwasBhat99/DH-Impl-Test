use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use rbdate::{num_days_start_to_end, timestamp, NaiveDate};
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
    config_params: &ConfigurationParameters,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = 0.0;
    let mut tot_prin_amt = 0.0;

    out_acc.acid = account.acid;
    out_acc.foracid = account.foracid;
    out_acc.sol_id = account.sol_id;
    out_acc.acct_opn_date = if let Some(dt) = account.acct_opn_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.gl_sub_head_code = account.gl_sub_head_code;
    out_acc.schm_code = account.schm_code;
    out_acc.schm_type = account.schm_type;
    out_acc.acct_crncy_code = account.acct_crncy_code;
    out_acc.rep_shdl_num = account.rep_shdl_num;
    out_acc.rep_shdl_date = if let Some(dt) = account.rep_shdl_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.dis_shdl_num = account.dis_shdl_num;
    out_acc.dis_shdl_date = if let Some(dt) = account.dis_shdl_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.dis_amt = account.dis_amt;
    out_acc.clr_bal_amt = account.clr_bal_amt;
    out_acc.sanct_lim = account.sanct_lim;
    out_acc.rephasement_principal = account.rephasement_principal;
    out_acc.ei_perd_end_date = if let Some(dt) = account.ei_perd_end_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.cust_id = account.cust_id;
    out_acc.cust_name = account.cust_name;
    out_acc.ei_schm_flg = account.ei_schm_flg;
    out_acc.int_basis = account.int_basis;
    out_acc.ei_formula_flg = account.ei_formula_flg;
    out_acc.ei_intcalc_freq = account.ei_intcalc_freq;
    out_acc.ei_method = account.ei_method;
    out_acc.int_rate = account.int_rate;
    out_acc.int_type = account.int_type;
    out_acc.next_repricing_date = if let Some(dt) = account.next_repricing_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.last_repricing_date = if let Some(dt) = account.last_repricing_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.repricing_freq = account.repricing_freq;
    out_acc.float_rate_benchmark = account.float_rate_benchmark;
    out_acc.spread = account.spread;
    out_acc.npa_flg = account.npa_flg;
    out_acc.npa_classification = account.npa_classification;
    out_acc.npa_amt = account.npa_amt;
    out_acc.cust_country_cd = account.cust_country_cd;
    out_acc.cust_credit_rating = account.cust_credit_rating;
    out_acc.cust_sector_cd = account.cust_sector_cd;
    out_acc.cust_industry_cd = account.cust_industry_cd;
    out_acc.exchangert = account.exchangert;
    out_acc.contractual_maturity_days = num_days_start_to_end(
        account
            .acct_opn_date
            .expect("error getting `acct_opn_date`"),
        account
            .ei_perd_end_date
            .expect("error getting `ei_perd_end_date`"),
    );
    out_acc.residual_maturity_days = num_days_start_to_end(
        *config_params.as_on_date(),
        account
            .ei_perd_end_date
            .expect("error getting `ei_perd_end_date`"),
    );
    out_acc.custom1 = account.custom1;
    out_acc.custom2 = account.custom2;
    out_acc.custom3 = account.custom3;

    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
