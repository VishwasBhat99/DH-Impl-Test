use bm_reader::{
    calc_yield_rate, get_days_diff, BmMaster, IntermediateBMPoint, IntermediateBMPoints,
};
use configuration_parameters::ConfigurationParameters;
use macros;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;

pub fn read_masters(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    balm_rating_map: &mut HashMap<(String, String), String>,
    llg_spread_map: &mut HashMap<(String, String), String>,
    spread_rate_map: &mut HashMap<(String, String), HashMap<(i64, i64), BmMaster>>,
    buckets_data: &mut HashMap<i64, i64>,
) {
    //Reading Balm Rating File
    let balm_rating_file =
        File::open(config_params.balm_rating_file()).expect("Could Not Read Balm Rating File");
    let balm_rating_reader = BufReader::new(balm_rating_file);
    for (line_no, line) in balm_rating_reader.lines().enumerate() {
        let data: String = match line {
            Ok(data) => data,
            Err(error) => {
                log_error!(
                    logger,
                    "Cannot read line {} from balm rating file: {:?}",
                    line_no,
                    error
                );
                continue;
            }
        };
        let balm_rating_fields: Vec<&str> = data.split('|').collect();
        let agency = balm_rating_fields[0].to_string();
        let agency_rating = balm_rating_fields[1].to_string();
        let balm_rating = balm_rating_fields[2].to_string();
        balm_rating_map.insert((agency, agency_rating), balm_rating);
    }

    //Reading LLG to Spread Mapper File
    let llg_spread_file = File::open(config_params.llg_to_spread_mapper_file())
        .expect("Could Not Read LLG to Spread Mapper File");
    let llg_spread_reader = BufReader::new(llg_spread_file);
    for (line_no, line) in llg_spread_reader.lines().enumerate() {
        let data: String = match line {
            Ok(data) => data,
            Err(error) => {
                log_error!(
                    logger,
                    "Cannot read line {} from llg to spread mapper file: {:?}",
                    line_no,
                    error
                );
                continue;
            }
        };
        let llg_spread_fields: Vec<&str> = data.split('|').collect();
        let llg = llg_spread_fields[0].to_string();
        let spreadid = llg_spread_fields[1].to_string();
        let curr = llg_spread_fields[2].to_string();
        llg_spread_map.insert((llg, curr), spreadid);
    }

    //Reading Spread Rating File
    let spread_rate_file =
        File::open(config_params.spread_rate_file()).expect("Could Not Read Spread Rating File");
    let spread_rate_reader = BufReader::new(spread_rate_file);
    for (line_no, line) in spread_rate_reader.lines().enumerate() {
        let mut temp_bucket_rate_map = HashMap::new();
        let data: String = match line {
            Ok(data) => data,
            Err(error) => {
                log_error!(
                    logger,
                    "Cannot read line {} from spread rating file: {:?}",
                    line_no,
                    error
                );
                continue;
            }
        };
        let spread_rate_fields: Vec<&str> = data.split('|').collect();
        let spreadid = spread_rate_fields[0].to_string();
        let balm_rating = spread_rate_fields[1].to_string();
        let bucketid = spread_rate_fields[2]
            .to_string()
            .parse::<i64>()
            .unwrap_or(0);
        let term = spread_rate_fields[3]
            .to_string()
            .parse::<i64>()
            .unwrap_or(0);
        let uom = spread_rate_fields[4].to_string();
        let spread_rate = spread_rate_fields[5]
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0);
        let data = BmMaster {
            vertex: term,
            uom,
            rate: spread_rate,
        };
        temp_bucket_rate_map.insert((term, bucketid), data.clone());
        if spread_rate_map.contains_key(&(spreadid.clone(), balm_rating.clone())) {
            let value = spread_rate_map
                .get_mut(&(spreadid.clone(), balm_rating.clone()))
                .unwrap_or_else(|| {
                    panic!(
                        "Could not get Data for Spread-Curve: {} and Balm-Rating: {}",
                        spreadid, balm_rating
                    )
                });
            value.insert((term, bucketid), data);
        } else {
            spread_rate_map.insert((spreadid, balm_rating), temp_bucket_rate_map);
        }
    }
    let days_after_30_yrs = rbdate::num_days_start_to_end(
        *config_params.as_on_date(),
        rbdate::incr_dt_by_mon_presrv_eom(*config_params.as_on_date(), 360)
            .expect("Error while incrementing 30 years from now."),
    );
    let last_dt_for_daily_bucket =
        rbdate::incr_dt_by_mon_presrv_eom_checked(*config_params.as_on_date(), 12)
            .expect("Error while incrementing last date for daily bucket.");
    let mut days_in_a_year =
        rbdate::num_days_start_to_end(*config_params.as_on_date(), last_dt_for_daily_bucket);

    for bkt in 1..=days_in_a_year {
        buckets_data.insert(bkt, bkt);
    }
    let mut bkt_limit = days_in_a_year;
    let mut start_month_bkt = days_in_a_year + 1;
    if start_month_bkt == 366 {
        start_month_bkt +=1;
        buckets_data.insert(366, 366);
    }
    //Monthly Buckets ranging from (12-13 M) to (359-350 M)
    for month in 13..=360 {
        let month_end_n_date =
            rbdate::incr_dt_by_mon_presrv_eom_checked(*config_params.as_on_date(), month).unwrap();
        let res_days_month_bucket =
            rbdate::num_days_start_to_end(*config_params.as_on_date(), month_end_n_date);
        for bkt in (bkt_limit + 1)..=res_days_month_bucket {
            buckets_data.insert(bkt, start_month_bkt);
        }
        start_month_bkt += 1;
        bkt_limit = res_days_month_bucket;
    }
    //Making use of BaseCurve functions for InterPolation
    for (_, bucket_map) in spread_rate_map.iter_mut() {
        let mut data_vec = IntermediateBMPoints::new();
        for (_, data) in bucket_map.iter_mut() {
            let int_bm_data = IntermediateBMPoint {
                vertex: data.vertex,
                uom: data.uom.to_string(),
                rate: data.rate,
                days_diff: get_days_diff(
                    *config_params.as_on_date(),
                    data.uom.to_string(),
                    data.vertex,
                )
                .unwrap_or(0),
                month: (get_days_diff(
                    *config_params.as_on_date(),
                    data.uom.to_string(),
                    data.vertex,
                )
                .unwrap_or(0) as f64
                    / 365.0
                    * 12.0)
                    .round() as i64,
            };
            data_vec.push(int_bm_data);
        }
        data_vec.sort_by(|a: &IntermediateBMPoint, b| a.days_diff.cmp(&b.days_diff));

        if !config_params.is_interpol_low_ver_req() {
            if data_vec[0].vertex != 1 && data_vec[0].uom != "D" {
                data_vec.push(IntermediateBMPoint {
                    vertex: 1,
                    uom: "D".to_string(),
                    rate: data_vec[0].rate,
                    days_diff: 1,
                    month: 0,
                });
            }
        }

        data_vec.sort_by(|a: &IntermediateBMPoint, b| a.days_diff.cmp(&b.days_diff));
        for day in 1..=days_after_30_yrs {
            let spread = calc_yield_rate(&mut data_vec, day, false, "M".to_string()).unwrap_or(0.0);
            let bucketid: i64 = if day <= days_in_a_year {
                day
            } else if day >= days_after_30_yrs {
                714
            } else {
                //Get bucket for the residual days
                *buckets_data
                    .get(&day)
                    .unwrap_or_else(|| panic!("Could not get BucketID for Res-Days: {}", day))
            };
            bucket_map.insert(
                //Last BucketID=714
                (day, if bucketid > 714 { 714 } else { bucketid }),
                BmMaster {
                    vertex: 0,
                    uom: "".to_string(),
                    rate: spread,
                },
            );
        }
    }
}
