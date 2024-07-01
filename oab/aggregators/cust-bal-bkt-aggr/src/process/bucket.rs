use crate::configuration_parameters::ConfigurationParameters;
use crate::process::account::*;
use std::collections::HashMap;

pub fn get_buckets_data(
    config_params: &ConfigurationParameters,
    month_bucketing_start_day: i64,
) -> HashMap<u32, u32> {
    let as_on_date = *config_params.as_on_date();
    let mut buckets_data: HashMap<u32, u32> = HashMap::new();
    for bkt in 1..=184 {
        if bkt > month_bucketing_start_day as u32 {
            //185 corresponds to 7M Bucket(First Monthly Bucket)
            buckets_data.insert(bkt, 185);
        } else {
            buckets_data.insert(bkt, bkt);
        }
    }
    let mut bkt_limit = 184;
    let mut start_month_bkt = 185;
    for month in 7..=24 {
        let month_end_n_date =
            rbdate::incr_dt_by_mon_presrv_eom_checked(as_on_date, month).unwrap();
        let res_days_month_bucket = rbdate::num_days_start_to_end(as_on_date, month_end_n_date);
        for bkt in (bkt_limit + 1)..=res_days_month_bucket {
            buckets_data.insert(bkt as u32, start_month_bkt);
        }
        start_month_bkt += 1;
        bkt_limit = res_days_month_bucket;
    }

    buckets_data
}

pub fn get_amts_in_buckets(
    config_params: &ConfigurationParameters,
    final_input_map: &mut HashMap<(String, i64, u32, bool), f64>,
    inp_acc: Account,
    stable_vec: &mut Vec<f64>,
    less_stable_vec: &mut Vec<f64>,
    stable_amt: &mut f64,
    is_nwd: bool,
) {
    let mut def_amt = 0.0;
    //In case of ASC: Traverse from 1(Min-BktID)-->23(Max-BktID)
    if config_params.allocation_order() == "ASC" {
        for bkt in 1..=203 {
            let amt = final_input_map
                .get_mut(&(inp_acc.file_id.to_string(), inp_acc.cust_id, bkt, is_nwd))
                .unwrap_or(&mut def_amt)
                .to_owned();
            final_input_map.remove_entry(&(
                inp_acc.file_id.to_string(),
                inp_acc.cust_id,
                bkt,
                is_nwd,
            ));
            if amt <= 0.0 {
                if *stable_amt >= amt.abs() {
                    stable_vec.push(amt);
                    less_stable_vec.push(0.0);
                    *stable_amt -= amt.abs();
                } else {
                    stable_vec.push(-*stable_amt);
                    less_stable_vec.push(amt + *stable_amt);
                    *stable_amt = 0.0;
                }
                continue;
            }
            if *stable_amt >= amt {
                stable_vec.push(amt);
                less_stable_vec.push(0.0);
                *stable_amt -= amt;
            } else {
                stable_vec.push(*stable_amt);
                less_stable_vec.push(amt - *stable_amt);
                *stable_amt = 0.0;
            }
        }
    }
    //In case of DESC: Traverse from 203(Max-BktID)-->1(Min-BktID)
    else {
        let mut bkt = 203;
        while bkt > 0 {
            let amt = final_input_map
                .get_mut(&(inp_acc.file_id.to_string(), inp_acc.cust_id, bkt, is_nwd))
                .unwrap_or(&mut def_amt)
                .to_owned();
            final_input_map.remove_entry(&(
                inp_acc.file_id.to_string(),
                inp_acc.cust_id,
                bkt,
                is_nwd,
            ));
            if amt <= 0.0 {
                stable_vec.push(0.0);
                less_stable_vec.push(0.0);
                bkt -= 1;
                continue;
            }
            if *stable_amt >= amt {
                stable_vec.push(amt);
                less_stable_vec.push(0.0);
                *stable_amt -= amt;
            } else {
                stable_vec.push(*stable_amt);
                less_stable_vec.push(amt - *stable_amt);
                *stable_amt = 0.0;
            }
            bkt -= 1;
        }
        stable_vec.reverse();
        less_stable_vec.reverse();
    }
}
