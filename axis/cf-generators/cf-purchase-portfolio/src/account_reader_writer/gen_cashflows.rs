use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_with_cashflows::Cashflow;
use chrono::Datelike;
use configuration_parameters::ConfigurationParameters;
use rbdate::{get_month_end_date, incr_dt_by_days, increment_date_by_months, timestamp};
use rbdate::{incr_dt_by_mon_presrv_eom_checked, NaiveDate};
use sdb_day_convention::{days_with_convn, Conventions};
use slog::Logger;
#[derive(Debug, Clone)]
pub struct CashflowType {
    pub npa: Vec<Cashflow>,
    pub performing: Vec<Cashflow>,
    pub od: Vec<Cashflow>,
}
pub fn generate_cashflows(
    account: &mut InputAccount,
    _config_params: &ConfigurationParameters,
    _log: &Logger,
    as_on_date: NaiveDate,
    convention: Conventions,
) -> Result<CashflowType, String> {
    let mut last_cf_date = account.cf_start_date;
    let mut cf_date =
        incr_dt_by_mon_presrv_eom_checked(last_cf_date, account.payment_frequency as usize)
            .unwrap_or(NaiveDate::from_ymd(1900, 1, 1));
    //Handling leap year case if above function is being used.
    if last_cf_date.month() == 2 && last_cf_date.day() == 29 {
        cf_date = get_month_end_date(cf_date);
    }
    let maturity_date = account.maturity_date;
    let mut cfs: CashflowType = CashflowType {
        npa: Vec::new(),
        performing: Vec::new(),
        od: Vec::new(),
    };
    match account.final_npa_class.to_uppercase().as_str() {
        "S" | "D" | "L" => {
            //Write this cashflow to NPA file.
            cfs.npa = vec![new_cashflow(
                0.0,
                account.derived_principal,
                timestamp(account.maturity_date),
            )];
            return Ok(cfs);
        }
        _ => {
            //Check for overdue.
            if account.customer_od_bank_share != 0.0 {
                cfs.od = vec![new_cashflow(
                    0.0,
                    account.customer_od_bank_share,
                    timestamp(incr_dt_by_days(as_on_date, 45)),
                )];
                account.derived_principal = account.principal_os * account.bank_share / 100.0
                    - account.customer_od_bank_share.abs();
            }
            let mut derived_prin_amt = account.derived_principal;
            let mut principal_amt;
            let mut interest_amt;

            //Cmonth_emi_due can be zero.Calculate the emi for such accounts
            if account.cmonth_emi_due == 0.0 {
                while cf_date < maturity_date {
                    if cf_date > as_on_date {
                        let pv = account.derived_principal;
                        let rate = account.contract_yield / 1200.0;
                        let days = days_with_convn(cf_date, account.cf_end_date, &convention)
                            .expect("Could not get days by convention.");
                        let mut no_of_days = days.days_btw_dts as f64;
                        let days_in_yr = days.day_in_yr as f64;
                        let no_instalments = (no_of_days / (days_in_yr)) * 12.0; //Rounding off the decimals.
                        if rate <= 0.0 && no_instalments <= 0.0 {
                            account.cmonth_emi_due = account.due_from_customer;
                            principal_amt = account.derived_principal;
                            interest_amt = 0.0;
                            let cf_dt = timestamp(account.maturity_date);
                            cfs.performing = vec![new_cashflow(interest_amt, principal_amt, cf_dt)];
                            return Ok(cfs);
                        } else {
                            account.cmonth_emi_due =
                                cal_emi_amount(pv, rate, no_instalments as f64);
                        }
                        account.derived_cmonth_emi_due = account.cmonth_emi_due;
                        break;
                    }
                    let old_cf_date = cf_date;
                    last_cf_date = cf_date;
                    cf_date = incr_dt_by_mon_presrv_eom_checked(
                        old_cf_date,
                        account.payment_frequency as usize,
                    )
                    .unwrap_or(NaiveDate::from_ymd(1900, 1, 1));
                    if old_cf_date.month() == 2 && old_cf_date.day() == 29 {
                        cf_date = get_month_end_date(cf_date);
                    }
                }
            }
            while cf_date < maturity_date {
                if cf_date > as_on_date {
                    interest_amt = cal_int_amt(
                        last_cf_date,
                        cf_date,
                        derived_prin_amt,
                        account.contract_yield,
                        convention,
                    );

                    principal_amt = account.derived_cmonth_emi_due - interest_amt;

                    if derived_prin_amt <= 0.0 && cfs.performing.is_empty() {
                        //No cashflows are generated for the account. The account is to be skipped.
                        return Ok(cfs);
                    } else if derived_prin_amt <= 0.0 {
                        break;
                    }
                    cfs.performing.push(new_cashflow(
                        interest_amt,
                        principal_amt,
                        timestamp(cf_date),
                    ));
                    let temp_der_prin_amt = derived_prin_amt;
                    derived_prin_amt -= principal_amt;
                    if derived_prin_amt <= 0.0 && cfs.performing.len() != 0 {
                        let len = cfs.performing.len() - 1;
                        cfs.performing[len].principal_amount = temp_der_prin_amt;
                        cfs.performing[len].date = timestamp(maturity_date);
                        break;
                    }
                    last_cf_date = cf_date;
                }
                last_cf_date = cf_date;
                let old_cf_date = cf_date;
                cf_date =
                    incr_dt_by_mon_presrv_eom_checked(cf_date, account.payment_frequency as usize)
                        .unwrap_or(NaiveDate::from_ymd(1900, 1, 1));
                if old_cf_date.month() == 2 && old_cf_date.day() == 29 {
                    cf_date = get_month_end_date(cf_date);
                }
            }
            //Adjustment value after cf-date calculations.
            if derived_prin_amt > 0.0 {
                interest_amt = cal_int_amt(
                    last_cf_date,
                    maturity_date,
                    derived_prin_amt,
                    account.contract_yield,
                    convention,
                );
                cfs.performing.push(new_cashflow(
                    interest_amt,
                    derived_prin_amt,
                    timestamp(maturity_date),
                ));
            }
            Ok(cfs)
        }
    }
}
fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;

    cf
}

fn cal_int_amt(
    last_cf_date: NaiveDate,
    next_cf_date: NaiveDate,
    outstanding_amount: f64,
    int_rate: f64,
    convention: Conventions,
) -> f64 {
    let days = days_with_convn(last_cf_date, next_cf_date, &convention)
        .expect("Could not get days by convention.");
    let no_of_days = days.days_btw_dts as f64;
    let days_in_yr = days.day_in_yr as f64;
    (outstanding_amount * int_rate * no_of_days) / (days_in_yr * 100.0)
}

fn cal_emi_amount(present_value: f64, rate: f64, num_of_insts: f64) -> f64 {
    // EMI Amount = PV*(Rate*(1+Rate)^N)/((1+Rate)^N-1)
    present_value
        * (rate * (1.0 + rate).powf(num_of_insts) / ((1.0 + rate).powf(num_of_insts) - 1.0))
}
