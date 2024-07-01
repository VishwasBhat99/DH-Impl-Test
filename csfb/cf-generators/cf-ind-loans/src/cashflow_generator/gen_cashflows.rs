use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use chrono::{Datelike, Duration};
use configuration_parameters::ConfigurationParameters;
use macros;
use math::round::half_away_from_zero;
use npa_cfdate_adjusment::npa_cfdate_adjusment;
use rbdate::*;
use sdb_cf_gen::*;
use slog::Logger;
use std::convert::TryInto;

pub fn generate_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Result<Vec<Cashflow>, String> {
    let mut cfs: Vec<Cashflow> = Vec::new();
    let as_on_date = *config_params.as_on_date();
    let mut is_both_pmi_emi = account.remark.to_lowercase().trim().contains("both");

    //For Bullet Payment
    if account.bullet_pay_flag.trim().to_uppercase() == "Y" {
        let cf = gen_cf(
            0.0,
            account.os_loan_bal_lcy,
            account.ei_end_date_crnt.unwrap_or(as_on_date),
            config_params.precision(),
        );
        let mut new_cf = Cashflow::new();
        new_cf.prin_amt = cf.prin_amt;
        new_cf.int_amt = cf.int_amt;
        new_cf.date = cf.date;
        return Ok(vec![new_cf]);
    }

    // For Overdue
    if account.os_p_bal_due_local_ccy != 0.0 || account.os_i_bal_due_local_ccy != 0.0 {
        let cf = gen_cf(
            0.0,
            account.os_p_bal_due_local_ccy + account.os_i_bal_due_local_ccy,
            as_on_date,
            config_params.precision(),
        );
        cfs.push(conv_cf(
            cf,
            account.hdfc_ltd_percent,
            config_params.precision(),
            &account.npa_class,
        ));
        return Ok(cfs);
    }

    // For Excess EI
    if account.ei_amt_paid_adv_lcy != 0.0 {
        let cf = gen_cf(
            0.0,
            account.ei_amt_paid_adv_lcy,
            as_on_date,
            config_params.precision(),
        );
        cfs.push(conv_cf(
            cf,
            account.hdfc_ltd_percent,
            config_params.precision(),
            &account.npa_class,
        ));
        return Ok(cfs);
    }

    let mat_date = match account.maturity_date {
        Some(date) => date,
        None => {
            log_error!(
                log,
                "`maturity_date`: `{:?}` is not well-formatted for account: `{}`.",
                account.maturity_date,
                account.acc_no,
            );
            as_on_date
        }
    };

    let (mut from_amort_date, mut to_amort_date) =
        get_from_to_amort_dates(account, as_on_date, log);

    let freq = get_freq(&account.ei_pay_freq_crnt);
    let mut open_prin = account.os_loan_bal_lcy;
    let old_emi_amount = account.ei_amt_crnt;
    let mut month_to_inc = freq;
    if account.int_rate == 0.0 {
        log_error!(
            log,
            "`int_rate`: `{}` for account: `{}`, so treated as bullet payment.",
            account.int_rate,
            account.acc_no,
        );
        open_prin = account.os_loan_bal_lcy + account.pre_ei_bal_lcy;
        let cf = calc_bult_cf(open_prin, mat_date, config_params.precision());
        cfs.push(conv_cf(
            cf,
            account.hdfc_ltd_percent,
            config_params.precision(),
            &account.npa_class,
        ));
        return Ok(cfs);
    }

    // Pre-EI CF (Principal Moratorium)
    if account.pre_ei_bal_lcy != 0.0 || is_both_pmi_emi {
        open_prin = account.pre_ei_bal_lcy;
        from_amort_date = from_amort_date;
        to_amort_date = from_amort_date;

        let mut counter = 1;
        account.ei_amt_crnt = calc_emi_amt(
            open_prin,
            account.int_rate,
            account.ei_orginal_term as f64,
            config_params.precision(),
        );
        while half_away_from_zero(open_prin, config_params.precision()) > 0.0 {
            let mut cf = calc_monthly_cf(
                open_prin,
                account.int_rate,
                account.ei_amt_crnt,
                freq as f64,
                to_amort_date,
                config_params.precision(),
            );
            if counter == 4 && cf.prin_amt == 0.0 {
                cf.prin_amt = account.pre_ei_bal_lcy;
                cf.date = timestamp(mat_date);
                cfs.push(conv_cf(
                    cf,
                    account.hdfc_ltd_percent,
                    config_params.precision(),
                    &account.npa_class,
                ));
                break;
            }
            if counter <= 3 {
                cf.prin_amt = 0.0;
                counter += 1;
            }

            let mut from_date =
                increment_date_by_months(from_amort_date, month_to_inc.try_into().unwrap());
            if account.ei_pay_day == 31 && from_amort_date.day() == 30 {
                from_date = get_month_end_date(from_date);
            }
            to_amort_date = from_date;
            month_to_inc += freq;
            open_prin -= cf.prin_amt;
            account.resid_int = calc_resid_int(cf.int_amt, cf.prin_amt, account.ei_amt_crnt);
            cfs.push(conv_cf(
                cf,
                account.hdfc_ltd_percent,
                config_params.precision(),
                &account.npa_class,
            ));
        }

        if !is_both_pmi_emi {
            return Ok(cfs);
        }
    }

    // Handling both PMI and EMI cases
    if is_both_pmi_emi {
        open_prin = account.os_loan_bal_lcy;
        account.ei_amt_crnt = old_emi_amount;
        let (from_date, to_date) = get_from_to_amort_dates(account, as_on_date, log);
        from_amort_date = from_date;
        to_amort_date = to_date;
    }

    // For Fresh accounts
    let mut last_paid_date;
    if let Some(date) = account.emi_last_paid_date_crnt {
        last_paid_date = date;
    } else {
        if let Some(cf) = calc_daywise_cf(
            open_prin,
            account.int_rate,
            account.ei_amt_crnt,
            from_amort_date,
            config_params.precision(),
        ) {
            last_paid_date = date_from_timestamp(cf.date);
            account.resid_int = calc_resid_int(cf.int_amt, cf.prin_amt, account.ei_amt_crnt);
            cfs.push(conv_cf(
                cf,
                account.hdfc_ltd_percent,
                config_params.precision(),
                &account.npa_class,
            ));
        } else {
            last_paid_date = from_amort_date;
        }
    }

    // Matutity Date greater than Cashflow Date
    if mat_date <= to_amort_date {
        let cf = calc_bult_cf(open_prin, mat_date, config_params.precision());
        cfs.push(conv_cf(
            cf,
            account.hdfc_ltd_percent,
            config_params.precision(),
            &account.npa_class,
        ));
        return Ok(cfs);
    }
    let ei_pay_day = account.ei_pay_day;
    let mut is_incr_flag = false;
    let mut prev_cf_date = if i64::from(from_amort_date.day()) <= ei_pay_day {
        is_incr_flag = true;
        match NaiveDate::from_ymd_opt(
            from_amort_date.year(),
            from_amort_date.month(),
            ei_pay_day.try_into().expect("Cannot read ei_pay_day"),
        ) {
            Some(prev_cf_date) => prev_cf_date,
            None => get_month_end_date(from_amort_date),
        }
    } else {
        increment_date_by_months(
            match NaiveDate::from_ymd_opt(
                from_amort_date.year(),
                from_amort_date.month(),
                ei_pay_day.try_into().expect("Cannot read ei_pay_day"),
            ) {
                Some(prev_cf_date) => prev_cf_date,
                None => get_month_end_date(from_amort_date),
            },
            month_to_inc.try_into().unwrap(),
        )
    };

    // Moratorium Product CF

    if is_incr_flag {
        if (account.mor_int_calc.to_lowercase().contains("simple")
        || account.mor_int_calc.to_lowercase().contains("compound"))
        && !is_both_pmi_emi
    {
        let mut mor_from_amort_date = from_amort_date;
        let mut mor_to_amort_date = from_amort_date;
        if let Some(to_mor_date) = account.to_mor_date {
            while prev_cf_date < decr_dt_by_mon_presrv_eom(to_amort_date, month_to_inc).unwrap() {
                prev_cf_date =
                    increment_date_by_months(prev_cf_date, month_to_inc.try_into().unwrap());
                mor_to_amort_date = prev_cf_date;
                let int_amt = calc_int_amt_monthly(open_prin, account.int_rate, freq as f64);
                let cf = gen_cf(int_amt, 0.0, prev_cf_date, config_params.precision());
                if prev_cf_date < decr_dt_by_mon_presrv_eom(to_amort_date, month_to_inc).unwrap() {
                    cfs.push(conv_cf(
                        cf,
                        account.hdfc_ltd_percent,
                        config_params.precision(),
                        &account.npa_class,
                    ));
                }
                open_prin += int_amt;
            }
            last_paid_date = to_mor_date + Duration::days(account.ei_pay_day);
        }
    }
    } else {
        if (account.mor_int_calc.to_lowercase().contains("simple")
        || account.mor_int_calc.to_lowercase().contains("compound"))
        && !is_both_pmi_emi
    {
        let mut mor_from_amort_date = from_amort_date;
        let mut mor_to_amort_date = from_amort_date;
        if let Some(to_mor_date) = account.to_mor_date {
            while prev_cf_date < to_mor_date {
                mor_to_amort_date = prev_cf_date;
                let int_amt = calc_int_amt_monthly(open_prin, account.int_rate, freq as f64);
                let cf = gen_cf(int_amt, 0.0, prev_cf_date, config_params.precision());
                cfs.push(conv_cf(
                    cf,
                    account.hdfc_ltd_percent,
                    config_params.precision(),
                    &account.npa_class,
                ));
                open_prin += int_amt;
                prev_cf_date =
                    increment_date_by_months(prev_cf_date, month_to_inc.try_into().unwrap());
            }
            last_paid_date = to_mor_date + Duration::days(account.ei_pay_day);
        }
    }
    }
    // Normal EI CF (No Moratorium)
    from_amort_date = prev_cf_date;
    let mut from_date = prev_cf_date;
    while from_date < mat_date || is_both_pmi_emi {
        if account.ei_pay_day == 31 && from_amort_date.day() == 30 {
            from_date = get_month_end_date(from_date);
        }
        to_amort_date = from_date;
        let cf_date = to_amort_date;
        let cf = calc_monthly_cf(
            open_prin,
            account.int_rate,
            account.ei_amt_crnt,
            freq as f64,
            cf_date,
            config_params.precision(),
        );
        last_paid_date = date_from_timestamp(cf.date);
        open_prin -= cf.prin_amt;
        account.resid_int = calc_resid_int(cf.int_amt, cf.prin_amt, account.ei_amt_crnt);
        cfs.push(conv_cf(
            cf,
            account.hdfc_ltd_percent,
            config_params.precision(),
            &account.npa_class,
        ));
        is_both_pmi_emi = false;
        from_date =
        increment_date_by_months(from_amort_date, month_to_inc.try_into().unwrap());
        month_to_inc += freq;
    }
    if let Some(cf) = adjust_cf(
        to_amort_date,
        mat_date,
        open_prin,
        account.int_rate,
        freq as f64,
        config_params.precision(),
        account.os_loan_bal_lcy,
    ) {
        cfs.push(conv_cf(
            cf,
            account.hdfc_ltd_percent,
            config_params.precision(),
            &account.npa_class,
        ));
    }

    Ok(cfs)
}

