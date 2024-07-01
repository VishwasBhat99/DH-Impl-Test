use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::NaiveDateTime;
use slog::Logger;
use stamp_ftp::account_with_cashflows::AccountWithCashflows;
use stamp_ftp::account_with_cashflows::Cashflow;
use stamp_ftp::bm_reader;
use stamp_ftp::bm_reader::yieldrate_calc;
use stamp_ftp::currency::create_currency_converter;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};
use std::path::Path;

pub fn calc_ftp_cflevel(
    mut cf_data_out: AccountWithCashflows,
    basecurve: i32,
    lst_adjustments: Vec<i32>,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> AccountWithCashflows {
    let bc_file_path = config_params.bc_file_path();
    let from_date = config_params.from_date();
    let to_date = config_params.to_date();
    let mut cf_ftp = Vec::new();
    let mut total_balance = DEFAULT_FLOAT;
    let mut adj_weighted = vec![DEFAULT_FLOAT; 6];
    let mut residual_days = DEFAULT_INT;
    let full_file_path = format!(
        "{}{}_{}.txt",
        bc_file_path,
        to_date.format("%d-%m-%Y"),
        basecurve
    );

    //Added +1 -- It includes both from date and to dates.
    let run_duration = rbdate::num_days_start_to_end(*from_date, *to_date) + 1;

    if Path::new(&full_file_path).exists() {
        let mut lst_bm = bm_reader::get_bm_points(&full_file_path, *to_date, log);
        let mut base_prin_prod = DEFAULT_FLOAT;
        // Copying Adjustment Codes
        let mut adj_codes = vec![DEFAULT_INT; 6];
        for index in 0..lst_adjustments.len() {
            adj_codes[index] = lst_adjustments[index].into();
        }
        for cf in cf_data_out.cashflows.iter_mut() {
            let cashflow_date = NaiveDateTime::from_timestamp(cf.date, 0).date();
            if *to_date < cashflow_date {
                residual_days = rbdate::num_days_start_to_end(*to_date, cashflow_date);
            }

            if residual_days <= 0 {
                residual_days = 1;
            }

            let intr_calc_days = if residual_days <= run_duration {
                residual_days
            } else {
                run_duration
            };

            let yield_rate = yieldrate_calc::calc_yieldrate(&mut lst_bm, residual_days);

            // Calculation of Adjustment Rates
            let mut adj_rates = vec![DEFAULT_FLOAT; 6];
            for index in 0..lst_adjustments.len() {
                let adj_path = format!(
                    "{}{}_{}.txt",
                    bc_file_path,
                    to_date.format("%d-%m-%Y"),
                    lst_adjustments[index]
                );
                if Path::new(&adj_path).exists() {
                    let mut adj_lst_bm = bm_reader::get_bm_points(&adj_path, *to_date, log);

                    adj_rates[index] =
                        yieldrate_calc::calc_yieldrate(&mut adj_lst_bm, residual_days);
                }
            }

            let mut cf_obj = Cashflow::new();
            cf_obj.interest_amount = cf.interest_amount;
            cf_obj.principal_amount = cf.principal_amount;
            cf_obj.date = cf.date;
            cf_obj.base_rate = yield_rate;
            cf_obj.base_rate_amount = if cf_data_out.currency.to_uppercase() == "OMR" {
                (cf.principal_amount * yield_rate * intr_calc_days as f64) / (365.0 * 100.0)
            } else {
                (cf.principal_amount * yield_rate * intr_calc_days as f64) / (360.0 * 100.0)
            };

            // Assigning the adjustment rates
            cf_obj.adj1_rate = adj_rates[0];
            cf_obj.adj2_rate = adj_rates[1];
            cf_obj.adj3_rate = adj_rates[2];
            cf_obj.adj4_rate = adj_rates[3];
            cf_obj.adj5_rate = adj_rates[4];
            cf_obj.adj6_rate = adj_rates[5];

            base_prin_prod += cf_obj.base_rate * cf.principal_amount;

            for index in 0..adj_weighted.len() {
                adj_weighted[index] += adj_rates[index] * cf.principal_amount;
            }
            total_balance += cf.principal_amount;

            cf_ftp.push(cf_obj);
        }
        cf_data_out.adj_code_1 = adj_codes[0].to_string();
        cf_data_out.adj_code_2 = adj_codes[1].to_string();
        cf_data_out.adj_code_3 = adj_codes[2].to_string();
        cf_data_out.adj_code_4 = adj_codes[3].to_string();
        cf_data_out.adj_code_5 = adj_codes[4].to_string();
        cf_data_out.adj_code_6 = adj_codes[5].to_string();

        // Base Rate calculation
        cf_data_out.base_rate = if total_balance != DEFAULT_FLOAT {
            base_prin_prod / total_balance
        } else {
            base_prin_prod
        };

        // Adjustment Rate calculation
        for index in 0..adj_weighted.len() {
            adj_weighted[index] = if total_balance != DEFAULT_FLOAT {
                adj_weighted[index] / total_balance
            } else {
                adj_weighted[index]
            }
        }
        cf_data_out.adj_rate_1 = adj_weighted[0];
        cf_data_out.adj_rate_2 = adj_weighted[1];
        cf_data_out.adj_rate_3 = adj_weighted[2];
        cf_data_out.adj_rate_4 = adj_weighted[3];
        cf_data_out.adj_rate_5 = adj_weighted[4];
        cf_data_out.adj_rate_6 = adj_weighted[5];
    } else {
        for cf in cf_data_out.cashflows.iter_mut() {
            let mut cf_obj = Cashflow::new();
            cf_obj.interest_amount = cf.interest_amount;
            cf_obj.principal_amount = cf.principal_amount;
            cf_obj.date = cf.date;
            cf_obj.base_rate = DEFAULT_FLOAT;
            cf_obj.base_rate_amount = DEFAULT_FLOAT;

            total_balance += cf.principal_amount;

            cf_ftp.push(cf_obj);
        }

        log_debug!(
            log,
            "File does not exist's in the path : {}. Hence Base rate will be zero for the account :{}",
            full_file_path, cf_data_out.account_id
        );
    }
    // Resetting interest calculation days
    let intr_calc_days = rbdate::num_days_start_to_end(*from_date, *to_date) + 1;
    // Day Count Basis
    cf_data_out.day_count_basis = "ACT/ACT".to_string();

    // FTP Rate calculation
    cf_data_out.ftp_rate = cf_data_out.base_rate;
    for index in 0..adj_weighted.len() {
        cf_data_out.ftp_rate += adj_weighted[index];
    }
    // Lock Spread calculation
    cf_data_out.lock_spread = cf_data_out.int_rate - cf_data_out.ftp_rate;
    if cf_data_out.currency.to_uppercase() == "OMR" {
        // FTP Amount calculation
        cf_data_out.ftp_amt_ccy =
            (cf_data_out.balance_ccy * cf_data_out.ftp_rate * intr_calc_days as f64)
                / (365.00 * 100.0);
        // Interest Amount calculation
        cf_data_out.int_amt_ccy =
            cf_data_out.balance_ccy * cf_data_out.int_rate * intr_calc_days as f64
                / (365.00 * 100.0);
    } else {
        cf_data_out.ftp_amt_ccy =
            (cf_data_out.balance_ccy * cf_data_out.ftp_rate * intr_calc_days as f64)
                / (360.00 * 100.0);
        // Interest Amount calculation
        cf_data_out.int_amt_ccy =
            cf_data_out.balance_ccy * cf_data_out.int_rate * intr_calc_days as f64
                / (360.00 * 100.0);
    }

    let currency_converter = create_currency_converter(
        config_params.source_local_currency(),
        config_params.exch_rate_file_path(),
    );
    // Balance in HCY
    cf_data_out.balance_hcy =
        currency_converter.convert(&cf_data_out.currency, cf_data_out.balance_ccy, log);
    // FTP Amount calculation
    cf_data_out.ftp_amt_hcy =
        currency_converter.convert(&cf_data_out.currency, cf_data_out.ftp_amt_ccy, log);
    // Interest Amount calculation
    cf_data_out.int_amt_hcy =
        currency_converter.convert(&cf_data_out.currency, cf_data_out.int_amt_ccy, log);
    // EOP Balance Amount calculation
    cf_data_out.eop_balance_hcy =
        currency_converter.convert(&cf_data_out.currency, cf_data_out.eop_balance_ccy, log);

    cf_data_out.cashflows = protobuf::RepeatedField::from_vec(cf_ftp);

    cf_data_out
}
