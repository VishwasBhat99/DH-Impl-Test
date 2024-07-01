use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use protobuf;
use rbdate;
use rbdate::date_from_timestamp;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
    od_flag: String,
) -> AccountWithCashflows {
    let mut account_with_cashflows = AccountWithCashflows::new();
    let mut total_interest_amount = 0.0;
    let mut total_principal_amount = 0.0;

    account_with_cashflows.cashflow_type = account.cashflow_type;
    account_with_cashflows.type_id = account.type_id;
    account_with_cashflows.subtype_id = account.subtype_id;
    account_with_cashflows.amount = account.clr_bal_amt;
    account_with_cashflows.acct_crncy_code = account.acct_crncy_code;
    account_with_cashflows.int_rate = account.int_rate;
    account_with_cashflows.repricing_freq = account.repricing_freq;
    account_with_cashflows.maturity_date = rbdate::timestamp(account.end_date);
    account_with_cashflows.foracid = account.foracid;
    account_with_cashflows.start_date =
        rbdate::timestamp(account.next_repricing_date.unwrap_or(account.acct_opn_date));
    account_with_cashflows.is_floating = account.fixedornot;
    account_with_cashflows.float_rate_benchmark = account.float_rate_benchmark;
    account_with_cashflows.sol_id = account.sol_id;
    account_with_cashflows.cust_id = account.cust_id;
    account_with_cashflows.cust_name = account.cust_name;
    account_with_cashflows.region_id = 0.to_string();
    account_with_cashflows.schm_type = account.schm_type;
    account_with_cashflows.gl_subhead_code = account.gl_sub_head_code;
    account_with_cashflows.npa = 99.to_string();
    account_with_cashflows.floor_rate = 0.0;
    account_with_cashflows.cap_rate = 0.0;
    account_with_cashflows.cust_country_cd = account.cust_country_cd;
    account_with_cashflows.cust_credit_rating = account.cust_credit_rating;
    account_with_cashflows.cust_sector_cd = account.cust_sector_cd;
    account_with_cashflows.cust_industry_cd = account.cust_industry_cd;
    account_with_cashflows.exchange_rt = account.exchange_rt;
    account_with_cashflows.custom1 = account.custom1;
    account_with_cashflows.custom2 = account.custom2;
    account_with_cashflows.npa_classification = account.npa_classification;
    account_with_cashflows.floating_type = account.floating_type;
    account_with_cashflows.out_bal_amount = account.out_bal_amount;
    account_with_cashflows.acid = account.acid;
    account_with_cashflows.final_npa_class = account.final_npa_class;
    account_with_cashflows.der_pegged_flg = account.der_pegged_flg;
    account_with_cashflows.int_tbl_code = account.int_tbl_code;

    let mut cf_date = String::new();
    for cf in &cashflows {
        total_interest_amount += cf.interest_amount;
        total_principal_amount += cf.principal_amount;
        cf_date = date_from_timestamp(cf.date).format("%d-%m-%Y").to_string();
    }

    account_with_cashflows.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    account_with_cashflows
}
