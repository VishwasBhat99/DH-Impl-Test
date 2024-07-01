use super::structs::{LLGKey, LLGVal};
use rbdate::NaiveDate;
use sdb_io::buf_file_wrtr;
use std::collections::HashMap;
use std::io::Write;

pub fn write_to_file(
    acc_level_op: String,
    summary_map: HashMap<LLGKey, LLGVal>,
    output_path: &str,
    as_on_date: &NaiveDate,
) {
    let acc_level_path = output_path.to_string() + &"_acc_level_data.txt".to_string();
    let summary_path = output_path.to_string() + &"_summary_data.txt".to_string();
    let mut acc_level_writer = match buf_file_wrtr(&acc_level_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` : {}",
            acc_level_path, error,
        ),
    };
    let mut summary_writer = match buf_file_wrtr(&summary_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` : {}",
            summary_path, error,
        ),
    };
    // Writing acc level data
    match acc_level_writer.write_all(acc_level_op.as_bytes()) {
        Ok(_) => println!("Successfully processed all accounts."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`: {}.",
            acc_level_path, error,
        ),
    }

    // Writing summary data
    for (llg_key, llg_val) in summary_map.iter() {
        let weighted_dur = if llg_val.tot_bal_ccy.parse::<f64>().unwrap_or(0.0) == 0.0 {
            0.0
        } else {
            llg_val.weighted_dur.parse::<f64>().unwrap_or(0.0)
                / llg_val.tot_bal_ccy.parse::<f64>().unwrap_or(0.0)
        };

        write!(
            summary_writer,
            "{}|{}|{}|{}|{}|{}\n",
            as_on_date.to_string(),
            llg_key.llg,
            llg_key.ccy,
            llg_val.tot_bal_ccy,
            llg_val.tot_bal_hcy,
            weighted_dur
        )
        .expect("summary file writing error");
    }
}
