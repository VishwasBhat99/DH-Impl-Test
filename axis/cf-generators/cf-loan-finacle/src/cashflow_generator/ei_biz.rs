use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use cashflow_generator::cashflow_appender::create_account_with_cashflows;
use cashflow_generator::gen_cashflows::generate_cashflows;
use cashflow_generator::ln3_biz::*;
use cashflow_generator::tenor::*;
use statics::*;
use std::collections::HashMap;

use rbdate::{num_days_start_to_end, NaiveDate};

pub fn process_ei_loan(
    mut loan_input: Vec<InputAccount>,
    as_on_date: NaiveDate,
    account_with_cashflows_prin: &mut Vec<Cashflow>,
    account_with_cashflows_od: &mut Vec<Cashflow>,
    m_prvs_inst_date: &mut NaiveDate,
    m_prvs_end_date: &mut NaiveDate,
    g_cur_inst_date: &mut NaiveDate,
) {
    let mut prin_cf_vec: Vec<Cashflow> = Vec::new();
    let mut od_cf_vec: Vec<Cashflow> = Vec::new();
    let mut mprev_end_date = NaiveDate::from_ymd(1970, 1, 1);
    let mut od_flag = "".to_string();
    if loan_input[0].ei_int_calc_freq.is_empty()
        || loan_input[0].ei_int_calc_freq.to_uppercase() == "NULL"
    {
        let cashflows = generate_cashflows(
            loan_input[0].clr_bal_amt.abs(),
            0.0,
            &as_on_date,
            &as_on_date,
        );

        account_with_cashflows_prin.push(cashflows);
        return;
    }

    let mut loan_input_first = loan_input[0].clone();
    let mut final_dt = loan_input[0].flow_start_date;

    let mut final_clr_bal = if loan_input[0].clr_bal_amt.abs() > loan_input[0].out_bal_amount.abs()
    {
        loan_input[0].out_bal_amount.abs()
    } else {
        loan_input[0].clr_bal_amt.abs()
    };
    let mut first_clr_bal = if loan_input[0].clr_bal_amt.abs() > loan_input[0].out_bal_amount.abs()
    {
        loan_input[0].out_bal_amount.abs()
    } else {
        loan_input[0].clr_bal_amt.abs()
    };

    for (_index, loan_input) in loan_input.iter_mut().enumerate() {
        //Generate the EI cashflow for all accounts
        generate_ei_cashflow(
            loan_input_first.clone(),
            loan_input.clone(),
            as_on_date,
            account_with_cashflows_prin,
            account_with_cashflows_od,
            m_prvs_inst_date,
            m_prvs_end_date,
            g_cur_inst_date,
            &mut final_clr_bal,
            &mut first_clr_bal,
        );
        final_dt = g_cur_inst_date.to_owned();
    }

    if first_clr_bal > 0.0 {
        let cashflows = generate_cashflows(
            (first_clr_bal * 100.0).round() / 100.0,
            0.0,
            m_prvs_end_date,
            &final_dt,
        );

        account_with_cashflows_prin.push(cashflows);
    }
}

