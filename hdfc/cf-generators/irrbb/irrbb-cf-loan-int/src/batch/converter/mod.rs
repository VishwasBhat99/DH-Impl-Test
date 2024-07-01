use macros;
use rbdate;
use rbdate::timestamp;
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
use rbdate::DateParser;
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
    let total_no_inst = input_account.orig_term;
    let paid_inst = input_account.num_inst_paid;
    let mut pendng_inst = total_no_inst - paid_inst;
    let cf_freq = match &input_account.interest_pay_freq[..] {
        "Monthly" => 1,
        "Bi-Monthly" => 2,
        "Quarterly" => 3,
        "Half Yearly" => 6,
        "Yearly" => 12,
        _ => {
            log_warn!(
                log,
                "Payment frequency '{}' is incorrect for account: {}. Using Default Pay Freq: Monthly",
                input_account.interest_pay_freq, input_account.account_number
            );
            1
        }
    };
    let mat_dt = if input_account.mat_date.is_some() {
        input_account
            .mat_date
            .expect("Unexpected unwrap error on maturity date.")
    } else {
        as_on_date
    };
    let date_parser = DateParser::new("%d-%m-%Y".to_string(), true);
    let mut last_pay_date = {
        if input_account.last_inst_date.is_some() {
            input_account.last_inst_date.unwrap()
        } else {
            // log as error : Cannot parse last installment date for account: {}, hence use next due date as last put date.
            if input_account.due_date.is_some() {
                let next_due_date = input_account.due_date.unwrap();
                rbdate::decr_dt_by_mon_presrv_eom(next_due_date, cf_freq)
                    .expect("Cannot get start date for 1st installment")
            } else {
                log_warn!(
                    log,
                    "Cannot get last installment date for account: {}, Using Maturity as Next Payment.",
                    input_account.account_number
                );
                rbdate::decr_dt_by_mon_presrv_eom(mat_dt, cf_freq)
                    .expect("Cannot get last pay date using maturity date.")
            }
        }
    };
    let mut outstanding_amount = input_account.current_book_balance;
    let cf_st_date;
    if is_contractual {
        cf_st_date = rbdate::decr_dt_by_mon_presrv_eom(
            input_account.first_inst_date.unwrap_or(as_on_date),
            cf_freq,
        )
        .expect("Cannot decr month to first inst date.");
        pendng_inst = total_no_inst;
        outstanding_amount = input_account.original_balance;
        last_pay_date = rbdate::decr_dt_by_mon_presrv_eom(
            input_account.first_inst_date.unwrap_or(as_on_date),
            cf_freq * 2,
        )
        .expect("Cannot decr month to first inst date.");
    } else {
        cf_st_date = last_pay_date;
    }
    let emi_amount: f64;
    if input_account.emi == DEFAULT_FLOAT {
        let present_value: f64 = input_account.current_book_balance;
        let rate: f64 = input_account.final_int_rate / 100.0;
        let num_of_insts: f64 = input_account.orig_term as f64;
        emi_amount = if num_of_insts > 0.0 && rate > 0.0 {
            cal_emi_amount(present_value, rate, num_of_insts)
        } else {
            present_value
        };
        pendng_inst = input_account.orig_term;
    } else {
        emi_amount = input_account.emi;
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
                input_account.account_number
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
            input_account.intt_rate,
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
    if (total_principal_output - input_account.current_book_balance) > 0.1 {
        let mismatched_amounts_error_string =
            format!(
            "Total principal amount calculated doesn't match outstanding amount for Account: {}\
             PrincipalAmount: {}, OutstandingAmount: {}",
            input_account.account_number, total_principal_output, input_account.current_book_balance
        );
        if !is_contractual {
            log_warn!(
                log,
                "Mismatch occured while calculating cashflows: {}",
                mismatched_amounts_error_string
            );
        }
    }

    //IRRBB logic start

    let mut vec_of_cfs: Vec<Cashflow> = Vec::new();
    let mut prin_amt = 0.0;
    let mut highest_date = rbdate::timestamp(date_parser.parse("01-01-1099"));
    let mut broken_int = 0.0;
    let next_rep_date = rbdate::timestamp(input_account.next_repricing_date.unwrap_or(as_on_date));

    let mut next_cf_date = timestamp(date_parser.parse("01-01-3099"));

    for cf in cfs_vec {
        if cf.date >= next_rep_date {
            prin_amt += cf.principal_amount;
            if cf.interest_amount > 0.0 && cf.date <= next_cf_date {
                if next_cf_date == cf.date {
                    broken_int += cf.interest_amount;
                } else {
                    broken_int = cf.interest_amount;
                }
                next_cf_date = cf.date;
            }
            continue;
        }
        if cf.date > highest_date && cf.interest_amount > 0.0 {
            highest_date = cf.date;
        }
        vec_of_cfs.push(cf);
    }
    let mut final_cf = Cashflow::new();
    //cf int amt
    if highest_date == timestamp(date_parser.parse("01-01-1099")) {
        let last_cf_date = rbdate::timestamp(
            input_account
                .last_inst_date
                .unwrap_or(input_account.org_date.unwrap_or(as_on_date)),
        );
        if last_cf_date > next_rep_date {
            final_cf.interest_amount = broken_int;
        } else {
            let num_of_days = rbdate::num_days_start_to_end(
                rbdate::date_from_timestamp(last_cf_date),
                rbdate::date_from_timestamp(next_rep_date),
            );
            let num_of_days_last_to_next = rbdate::num_days_start_to_end(
                rbdate::date_from_timestamp(last_cf_date),
                rbdate::date_from_timestamp(next_cf_date),
            );
            final_cf.interest_amount =
                (num_of_days as f64 * broken_int) / num_of_days_last_to_next as f64;
        }
    } else {
        if next_rep_date == highest_date {
            final_cf.interest_amount = broken_int;
        } else {
            let num_of_days = rbdate::num_days_start_to_end(
                rbdate::date_from_timestamp(highest_date),
                rbdate::date_from_timestamp(next_rep_date),
            );
            let num_of_days_last_to_next = rbdate::num_days_start_to_end(
                rbdate::date_from_timestamp(highest_date),
                rbdate::date_from_timestamp(next_cf_date),
            );
            final_cf.interest_amount =
                (num_of_days as f64 * broken_int) / num_of_days_last_to_next as f64;
        }
    }
    final_cf.principal_amount = prin_amt;
    final_cf.date = rbdate::timestamp(input_account.next_repricing_date.unwrap_or(as_on_date));
    vec_of_cfs.push(final_cf);
    //IRRBB logic end

    let cashflows = protobuf::RepeatedField::from_vec(vec_of_cfs);
    let cashflows_count = cashflows.len() as u64;

    let mut op = Account::new();
    op.account_number = input_account.account_number;
    op.curr_code = input_account.curr_code;
    op.intt_rate = input_account.intt_rate;
    op.product_code = input_account.product_code;
    op.mat_date = {
        if input_account.mat_date.is_some() {
            rbdate::timestamp(input_account.mat_date.unwrap())
        } else {
            0
        }
    };
    op.rate_flag = input_account.rate_flag;
    op.repricing_index = {
        if input_account.repricing_index == "" {
            "NA".to_string()
        } else {
            input_account.repricing_index
        }
    };
    op.psl = input_account.psl;
    op.npa = input_account.npa;
    op.indv_corp_flag = input_account.indv_corp_flag;
    op.customer_type = input_account.customer_type;
    op.asset_class_id = input_account.asset_class_id;
    op.customer_id = input_account.customer_id;
    op.prod_type = input_account.prod_type;
    op.final_int_rate = input_account.final_int_rate;
    op.cost_centre = input_account.cost_centre;
    op.alm_line = input_account.alm_line;
    op.coa = input_account.coa;
    op.division = input_account.division;
    op.rep_freq = input_account.rep_freq;
    op.next_repricing_date = {
        if input_account.next_repricing_date.is_some() {
            rbdate::timestamp(input_account.next_repricing_date.unwrap())
        } else {
            0
        }
    };
    op.last_repricing_date = {
        if input_account.last_repricing_date.is_some() {
            rbdate::timestamp(input_account.last_repricing_date.unwrap())
        } else {
            0
        }
    };
    op.set_asset_class(input_account.asset_class);
    op.value_date = {
        if input_account.org_date.is_some() {
            rbdate::timestamp(input_account.org_date.unwrap())
        } else {
            0
        }
    };
    op.set_branch(input_account.branch);
    let org_tenor = get_months(input_account.org_date, input_account.mat_date);
    let rep_tenor = get_months(
        input_account.next_repricing_date,
        input_account.last_repricing_date,
    );
    op.set_org_tenor(org_tenor);
    op.set_rep_tenor(rep_tenor);
    op.set_weaker(input_account.weaker);
    op.set_current_book_bal(input_account.current_book_balance);
    op.set_al_line(input_account.al_line);
    op.set_balm_l2(input_account.balm_l2);
    op.set_ia_line(input_account.ia_line);
    op.set_shceme_id(input_account.scheme_id.clone());
    op.set_orig_bm(input_account.orig_bm);
    op.set_der_int_rate(input_account.der_int_rate);
    op.set_bnchmrk_rate(input_account.bnchmrk_rate);
    op.set_spread(input_account.spread);
    op.set_fully_floating_flg(input_account.fully_floating_flg);
    op.customer_name = input_account.customer_name;
    op.orig_bal = input_account.original_balance;
    op.gr_ofs_gl = input_account.gr_ofs_gl;
    op.set_scheme_id(input_account.scheme_id);
    op.set_is_ofs_gl(input_account.is_ofs_gl);
    op.set_re_ofs_gl(input_account.re_ofs_gl);
    op.set_ui_ofs_gl(input_account.ui_ofs_gl);
    op.set_gr_dr(input_account.gr_dr);
    op.set_gr_cr(input_account.gr_cr);
    op.set_re_dr(input_account.re_dr);
    op.set_re_cr(input_account.re_cr);
    op.set_is_dr(input_account.is_dr);
    op.set_is_cr(input_account.is_cr);
    op.set_ui_dr(input_account.ui_dr);
    op.set_ui_cr(input_account.ui_cr);
    op.set_weaker_desc(input_account.weaker_desc);
    op.set_int_income_gl(input_account.int_income_gl);
    op.set_overdue_int_gl(input_account.overdue_int_gl);
    op.set_int_on_cancellation_gl(input_account.int_on_cancellation_gl);
    op.set_writeoff_gl(input_account.writeoff_gl);
    op.set_int_income_gl_amt(input_account.int_income_gl_amt);
    op.set_overdue_int_gl_amt(input_account.overdue_int_gl_amt);
    op.set_int_on_cancellation_gl_amt(input_account.int_on_cancellation_gl_amt);
    op.set_writeoff_gl_amt(input_account.writeoff_gl_amt);
    op.sma_flag = input_account.sma_flag;
    op.set_cashflows(cashflows);

    let cd = AccountDescriptor {
        cashflows_count,
        total_amount_input: input_account.current_book_balance,
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
