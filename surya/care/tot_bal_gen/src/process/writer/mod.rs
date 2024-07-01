use self::output::Output;
use protobuf::Message;
use sdb_io::buf_file_wrtr;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;

mod output;

// without cf write
pub fn write_file_data(
    aggr_data: &mut HashMap<String, ((f64, i64), (f64, i64))>,
    op_amt: &mut f64,
    limit_amt: &mut f64,
    op_path: &str,
    is_limit_req: bool,
) {
    let mut output_file = match buf_file_wrtr(op_path, None) {
        Ok(create) => create,
        Err(error) => {
            panic!(
                "Could not create output file: `{}` on location `{}`: {}.",
                op_path,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    for (key, ((amt, count), (limit_amount, limit_count))) in aggr_data.drain() {
        if is_limit_req {
            write!(
                output_file,
                "{}|{}|{}|{}|{}\n",
                key, count, amt, limit_count, limit_amount
            )
            .expect("Unable to generate summary file.");
        } else {
            write!(output_file, "{}|{}|{}\n", key, count, amt)
                .expect("Unable to generate summary file.");
        }
        *op_amt += amt;
        *limit_amt += limit_amount;
    }
}

// with cf write
pub fn write_aggr_smry(
    aggr_data: &mut HashMap<String, ((f64, i64), (f64, i64))>,
    op_amt: &mut f64,
    limit_bal: &mut f64,
    exp_base_map: HashMap<String, (f64, f64)>,
    op_path: &str,
    cf_op_path: &str,
    is_limit_req: bool,
) {
    let mut output_file = match buf_file_wrtr(op_path, None) {
        Ok(create) => create,
        Err(error) => {
            panic!(
                "Could not create output file: `{}` on location `{}`: {}.",
                op_path,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let cf_output_file = File::create(cf_op_path).expect("unable to create output file");

    for (key, ((amt, count), (limit_amt, _limit_count))) in aggr_data.drain() {
        let (prev_exp_amt, prev_limit_amt) = exp_base_map.get(&key).unwrap_or(&(0.0, 0.0));
        let exp_status = if *prev_exp_amt < amt {
            "INC"
        } else if *prev_exp_amt > amt {
            "DEC"
        } else if *prev_exp_amt == amt {
            "SAME"
        } else {
            "NA"
        };
        let mut limit_status = "NA";
        if is_limit_req {
            limit_status = if *prev_limit_amt < limit_amt {
                "INC"
            } else if *prev_limit_amt > limit_amt {
                "DEC"
            } else if *prev_limit_amt == limit_amt {
                "SAME"
            } else {
                "NA"
            };
            write!(
                output_file,
                "{}|{}|{}|{}|{}|{}\n",
                key, count, amt, exp_status, limit_amt, limit_status
            )
            .expect("Unable to generate summary file.");
        } else {
            write!(output_file, "{}|{}|{}|{}\n", key, count, amt, exp_status)
                .expect("Unable to generate summary file.");
        }
        let cf_op: Output = get_write_date(
            key,
            amt,
            count,
            exp_status.to_string(),
            limit_amt,
            limit_status.to_string(),
        );
        write_to_file(&cf_output_file, cf_op);
        *op_amt += amt;
        *limit_bal += limit_amt
    }
}

fn get_write_date<'a>(
    cust_id: String,
    tot_bal: f64,
    count: i64,
    exp_status: String,
    limit_bal: f64,
    limit_status: String,
) -> Output {
    let mut output_data = Output::new();
    output_data.set_cust_id(cust_id);
    output_data.set_count(count);
    output_data.set_tot_bal(tot_bal);
    output_data.set_exp_status(exp_status);
    output_data.set_limit_bal(limit_bal);
    output_data.set_limit_status(limit_status);
    output_data
}

fn write_to_file(mut output_file: &File, output_data: Output) {
    let account_byte_info = output_data
        .write_length_delimited_to_bytes()
        .expect("unable convert into bytes");
    output_file
        .write_all(&account_byte_info)
        .expect("unable to write to output file");
}
