use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::gen_cashflows::generate_cashflows;
use cashflow_generator::ln3_biz::get_next_inst_date;
use rbdate::num_days_start_to_end;
use rbdate::NaiveDate;
use statics::*;

use super::account_with_cashflows::Cashflow;

pub fn process_non_ei_loan(
    mut loan_input: Vec<InputAccount>,
    as_on_date: NaiveDate,
    account_with_cashflows_prin: &mut Vec<Cashflow>,
    account_with_cashflows_od: &mut Vec<Cashflow>,
    m_prvs_inst_date: &mut NaiveDate,
    m_prvs_end_date: &mut NaiveDate,
    g_cur_inst_date: &mut NaiveDate,
) {
    let mut total_cashflow = 0.0;
    let clr_bal_amt = loan_input[0].clr_bal_amt;
    *m_prvs_inst_date = loan_input[0]
        .rep_shdl_date
        .unwrap_or(NaiveDate::from_ymd(1970, 1, 1));
    //To handle overdue logic
    //No changes observerd using this logic.
    for (_, loan_input_val) in loan_input.iter_mut().enumerate() {
        //Changes to consider only future flows as totalcashflow
        let mut flwdate = loan_input_val.flow_start_date;
        for _j in 1..=loan_input_val.no_of_flows {
            if flwdate >= as_on_date {
                let temp = loan_input_val.flow_amt;
                total_cashflow += temp;
            }
            let dt = get_next_inst_date(flwdate, loan_input_val.lr_freq_type.to_owned());
            flwdate = dt;
        }
    }

    let loan_input_first = loan_input[0].clone();
    let mut final_clr_bal = loan_input_first.clr_bal_amt.abs();
    let mut final_dt = loan_input[0].flow_start_date;
    let mut first_clr_bal = loan_input_first.clr_bal_amt.abs();
    let mut cash_vec: Vec<Cashflow> = Vec::new();
    for (_, loan_input_value) in loan_input.iter().enumerate() {
        //Generate NonEI cashflows.
        generate_non_ei_cashflow(
            loan_input_first.clone(),
            loan_input_value.clone(),
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
    if final_clr_bal > 0.0 {
        let cashflows = generate_cashflows(
            (final_clr_bal * 100.0).round() / 100.0,
            0.0,
            m_prvs_end_date,
            &final_dt,
        );

        account_with_cashflows_prin.push(cashflows);
    }
}
pub fn generate_non_ei_cashflow(
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
    let mut prin_inst_amt = (lrs_input.flow_amt * 100.0).round() / 100.0;
    let mut intr_inst_amt: f64 = 0.0;
    let mut cur_inst_date = lrs_input.flow_start_date;
    let instr_freq = lrs_input.lr_freq_type;
    let mut pmt_prd = lrs_input.no_of_flows;
    let early_date = lrs_input.end_date;

    let mut overdue_diff: f64;
    let mut days_diff: i64;

    //Generate principal amt and interest amt for all installment dates.
    while (first_clr_bal.to_owned() > 0.0) && (pmt_prd > 0) {
        if cur_inst_date > loan_input.end_date {
            break;
        }
        if loan_input.sanct_lim < first_clr_bal.to_owned() {
            loan_input.sanct_lim = (first_clr_bal.to_owned() * 100.0).round() / 100.0;
        }

        if prin_inst_amt > first_clr_bal.to_owned() {
            prin_inst_amt = (first_clr_bal.to_owned() * 100.0).round() / 100.0;
        }

        loan_input.sanct_lim = ((loan_input.sanct_lim - prin_inst_amt) * 100.0).round() / 100.0;

        if loan_input.dis_amt <= 0.0 {
            break;
        }

        intr_inst_amt = calculate_si_by_days(
            first_clr_bal.to_owned().abs(),
            loan_input.int_rate,
            num_days_start_to_end(m_prvs_inst_date.to_owned(), cur_inst_date.to_owned()),
        );

        if prin_inst_amt > first_clr_bal.to_owned() {
            prin_inst_amt = (first_clr_bal.to_owned() * 100.0).round() / 100.0;
        }
        if prin_inst_amt > 0.0 {
            unsafe {
                PEND_PRIN_AMT = ((PEND_PRIN_AMT - prin_inst_amt) * 100.0).round() / 100.0;
            }
        }
        if (cur_inst_date > as_on_date) && (prin_inst_amt > 0.0) {
            //Store cashflows for principal and interest amounts.
            *final_clr_bal = *final_clr_bal - prin_inst_amt; //extra
            let cashflows =
                generate_cashflows(prin_inst_amt.to_owned(), 0.0, &early_date, &cur_inst_date);
            account_with_cashflows_prin.push(cashflows);

            if intr_inst_amt > 0.0 {
                let cashflows = generate_cashflows(0.0, intr_inst_amt, &early_date, &cur_inst_date);
                account_with_cashflows_prin.push(cashflows);
            }

            *first_clr_bal = ((first_clr_bal.to_owned() - prin_inst_amt) * 100.0).round() / 100.0;
        } else {
            unsafe {
                overdue_diff = ((*first_clr_bal - PEND_PRIN_AMT) * 100.0).round() / 100.0;
            }

            if overdue_diff > 0.0 {
                days_diff = num_days_start_to_end(cur_inst_date, as_on_date);
                if days_diff > 0 {
                    *first_clr_bal =
                        ((first_clr_bal.to_owned() - overdue_diff) * 100.0).round() / 100.0;
                    *final_clr_bal = *final_clr_bal - overdue_diff;
                    let cashflows =
                        generate_cashflows(overdue_diff, 0.0, &early_date, &cur_inst_date);
                    account_with_cashflows_od.push(cashflows);
                } else {
                    //Store cashflow as principal.
                    *first_clr_bal =
                        ((first_clr_bal.to_owned() - overdue_diff) * 100.0).round() / 100.0;
                    *final_clr_bal = *final_clr_bal - overdue_diff;
                    let cashflows =
                        generate_cashflows(overdue_diff, 0.0, &early_date, &cur_inst_date);
                    account_with_cashflows_prin.push(cashflows);
                }
            }
        }
        *m_prvs_inst_date = cur_inst_date;
        *m_prvs_end_date = early_date;
        *g_cur_inst_date = cur_inst_date;
        cur_inst_date = get_next_inst_date(cur_inst_date, instr_freq.to_owned());
        pmt_prd -= 1;
    }
}

pub fn calculate_si_by_days(osbal: f64, int_rate: f64, days_count: i64) -> f64 {
    (osbal * int_rate * days_count as f64) / 36500.0
}
