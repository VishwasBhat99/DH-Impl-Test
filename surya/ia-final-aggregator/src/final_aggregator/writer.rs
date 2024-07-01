use super::organize::{BucketValue, LLGKey, SummaryValue};
use sdb_io::buf_file_wrtr;
use std::collections::HashMap;
use std::env;
use std::io::prelude::*;

pub fn write_to_file(
    op_path: &str,
    aggregate_map: HashMap<LLGKey, BucketValue>,
    smry_map: HashMap<LLGKey, SummaryValue>,
) {
    let principal_fle_path = op_path.to_string() + "_principal_amt.txt";
    let rate_fle_path = op_path.to_string() + "_rate.txt";
    let smry_fle_path = op_path.to_string() + "_smry.txt";

    let mut principal_writer = match buf_file_wrtr(&principal_fle_path, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                principal_fle_path,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let mut rate_writer = match buf_file_wrtr(&rate_fle_path, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                rate_fle_path,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let mut smry_writer = match buf_file_wrtr(&smry_fle_path, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                smry_fle_path,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    for (key, value) in aggregate_map {
        let len= value.principal_vec.len();
        let mut write_prin: String = "".to_string();
        let mut write_rate: String = "".to_string();
        let mut write_key = key.as_on.to_string()
            + "|"
            + &key.llg_id.to_string()
            + "|"
            + &key.ccy.to_string()
            + "|"
            + &key.ex_rt.to_string()
            + "|"
            + &key.bm_id.to_string()
            + "|"
            + &key.tenor.to_string()
            + "|";
        if value.principal_vec[0] == 0.0 {
            write_key += "0.0";
        } else {
            write_key += &(value.spread / value.principal_vec[0]).to_string();
        }
        write_key = write_key
            + "|"
            + &key.rep_dt.to_string()
            + "|"
            + &key.rep_freq.to_string();
        for bucket_no in 0..len {
            let amt = &value.principal_vec[bucket_no];
            write_prin =
                write_prin + &"|".to_string() + &value.principal_vec[bucket_no].to_string();
            if *amt == 0.0 {
                write_rate = write_rate + &"|0.0".to_string();
            } else {
                write_rate = write_rate
                    + &"|".to_string()
                    + &(&value.rate_vec[bucket_no] / &value.principal_vec[bucket_no]).to_string();
            }
        }
        match principal_writer.write((write_key.to_string() + &write_prin + &"\n").as_bytes()) {
            Ok(_) => {}
            Err(err) => println!("Error writing to principal output file. Error: {}", err),
        }
        match rate_writer.write((write_key.to_string() + &write_rate + &"\n").as_bytes()) {
            Ok(_) => {}
            Err(err) => println!("Error writing to rate output file. Error: {}", err),
        }
    }
    for (key, value) in smry_map {
        let mut write = key.as_on.to_string()
            + "|"
            + &key.llg_id.to_string()
            + "|"
            + &key.ccy.to_string()
            + "|"
            + &key.ex_rt.to_string()
            + "|"
            + &key.bm_id.to_string()
            + "|"
            + &key.tenor.to_string()
            + "|";
        if value.principal_smry == 0.0 {
            write += "0.0";
        } else {
            write += &(value.spread / value.principal_smry).to_string();
        }
        write = write
            + "|"
            + &key.rep_dt.to_string()
            + "|"
            + &key.rep_freq.to_string()
            + "|"
            + &value.principal_smry.to_string();
        if value.principal_smry == 0.0 {
            write += "|0.0";
        } else {
            write =
                write + &"|".to_string() + &(value.rate_smry / value.principal_smry).to_string();
        }
        match smry_writer.write((write.to_string() + &"\n").as_bytes()) {
            Ok(_) => {}
            Err(err) => println!("Error writing to output file. Error: {}", err),
        }
    }
}
