use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use cashflow_generator::ln2_biz::OutputCF;
use rbdate::NaiveDate;

pub fn create_account_with_cashflows(
    account_data: &mut InputAccount,
    cf: OutputCF,
    cashflows: Vec<Cashflow>,
    rep_flg: bool,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    out_acc.amount = cf.inst_amt;
    out_acc.ccy = account_data.currency.to_owned();
    out_acc.int_rate = account_data.interest_rate;
    out_acc.repricing_freq = account_data.princ_pay_freq.to_owned();
    out_acc.early_date = match rep_flg {
        true => cf.repricing_dt.and_hms(0, 0, 0).timestamp(),
        false => cf.inst_dt.and_hms(0, 0, 0).timestamp(),
    };
    out_acc.maturity_date = cf.inst_dt.and_hms(0, 0, 0).timestamp();
    out_acc.deal_number = account_data.acc_no.to_string();
    out_acc.start_date = match account_data.acc_open_dt {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
    out_acc.intr_computation_freq = account_data.intr_comp_freq.to_string();
    out_acc.is_floating_rate = account_data.interest_type.to_string();
    out_acc.floating_rate_benchmark = "".to_string();
    out_acc.biz_uid = "".to_string();
    out_acc.cust_id = account_data.cus_num.to_string();
    out_acc.cust_name = account_data.cus_name.to_string();
    out_acc.spread = "".to_string();
    out_acc.sub_gl_code = account_data.sub_gl_code.to_string();
    out_acc.min_intr_rate = "".to_string();
    out_acc.max_intr_rate = "".to_string();
    out_acc.deposit_amt = 0.0;
    out_acc.maturity_amt = 0.0;
    out_acc.currency_conversion_rate = account_data.currency_conversion_rate;
    out_acc.cust_ctry_code = account_data.cust_ctry_code.to_owned();
    out_acc.cust_crdt_rtng = account_data.cust_crdt_rtng.to_string();
    out_acc.cust_sect_code = account_data.cust_sect_code.to_string();
    out_acc.cust_indt_code = account_data.cust_indt_code.to_owned();
    out_acc.custom1 = account_data.custom1.to_string();
    out_acc.custom2 = account_data.custom2.to_string();
    out_acc.acid = account_data.acc_no.to_string();
    out_acc.floating_type = account_data.floating_type.to_string();
    out_acc.npa_classification = account_data.npa_classification.to_string();
    out_acc.frequency_type = account_data.frequency_type.to_string();
    out_acc.final_npa_class = account_data.final_npa_class.to_string();
    out_acc.next_repricing_date = match account_data.repricing_dt {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}

pub fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = rbdate::NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}