fn conv_cf(inp_cf: sdb_cf_gen::Cashflow, percent: f64, precision: i8, npa_class: &str) -> Cashflow {
    let npa_adj_date =
        npa_cfdate_adjusment(date_from_timestamp(inp_cf.date), npa_class.to_string()).expect(
            &format!("Error while adjusting date for npa_class: `{}`.", npa_class),
        );
    let mut op_cf = Cashflow::new();
    op_cf.int_amt = half_away_from_zero(inp_cf.int_amt * percent / 100.0, precision);
    op_cf.prin_amt = half_away_from_zero(inp_cf.prin_amt * percent / 100.0, precision);
    op_cf.date = timestamp(npa_adj_date);

    op_cf
}

fn get_from_to_amort_dates(
    account: &mut InputAccount,
    as_on_date: NaiveDate,
    log: &Logger,
) -> (NaiveDate, NaiveDate) {
    let first_emi_date = account
        .ei_start_date_crnt
        .unwrap_or(account.emi_last_paid_date_crnt.unwrap_or(as_on_date));
    let (from_date, to_date) = if first_emi_date > as_on_date {
        (first_emi_date, first_emi_date)
    } else {
        let mut cf_date = NaiveDate::from_ymd_opt(
            account.emi_last_paid_date_crnt.unwrap_or(as_on_date).year(),
            account
                .emi_last_paid_date_crnt
                .unwrap_or(as_on_date)
                .month(),
            account.ei_pay_day as u32,
        )
        .unwrap_or(as_on_date);
        while cf_date < as_on_date {
            cf_date = rbdate::increment_date_by_months(
                cf_date,
                get_freq(&account.ei_pay_freq_crnt) as u16,
            )
        }
        (cf_date, cf_date)
    };
    let from_amort_date = match account.from_mor_date {
        Some(date) => date,
        None => {
            log_warn!(log, "Could not fetch From-Amort-Date from Input\n Derived From-Amort-Date: {} for account: {}",
                            from_date,
                            account.acc_no);
            from_date
        }
    };
    let to_amort_date = match account.to_mor_date {
        Some(date) => date,
        None => {
            log_warn!(log, "Could not fetch To-Amort-Date from Input\n Derived To-Amort-Date: {} for account: {}",
                            to_date,
                            account.acc_no);
            to_date
        }
    };
    (from_amort_date, to_amort_date)
}

