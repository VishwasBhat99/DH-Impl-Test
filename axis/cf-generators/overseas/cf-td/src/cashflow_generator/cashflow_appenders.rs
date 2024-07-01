use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use chrono::naive;
use rbdate;
use rbdate::NaiveDate;

pub fn create_account_with_cashflows(
    account_data: InputAccount,
    cashflows: Vec<Cashflow>,
    round_off_ex_rt: i64,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    out_acc.amount = account_data.amount;
    out_acc.ccy = account_data.ccy;
    out_acc.intr_rate = account_data.intrrt;
    out_acc.repricing_freq = "".to_string();
    out_acc.early_dt = rbdate::timestamp(account_data.maturitydt);
    out_acc.maturity_dt = rbdate::timestamp(account_data.maturitydt);
    out_acc.deal_number = account_data.foracid;
    out_acc.start_dt = rbdate::timestamp(account_data.startdt);
    out_acc.intr_computation_freq = account_data.intrcompfreq;
    out_acc.is_floating_rt = account_data.isfloatingrt;
    out_acc.floating_rt_benchmark = account_data.floatingrtbenchmark;
    out_acc.biz_uid = account_data.branchcode;
    out_acc.cust_id = account_data.custid;
    out_acc.cust_name = account_data.custname;
    out_acc.spread = 0.0;
    out_acc.schm_code = account_data.schmcode;
    out_acc.min_intr_rt = account_data.minintrt;
    out_acc.max_intr_rt = account_data.maxintrt;
    out_acc.deposit_amt = account_data.amount;
    out_acc.maturity_amt = account_data.maturityamt;
    out_acc.currency_conversion_rate = format!(
        "{:.1$}",
        account_data.currencyconversionrt, round_off_ex_rt as usize
    )
    .parse::<f64>()
    .unwrap_or(account_data.currencyconversionrt);
    out_acc.cust_ctry_code = account_data.custctrycode;
    out_acc.cust_crdt_rtng = account_data.custcrdtrtng;
    out_acc.cust_sect_code = account_data.custsectcode;
    out_acc.cust_indt_code = account_data.custindtcode;
    out_acc.custom1 = account_data.custom1;
    out_acc.custom2 = account_data.custom2;
    out_acc.gl_sub_head_code=account_data.gl_sub_head_code;
    out_acc.cust_hlth_code = account_data.cust_hlth_code;
    out_acc.schm_type = account_data.schm_type;
    //Uncomment below lines for verification of cashflows.
    // for cf in cashflows{
    //     if cf.principal_amount!=0.0{
    //         println!("{}|{}|0.0|{}",out_acc.deal_number,cf.principal_amount,naivedate_from_timestamp(cf.date));
    //     }
    //     else{
    //         println!("{}|{}|0.0|{}",out_acc.deal_number,cf.interest_amount,naivedate_from_timestamp(cf.date)); 
    //     }
    // }
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}

pub fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = rbdate::NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}
