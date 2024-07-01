use self::account_field_names::{AccFieldNames, DefaultValues};
use self::master::read_masters;
use self::rules::get_llg;
use self::structs::{AggrData, AggrVal};
use bm_reader::BmMaster;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::{reader, reader::account_with_cfs::get_field_value};
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

mod account_field_names;
mod master;
mod rules;
mod structs;

pub fn process(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_cf = 0;
    let mut buckets_data: HashMap<i64, i64> = HashMap::new();

    //Read (Input+Metadata), Req-Fields and Rules File
    let req_data = AccFieldNames::new_from_path(config_params.req_fields_file_path());
    let def_values = DefaultValues::new_from_path(config_params.default_values_file());
    let cf_file_rdr = reader::Reader::new_at_path(
        config_params.metadata_file_path(),
        config_params.input_file_path(),
    );
    let mut file_rdr = reader::Reader::new_at_path(
        config_params.metadata_file_path(),
        config_params.input_file_path(),
    );
    let rules = AggRules::new_from_path(config_params.rules_file_path(), &cf_file_rdr);

    //Init Writer
    let mut op_writer = get_writer(config_params.output_file_path());

    //Read Master Files
    let mut llg_spread_map: HashMap<(String, String), String> = HashMap::new();
    let mut balm_rating_map: HashMap<(String, String), String> = HashMap::new();
    let mut spread_rate_map: HashMap<(String, String), HashMap<(i64, i64), BmMaster>> =
        HashMap::new();
    read_masters(
        &config_params,
        logger,
        &mut balm_rating_map,
        &mut llg_spread_map,
        &mut spread_rate_map,
        &mut buckets_data,
    );

    let mut data_map: HashMap<(i32, String), AggrData> = HashMap::new();

    for mut account in file_rdr.iter() {
        acc_enc += 1;
        let account_id =
            match get_field_value(&account, &cf_file_rdr, req_data.account_id.to_string()) {
                Ok(value) => value.to_string(),
                Err(_error) => panic!("{}", _error),
            };
        let currency = match get_field_value(&account, &cf_file_rdr, req_data.currency.to_string())
        {
            Ok(value) => value.to_string(),
            Err(_error) => panic!("{}", _error),
        };
        let mut agency = match get_field_value(&account, &cf_file_rdr, req_data.agency.to_string())
        {
            Ok(value) => value.to_string(),
            Err(_error) => panic!("{}", _error),
        };
        let mut agency_rating =
            match get_field_value(&account, &cf_file_rdr, req_data.agency_rating.to_string()) {
                Ok(value) => value.to_string(),
                Err(_error) => panic!("{}", _error),
            };
        let next_rep_date =
            match get_field_value(&account, &cf_file_rdr, req_data.next_rep_date.to_string()) {
                //Default Date would be 31-12-9999 (Some Maximum Date)
                Ok(value) => value.to_string().parse::<i64>().unwrap_or(253402214400),
                Err(_error) => panic!("{}", _error),
            };
        let rep_date: rbdate::NaiveDate = rbdate::date_from_timestamp(next_rep_date);

        let cashflows = match account.remove_cfs_for_key(&req_data.cashflows) {
            Ok(value) => value,
            Err(_error) => {
                log_info!(
                    logger,
                    "Account: {} \n Error while removing cashflow from the pool of cashflows.{:#?}",
                    account_id,
                    _error
                );
                continue;
            }
        };

        let mut llg_id = get_llg(&account, &rules, def_values.default_llg_code);
        let spread_id = llg_spread_map
            .get(&(llg_id.to_string(), currency.clone()))
            .unwrap_or(&def_values.default_spread)
            .to_string();
        if agency.is_empty() {
            agency = def_values.default_agency_name.to_string();
        }
        if agency_rating.is_empty() {
            agency_rating = def_values.default_agency_rating.to_string();
        }
        let balm_rating = balm_rating_map
            .get(&(agency.clone(), agency_rating.clone()))
            .unwrap_or(&def_values.default_balm_rating)
            .to_string();

        for cashflow in cashflows.iter() {
            acc_cf += 1;
            let mut temp_bucket_rate_map: HashMap<i64, AggrVal> = HashMap::new();
            let cf_date: rbdate::NaiveDate =
                if rep_date < rbdate::date_from_timestamp(cashflow.date) {
                    rep_date
                } else {
                    rbdate::date_from_timestamp(cashflow.date)
                };
            let cf_amt = cashflow.principal_amount;

            let thirty_years_from_now =
                rbdate::incr_dt_by_mon_presrv_eom(*config_params.as_on_date(), 360)
                    .expect("Error while incrementing 30 years from now.");
            let bucketid = if cf_date <= *config_params.as_on_date() {
                1 //First Bucket
            } else if cf_date > thirty_years_from_now {
                714 //Last Bucket
            } else {
                let cf_res_days =
                    rbdate::num_days_start_to_end(*config_params.as_on_date(), cf_date);
                *buckets_data
                    .get(&cf_res_days)
                    .unwrap_or(&def_values.default_bucket)
            };
            if cf_date <= *config_params.as_on_date() {
                if def_values.default_overdue_llg_code.is_some() {
                    llg_id = def_values
                        .default_overdue_llg_code
                        .expect("could not parse def-overdue-llg");
                } else {
                    log_warn!(
                        logger, "Writing overdues to LLG: {} as `default_overdue_llg_code` not passed in {} file",
                        llg_id,
                        config_params.default_values_file()
                    );
                }
            }

            let spread_data = spread_rate_map
                .get(&(spread_id.clone(), balm_rating.to_string()))
                .unwrap_or_else(|| {
                    spread_rate_map.get(&(def_values.default_spread.to_string(),def_values.default_balm_rating.to_string())).
                    unwrap_or_else(||
                    panic!(
                        "Could not get Data for Spread-Curve: {} and Balm-Rating: {}",
                        spread_id, balm_rating
                    ))
                })
                .get(&(if cf_date > thirty_years_from_now{
                    rbdate::num_days_start_to_end(*config_params.as_on_date(), thirty_years_from_now)
                }
                else if cf_date > *config_params.as_on_date(){
                    rbdate::num_days_start_to_end(*config_params.as_on_date(), cf_date)
                }
                else{
                    1
                },if bucketid > 714{
                    714
                }else{
                    bucketid
                }))
                .unwrap_or_else(|| {
                    panic!(
                        "Could not get Data for Bucket: {} where Spread-Curve: {} and Balm-Rating: {}",
                        bucketid, spread_id, balm_rating
                    )
                });
            temp_bucket_rate_map.insert(
                bucketid,
                AggrVal {
                    amt: cf_amt,
                    rate: spread_data.rate,
                    curveid: spread_id.clone(),
                },
            );
            data_map
                .entry((llg_id, currency.to_string()))
                .and_modify(|data| {
                    data.aggr_data(cf_amt, bucketid, spread_data.rate, spread_id.clone())
                })
                .or_insert(AggrData {
                    data: temp_bucket_rate_map,
                });
        }
    }

    for (llg_curr, data) in data_map.iter() {
        let mut bucket_line = String::new();
        let ason_spread_llg = format!(
            "{}|{}|{}",
            config_params.as_on_date().format("%d-%m-%Y"),
            llg_spread_map
                .get(&(llg_curr.0.to_string(), llg_curr.1.to_string()))
                .unwrap_or(&def_values.default_spread.to_string()),
            llg_curr.0,
        );
        for bucket in 1..=714 {
            bucket_line.push('|');
            bucket_line.push_str(
                &format!(
                    "{:.8}",
                    data.data
                        .get(&bucket)
                        .unwrap_or(&AggrVal {
                            amt: 0.0,
                            rate: 0.0,
                            curveid: def_values.default_spread.to_string(),
                        })
                        .rate
                )
                .to_string(),
            );
        }
        writeln!(
            op_writer,
            "{}|{}{}",
            ason_spread_llg, llg_curr.1, bucket_line
        )
        .expect("Unable to generate aggregated summary file.");
        if def_values.base_ccy == llg_curr.1 {
            writeln!(
                op_writer,
                "{}|{}{}",
                ason_spread_llg, def_values.consol_ccy, bucket_line
            )
            .expect("Unable to generate aggregated summary file.");
        }
    }

    // Generate Health Check Report
    let health_report = HealthReport::new(acc_enc, acc_enc, 0, 0.0, 0.0, acc_cf);
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file_path());
}

pub fn get_writer(file_path: &str) -> BufWriter<File> {
    match sdb_io::buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!("Unable to create file `{}`: {}", file_path, error),
    }
}
