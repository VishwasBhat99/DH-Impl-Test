use self::account::*;
use self::index::*;
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use csv::ReaderBuilder;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::BufRead, BufReader, Write};
use std::process::Command;

mod account;
mod index;

pub fn process(config_params: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let mut tot_agg_acc_encntrd = 0;
    let mut tot_amt_in_agg_ip = 0.0;
    let mut tot_amt_in_agg_op = 0.0;
    let mut skip_agg_rec_count = 0;
    let mut tot_cap_acc_encntrd = 0;
    let mut tot_amt_in_cap_ip = 0.0;
    let mut tot_amt_in_cap_op = 0.0;
    let mut skip_cap_rec_count = 0;

    //Getting Indexes where amounts should be swapped
    let indexes = get_indexes(config_params.as_on_date());

    //Init Writers
    let aggr_oppath = &config_params.aggr_input_file().replace(
        ".txt",
        &format!("_updated_{}.txt", config_params.as_on_date().format("%m%Y")),
    );
    let mut aggr_writer = match buf_file_wrtr(aggr_oppath, None) {
        Ok(output_file) => output_file,
        Err(error) => panic!(
            "Unable to create output file: `{}` due to : `{}`",
            aggr_oppath, error,
        ),
    };

    let commoncap_oppath = &config_params
        .common_cap_file()
        .replace(
            ".txt",
            &format!("_{}-{}.txt", indexes.from_year, indexes.to_year),
        )
        .replace("common_cap_op", "updated-common_cap_op");
    let mut commoncap_writer = match buf_file_wrtr(commoncap_oppath, None) {
        Ok(output_file) => output_file,
        Err(error) => panic!(
            "Unable to create output file: `{}` due to : `{}`",
            commoncap_oppath, error,
        ),
    };

    //Reading Income Master File
    let inc_exp_master =
        File::open(config_params.income_master_file()).expect("Could Not Read Income Master File");
    let income_master_reader = BufReader::new(inc_exp_master);
    let mut income_master_map: HashMap<String, f64> = HashMap::new();
    for (line_no, line) in income_master_reader.lines().enumerate() {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                log_error!(
                    log,
                    "Cannot read line {} from Aggr Input file: {:?}",
                    line_no,
                    error
                );
                continue;
            }
        };
        //ACCT_NO|GL_CODE|APR_AMT|MAY_AMT|JUN_AMT|JUL_AMT|AUG_AMT|SEP_AMT|OCT_AMT|NOV_AMT|DEC_AMT|JAN_AMT|FEB_AMT|MAR_AMT|IN_ADVSHEET|SECTOR|SUB_SECTOR|SUB_SECTOR2|CBS_CODE|CATG|CUST_NO|JL_LOD|STAFF_SHG
        let income_fields: Vec<&str> = acc_info.split('|').collect();
        let mut key_1 = income_fields[0].to_string();
        key_1.pop();
        let income = income_fields[indexes.income_master_index as usize]
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0);
        income_master_map.insert(key_1, income);
    }

    //Read and Write Aggr File
    let aggrfile_ippath = &config_params.aggr_input_file().replace(
        ".txt",
        &format!("_{}.txt", config_params.as_on_date().format("%m%Y")),
    );
    let mut aggr_file_reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(aggrfile_ippath)
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not find aggr input file: `{}` due to `{}`.",
            aggrfile_ippath, error
        ),
    };
    for (line_no, line) in aggr_file_reader.deserialize().enumerate() {
        tot_agg_acc_encntrd += 1;
        let mut aggr_data: AggrData = match line {
            Ok(acc) => acc,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{:?}` : {}",
                    aggrfile_ippath,
                    line_no + 1,
                    error
                );
                skip_agg_rec_count += 1;
                AggrData::def()
            }
        };
        tot_amt_in_agg_ip += aggr_data.tot_int_inc;
        let acc_id = aggr_data.account_id[3..]
            .to_string()
            .trim_start_matches(|c: char| c == '0')
            .to_string();
        aggr_data.tot_int_inc = *income_master_map
            .get(&acc_id)
            .unwrap_or(&aggr_data.tot_int_inc);
        aggr_writer
            .write_all(format_aggr_output(&aggr_data).as_bytes())
            .expect("Error writing Aggr Output File!!");
        tot_amt_in_agg_op += aggr_data.tot_int_inc;
    }

    //Read and Write Common Cap File
    let commoncap_ippath = &config_params.common_cap_file().replace(
        ".txt",
        &format!("_{}-{}.txt", indexes.from_year, indexes.to_year),
    );
    let commoncap_reader = match new_buf_rdr(commoncap_ippath) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not find common-cap file: `{}` due to : {}.",
            commoncap_ippath, error
        ),
    };
    for (line_no, line) in commoncap_reader.lines().enumerate() {
        tot_cap_acc_encntrd += 1;
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                skip_cap_rec_count += 1;
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{:?}` : {}",
                    config_params.common_cap_file(),
                    line_no + 1,
                    error
                );
                continue;
            }
        };
        //ACCT_NO|APR_AMT|MAY_AMT|JUN_AMT|JUL_AMT|AUG_AMT|SEP_AMT|OCT_AMT|NOV_AMT|DEC_AMT|JAN_AMT|FEB_AMT|MAR_AMT|PROD_CODE|CAP_FREQ|AS_ON_DATE
        let mut common_cap_fields: Vec<&str> = acc_info.split('|').collect();
        let acc_id = common_cap_fields[0][3..]
            .to_owned()
            .to_string()
            .trim_start_matches(|c: char| c == '0')
            .to_string();
        let amt_to_be_updated = common_cap_fields
            .to_owned()
            .get_mut(indexes.common_cap_index as usize)
            .unwrap_or(&mut "NA")
            .parse::<f64>()
            .unwrap_or(0.0);
        let income_master_amt = income_master_map
            .get(&acc_id)
            .unwrap_or(&amt_to_be_updated)
            .to_string();
        common_cap_fields[indexes.common_cap_index as usize] = &income_master_amt;
        tot_amt_in_cap_ip += amt_to_be_updated;
        commoncap_writer
            .write_all(format_commoncap_output(&common_cap_fields.to_owned()).as_bytes())
            .expect("Error writing Aggr Output File!!");
        tot_amt_in_cap_op += common_cap_fields[indexes.common_cap_index as usize]
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0);
    }

    // Generate Health Check Reports
    let health_report_aggr_file = health_report::HealthReport::new(
        tot_agg_acc_encntrd,
        tot_agg_acc_encntrd - skip_agg_rec_count,
        skip_agg_rec_count,
        tot_amt_in_agg_ip,
        tot_amt_in_agg_op,
        0,
    );
    let health_report_commoncap_file = health_report::HealthReport::new(
        tot_cap_acc_encntrd,
        tot_cap_acc_encntrd - skip_cap_rec_count,
        skip_cap_rec_count,
        tot_amt_in_cap_ip,
        tot_amt_in_cap_op,
        0,
    );
    health_report_aggr_file.gen_health_rpt(aggrfile_ippath);
    health_report_commoncap_file.gen_health_rpt(commoncap_ippath);

    //Copying Output to Input so that the data is updated before next run
    if cfg!(target_os = "windows") {
        Command::new("move")
            .args([commoncap_oppath, commoncap_ippath])
            .output()
            .expect("failed to move output to input")
    } else {
        Command::new("mv")
            .args([commoncap_oppath, commoncap_ippath])
            .output()
            .expect("failed to move output to input")
    };
    if cfg!(target_os = "windows") {
        Command::new("move")
            .args([aggr_oppath, aggrfile_ippath])
            .output()
            .expect("failed to move output to input")
    } else {
        Command::new("mv")
            .args([aggr_oppath, aggrfile_ippath])
            .output()
            .expect("failed to move output to input")
    };
}
