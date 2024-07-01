use super::read_config::read_config_files;
use super::read_config::{BMKey, BMRates};
use super::util::{add_days, get_maturity_date};
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::NaiveDate;
use slog::Logger;

#[derive(Debug)]
pub struct AccountData {
    pub coa: String,
    pub acc_id: String,
    pub acc_open_date: NaiveDate,
    pub tenor: String,
    pub payout_freq: String,
    pub int_pay_freq: String,
    pub maturity_date: NaiveDate,
    pub os_amount: f64,
    pub currency: String,
    pub cf_type: String,
    pub int_basis: String,
    pub int_rate: f64,
    pub bm: String,
    pub bm_freq: String,
    pub bm_res_days: i32,
    pub next_rep_date: NaiveDate,
    pub bm_rate: f64,
}

pub fn generate_new_acc(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) -> Vec<AccountData> {
    // Read all account generation mapping files into memory
    let (dis_amt_map, dis_bm_map, dis_tenor_map, bm_rate_map, coa_map) =
        read_config_files(config_params, logger, diag_logger);

    // DataStructure to store all new hypothetical accounts
    let mut new_accounts: Vec<AccountData> = Vec::new();
    let mut tot_amt_in_op = 0.0;
    let def_coa_bm = "FIXED".to_string(); // Default COA BM
    let coa_bm = coa_map.get(config_params.prj_coa()).unwrap_or(&def_coa_bm);

    let mut acc_count = 1;
    for (day, day_weightage) in dis_amt_map.iter().enumerate() {
        let acc_open_date = add_days(config_params.as_on_date(), &(day as u8));
        let acc_os_amount_by_day = config_params.new_business_value() * day_weightage / 100.0;
        if acc_os_amount_by_day != 0.0 {
            for (bm_key, bm_weightage) in &dis_bm_map {
                let acc_os_amount_by_bm = acc_os_amount_by_day * bm_weightage / 100.0;

                for (tenor_key, tenor_weightage) in &dis_tenor_map {
                    let acc_os_amount_by_tenor = acc_os_amount_by_bm * tenor_weightage / 100.0;
                    let maturity_date = get_maturity_date(&acc_open_date, &tenor_key.tenor);
                    let bm_int_key = BMRates {
                        bm: coa_bm.to_string(),
                        tenor: tenor_key.tenor.chars().filter(|c| c.is_digit(10)).collect(),
                        uom: tenor_key
                            .tenor
                            .chars()
                            .filter(|c| c.is_alphabetic())
                            .collect(),
                    };
                    let bm_rate_key = BMRates {
                        bm: bm_key.bm.to_string(),
                        tenor: tenor_key.tenor.chars().filter(|c| c.is_digit(10)).collect(),
                        uom: tenor_key
                            .tenor
                            .chars()
                            .filter(|c| c.is_alphabetic())
                            .collect(),
                    };
                    let month_num = 1; // Todo: Replace for month num identification logic
                    let int_rate =
                        bm_rate_map.get(&bm_int_key).unwrap_or(&vec![0.0])[month_num - 1];
                    let bm_rate =
                        bm_rate_map.get(&bm_rate_key).unwrap_or(&vec![0.0])[month_num - 1];
                    let next_rep_date = get_next_rep_date(acc_open_date, bm_key, maturity_date);
                    let new_acc = AccountData {
                        coa: config_params.prj_coa().to_string(),
                        acc_id: format!("ACC{}", acc_count),
                        acc_open_date: acc_open_date,
                        tenor: tenor_key.tenor.to_string(),
                        payout_freq: tenor_key.pay_freq.to_string(),
                        int_pay_freq: tenor_key.int_freq.to_string(),
                        maturity_date: maturity_date,
                        os_amount: acc_os_amount_by_tenor,
                        currency: config_params.currency().to_string(),
                        cf_type: config_params.cf_type().to_string(),
                        int_basis: config_params.interest_basis().to_string(),
                        int_rate: int_rate,
                        bm: bm_key.bm.to_string(),
                        bm_freq: bm_key.bm_freq.to_string(),
                        bm_res_days: bm_key.bm_res_days,
                        next_rep_date: next_rep_date,
                        bm_rate: bm_rate,
                    };
                    new_accounts.push(new_acc);
                    acc_count += 1;

                    tot_amt_in_op += acc_os_amount_by_tenor;
                }
            }
        }
    }
    log_debug!(diag_logger, "New Accounts: \n {:#?}", new_accounts);
    log_info!(
        logger,
        "Total Amount for Accounts Generated: {}",
        tot_amt_in_op
    );
    return new_accounts;
}

fn get_next_rep_date(acc_open_date: NaiveDate, bm_data: &BMKey, mat_date: NaiveDate) -> NaiveDate {
    if bm_data.bm_res_days == 0 {
        return mat_date;
    } else {
        match &bm_data.bm_freq[..] {
            "M" => rbdate::incr_dt_by_mon_presrv_eom(acc_open_date, 1).unwrap_or(mat_date),
            "Q" => rbdate::incr_dt_by_mon_presrv_eom(acc_open_date, 3).unwrap_or(mat_date),
            "H" => rbdate::incr_dt_by_mon_presrv_eom(acc_open_date, 6).unwrap_or(mat_date),
            "Y" => rbdate::incr_dt_by_mon_presrv_eom(acc_open_date, 12).unwrap_or(mat_date),
            _ => mat_date,
        }
    }
}
