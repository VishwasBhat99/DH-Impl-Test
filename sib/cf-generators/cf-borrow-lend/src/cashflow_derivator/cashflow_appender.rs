use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::timestamp;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = 0.0;
    let mut tot_prin_amt = 0.0;
    out_acc.as_on_date = timestamp(account.as_on_date);
    out_acc.deal_number = account.deal_number;
    out_acc.instrument_type = account.instrument_type;
    out_acc.branch = account.branch;
    out_acc.outstanding_amount = account.outstanding_amount;
    out_acc.currency = account.currency;
    out_acc.instrument_classification = account.instrument_classification;
    out_acc.counter_party_id = account.counter_party_id;
    out_acc.counter_party_name = account.counter_party_name;
    out_acc.counter_party_type = account.counter_party_type;
    out_acc.borrowing_date = timestamp(account.borrowing_date);
    out_acc.borrowing_maturity_date = timestamp(account.borrowing_maturity_date);
    out_acc.interest_rate = account.interest_rate;
    out_acc.interest_rate_classification = account.interest_rate_classification;
    out_acc.frequency = account.frequency;
    out_acc.basis = account.basis;
    out_acc.next_repricing_date = timestamp(account.next_repricing_date);
    out_acc.last_repricing_date = timestamp(account.last_repricing_date);
    out_acc.repricing_frequency = account.repricing_frequency;
    out_acc.coupon_payment_start_date = timestamp(account.coupon_payment_start_date);
    out_acc.coupon_payment_frequency = account.coupon_payment_frequency;
    out_acc.benchmark = account.benchmark;
    out_acc.spread = account.spread;
    out_acc.isin_code = account.isin_code;
    out_acc.mduration = account.mduration;
    out_acc.treasury_gl_code = account.treasury_gl_code;
    out_acc.accrued_interest = account.accrued_interest;
    out_acc.accrued_gl = account.accrued_gl;
    out_acc.deal_date = timestamp(account.deal_date);
    out_acc.value_date = timestamp(account.value_date);
    out_acc.avg_mon_balance = account.avg_mon_balance;
    out_acc.cdr_flg = account.cdr_flg;
    out_acc.cf_type = account.cf_type;
    out_acc.cf_sub_type = account.cf_sub_type;
    out_acc.cf_amount = account.cf_amount;
    out_acc.cf_date = timestamp(account.cf_date);
    out_acc.cf_currency = account.cf_currency;
    out_acc.outstanding_diff_amount = account.outstanding_diff_amount;
    out_acc.group = account.group;
    out_acc.llg = account.llg;
    out_acc.other_llg_classification = account.other_llg_classification;
    //Adjustment_cashflow
    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    let mut rem_cf = Cashflow::new();
    let mut all_cfs = protobuf::RepeatedField::from_vec(cashflows);
    rem_cf.prin_amt = out_acc.outstanding_diff_amount - tot_prin_amt;
    rem_cf.date = out_acc.borrowing_maturity_date;
    all_cfs.push(rem_cf);

    out_acc.cashflows = all_cfs;
    out_acc
}
