use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use cashflow_generator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use cashflow_generator::cashflow_appender::create_account_with_cashflows;
use cashflow_generator::ei_biz::process_ei_loan;
use cashflow_generator::gen_cashflows::generate_cashflows;
use cashflow_generator::non_ei_biz::process_non_ei_loan;
use cashflow_generator::tenor::*;
use chrono::Duration;
use macros;
use rbdate::{incr_dt_by_mon_presrv_eom, increment_date_by_months, NaiveDate};
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::time::SystemTime;
pub fn process_ln3(
    mut loan_input: Vec<InputAccount>,
    as_on_date: NaiveDate,
    log: &Logger,
    diag_log: &Logger,
    account_with_cashflows_prin: &mut Vec<Cashflow>,
    account_with_cashflows_od: &mut Vec<Cashflow>,
    m_prvs_inst_date: &mut NaiveDate,
    m_prvs_end_date: &mut NaiveDate,
    g_cur_inst_date: &mut NaiveDate,
) {
    *m_prvs_inst_date = NaiveDate::from_ymd(1907, 1, 1);

    let mut early_date: NaiveDate = NaiveDate::from_ymd(1970, 1, 1);
    let mut prin_cf_vec: Vec<Cashflow> = Vec::new();
    let mut od_cf_vec: Vec<Cashflow> = Vec::new();
    let mut od_flag = "".to_string();

    if loan_input[0].end_date == NaiveDate::from_ymd(1970, 1, 1) {
        early_date = NaiveDate::from_ymd(3099, 1, 1)
    } else {
        early_date = NaiveDate::from_ymd(3099, 1, 1);
    }
    //Validating Clear Balance Amount
    if loan_input[0].clr_bal_amt > 0.0 {
        return;
    }

    if loan_input[0].dis_amt == 0.0 {
        loan_input[0].dis_amt = loan_input[0].sanct_lim;
    }

    if (loan_input[0].interest_rate_available.to_uppercase() == "FALSE")
        || (loan_input[0].dis_amt == 0.0)
    {
        loan_input[0].clr_bal_amt = loan_input[0].clr_bal_amt.abs();
        if loan_input[0].clr_bal_amt > 0.0 {
            //Store_Cashflow
            let cashflows = generate_cashflows(
                loan_input[0].clr_bal_amt,
                loan_input[0].clr_bal_amt,
                &as_on_date,
                &as_on_date,
            );
            account_with_cashflows_prin.push(cashflows);
            return;
        }
    }
    unsafe {
        if loan_input[0].rep_shdl_num == REPSHDLCONST {
            PEND_PRIN_AMT = loan_input[0].dis_amt;
        } else {
            PEND_PRIN_AMT = loan_input[0].rephasement_principal;
        }
        if PEND_PRIN_AMT == 0.0 {
            PEND_PRIN_AMT = loan_input[0].dis_amt;
        }
        if PEND_PRIN_AMT < loan_input[0].clr_bal_amt.abs() {
            PEND_PRIN_AMT = loan_input[0].clr_bal_amt.abs();
        }
        if loan_input[0].ei_schm_flag == "Y" {
            //Process as EI account.
            process_ei_loan(
                loan_input.to_owned(),
                as_on_date,
                account_with_cashflows_prin,
                account_with_cashflows_od,
                m_prvs_inst_date,
                m_prvs_end_date,
                g_cur_inst_date,
            );
        } else {
            //Process as NonEI account.
            process_non_ei_loan(
                loan_input.to_owned(),
                as_on_date,
                account_with_cashflows_prin,
                account_with_cashflows_od,
                m_prvs_inst_date,
                m_prvs_end_date,
                g_cur_inst_date,
            );
        }
    }
}

pub fn get_month_equivalent(freq: String) -> f64 {
    let mut month_equivalent: f64;
    month_equivalent = match freq.as_str() {
        "D" => 1.0 / 30.0,
        "W" => 7.0 / 30.0,
        "F" => 14.0 / 30.0,
        "M" => 1.0,
        "Q" => 3.0,
        "H" => 6.0,
        "Y" => 12.0,
        "B" => 1.0,
        _ => 1.0,
    };
    month_equivalent
}
pub fn calculate_si_by_months(osbal: f64, int_rate: f64, num_months: f64) -> f64 {
    (osbal * int_rate * num_months) / 1200.0
}

pub fn get_next_inst_date(prev_inst_date: NaiveDate, freq: String) -> NaiveDate {
    let nxt_inst_dt = match freq.as_str() {
        "D" => prev_inst_date + (Duration::days(1)),
        "W" => prev_inst_date + (Duration::days(7)),
        "F" => prev_inst_date + (Duration::days(14)),
        "M" => increment_date_by_months(prev_inst_date, 1),
        "Q" => increment_date_by_months(prev_inst_date, 3),
        "H" => increment_date_by_months(prev_inst_date, 6),
        "Y" => increment_date_by_months(prev_inst_date, 12),
        _ => increment_date_by_months(prev_inst_date, 1),
    };
    nxt_inst_dt
}

