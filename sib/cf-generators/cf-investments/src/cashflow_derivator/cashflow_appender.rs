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
    out_acc.portfolio = account.portfolio;
    out_acc.instrument_id = account.instrument_id;
    out_acc.concat_id = account.concat_id;
    out_acc.instrument_type = account.instrument_type;
    out_acc.branch_code = account.branch_code;
    out_acc.category = account.category;
    out_acc.isin_code = account.isin_code;
    out_acc.security_name = account.security_name;
    out_acc.face_value = account.face_value;
    out_acc.book_value = account.book_value;
    out_acc.appreciation_value = account.appreciation_value;
    out_acc.depreciation_value = account.depreciation_value;
    out_acc.wap = account.wap;
    out_acc.market_value = account.market_value;
    out_acc.currency = account.currency;
    out_acc.acc_yield = account.acc_yield;
    out_acc.maturity_date = timestamp(account.maturity_date);
    out_acc.coupon_classification = account.coupon_classification;
    out_acc.coupon_rate = account.coupon_rate;
    out_acc.face_value_per_unit = account.face_value_per_unit;
    out_acc.outstanding_quantity = account.outstanding_quantity;
    out_acc.accrued_interest = account.accrued_interest;
    out_acc.coupon_frequency = account.coupon_frequency;
    out_acc.coupon_basis = account.coupon_basis;
    out_acc.put_date = timestamp(account.put_date);
    out_acc.call_date = timestamp(account.call_date);
    out_acc.last_coupon = timestamp(account.last_coupon);
    out_acc.next_coupon = timestamp(account.next_coupon);
    out_acc.issue_date = timestamp(account.issue_date);
    out_acc.last_repricing_date = timestamp(account.last_repricing_date);
    out_acc.next_repricing_date = timestamp(account.next_repricing_date);
    out_acc.place = account.place;
    out_acc.country = account.country;
    out_acc.slr_non_slr = account.slr_non_slr;
    out_acc.listed_unlisted = account.listed_unlisted;
    out_acc.secured_unsecured = account.secured_unsecured;
    out_acc.issuer_id = account.issuer_id;
    out_acc.issuer_name = account.issuer_name;
    out_acc.issuer_type = account.issuer_type;
    out_acc.sub_issuer_type = account.sub_issuer_type;
    out_acc.external_rating_agency = account.external_rating_agency;
    out_acc.rating = account.rating;
    out_acc.issuer_guaranteed_by = account.issuer_guaranteed_by;
    out_acc.industry = account.industry;
    out_acc.sub_industry = account.sub_industry;
    out_acc.npa_classification = account.npa_classification;
    out_acc.deal_value_date = timestamp(account.deal_value_date);
    out_acc.duration = account.duration;
    out_acc.mduration = account.mduration;
    out_acc.benchmark_mark = account.benchmark_mark;
    out_acc.spread_rate = account.spread_rate;
    out_acc.treasury_glcode = account.treasury_glcode;
    out_acc.avg_mon_balance = account.avg_mon_balance;
    out_acc.deal_date = timestamp(account.deal_date);
    out_acc.cdr_flg = account.cdr_flg;
    out_acc.gl_sub_head_code = account.gl_sub_head_code;
    out_acc.group = account.group;
    out_acc.llg = account.llg;
    out_acc.other_llg_classification = account.other_llg_classification;
    out_acc.cashflow_type = account.cashflow_type;
    out_acc.cashflow_amount = account.cashflow_amount;
    out_acc.cashflow_currency = account.cashflow_currency;
    out_acc.cashflow_date = timestamp(account.cashflow_date);
    //Adjustment_cashflow
    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    let mut rem_cf = Cashflow::new();
    let mut all_cfs = protobuf::RepeatedField::from_vec(cashflows);
    rem_cf.prin_amt = out_acc.book_value - tot_prin_amt;
    rem_cf.date = out_acc.maturity_date;
    all_cfs.push(rem_cf);

    out_acc.cashflows = all_cfs;
    out_acc
}