fn adjust_cf(
    to_amort_date: NaiveDate,
    mat_date: NaiveDate,
    open_prin: f64,
    int_rate: f64,
    freq: f64,
    precision: i8,
    outstanding_balance: f64,
) -> Option<sdb_cf_gen::Cashflow> {

    let days = num_days_start_to_end(to_amort_date, mat_date);

    if days > 0 && days < 30 {
        let interest_amount =
            calculate_interest_amount(open_prin, int_rate, days as f64);
            Some(gen_cf(
                interest_amount,
                open_prin,
                mat_date,
                precision,
            )) 
    } else {
        if open_prin > 0.0 {
            Some(gen_cf(
                calc_int_amt_monthly(open_prin, int_rate, freq),
                open_prin,
                mat_date,
                precision,
            ))
        } else {
            None
        }
    }
}

pub fn generate_cashflows_securitised(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Result<Vec<Cashflow>, String> {
    let mut cfs: Vec<Cashflow> = Vec::new();
    let as_on_date = *config_params.as_on_date();
    let mut is_both_pmi_emi = account.remark.to_lowercase().trim().contains("both");

    // For Overdue
    if account.os_p_bal_due_local_ccy != 0.0 || account.os_i_bal_due_local_ccy != 0.0 {
        let cf = gen_cf(
            0.0,
            account.os_p_bal_due_local_ccy + account.os_i_bal_due_local_ccy,
            as_on_date,
            config_params.precision(),
        );
        cfs.push(conv_cf(
            cf,
            account.sec_percent,
            config_params.precision(),
            &account.npa_class,
        ));
        return Ok(cfs);
    }

    // For Excess EI
    if account.ei_amt_paid_adv_lcy != 0.0 {
        let cf = gen_cf(
            0.0,
            account.ei_amt_paid_adv_lcy,
            as_on_date,
            config_params.precision(),
        );
        cfs.push(conv_cf(
            cf,
            account.sec_percent,
            config_params.precision(),
            &account.npa_class,
        ));
        return Ok(cfs);
    }

    let mat_date = match account.maturity_date {
        Some(date) => date,
        None => {
            log_error!(
                log,
                "`maturity_date`: `{:?}` is not well-formatted for account: `{}`.",
                account.maturity_date,
                account.acc_no,
            );
            as_on_date
        }
    };

    let (mut from_amort_date, mut to_amort_date) =
        get_from_to_amort_dates(account, as_on_date, log);

    let freq = get_freq(&account.ei_pay_freq_crnt);
    let mut open_prin = account.os_loan_bal_lcy;
    let old_emi_amount = account.ei_amt_crnt;
    let mut month_to_inc = freq;

    if account.int_rate == 0.0 {
        log_error!(
            log,
            "`int_rate`: `{}` for account: `{}`, so treated as bullet payment.",
            account.int_rate,
            account.acc_no,
        );
        open_prin = account.os_loan_bal_lcy + account.pre_ei_bal_lcy;
        let cf = calc_bult_cf(open_prin, mat_date, config_params.precision());
        cfs.push(conv_cf(
            cf,
            account.sec_percent,
            config_params.precision(),
            &account.npa_class,
        ));
        return Ok(cfs);
    }

    // Pre-EI CF (Principal Moratorium)
    if account.pre_ei_bal_lcy != 0.0 || is_both_pmi_emi {
        open_prin = account.pre_ei_bal_lcy;
        from_amort_date = from_amort_date;
        to_amort_date = from_amort_date;

        let mut counter = 1;
        account.ei_amt_crnt = calc_emi_amt(
            open_prin,
            account.int_rate,
            account.ei_orginal_term as f64,
            config_params.precision(),
        );
        while half_away_from_zero(open_prin, config_params.precision()) > 0.0 {
            let mut cf = calc_monthly_cf(
                open_prin,
                account.int_rate,
                account.ei_amt_crnt,
                freq as f64,
                to_amort_date,
                config_params.precision(),
            );
            if counter == 4 && cf.prin_amt == 0.0 {
                cf.prin_amt = account.pre_ei_bal_lcy;
                cf.date = timestamp(mat_date);
                cfs.push(conv_cf(
                    cf,
                    account.sec_percent,
                    config_params.precision(),
                    &account.npa_class,
                ));
                break;
            }
            if counter <= 3 {
                cf.prin_amt = 0.0;
                counter += 1;
            }

            let mut from_date =
                increment_date_by_months(from_amort_date, month_to_inc.try_into().unwrap());
            if account.ei_pay_day == 31 && from_amort_date.day() == 30 {
                from_date = get_month_end_date(from_date);
            }
            to_amort_date = from_date;
            month_to_inc += freq;
            open_prin -= cf.prin_amt;
            account.resid_int = calc_resid_int(cf.int_amt, cf.prin_amt, account.ei_amt_crnt);
            cfs.push(conv_cf(
                cf,
                account.sec_percent,
                config_params.precision(),
                &account.npa_class,
            ));
        }

        if !is_both_pmi_emi {
            return Ok(cfs);
        }
    }

    // Handling both PMI and EMI cases
    if is_both_pmi_emi {
        open_prin = account.os_loan_bal_lcy;
        account.ei_amt_crnt = old_emi_amount;
        let (from_date, to_date) = get_from_to_amort_dates(account, as_on_date, log);
        from_amort_date = from_date;
        to_amort_date = to_date;
    }

    // For Fresh accounts
    let mut last_paid_date;
    if let Some(date) = account.emi_last_paid_date_crnt {
        last_paid_date = date;
    } else {
        if let Some(cf) = calc_daywise_cf(
            open_prin,
            account.int_rate,
            account.ei_amt_crnt,
            from_amort_date,
            config_params.precision(),
        ) {
            last_paid_date = date_from_timestamp(cf.date);
            account.resid_int = calc_resid_int(cf.int_amt, cf.prin_amt, account.ei_amt_crnt);
            cfs.push(conv_cf(
                cf,
                account.sec_percent,
                config_params.precision(),
                &account.npa_class,
            ));
        } else {
            last_paid_date = from_amort_date;
        }
    }

    // Moratorium Product CF
    if (account.mor_int_calc.to_lowercase().contains("simple")
        || account.mor_int_calc.to_lowercase().contains("compound"))
        && !is_both_pmi_emi
    {
        let mut mor_from_amort_date = from_amort_date;
        let mut mor_to_amort_date = from_amort_date;
        if let Some(to_mor_date) = account.to_mor_date {
            while mor_from_amort_date < to_mor_date {
                mor_from_amort_date =
                    increment_date_by_months(mor_from_amort_date, month_to_inc.try_into().unwrap());
                mor_to_amort_date = mor_from_amort_date;
                open_prin += calc_int_amt_monthly(open_prin, account.int_rate, freq as f64);
            }
            last_paid_date = to_mor_date + Duration::days(account.ei_pay_day);
        }
    }

    // Matutity Date greater than Cashflow Date
    if mat_date <= to_amort_date {
        let cf = calc_bult_cf(open_prin, mat_date, config_params.precision());
        cfs.push(conv_cf(
            cf,
            account.sec_percent,
            config_params.precision(),
            &account.npa_class,
        ));
        return Ok(cfs);
    }

    // Normal EI CF (No Moratorium)
    while to_amort_date < mat_date || is_both_pmi_emi {
        let cf_date = to_amort_date;
        let cf = calc_monthly_cf(
            open_prin,
            account.int_rate,
            account.ei_amt_crnt,
            freq as f64,
            cf_date,
            config_params.precision(),
        );
        last_paid_date = date_from_timestamp(cf.date);
        open_prin -= cf.prin_amt;
        account.resid_int = calc_resid_int(cf.int_amt, cf.prin_amt, account.ei_amt_crnt);
        cfs.push(conv_cf(
            cf,
            account.sec_percent,
            config_params.precision(),
            &account.npa_class,
        ));
        let mut from_date =
            increment_date_by_months(from_amort_date, month_to_inc.try_into().unwrap());
        if account.ei_pay_day == 31 && from_amort_date.day() == 30 {
            from_date = get_month_end_date(from_date);
        }
        to_amort_date = from_date;
        month_to_inc += freq;
        is_both_pmi_emi = false;
    }

    if let Some(cf) = adjust_cf(
        to_amort_date,
        mat_date,
        open_prin,
        account.int_rate,
        freq as f64,
        config_params.precision(),
        account.os_loan_bal_lcy,
    ) {
        cfs.push(conv_cf(
            cf,
            account.sec_percent,
            config_params.precision(),
            &account.npa_class,
        ));
    }
    Ok(cfs)
}


fn calculate_interest_amount(original_balance: f64, interest_rate: f64, days: f64) -> f64 {
    (original_balance * interest_rate * days as f64) / (360.0 * 100.0)
}