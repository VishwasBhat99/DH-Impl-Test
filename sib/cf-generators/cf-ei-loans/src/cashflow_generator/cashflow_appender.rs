use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashFlows;
use cashflow_generator::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use rbdate::{num_days_start_to_end, timestamp};
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
    config_params: &ConfigurationParameters,
) -> (AccountWithCashFlows, f64, f64) {
    let mut out_acc = AccountWithCashFlows::new();
    let mut tot_prin_amt = 0.0;
    let mut tot_int_amt = 0.0;

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
    out_acc.cust_country_id = account.cust_country_cd;
    out_acc.cust_credit_rating = account.cust_credit_rating;
    out_acc.cust_sector_cd = account.cust_sector_cd;
    out_acc.cust_industry_cd = account.cust_industry_cd;
    out_acc.exchangert = account.exchangert;
    out_acc.contractual_maturity_days = num_days_start_to_end(
        account
            .acct_opn_date
            .expect("Error getting `acct_opn_date`"),
        account
            .ei_perd_end_date
            .expect("Error getting `ei_perd_end_date`"),
    );
    out_acc.residual_maturity_days = num_days_start_to_end(
        *config_params.as_on_date(),
        account
            .ei_perd_end_date
            .expect("Error getting `ei_perd_end_date`"),
    );
    out_acc.custom1 = account.custom1;
    out_acc.custom2 = account.custom2;
    out_acc.custom3 = account.custom3;
    out_acc.gnt_type = account.gnt_type;
    out_acc.status_code = account.status_code;
    out_acc.occupation_code = account.occupation_code;
    out_acc.sector = account.sector;
    out_acc.sector_code = account.sector_code;
    out_acc.subsector_code = account.subsector_code;
    out_acc.staffflag = account.staffflag;
    out_acc.cre_free_text_1 = account.cre_free_text_1;
    out_acc.prov_perc = account.prov_perc;
    out_acc.ltv = account.ltv;
    out_acc.npa_prov = account.npa_prov;
    out_acc.dumm3 = account.dumm3;
    out_acc.dumm4 = account.dumm4;
    out_acc.dumm5 = account.dumm5;
    out_acc.dumm6 = account.dumm6;
    out_acc.dumm7 = account.dumm7;
    out_acc.dumm8 = account.dumm8;
    out_acc.dumm9 = account.dumm9;
    out_acc.dumm10 = account.dumm10;
    out_acc.constcatgorycode = account.constcatgorycode;
    out_acc.ratingagc = account.ratingagc;
    out_acc.rating = account.rating;
    out_acc.supperannuation_flag = account.supperannuation_flag;
    out_acc.turn_amt1 = account.turn_amt1;
    out_acc.turn_amt2 = account.turn_amt2;
    out_acc.turn_amt3 = account.turn_amt3;
    out_acc.ftp_char1 = account.ftp_char1;
    out_acc.ftp_char2 = account.ftp_char2;
    out_acc.ftp_amt1 = account.ftp_amt1;
    out_acc.ftp_amt2 = account.ftp_amt2;
    out_acc.ftp_date1 = if let Some(dt) = account.ftp_date1 {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.ftp_date2 = if let Some(dt) = account.ftp_date2 {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };

    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    let mut cf_ovd = "N";
    for cf in out_acc.cashflows.iter().cloned() {
        tot_prin_amt += cf.get_principal_amount();
        tot_int_amt += cf.get_interest_amount();
        if cf.date <= timestamp(*config_params.as_on_date()) {
            cf_ovd = "Y";
        }
    }
    out_acc.is_cf_overdue = cf_ovd.to_string();
    (out_acc, tot_prin_amt, tot_int_amt)
}
