use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use cashflow_generator::cashflow_appenders::create_account_with_cashflows;
use cashflow_generator::gen_cashflows::generate_cashflows;
use chrono::Duration;
use rbdate::{increment_date_by_months, num_days_start_to_end, NaiveDate};
use slog::Logger;

pub fn process_td1(
    input_rcrds: Vec<InputAccount>,
    as_on_date: NaiveDate,
    calc_ir_from_ason: String,
    _log: &Logger,
    _diag_log: &Logger,
    account_with_cashflows: &mut Vec<AccountWithCashflows>,
    round_off_ex_rt: i64,
) {
    let mut intr_inst_amt: f64;
    let mut cashflows: Vec<Cashflow> = Vec::new();
    let lst_installment_dts: Vec<NaiveDate> =
        gen_installment_dts(input_rcrds.to_owned(), as_on_date, calc_ir_from_ason);

    if !lst_installment_dts.is_empty() {
        for item in 0..(lst_installment_dts.len() - 1) {
            intr_inst_amt = (calculate_si_by_days(
                input_rcrds[0].amount,
                input_rcrds[0].intrrt,
                num_days_start_to_end(lst_installment_dts[item], lst_installment_dts[item + 1]),
            ) * 100.0)
                .round()
                / 100.0;
            cashflows.push(generate_cashflows(
                0.0,
                intr_inst_amt,
                lst_installment_dts[item + 1],
            ));
        }
    }
    cashflows.push(generate_cashflows(
        input_rcrds[0].amount,
        0.0,
        input_rcrds[0].maturitydt,
    ));
    account_with_cashflows.push(create_account_with_cashflows(
        input_rcrds[0].to_owned(),
        cashflows,
        round_off_ex_rt,
    ));
}

pub fn gen_installment_dts(
    input_rcrds: Vec<InputAccount>,
    as_on_dt: NaiveDate,
    calc_ir: String,
) -> Vec<NaiveDate> {
    let mut cur_inst_dt;
    let mut processing_over = false;
    let calc_ir_from_as_on;
    let mut prvs_inst_dt = input_rcrds[0].startdt;
    let mut lst_inst_dt: Vec<NaiveDate> = Vec::new();

    let mut lst_temp_inst_dts;

    if input_rcrds[0].maturitydt > as_on_dt {
        cur_inst_dt = get_next_inst_date(
            prvs_inst_dt,
            input_rcrds[0].intrcompfreq.to_owned(),
            input_rcrds[0].maturitydt.to_owned(),
        );
        if cur_inst_dt > as_on_dt {
            lst_inst_dt.push(prvs_inst_dt);
        }

        while !processing_over {
            cur_inst_dt = get_next_inst_date(
                prvs_inst_dt,
                input_rcrds[0].intrcompfreq.to_owned(),
                input_rcrds[0].maturitydt,
            );
            // To make maturity date as last installment date
            if cur_inst_dt >= input_rcrds[0].maturitydt {
                cur_inst_dt = input_rcrds[0].maturitydt;
                processing_over = true;
            }
            lst_inst_dt.push(cur_inst_dt);
            prvs_inst_dt = cur_inst_dt;
        }
        calc_ir_from_as_on = calc_ir;
        // If the Intereset calc_ir_from_as_on is N or calc_ir_from_as_on key is not present, then "lst_inst_dt:Vec<NaiveDate>" will have all the dates from account start date to maturity date.
        // If the Intereset calc_ir_from_as_on is Y, then "lst_inst_dt:Vec<NaiveDate>" will have all the dates which are greater than ason date to maturity date.
        if !calc_ir_from_as_on.is_empty() && calc_ir_from_as_on.to_uppercase() == *"Y" {
            // To add only one record with ason date
            lst_inst_dt.retain(|&x| !(x <= as_on_dt));
            lst_inst_dt.insert(0, as_on_dt);
            lst_inst_dt.sort();
        } else {
            lst_temp_inst_dts = lst_inst_dt.clone();

            lst_temp_inst_dts.retain(|&x| !(x > as_on_dt));
            if !lst_temp_inst_dts.is_empty() {
                lst_inst_dt.retain(|&x| !(x < lst_temp_inst_dts[lst_temp_inst_dts.len() - 1]));
            }
        }
    }
    lst_inst_dt
}

pub fn get_next_inst_date(
    prev_inst_date: NaiveDate,
    freq: String,
    maturity_date: NaiveDate,
) -> NaiveDate {
    let nxt_inst_dt = match freq.as_str() {
        "D" => prev_inst_date + (Duration::days(1)),
        "W" => prev_inst_date + (Duration::days(7)),
        "F" => prev_inst_date + (Duration::days(14)),
        "M" | "1" => increment_date_by_months(prev_inst_date, 1),
        "Q" | "2" => increment_date_by_months(prev_inst_date, 3),
        "H" | "3" => increment_date_by_months(prev_inst_date, 6),
        "Y" | "4" => increment_date_by_months(prev_inst_date, 12),
        _ => maturity_date,
    };
    nxt_inst_dt
}

pub fn calculate_si_by_days(os_bal: f64, int_rate: f64, days_count: i64) -> f64 {
    (os_bal * int_rate * days_count as f64) / 36500.0
}
