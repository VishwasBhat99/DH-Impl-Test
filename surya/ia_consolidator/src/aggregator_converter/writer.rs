use super::file_map::{BucketValue, SummaryValue};
use sdb_io::buf_file_wrtr;
use std::collections::HashMap;
use std::env;
use std::io::prelude::*;

pub fn write_to_file(amt: &str, rate: &str, map: &HashMap<Vec<String>, BucketValue>, output: &str) {
    let mut amt_writer = match buf_file_wrtr(&amt, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                output,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let mut rate_writer = match buf_file_wrtr(&rate, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                output,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    for (key, value) in map {
        let write_key = key[0].to_string()
            + &"|".to_string()
            + &key[1].to_string()
            + &"|".to_string()
            + &key[2].to_string()
            + &"|".to_string()
            + &key[3].to_string()
            + &"|".to_string()
            + &key[4].to_string()
            + &"|".to_string()
            + &key[5].to_string()
            + &"|".to_string()
            + &key[6].to_string()
            + &"|".to_string()
            + &key[7].to_string()
            + &"|".to_string()
            + &key[8].to_string();
        let mut write_amt: String = "".to_string();
        let mut write_rate: String = "".to_string();
        let len = value.principal_vec.len();
        for bucket_no in 0..len {
            let amt = &value.principal_vec[bucket_no].parse::<f64>().unwrap();
            write_amt = write_amt + &"|".to_string() + &value.principal_vec[bucket_no].to_string();
            if *amt == 0.0 {
                write_rate = write_rate + &"|0.0".to_string();
            } else {
                write_rate = write_rate
                    + &"|".to_string()
                    + &(&value.rate_vec[bucket_no].parse::<f64>().unwrap_or(0.0)
                        / &value.principal_vec[bucket_no].parse::<f64>().unwrap_or(0.0))
                        .to_string();
            }
        }
        match amt_writer.write((write_key.to_string() + &write_amt + &"\n").as_bytes()) {
            Ok(_val) => {}
            Err(err) => println!("Error writing to output file. Error: {}", err),
        }
        match rate_writer.write((write_key.to_string() + &write_rate + &"\n").as_bytes()) {
            Ok(_val) => {}
            Err(err) => println!("Error writing to output file. Error: {}", err),
        }
    }
}

pub fn write_to_file_smry(smry: &str, map: &HashMap<Vec<String>, SummaryValue>, output: &str) {
    let mut smry_writer = match buf_file_wrtr(&smry, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                output,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    for (key, value) in map {
        let write_key = key[0].to_string()
            + &"|".to_string()
            + &key[1].to_string()
            + &"|".to_string()
            + &key[2].to_string()
            + &"|".to_string()
            + &key[3].to_string()
            + &"|".to_string()
            + &key[4].to_string()
            + &"|".to_string()
            + &key[5].to_string()
            + &"|".to_string()
            + &key[6].to_string()
            + &"|".to_string()
            + &key[7].to_string()
            + &"|".to_string()
            + &key[8].to_string();
        let mut write_smry: String = "".to_string();
        if value.rate_smry == 0.0 {
            write_smry = "|".to_string() + &value.principal_smry.to_string() + &"|0.0".to_string();
        } else {
            write_smry = "|".to_string()
                + &value.principal_smry.to_string()
                + &"|".to_string()
                + &(&value.rate_smry / &value.principal_smry).to_string();
        }
        match smry_writer.write((write_key.to_string() + &write_smry + &"\n").as_bytes()) {
            Ok(_val) => {}
            Err(err) => println!("Error writing to output file. Error: {}", err),
        }
    }
}
