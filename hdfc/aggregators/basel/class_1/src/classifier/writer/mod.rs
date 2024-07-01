use super::structs::CustKey;
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub fn write_classified_data(
    config_params: &ConfigurationParameters,
    op: String,
    logger: &Logger,
    is_nwd_final: &str,
    cust_type: String,
    ret_cust_types: Vec<&str>,
    null_cust_typ_writer: &mut BufWriter<File>,
    unmapped_cust_id_writer: &mut BufWriter<File>,
    wd_ret_writer: &mut BufWriter<File>,
    wd_non_ret_writer: &mut BufWriter<File>,
    nwd_ret_writer: &mut BufWriter<File>,
    nwd_non_ret_writer: &mut BufWriter<File>,
) {
    if cust_type == "" {
        match null_cust_typ_writer.write(op.as_bytes()) {
            Ok(val) => {
                log_debug!(
                    logger,
                    "{} Bytes written to null customer type file created successfully.",
                    val
                );
            }
            Err(err) => log_warn!(
                logger,
                "Error writing to null customer type file. Error: {}",
                err
            ),
        }
    } else if ret_cust_types.contains(&cust_type.as_str()) && is_nwd_final == "FALSE" {
        let mut op_line = String::new();
        op_line.push_str("1|");
        op_line.push_str(&op);
        match wd_ret_writer.write(op_line.as_bytes()) {
            Ok(val) => {
                log_debug!(
                    logger,
                    "{} Bytes written to WD-Ret file created successfully.",
                    val
                );
            }
            Err(err) => log_warn!(
                logger,
                "Error writing to WD-Ret output file. Error: {}",
                err
            ),
        }
    } else if !ret_cust_types.contains(&cust_type.as_str()) && is_nwd_final == "FALSE" {
        match wd_non_ret_writer.write(op.as_bytes()) {
            Ok(val) => {
                log_debug!(
                    logger,
                    "{} Bytes written to WD-Non-Ret file created successfully.",
                    val
                );
            }
            Err(err) => log_warn!(
                logger,
                "Error writing to WD-Non-Ret output file. Error: {}",
                err
            ),
        }
    } else if ret_cust_types.contains(&cust_type.as_str()) && is_nwd_final == "TRUE" {
        let mut op_line = String::new();
        op_line.push_str("1|");
        op_line.push_str(&op);
        match nwd_ret_writer.write(op_line.as_bytes()) {
            Ok(val) => {
                log_debug!(
                    logger,
                    "{} Bytes written to NWD-Ret file created successfully.",
                    val
                );
            }
            Err(err) => log_warn!(
                logger,
                "Error writing to NWD-Ret output file. Error: {}",
                err
            ),
        }
    } else if cust_type == "NA" && config_params.is_acc_cust_type() == "NA" {
        match unmapped_cust_id_writer.write(op.as_bytes()) {
            Ok(val) => {
                log_debug!(
                    logger,
                    "{} Bytes written to unmapped cust id op file created successfully.",
                    val
                );
            }
            Err(err) => log_warn!(
                logger,
                "Error writing to unmapped cust id op file. Error: {}",
                err
            ),
        }
    } else {
        match nwd_non_ret_writer.write(op.as_bytes()) {
            Ok(val) => {
                log_debug!(
                    logger,
                    "{} Bytes written to NWD-Non-Ret file created successfully.",
                    val
                );
            }
            Err(err) => log_warn!(
                logger,
                "Error writing to NWD-Non-Ret output file. Error: {}",
                err
            ),
        }
    }
}

pub fn write_cust_data(
    cust_ids: Vec<CustKey>,
    config_params: &ConfigurationParameters,
    logger: &Logger,
) {
    // init cust id file
    let cust_id_op = format!("{}-cust-ids.txt", config_params.output_file_path());
    // init writers
    let mut cust_id_writer = match buf_file_wrtr(&cust_id_op, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                cust_id_op,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let mut op_line = String::new();
    for id in cust_ids {
        op_line.push_str(&id.cust_id);
        op_line.push('|');
        op_line.push_str(&id.currency);
        op_line.push('\n')
    }
    match cust_id_writer.write(op_line.as_bytes()) {
        Ok(val) => {
            log_debug!(
                logger,
                "{} Bytes written to customer id's file created successfully.",
                val
            );
        }
        Err(err) => log_warn!(
            logger,
            "Error writing to customer id's file. Error: {}",
            err
        ),
    }
}
