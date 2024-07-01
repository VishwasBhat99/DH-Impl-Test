use macros;
use rbdate;
pub mod account;
pub mod input;
pub mod tenor_calculations;

use self::account::Account;
use self::account::Cashflow;
use self::tenor_calculations::get_months;
use super::super::statics::DEFAULT_FLOAT;
use super::output_descriptor::AccountDescriptor;
use chrono::Datelike;
use protobuf;
use rbdate::NaiveDate;
use sdb_day_convention::conventions::Conventions;
use sdb_day_convention::days_with_convn;
use slog::Logger;

pub fn convert(
    input_account: input::Input,
    convention: Conventions,
    as_on_date: NaiveDate,
    is_contractual: bool,
    log: &Logger,
) -> Result<(Account, AccountDescriptor), String> {
    let total_no_inst = input_account.loan_trm;
    let paid_inst = 0; //number of paid installments
    let mut pendng_inst = total_no_inst - paid_inst;
    let cf_freq = match &input_account.repay_freq[..] {
        "Monthly" => 1,
        "Bi-Monthly" => 2,
        "Quarterly" => 4,
        "Half Yearly" => 6,
        "Yearly" => 12,
        _ => {
            log_warn!(
                log,
                "Payment frequency '{}' is incorrect for account: {}. Using Default Pay Freq: Monthly",
                input_account.repay_freq, input_account.cust_acct_no
            );
            1
        }
    };
    let mat_dt = if input_account.mat_dt.is_some() {
        input_account
            .mat_dt
            .expect("Unexpected unwrap error on maturity date.")
    } else {
        as_on_date
    };
    let mut last_pay_date = {
        if input_account.lst_fin_date.is_some() {
            input_account.lst_fin_date.unwrap()
        } else {
            // log as error : Cannot parse last installment date for account: {}, hence use next due date as last put date.
            if input_account.lst_fin_date.is_some() {
                let next_due_date = input_account.lst_fin_date.unwrap();
                rbdate::decr_dt_by_mon_presrv_eom(next_due_date, cf_freq)
                    .expect("Cannot get start date for 1st installment")
            } else {
                log_warn!(
                    log,
                    "Cannot get last installment date for account: {}, Using Maturity as Next Payment.",
                    input_account.key_1
                );
                rbdate::decr_dt_by_mon_presrv_eom(mat_dt, cf_freq)
                    .expect("Cannot get last pay date using maturity date.")
            }
        }
    };
    let mut outstanding_amount = input_account.loan_bal;
    let cf_st_date;
    if is_contractual {
        cf_st_date = rbdate::decr_dt_by_mon_presrv_eom(
            input_account.apprv_date.unwrap_or(as_on_date),
            cf_freq,
        )
        .expect("Cannot decr month to first inst date.");
        pendng_inst = total_no_inst;
        outstanding_amount = input_account.adv_bal;
        last_pay_date = rbdate::decr_dt_by_mon_presrv_eom(
            input_account.apprv_date.unwrap_or(as_on_date),
            cf_freq * 2,
        )
        .expect("Cannot decr month to first inst date.");
    } else {
        cf_st_date = last_pay_date;
    }
    let emi_amount: f64;
    if input_account.loan_repay == DEFAULT_FLOAT {
        let present_value: f64 = input_account.loan_bal;
        let rate: f64 = input_account.store_rate / 100.0;
        let num_of_insts: f64 = input_account.loan_trm as f64;
        emi_amount = if num_of_insts > 0.0 && rate > 0.0 {
            cal_emi_amount(present_value, rate, num_of_insts)
        } else {
            present_value
        };
        pendng_inst = input_account.loan_trm;
    } else {
        emi_amount = input_account.loan_repay;
    }
    // `15` is a rough guess at the average. Some cashflows will be less, some more.
    let mut cfs_vec = Vec::with_capacity(15);

    let mut total_interest_output = 0.0;
    let mut total_principal_output = 0.0;

    if outstanding_amount <= DEFAULT_FLOAT {
        let cf_dt_timestmp = rbdate::timestamp(as_on_date);
        let cf = new_cashflow(cf_dt_timestmp, outstanding_amount, DEFAULT_FLOAT);
        cfs_vec.push(cf);
        total_principal_output += outstanding_amount;
    }
    let mut month_to_incr = cf_freq;
    while pendng_inst > 0 && outstanding_amount > 0.0 {
        let cf_date = rbdate::incr_dt_by_mon_presrv_eom(cf_st_date, month_to_incr);
        let next_cf_date;
        if cf_date.is_none() {
            return Err(format!(
                "Cannot calculate next cf date for account: {}",
                input_account.key_1
            ));
        } else {
            next_cf_date = cf_date.expect("Error unwrapping the next cf date.");
            if next_cf_date > mat_dt {
                break;
            }
        }
        let int_amt = cal_int_amt(
            last_pay_date,
            next_cf_date,
            outstanding_amount,
            input_account.store_rate,
            convention,
        );
        let mut prin_amt = emi_amount - int_amt;
        if prin_amt < 0.0 {
            prin_amt = 0.0;
        }
        if outstanding_amount < prin_amt {
            prin_amt = outstanding_amount;
        }
        let cf_dt_timestmp = rbdate::timestamp(next_cf_date);
        let cf = new_cashflow(cf_dt_timestmp, prin_amt, int_amt);
        cfs_vec.push(cf);
        total_interest_output += int_amt;
        total_principal_output += prin_amt;
        outstanding_amount -= prin_amt;
        last_pay_date = next_cf_date;
        month_to_incr += cf_freq;
        pendng_inst -= 1;
    }
    if outstanding_amount > 0.0 {
        let cf_dt_timestmp = rbdate::timestamp(mat_dt);
        let cf = new_cashflow(cf_dt_timestmp, outstanding_amount, 0.0);
        cfs_vec.push(cf);
        total_principal_output += outstanding_amount;
    }
    if (total_principal_output - input_account.loan_bal) > 0.1 {
        let mismatched_amounts_error_string = format!(
            "Total principal amount calculated doesn't match outstanding amount for Account: {}\
             PrincipalAmount: {}, OutstandingAmount: {}",
            input_account.key_1, total_principal_output, input_account.loan_bal
        );
        if !is_contractual {
            log_warn!(
                log,
                "Mismatch occured while calculating cashflows: {}",
                mismatched_amounts_error_string
            );
        }
    }
    let cashflows = protobuf::RepeatedField::from_vec(cfs_vec);
    let cashflows_count = cashflows.len() as u64;

    let mut op = Account::new();
    op.key_1 = input_account.key_1;
    op.br_no = input_account.br_no;
    op.act_type = input_account.act_type;
    op.purpose_code_a = input_account.purpose_code_a;
    op.applic_amount = input_account.applic_amount;
    op.repay_count = input_account.repay_count;
    op.repay_day = input_account.repay_day;
    op.repay_freq = input_account.repay_freq;
    op.app_amt = input_account.app_amt;
    op.loan_bal = input_account.loan_bal;
    op.adv_bal = input_account.adv_bal;
    op.theo_loan_bal = input_account.theo_loan_bal;
    op.loan_repay = input_account.loan_repay;
    op.pend_dues = input_account.pend_dues;
    op.apprv_date = {
        if input_account.apprv_date.is_some() {
            rbdate::timestamp(input_account.apprv_date.unwrap())
        } else {
            0
        }
    };
    op.lst_fin_date = {
        if input_account.lst_fin_date.is_some() {
            rbdate::timestamp(input_account.lst_fin_date.unwrap())
        } else {
            0
        }
    };
    op.lst_arr_date = {
        if input_account.lst_arr_date.is_some() {
            rbdate::timestamp(input_account.lst_arr_date.unwrap())
        } else {
            0
        }
    };
    op.int_rate = input_account.int_rate;
    op.cat = input_account.cat;
    op.loan_trm = input_account.loan_trm;
    op.bad_debt_ind = input_account.bad_debt_ind;
    op.arr_int_accr = input_account.arr_int_accr;
    op.arr_int_incr = input_account.arr_int_incr;
    op.rt_incr = input_account.rt_incr;
    op.customer_no = input_account.customer_no;
    op.currency_ind = input_account.currency_ind;
    op.store_rate = input_account.store_rate;
    op.cr_rating = input_account.cr_rating;
    op.gl_class_code = input_account.gl_class_code;
    op.theo_unpd_arrs_int = input_account.theo_unpd_arrs_int;
    op.security_amount = input_account.security_amount;
    op.last_credit_dt = input_account.last_credit_dt;
    op.old_bad_debt_ind = input_account.old_bad_debt_ind;
    op.npa_date = {
        if input_account.npa_date.is_some() {
            rbdate::timestamp(input_account.npa_date.unwrap())
        } else {
            0
        }
    };
    op.collection_amt = input_account.collection_amt;
    op.provision_amount = input_account.provision_amount;
    op.last_repriced_date = {
        if input_account.last_repriced_date.is_some() {
            rbdate::timestamp(input_account.last_repriced_date.unwrap())
        } else {
            0
        }
    };
    op.next_repriced_date = {
        if input_account.next_repriced_date.is_some() {
            rbdate::timestamp(input_account.next_repriced_date.unwrap())
        } else {
            0
        }
    };
    op.repricing_frequency = input_account.repricing_frequency;
    op.inca = input_account.inca;
    op.rating_source = input_account.rating_source;
    op.rating_code = input_account.rating_code;
    op.benchmark = input_account.benchmark;
    op.name = input_account.name;
    op.cust_acct_no = input_account.cust_acct_no;
    op.prim_acct = input_account.prim_acct;
    op.segment_code = input_account.segment_code;
    op.industry_code = input_account.industry_code;
    op.grup_code = input_account.grup_code;
    op.bus_sector_code = input_account.bus_sector_code;
    op.tier_cust_type = input_account.tier_cust_type;
    op.a1 = input_account.a1;
    op.a2 = input_account.a2;
    op.a3 = input_account.a3;
    op.a4 = {
        if input_account.a4.is_some() {
            rbdate::timestamp(input_account.a4.unwrap())
        } else {
            0
        }
    };

    op.a5 = {
        if input_account.a5.is_some() {
            rbdate::timestamp(input_account.a5.unwrap())
        } else {
            0
        }
    };
    op.a6 = {
        if input_account.a6.is_some() {
            rbdate::timestamp(input_account.a6.unwrap())
        } else {
            0
        }
    };
    op.a7 = input_account.a7;
    op.a8 = input_account.a8;
    op.a9 = input_account.a9;
    op.a10 = input_account.a10;
    op.asondate = {
        if input_account.asondate.is_some() {
            rbdate::timestamp(input_account.asondate.unwrap())
        } else {
            0
        }
    };
    op.mat_dt = {
        if input_account.mat_dt.is_some() {
            rbdate::timestamp(input_account.mat_dt.unwrap())
        } else {
            0
        }
    };
    let org_tenor = get_months(input_account.apprv_date, input_account.mat_dt);
    let rep_tenor = get_months(
        input_account.next_repriced_date,
        input_account.last_repriced_date,
    );
    op.set_cashflows(cashflows);

    let cd = AccountDescriptor {
        cashflows_count,
        total_amount_input: input_account.loan_bal,
        total_principal_output: total_principal_output,
        total_interest_output: total_interest_output,
    };
    Ok((op, cd))
}

fn new_cashflow(d: i64, p: f64, i: f64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.set_date(d);
    cf.set_principal_amount(p);
    cf.set_interest_amount(i);

    cf
}

fn cal_int_amt(
    last_pay_date: NaiveDate,
    next_cf_date: NaiveDate,
    outstanding_amount: f64,
    int_rate: f64,
    convention: Conventions,
) -> f64 {
    let days = days_with_convn(last_pay_date, next_cf_date, &convention).unwrap();
    let no_of_days = days.days_btw_dts as f64;
    let days_in_yr = days.day_in_yr as f64;
    let int_amt = (outstanding_amount * int_rate * no_of_days) / (days_in_yr * 100.0);

    int_amt
}

fn cal_emi_amount(present_value: f64, mut rate: f64, num_of_insts: f64) -> f64 {
    // EMI Amount = PV*(Rate*(1+Rate)^N)/((1+Rate)^N-1)
    // Rate is divided by 12 as it is monthly
    rate /= 12.0;
    present_value
        * (rate * (1.0 + rate).powf(num_of_insts) / ((1.0 + rate).powf(num_of_insts) - 1.0))
}
