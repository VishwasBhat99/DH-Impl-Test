use rbdate::incr_dt_by_mon_presrv_eom;
use rbdate::NaiveDate;
use rbdate::NaiveDateTime;
use sdb_dyn_proto_rdr::reader::account_with_cfs::get_field_value;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use sdb_dyn_proto_rdr::reader::Reader;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct RangeSlab {
    id: String,
    from: f64,
    to: f64,
}

pub struct MapSlab {
    id: String,
    val: String,
}

pub fn get_dim(
    fields: &Vec<String>,
    dim_type: &str,
    account: &AccountWithCFs,
    method_reader: &Reader,
    num_slabs: &Vec<RangeSlab>,
    prd_slabs: &Vec<RangeSlab>,
    map_slabs: &Vec<MapSlab>,
) -> String {
    // TODO: Use ENUM for dimention type
    match dim_type {
        "NUMSLAB" => {
            let key = &fields[0];
            let value = match get_field_value(&account, &method_reader, key.to_string()) {
                Ok(value) => value.to_string().parse::<f64>().unwrap_or(0.0),
                Err(_error) => panic!("{}", _error),
            };

            for val in num_slabs {
                if value >= val.from && value < val.to {
                    return val.id.to_string();
                } else {
                    continue;
                }
            }
            return "NA".to_string();
        }
        "PRDSLAB" => {
            let st_dt_key = &fields[0];
            let end_dt_key = &fields[1];
            let st_dt = match get_field_value(&account, &method_reader, st_dt_key.to_string()) {
                Ok(value) => value,
                Err(_error) => panic!("{}", _error),
            };
            let end_dt = match get_field_value(&account, &method_reader, end_dt_key.to_string()) {
                Ok(value) => value,
                Err(_error) => panic!("{}", _error),
            };
            let st_dt = naivedate_from_timestamp(st_dt.to_string().parse::<i64>().unwrap_or(0));
            let end_dt = naivedate_from_timestamp(end_dt.to_string().parse::<i64>().unwrap_or(0));
            let value = if end_dt > st_dt {
                rbdate::num_days_start_to_end(st_dt, end_dt) as f64
            } else {
                rbdate::num_days_start_to_end(end_dt, st_dt) as f64
            };

            for val in prd_slabs {
                if value >= val.from && value < val.to {
                    return val.id.to_string();
                }
            }
            return "NA".to_string();
        }
        "SRCMAP" => {
            let key = &fields[0];
            let value = match get_field_value(&account, &method_reader, key.to_string()) {
                Ok(value) => value,
                Err(_error) => panic!("{}", _error),
            };

            for val in map_slabs {
                if &value == &val.val {
                    return val.id.to_string();
                }
            }
            return "NA".to_string();
        }
        "NA" => {
            return "0".to_string();
        }
        _ => panic!("Unknown dim type encountered in config.json."),
    }
}

pub fn get_num_slabs(path: &str) -> Vec<RangeSlab> {
    let mut slabs: Vec<RangeSlab> = Vec::new();
    let input_file = match File::open(path) {
        Ok(input_file) => input_file,
        Err(error) => panic!("{}", error),
    };
    let reader = BufReader::new(input_file);
    for line in reader.lines() {
        match line {
            Ok(slab_info) => {
                let info: Vec<&str> = slab_info.split('|').collect();
                let new_slab = RangeSlab {
                    id: info[0].to_string(),
                    from: info[1].parse::<f64>().expect("Invalid from slab config"),
                    to: info[2].parse::<f64>().expect("Invalid to slab config"),
                };
                slabs.push(new_slab)
            }
            Err(error) => {
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
    }
    slabs
}

pub fn get_prd_slabs(path: &str, as_on_date: &NaiveDate) -> Vec<RangeSlab> {
    let mut slabs: Vec<RangeSlab> = Vec::new();
    let input_file = match File::open(path) {
        Ok(input_file) => input_file,
        Err(error) => panic!("{}", error),
    };
    let reader = BufReader::new(input_file);
    for line in reader.lines() {
        match line {
            Ok(slab_info) => {
                let info: Vec<&str> = slab_info.split('|').collect();
                let from_days = get_days(info[1], as_on_date);
                let to_days = get_days(info[2], as_on_date);
                let new_slab = RangeSlab {
                    id: info[0].to_string(),
                    from: from_days,
                    to: to_days,
                };
                slabs.push(new_slab)
            }
            Err(error) => {
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
    }
    slabs
}

fn get_days(info: &str, as_on_date: &NaiveDate) -> f64 {
    let mut alpha_code: Vec<&str> = info.split(|c: char| c.is_numeric()).collect();
    alpha_code.retain(|&x| x != "");
    let mut num_code: Vec<&str> = info.split(|c: char| c.is_alphabetic()).collect();
    num_code.retain(|&x| x != "");
    let mut days = 0.0;
    for (i, num_val) in num_code.iter().enumerate() {
        let period = num_val.to_string() + alpha_code[i];
        days += num_days(&period, as_on_date);
    }
    days
}
fn num_days(info: &str, as_on_date: &NaiveDate) -> f64 {
    if info.contains("D") {
        let period: i64 = info
            .trim_matches('D')
            .parse::<i64>()
            .expect("Invalid from day format");
        return period as f64;
    } else if info.contains("M") {
        let period: usize = info
            .trim_matches('M')
            .parse::<usize>()
            .expect("Invalid from month format");
        let new_date = incr_dt_by_mon_presrv_eom(*as_on_date, period)
            .expect("Cannot add month to as on date as per prd slab config");
        return rbdate::num_days_start_to_end(*as_on_date, new_date) as f64;
    } else if info.contains("Y") {
        let period: usize = info
            .trim_matches('Y')
            .parse::<usize>()
            .expect("Invalid from year format");
        let new_date = incr_dt_by_mon_presrv_eom(*as_on_date, period * 12)
            .expect("Cannot add month to as on date as per prd slab config");
        return rbdate::num_days_start_to_end(*as_on_date, new_date) as f64;
    } else {
        panic!("Invalid period type in prd config file.");
    }
}
pub fn get_map_slabs(path: &str) -> Vec<MapSlab> {
    let mut slabs: Vec<MapSlab> = Vec::new();
    let input_file = match File::open(path) {
        Ok(input_file) => input_file,
        Err(error) => panic!("{}", error),
    };
    let reader = BufReader::new(input_file);
    for line in reader.lines() {
        match line {
            Ok(slab_info) => {
                let info: Vec<&str> = slab_info.split('|').collect();
                let new_slab = MapSlab {
                    id: info[0].to_string(),
                    val: info[1].to_string(),
                };
                slabs.push(new_slab)
            }
            Err(error) => {
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
    }
    slabs
}

pub fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}
