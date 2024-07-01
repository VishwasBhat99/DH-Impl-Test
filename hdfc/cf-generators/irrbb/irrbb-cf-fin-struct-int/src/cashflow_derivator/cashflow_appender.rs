use super::tenor_calculations::get_months;
use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use rbdate::DateParser;
use statics::*;

pub fn create_account_with_cashflows(
    input_account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut tot_int_amt = DEFAULT_FLOAT;
    let mut tot_prin_amt = DEFAULT_FLOAT;
    let mut out_acc = AccountWithCashflows::new();

    out_acc.account_number = input_account.account_number;
    out_acc.org_date = {
        if let Some(dt) = input_account.org_date {
            timestamp(dt)
        } else {
            DEFAULT_INT
        }
    };
    out_acc.branch = input_account.branch;
    out_acc.curr_code = input_account.curr_code;
    out_acc.intt_rate = input_account.intt_rate;
    out_acc.product_code = input_account.product_code;
    out_acc.mat_date = {
        if let Some(dt) = input_account.mat_date {
            timestamp(dt)
        } else {
            DEFAULT_INT
        }
    };
    out_acc.rate_flag = input_account.rate_flag;
    out_acc.repricing_index = {
        if input_account.repricing_index.is_empty() {
            "NA".to_string()
        } else {
            input_account.repricing_index
        }
    };
    out_acc.psl = input_account.psl;
    out_acc.npa = input_account.npa;
    out_acc.indv_corp_flag = input_account.indv_corp_flag;
    out_acc.customer_type = input_account.customer_type;
    out_acc.asset_class_id = input_account.asset_class_id;
    out_acc.customer_id = input_account.customer_id;
    out_acc.prod_type = input_account.prod_type;
    out_acc.final_int_rate = input_account.final_int_rate;
    out_acc.cost_centre = input_account.cost_centre;
    out_acc.alm_line = input_account.alm_line;
    out_acc.coa = input_account.coa;
    out_acc.division = input_account.division;
    out_acc.rep_freq = input_account.rep_freq;
    out_acc.next_repricing_date = {
        if let Some(dt) = input_account.next_repricing_date {
            timestamp(dt)
        } else {
            DEFAULT_INT
        }
    };
    out_acc.last_repricing_date = {
        if let Some(dt) = input_account.last_repricing_date {
            timestamp(dt)
        } else {
            DEFAULT_INT
        }
    };
    out_acc.asset_class = input_account.asset_class;
    out_acc.al_line = input_account.al_line;
    out_acc.balm_l2 = input_account.balm_l2;
    out_acc.bmid = input_account.bmid;
    out_acc.ia_line = input_account.ia_line;
    out_acc.scheme_id = input_account.scheme_id;
    out_acc.weaker_code = input_account.weaker_code;
    out_acc.der_int_rate = input_account.der_int_rate;
    out_acc.bnchmrk_rate = input_account.bnchmrk_rate;
    out_acc.spread = input_account.spread;
    out_acc.fully_floating_flg = input_account.fully_floating_flg;
    out_acc.gr_ofs_gl = input_account.gr_ofs_gl;
    out_acc.org_tenor = get_months(input_account.org_date, input_account.mat_date);

    //IRRBB logic start
    let date_parser = DateParser::new("%d-%m-%Y".to_string(), true);
    let as_on_dt = input_account
        .as_on_date
        .expect("Unable to parse As On Date from Input Account");
    let mut vec_of_cfs: Vec<Cashflow> = Vec::new();
    let mut prin_amt = 0.0;
    let mut highest_date = rbdate::timestamp(date_parser.parse("01-01-1099"));
    let mut broken_int = 0.0;
    let next_rep_date = rbdate::timestamp(input_account.next_repricing_date.unwrap_or(as_on_dt));

    let mut next_cf_date = timestamp(date_parser.parse("01-01-3099"));

    for cf in cashflows {
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
                .unwrap_or(input_account.org_date.unwrap_or(as_on_dt)),
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
    final_cf.date = rbdate::timestamp(input_account.next_repricing_date.unwrap_or(as_on_dt));
    vec_of_cfs.push(final_cf);
    //IRRBB logic end

    for cf in &vec_of_cfs {
        tot_int_amt += cf.interest_amount;
        tot_prin_amt += cf.principal_amount;
    }
    out_acc.total_interest_amount = tot_int_amt;
    out_acc.total_principal_amount = tot_prin_amt;
    out_acc.sma_flag = input_account.sma_flag;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(vec_of_cfs);
    out_acc
}