pub fn generate_ei_cashflow(
    mut loan_input: InputAccount,
    lrs_input: InputAccount,
    as_on_date: NaiveDate,
    account_with_cashflows_prin: &mut Vec<Cashflow>,
    account_with_cashflows_od: &mut Vec<Cashflow>,
    m_prvs_inst_date: &mut NaiveDate,
    m_prvs_end_date: &mut NaiveDate,
    g_cur_inst_date: &mut NaiveDate,
    final_clr_bal: &mut f64,
    first_clr_bal: &mut f64,
) {
    {
        let mut prin_cf: Vec<Cashflow> = Vec::new();
        let mut od_cf: Vec<Cashflow> = Vec::new();
        let mut prin_inst_amt = 0.0;
        let mut intr_inst_amt = 0.0;
        let mut od_flag = "".to_string();
        let mut cur_inst_dt = lrs_input.flow_start_date;
        let mut inst_freq = &lrs_input.lr_freq_type;
        let lr_freq = get_month_equivalent(inst_freq.to_string());
        let mut pmt_prd = lrs_input.no_of_flows;
        let num_of_dmds = lrs_input.num_of_dmds;
        let flow_amt = (lrs_input.flow_amt * 100.0).round() / 100.0;
        let cash_flow_code = &lrs_input.cashflow_code;
        let ei_int_calc_freq = get_month_equivalent(loan_input.ei_int_calc_freq.to_owned());
        let mut early_date = lrs_input.end_date;
        let mut overdue_diff = 0.0;
        let mut processing_over = false;
        let num_of_flows = pmt_prd;
        let temp_const =
            ((1.0 + (loan_input.int_rate * ei_int_calc_freq * lr_freq) / 1200.0) * 100.0).round()
                / 100.0;

        loan_input.clr_bal_amt = loan_input.clr_bal_amt.abs().to_owned();

        while (pmt_prd > 0) & (processing_over == false) && *first_clr_bal > 0.0 {
            if cur_inst_dt > loan_input.end_date {
                break;
            }
            if loan_input.ei_formula_flg == "F" || loan_input.ei_formula_flg == "R" {
                if loan_input.rep_shdl_num == 1 {
                    intr_inst_amt = (get_interest_for_accounts(
                        loan_input.ei_formula_flg.to_owned(),
                        loan_input.int_rate.to_owned(),
                        lr_freq,
                        loan_input.sanct_lim.to_owned(),
                        num_of_flows,
                        loan_input.ei_method.to_owned(),
                        loan_input.ei_int_calc_freq.to_owned(),
                        flow_amt,
                        num_of_dmds,
                    ) * 100.0)
                        .round()
                        / 100.0;
                } else {
                    intr_inst_amt = (get_interest_for_accounts(
                        loan_input.ei_formula_flg.to_owned(),
                        loan_input.int_rate.to_owned(),
                        lr_freq,
                        loan_input.dis_amt.to_owned(),
                        num_of_flows,
                        loan_input.ei_method.to_owned(),
                        loan_input.ei_int_calc_freq.to_owned(),
                        flow_amt,
                        num_of_dmds,
                    ) * 100.0)
                        .round()
                        / 100.0;
                }
            } else {
                unsafe {
                    if PEND_PRIN_AMT <= 0.0 {
                        break;
                    }

                    if (cash_flow_code != "PLPMT") & (loan_input.ei_formula_flg != "O") {
                        intr_inst_amt = flow_amt
                            * (1.0
                                - (temp_const.powf(-1.0 * pmt_prd as f64 / ei_int_calc_freq)
                                    / (temp_const.powf((1.0 / ei_int_calc_freq) - 1.0)))
                                    * 100.0)
                                .round()
                            / 100.0;
                    } else if loan_input.ei_formula_flg == "O" {
                        intr_inst_amt = calculate_si_by_months(
                            PEND_PRIN_AMT,
                            loan_input.int_rate,
                            ei_int_calc_freq,
                        );
                    } else {
                        intr_inst_amt = 0.0;
                    }

                    prin_inst_amt = ((flow_amt - intr_inst_amt) * 100.0).round() / 100.0;

                    if loan_input.sanct_lim < loan_input.clr_bal_amt.to_owned() {
                        loan_input.sanct_lim =
                            (loan_input.clr_bal_amt.to_owned() * 100.0).round() / 100.0;
                    }
                    if prin_inst_amt > loan_input.clr_bal_amt.to_owned() {
                        prin_inst_amt = (loan_input.clr_bal_amt.to_owned() * 100.0).round() / 100.0;
                    }
                    loan_input.sanct_lim =
                        ((loan_input.sanct_lim - prin_inst_amt) * 100.0).round() / 100.0;
                    if prin_inst_amt > PEND_PRIN_AMT {
                        prin_inst_amt = PEND_PRIN_AMT;
                    }

                    if prin_inst_amt > 0.0 {
                        PEND_PRIN_AMT = ((PEND_PRIN_AMT - prin_inst_amt) * 100.0).round() / 100.0;
                    }
                }
            }
            if cur_inst_dt > as_on_date {
                if loan_input.ei_formula_flg == "O" {
                    intr_inst_amt = (calculate_si_by_months(
                        first_clr_bal.to_owned(),
                        loan_input.int_rate,
                        ei_int_calc_freq,
                    ) * 100.0)
                        .round()
                        / 100.0;
                    prin_inst_amt = ((flow_amt - intr_inst_amt) * 100.0).round() / 100.0;
                }

                if prin_inst_amt > loan_input.clr_bal_amt.to_owned() {
                    prin_inst_amt = (loan_input.clr_bal_amt.to_owned() * 100.0).round() / 100.0;
                    processing_over = true;
                }
                if prin_inst_amt > 0.0 {
                    //Store the cashflow amount principal
                    if *first_clr_bal < prin_inst_amt {
                        prin_inst_amt = *first_clr_bal;
                    }
                    let cashflows =
                        generate_cashflows(prin_inst_amt, 0.0, &early_date, &cur_inst_dt);
                    account_with_cashflows_prin.push(cashflows);
                    //Store the cashflow amoutn interest
                    let cashflows =
                        generate_cashflows(0.0, intr_inst_amt, &early_date, &cur_inst_dt);
                    account_with_cashflows_prin.push(cashflows);
                    *final_clr_bal = *final_clr_bal - prin_inst_amt;
                    *first_clr_bal = *first_clr_bal - prin_inst_amt;
                }

                if prin_inst_amt > 0.0 {
                    loan_input.clr_bal_amt =
                        ((loan_input.clr_bal_amt.to_owned() - prin_inst_amt) * 100.0).round()
                            / 100.0;
                }
            } else {
                unsafe {
                    overdue_diff =
                        ((final_clr_bal.to_owned() - PEND_PRIN_AMT) * 100.0).round() / 100.0;
                }

                if overdue_diff > 0.0 {
                    loan_input.clr_bal_amt =
                        ((loan_input.clr_bal_amt.to_owned() - overdue_diff) * 100.0).round()
                            / 100.0;
                    //Get the tenor for the OD account.
                    let tenor = num_days_start_to_end(cur_inst_dt, as_on_date);
                    if tenor > 0 {
                        let cashflows =
                            generate_cashflows(overdue_diff, 0.0, &early_date, &cur_inst_dt);
                        account_with_cashflows_od.push(cashflows);

                        *final_clr_bal = *final_clr_bal - overdue_diff;
                        *first_clr_bal = *first_clr_bal - overdue_diff;
                    } else {
                        let cashflows =
                            generate_cashflows(overdue_diff, 0.0, &early_date, &cur_inst_dt);
                        account_with_cashflows_prin.push(cashflows);

                        *final_clr_bal = *final_clr_bal - overdue_diff;
                        *first_clr_bal = *first_clr_bal - overdue_diff;
                    }
                }
            }
            *m_prvs_inst_date = cur_inst_dt;
            *m_prvs_end_date = early_date;
            *g_cur_inst_date = cur_inst_dt;
            cur_inst_dt = get_next_inst_date(cur_inst_dt, inst_freq.to_string());
            pmt_prd -= 1;
        }
    }
}
