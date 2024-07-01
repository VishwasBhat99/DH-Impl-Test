use configuration_parameters::ConfigurationParameters;
use rbdate::incr_dt_by_mon_presrv_eom;
use rbdate::NaiveDate;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use structs::MatSlab;

fn get_num_days(info: &str, as_on_date: &NaiveDate) -> i64 {
    if info.contains("D") {
        let period: i64 = info
            .trim_matches('D')
            .parse::<i64>()
            .expect("Invalid from day format");
        return period as i64;
    } else if info.contains("M") {
        let period: usize = info
            .trim_matches('M')
            .parse::<usize>()
            .expect("Invalid from month format");
        let new_date = incr_dt_by_mon_presrv_eom(*as_on_date, period)
            .expect("Cannot add month to as on date as per prd slab config");
        return rbdate::num_days_start_to_end(*as_on_date, new_date);
    } else if info.contains("Y") {
        let period: usize = info
            .trim_matches('Y')
            .parse::<usize>()
            .expect("Invalid from year format");
        let new_date = incr_dt_by_mon_presrv_eom(*as_on_date, period * 12)
            .expect("Cannot add month to as on date as per prd slab config");
        return rbdate::num_days_start_to_end(*as_on_date, new_date);
    } else {
        panic!("Invalid period type in prd config file.");
    }
}

pub fn get_mat_slabs(
    config_params: &ConfigurationParameters,
    start_date: NaiveDate,
) -> Vec<MatSlab> {
    let mat_conf_file = match File::open(config_params.mat_config_file()) {
        Ok(mat_conf_file) => mat_conf_file,
        Err(error) => panic!("{}", error),
    };
    let mat_conf_reader = BufReader::new(mat_conf_file);
    let mut slabs: Vec<MatSlab> = Vec::new();
    for line in mat_conf_reader.lines() {
        match line {
            Ok(slab_info) => {
                let info: Vec<&str> = slab_info.split('|').collect();
                let from_prd: String = format!("{}{}", info[2].trim(), info[3].trim());
                let to_prd: String = format!("{}{}", info[4].trim(), info[5].trim());
                let new_slab = MatSlab {
                    mat_id: info[0].parse().unwrap_or(0),
                    mat_name: info[1].to_string(),
                    from_days: get_num_days(&from_prd, &start_date),
                    to_days: get_num_days(&to_prd, &start_date),
                    threshold_ir: info[6].parse().unwrap_or(0.0),
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

pub fn get_mat_bkt_id(mat_slabs: &Vec<MatSlab>, mat_tenor: i64) -> (i64, String, f64) {
    for slab in mat_slabs {
        if mat_tenor > slab.from_days && mat_tenor <= slab.to_days {
            return (slab.mat_id, slab.mat_name.to_string(), slab.threshold_ir);
        }
    }
    return (0, "Undefined".to_string(), 0.0);
}
