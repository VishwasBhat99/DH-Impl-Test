use process::account_reader::input_account::InputAccount;
use process::account_with_cashflows::Account;
use rbdate::NaiveDate;

pub fn append_cf_data(out_acc: &mut Account, account_data: &InputAccount, _ason: NaiveDate) {
    out_acc.key_1 = account_data.key_1.to_string();
    out_acc.br_no = account_data.br_no.to_string();
    out_acc.act_type = account_data.act_type.to_string();
    out_acc.purpose_code_a = account_data.purpose_code_a.to_string();
    out_acc.applic_amount = account_data.applic_amount;
    out_acc.repay_count = account_data.repay_count;
    out_acc.repay_day = account_data.repay_day;
    out_acc.repay_freq = account_data.repay_freq;
    out_acc.app_amt = account_data.app_amt;
    out_acc.loan_bal = account_data.loan_bal;
    out_acc.adv_bal = account_data.adv_bal;
    out_acc.theo_loan_bal = account_data.theo_loan_bal;
    out_acc.loan_repay = account_data.loan_repay;
    out_acc.pend_dues = account_data.pend_dues;
    out_acc.apprv_date = match account_data.apprv_date {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
    out_acc.lst_fin_date = match account_data.lst_fin_date {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
    out_acc.lst_arr_date = match account_data.lst_arr_date {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
    out_acc.pend_dues_date = match account_data.pend_dues_date {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
    out_acc.int_rate = account_data.int_rate;
    out_acc.cat = account_data.cat.to_string();
    out_acc.loan_trm = account_data.loan_trm;
    out_acc.bad_debt_ind = account_data.bad_debt_ind.to_string();
    out_acc.arr_int_accr = account_data.arr_int_accr;
    out_acc.arr_int_incr = account_data.arr_int_incr;
    out_acc.rt_incr = account_data.rt_incr;
    out_acc.customer_no = account_data.customer_no.to_string();
    out_acc.currency_ind = account_data.currency_ind.to_string();
    out_acc.store_rate = account_data.store_rate;
    out_acc.cr_rating = account_data.cr_rating.to_string();
    out_acc.gl_class_code = account_data.gl_class_code.to_string();
    out_acc.theo_unpd_arrs_int = account_data.theo_unpd_arrs_int;
    out_acc.security_amount = account_data.security_amount;
    out_acc.last_credit_dt = match account_data.last_credit_dt {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
    out_acc.old_bad_debt_ind = account_data.old_bad_debt_ind.to_string();
    out_acc.npa_date = match account_data.npa_date {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
    out_acc.collection_amt = account_data.collection_amt;
    out_acc.provision_amount = account_data.provision_amount;
    out_acc.last_repriced_date = match account_data.last_repriced_date {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
    out_acc.next_repriced_date = match account_data.next_repriced_date {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
    out_acc.repricing_frequency = account_data.repricing_frequency.to_string();
    out_acc.inca = account_data.inca.to_string();
    out_acc.rating_source = account_data.rating_source.to_string();
    out_acc.rating_code = account_data.rating_code.to_string();
    out_acc.benchmark = account_data.benchmark.to_string();
    out_acc.name = account_data.name.to_string();
    out_acc.cust_acct_no = account_data.cust_acct_no.to_string();
    out_acc.prim_acct = account_data.prim_acct.to_string();
    out_acc.segment_code = account_data.segment_code.to_string();
    out_acc.industry_code = account_data.industry_code.to_string();
    out_acc.grup_code = account_data.grup_code.to_string();
    out_acc.bus_sector_code = account_data.bus_sector_code.to_string();
    out_acc.tier_cust_type = account_data.tier_cust_type.to_string();
    out_acc.a1 = account_data.a1;
    out_acc.a2 = account_data.a2;
    out_acc.a3 = account_data.a3;
    out_acc.a4 = account_data.a4.to_string();
    out_acc.a5 = account_data.a5.to_string();
    out_acc.a6 = match account_data.a6 {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
    out_acc.a7 = account_data.a7.to_string();
    out_acc.a8 = account_data.a8.to_string();
    out_acc.a9 = account_data.a9.to_string();
    out_acc.a10 = account_data.a10.to_string();
    out_acc.asondate = match account_data.asondate {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
    out_acc.mat_dt = match account_data.mat_dt {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
}
