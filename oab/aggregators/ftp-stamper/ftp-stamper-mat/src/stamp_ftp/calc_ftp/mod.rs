use super::ftp_rates_reader::FtpRateLock;
use super::required_fields::RequiredFields;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::NaiveDate;
use rbdate::NaiveDateTime;
use slog::Logger;
use stamp_ftp::account_with_cashflows::AccountWithCashflows;
use statics::DEFAULT_INT;
mod ftp_calculator;

pub fn calc_ftp(
    mut cf_data_out: AccountWithCashflows,
    required_fields: &RequiredFields,
    method: i32,
    basecurve: i32,
    lst_adjustments: Vec<i32>,
    config_params: &ConfigurationParameters,
    log: &Logger,
    ftp_rate_lock: &FtpRateLock,
) -> AccountWithCashflows {
    let from_date = config_params.from_date();
    let to_date = config_params.to_date();
    let ftp_rates = &ftp_rate_lock.ftp_rates;
    let lock_adjs = &ftp_rate_lock.lock_adjs;
    match method {
        1001 => {
            //Matched Term1 Method implementation
            //CPD : Last Reprice Date, TSD: Last Reprice Date, TED: Maturity Date
            let cpd_naive_date_time =
                NaiveDateTime::from_timestamp(cf_data_out.lst_repricing_dt, 0);
            let mut cpd = cpd_naive_date_time.date();
            let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
            let ted = ted_naive_date_time.date();

            if cpd <= NaiveDate::from_ymd(1970, 01, 01) {
                let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_dt, 0);
                cpd = cpd_naive_date_time.date();
            }

            let cf_data_out1 = ftp_calculator::calc_ftp_cflevel(
                cf_data_out,
                basecurve,
                lst_adjustments,
                config_params,
                log,
                cpd,
                cpd,
                ted,
                false,
                false,
            );

            cf_data_out = cf_data_out1;
        }
        1002 => {
            //Matched Term2 Method implementation
            //CPD : Last Reprice Date, TSD: Last Reprice Date, TED: Next Reprice Date
            let cpd_naive_date_time =
                NaiveDateTime::from_timestamp(cf_data_out.lst_repricing_dt, 0);
            let mut cpd = cpd_naive_date_time.date();
            let next_repr_date = required_fields
                .nxt_reprice_date
                .parse::<i64>()
                .unwrap_or(DEFAULT_INT);
            let ted_naive_date_time = NaiveDateTime::from_timestamp(next_repr_date, 0);
            let mut ted = ted_naive_date_time.date();

            if cpd <= NaiveDate::from_ymd(1970, 01, 01) {
                let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_dt, 0);
                cpd = cpd_naive_date_time.date();
            }

            if ted <= NaiveDate::from_ymd(1970, 01, 01) {
                let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
                ted = cpd_naive_date_time.date();
            }

            let cf_data_out1 = ftp_calculator::calc_ftp_cflevel(
                cf_data_out,
                basecurve,
                lst_adjustments,
                config_params,
                log,
                cpd,
                cpd,
                ted,
                false,
                false,
            );
            cf_data_out = cf_data_out1;
        }
        1003 => {
            //Matched Term3 method
            //CPD : Start Date, TSD: Start Date, TED: Maturity Date
            let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_dt, 0);
            let cpd = cpd_naive_date_time.date();
            let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
            let ted = ted_naive_date_time.date();

            let cf_data_out1 = ftp_calculator::calc_ftp_cflevel(
                cf_data_out,
                basecurve,
                lst_adjustments,
                config_params,
                log,
                cpd,
                cpd,
                ted,
                false,
                false,
            );

            cf_data_out = cf_data_out1;
        }
        1011 => {
            //Cashflow1 Method
            //CPD: Last reprice date , TSD: Last Reprice date, TED: Cashflow date
            let cpd_naive_date_time =
                NaiveDateTime::from_timestamp(cf_data_out.lst_repricing_dt, 0);
            let mut cpd = cpd_naive_date_time.date();
            let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
            let ted = ted_naive_date_time.date();

            if cpd <= NaiveDate::from_ymd(1970, 01, 01) {
                let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_dt, 0);
                cpd = cpd_naive_date_time.date();
            }

            let cf_data_out1 = ftp_calculator::calc_ftp_cflevel(
                cf_data_out,
                basecurve,
                lst_adjustments,
                config_params,
                log,
                cpd,
                cpd,
                ted,
                true,
                false,
            );
            cf_data_out = cf_data_out1;
        }
        1012 => {
            //Cashflow2 Method
            //CPD: Start Date , TSD: Start Date, TED: Cashflow date
            let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_dt, 0);
            let cpd = cpd_naive_date_time.date();
            let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_dt, 0);
            let ted = ted_naive_date_time.date();

            let cf_data_out1 = ftp_calculator::calc_ftp_cflevel(
                cf_data_out,
                basecurve,
                lst_adjustments,
                config_params,
                log,
                cpd,
                cpd,
                ted,
                true,
                false,
            );
            cf_data_out = cf_data_out1;
        }
        1021 => {
            //Assign Rate1 Method
            //CPD: AsOn Date , TSD: AsOn Date, TED: Cashflow date

            let cf_data_out1 = ftp_calculator::calc_ftp_cflevel(
                cf_data_out,
                basecurve,
                lst_adjustments,
                config_params,
                log,
                *from_date,
                *from_date,
                *from_date,
                true,
                false,
            );
            cf_data_out = cf_data_out1;
        }
        1022 => {
            //Assign Rate2 Method
            //CPD: AsOn Date , TSD: AsOn Date, TED: Maturity Date

            let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
            let ted = ted_naive_date_time.date();

            let cf_data_out1 = ftp_calculator::calc_ftp_cflevel(
                cf_data_out,
                basecurve,
                lst_adjustments,
                config_params,
                log,
                *from_date,
                *from_date,
                ted,
                false,
                false,
            );
            cf_data_out = cf_data_out1;
        }
        1031 => {
            //Assign Rate with Lock1 Method
            //CPD: AsOn Date , TSD: AsOn Date, TED: Cashflow date
            if !(ftp_rates.is_empty()) && ftp_rates.contains_key(&cf_data_out.account_id) {
                let rates = ftp_rates
                    .get(&cf_data_out.account_id)
                    .expect("Error in getting FTP Rates");
                let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
                let ted = ted_naive_date_time.date();

                let cf_data_out1 = ftp_calculator::calc_ftp_lock(
                    cf_data_out,
                    rates,
                    &lock_adjs,
                    log,
                    from_date,
                    to_date,
                    *from_date,
                    ted,
                    config_params,
                );

                cf_data_out = cf_data_out1;
            } else {
                let cf_data_out1 = ftp_calculator::calc_ftp_cflevel(
                    cf_data_out,
                    basecurve,
                    lst_adjustments,
                    config_params,
                    log,
                    *from_date,
                    *from_date,
                    *from_date,
                    true,
                    true,
                );
                cf_data_out = cf_data_out1;
            }
        }
        1032 => {
            //Assign Rate with Lock2 Method
            //CPD: AsOn Date , TSD: AsOn Date, TED: Maturity date
            if ftp_rates.contains_key(&cf_data_out.account_id) {
                let rates = ftp_rates
                    .get(&cf_data_out.account_id)
                    .expect("Error in getting FTP Rates");

                let cf_data_out1 = ftp_calculator::calc_ftp_lock(
                    cf_data_out,
                    rates,
                    &lock_adjs,
                    log,
                    from_date,
                    to_date,
                    *from_date,
                    *from_date,
                    config_params,
                );
                cf_data_out = cf_data_out1;
            } else {
                let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
                let ted = ted_naive_date_time.date();

                let cf_data_out1 = ftp_calculator::calc_ftp_cflevel(
                    cf_data_out,
                    basecurve,
                    lst_adjustments,
                    config_params,
                    log,
                    *from_date,
                    *from_date,
                    ted,
                    false,
                    true,
                );
                cf_data_out = cf_data_out1;
            }
        }
        1033 => {
            //Assign Rate with Lock3 Method
            //CPD: Start Date , TSD: Start Date, TED: Maturity date
            if !(ftp_rates.is_empty()) && ftp_rates.contains_key(&cf_data_out.account_id) {
                let rates = ftp_rates
                    .get(&cf_data_out.account_id)
                    .expect("Error in getting FTP Rates");

                let cf_data_out1 = ftp_calculator::calc_ftp_lock(
                    cf_data_out,
                    rates,
                    &lock_adjs,
                    log,
                    from_date,
                    to_date,
                    *from_date,
                    *from_date,
                    config_params,
                );
                cf_data_out = cf_data_out1;
            } else {
                let cpd_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_dt, 0);
                let cpd = cpd_naive_date_time.date();
                let ted_naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
                let ted = ted_naive_date_time.date();

                let cf_data_out1 = ftp_calculator::calc_ftp_cflevel(
                    cf_data_out,
                    basecurve,
                    lst_adjustments,
                    config_params,
                    log,
                    cpd,
                    cpd,
                    ted,
                    false,
                    true,
                );
                cf_data_out = cf_data_out1;
            }
        }
        1034 => {
            //Reprice Term with lock Method
            //CPD: Last Reprice Date , TSD: Last Reprice Date, TED: Next Reprice date
            if !(ftp_rates.is_empty()) && ftp_rates.contains_key(&cf_data_out.account_id) {
                let rates = ftp_rates
                    .get(&cf_data_out.account_id)
                    .expect("Error in getting FTP Rates");

                let cf_data_out1 = ftp_calculator::calc_ftp_lock(
                    cf_data_out,
                    rates,
                    &lock_adjs,
                    log,
                    from_date,
                    to_date,
                    *from_date,
                    *from_date,
                    config_params,
                );
                cf_data_out = cf_data_out1;
            } else {
                let cpd_naive_date_time =
                    NaiveDateTime::from_timestamp(cf_data_out.lst_repricing_dt, 0);
                let mut cpd = cpd_naive_date_time.date();
                let next_repr_date = required_fields
                    .nxt_reprice_date
                    .parse::<i64>()
                    .unwrap_or(DEFAULT_INT);
                let ted_naive_date_time = NaiveDateTime::from_timestamp(next_repr_date, 0);
                let mut ted = ted_naive_date_time.date();

                if cpd <= NaiveDate::from_ymd(1970, 01, 01) {
                    let naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.val_dt, 0);
                    cpd = naive_date_time.date();
                }

                if ted <= NaiveDate::from_ymd(1970, 01, 01) {
                    let naive_date_time = NaiveDateTime::from_timestamp(cf_data_out.mat_dt, 0);
                    ted = naive_date_time.date();
                }

                let cf_data_out1 = ftp_calculator::calc_ftp_cflevel(
                    cf_data_out,
                    basecurve,
                    lst_adjustments,
                    config_params,
                    log,
                    cpd,
                    cpd,
                    ted,
                    false,
                    true,
                );
                cf_data_out = cf_data_out1;
            }
        }
        _ => {
            //Undefined method here
            log_error!(log, "Executing unimplemented method");
        }
    }

    cf_data_out
}
