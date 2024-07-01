use self::account::*;
use crate::configuration_parameters::ConfigurationParameters;
use crate::csv::ReaderBuilder;
use crate::macros;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

mod account;

pub fn process(config_params: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let mut tot_acc_encntrd = 0;
    let mut tot_amt = 0.0;
    let mut skip_rec_count = 0;

    //Two Output(Maturity and Non-maturity)
    let mat_op_path = format!("{}-maturity.txt", &config_params.output_file_path());
    let non_mat_op_path = format!("{}-non-maturity.txt", &config_params.output_file_path());
    let mut mat_writer = get_writer(&mat_op_path);
    let mut non_mat_writer = get_writer(&non_mat_op_path);

    let mut bills_data_map: HashMap<String, Vec<BillsReqData>> = HashMap::new();
    let mut bills_sorted_data: HashMap<String, Vec<BillsReqData>> = HashMap::new();
    let mut bills_reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(config_params.bills_input_file())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` due to `{}`",
            config_params.bills_input_file(),
            error
        ),
    };
    for (line_num, lines) in bills_reader.deserialize().enumerate().skip(1) {
        let bills_data: BillsData = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read line for line_num: `{}` due to :`{}` From Bills-Input File.",
                    line_num + 1,
                    error
                );
                continue;
            }
        };
        let mut bills_req_data = BillsReqData::new(&bills_data, *config_params.as_on_date());
        let mut key_1 = bills_data.acc_no.to_owned();
        key_1.pop();
        //Mat-Date Read in DD-MON-YY and wriiten in DD-MM-YYYY format
        let mat_date = rbdate::NaiveDate::parse_from_str(&bills_data.mat_dt, "%d-%b-%y")
            .unwrap_or(rbdate::NaiveDate::from_ymd(1970, 1, 1));
        bills_req_data.mat_dt = mat_date.format("%d-%m-%Y").to_string();

        //Overdue Bills should not be considered
        if mat_date < *config_params.as_on_date() {
            log_warn!(
                log,
                "Skipping Overdue Bills Data for Acc-ID: `{}` and Bill-ID: `{}` as Mat-Date: `{}` found.",
                bills_data.acc_no.to_owned(),
                bills_data.bill_id,
                bills_data.mat_dt
            );
            continue;
        }
        bills_data_map
            .entry(key_1)
            .and_modify(|data| data.push(bills_req_data.to_owned()))
            .or_insert_with(|| vec![bills_req_data.to_owned()]);
    }

    let mut od_reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(config_params.od_input_file())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` due to `{}`",
            config_params.od_input_file(),
            error
        ),
    };

    //Sorting Bill-Data by Mat-Date
    for (k, v) in bills_data_map {
        let mut sorted_vec = v.to_owned();
        sorted_vec.sort_by_key(|val| val.mat_dt.to_string());
        bills_sorted_data.insert(k, sorted_vec);
    }

    for (line_num, lines) in od_reader.deserialize().enumerate() {
        tot_acc_encntrd += 1;
        let mut od_data: ODData = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read line for line_num: `{}` due to :`{}` From OD-Input File.",
                    line_num + 1,
                    error
                );
                skip_rec_count += 1;
                continue;
            }
        };
        if !config_params
            .filter_bills_accs()
            .contains(&od_data.a1.trim().to_uppercase().to_string())
        {
            log_debug!(
                log,
                "Skipping Account: `{}` with Group: `{}`",
                od_data.key_1,
                od_data.a1
            );
            skip_rec_count += 1;
            continue;
        }
        let key_1 = od_data.key_1[3..]
            .to_string()
            .trim_start_matches(|c: char| c == '0')
            .to_string();
        tot_amt += od_data
            .curr_bal
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0)
            .abs();

        //If OD Account not found in Exim-Bills
        if !bills_sorted_data.contains_key(&key_1) {
            let od_output = format_output(&od_data);
            let non_mat_output = format!(
                "{}|{}",
                od_output,
                od_data
                    .curr_bal
                    .to_string()
                    .parse::<f64>()
                    .unwrap_or(0.0)
                    .abs()
            );
            writeln!(non_mat_writer, "{}", non_mat_output)
                .expect("Error in writing non maturity output!!");
        }
        //If OD Account found in Exim-Bills
        else {
            let mut out_bal = od_data
                .curr_bal
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0)
                .abs();
            let bills_data = bills_sorted_data
                .get(&key_1)
                .expect("Could not found OD Data in Bills");
            let mut is_acc_adjusted = false;
            //Writing Maturity Output
            for val in bills_data.iter() {
                if is_acc_adjusted {
                    //Bills Data after adjusted OD Account shall be ignored
                    break;
                }
                let mut op_amt = 0.0;
                let bill_amt = val
                    .curr_out_bal_lcy
                    .to_string()
                    .parse::<f64>()
                    .unwrap_or(0.0)
                    .abs();
                od_data.a13 = val.bill_id.to_string();
                od_data.a16 = val.nego_strt_dt.to_string();

                (op_amt, out_bal) = if bill_amt <= out_bal {
                    (bill_amt, out_bal - bill_amt)
                } else {
                    log_info!(
                        _diag_log,
                        "Adjusted Account: `{}` with Bill-Amount: `{}` from Bill: `{}`",
                        od_data.key_1,
                        out_bal,
                        val.bill_id
                    );
                    is_acc_adjusted = true;
                    (out_bal, 0.0)
                };
                let od_output = format_output(&od_data);
                let mat_output = format!(
                    "{}|{}|{}|{}|{}",
                    od_output,
                    val.bill_ccy.trim(),
                    op_amt,
                    val.int_accrued.trim(),
                    val.mat_dt.trim()
                );
                writeln!(mat_writer, "{}", mat_output)
                    .expect("Error in writing non maturity output!!");
            }
            //Remaining Amount should be written to non-maturity
            if out_bal != 0.0 {
                let od_output = format_output(&od_data);
                let adj_output = format!("{}|{}", od_output, out_bal);
                writeln!(non_mat_writer, "{}", adj_output)
                    .expect("Error in writing non maturity output!!");
                log_info!(
                    _diag_log,
                    "Excessive OD-Amount: `{}` written to Non-Maturity Output for Account: `{}`",
                    out_bal,
                    od_data.key_1
                );
            }
        }
    }
    // Generate Health Check Report
    let health_report = health_report::HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skip_rec_count,
        skip_rec_count,
        tot_amt,
        tot_amt,
        0,
    );
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file_path());
}

pub fn get_writer(file_path: &str) -> BufWriter<File> {
    match sdb_io::buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!("Unable to create file `{}` due to : {}", file_path, error),
    }
}