pub fn get_interest_for_accounts(
    ei_formula_flg: String,
    int_rate: f64,
    inst_freq: f64,
    prin_amt: f64,
    mut no_of_flows: i64,
    ei_method: String,
    intr_freq: String,
    flow_amt: f64,
    no_of_dmds: i64,
) -> f64 {
    let mut base = 0.0;
    let mut fraction = 0.0;
    let mut total_int = 0.0;
    let mut intr_amt = 0.0;
    let mut temp_ei_amt = 0.0;

    get_ei_amt(
        prin_amt,
        int_rate,
        no_of_flows,
        1.0,
        inst_freq,
        temp_ei_amt,
        ei_formula_flg.to_owned(),
        ei_method.to_owned(),
        intr_freq.to_owned(),
    );
    if (flow_amt - temp_ei_amt) > 0.0 {
        no_of_flows = 0;
        temp_ei_amt = flow_amt;
        get_ei_amt(
            prin_amt,
            int_rate,
            no_of_flows,
            1.0,
            inst_freq,
            temp_ei_amt,
            ei_formula_flg.to_owned(),
            ei_method,
            intr_freq,
        );
    }

    total_int = prin_amt * int_rate * (no_of_flows as f64 * inst_freq) / 1200.0;

    if ei_formula_flg == "R" {
        base = (no_of_flows * (no_of_flows + 1)) as f64 / 2.0;
        if base == 0.0 {
            return 0.0;
        }
        fraction = (no_of_flows - no_of_dmds) as f64 / base;
        intr_amt = fraction * total_int;
    } else {
        if no_of_flows == 0 {
            return 0.0;
        }
        intr_amt = total_int / no_of_flows as f64;
    }
    return intr_amt;
}

pub fn get_ei_amt(
    dis_amt: f64,
    int_rate: f64,
    mut no_of_flows: i64,
    ei_int_calc_freq: f64,
    mut lr_freq_type: f64,
    mut ei_amt: f64,
    ei_formula_flag: String,
    ei_method: String,
    intr_freq: String,
) {
    let mut temp_const = 0.0;
    let mut interest_adj_factor = 1;
    let mut interest_freq = 0.0;

    if intr_freq != "" {
        interest_freq = get_month_equivalent(intr_freq);
        if interest_freq > lr_freq_type {
            interest_adj_factor = (interest_freq / lr_freq_type) as i64;
            lr_freq_type = interest_freq;
            no_of_flows = no_of_flows / interest_adj_factor;
        }
    }

    if (no_of_flows == 0) & (ei_amt > 0.0) {
        if (ei_formula_flag == "R") | (ei_formula_flag == "F") {
            no_of_flows =
                (dis_amt / (ei_amt - ((dis_amt * int_rate * lr_freq_type) / 1200.0))) as i64;
        } else if ei_formula_flag == "M" {
            no_of_flows =
                (dis_amt * (2400.0 + int_rate) / (ei_amt * 2400.0 - dis_amt * int_rate)) as i64;
        } else {
            temp_const = 1.0 + (int_rate * ei_int_calc_freq * lr_freq_type) / 1200.0;
            let log_a = ((1.0 - (dis_amt * (temp_const.powf(1.0 / ei_int_calc_freq)) - 1.0) - 1.0)
                / ei_amt)
                .ln();
            let log_b = -1.0 * temp_const.ln();
            no_of_flows = (ei_int_calc_freq * (log_a / log_b)) as i64;
            if interest_adj_factor > 1 {
                no_of_flows = no_of_flows * interest_adj_factor;
            }
        }
    } else if no_of_flows > 0 {
        if (ei_formula_flag == "R") | (ei_formula_flag == "F") {
            ei_amt = (dis_amt * (1.0 + ((int_rate * lr_freq_type * no_of_flows as f64) / 1200.0)))
                / no_of_flows as f64;
        } else if ei_formula_flag == "M" {
            ei_amt = &(dis_amt / (2400 * no_of_flows) as f64)
                * (2400.0 + no_of_flows as f64 * int_rate + int_rate);
        } else if int_rate == 0.0 {
            ei_amt = dis_amt * no_of_flows as f64;
        } else {
            temp_const = 1.0 + (int_rate * ei_int_calc_freq * lr_freq_type) / 1200.0;
            ei_amt = (dis_amt * temp_const.powf(1.0 / ei_int_calc_freq) - 1.0)
                / (1.0 - temp_const.powf(-(no_of_flows as f64 / lr_freq_type)));
        }
    }

    if ei_method == "A" {
        temp_const = 1.0 + ((int_rate * lr_freq_type) / 1200.0);
        ei_amt = ei_amt / temp_const;
    }
}
