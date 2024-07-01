use chrono::Datelike;
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use pre_processor::output_field_name::AccFieldNames;
use rbdate::DateParser;
use sdb_io::buf_file_wrtr;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;

mod output_field_name;
mod writer;

pub fn process(config_param: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let mut output_data_mtn: Vec<AccFieldNames> = Vec::new();
    let mut output_data_other: Vec<AccFieldNames> = Vec::new();
    let date_parser = DateParser::new("%d-%b-%Y".to_string(), false);
    let mut ex_rt: HashMap<String, f64> = HashMap::new();
    let mut bloom_header_flag = true;
    let mut count = *config_param.order_number();

    let ason_date_year = config_param.as_on_date().year().to_string();
    let ason_date_cen: Vec<&str> = ason_date_year.split("").collect();
    let century = ason_date_cen[1].to_owned() + ason_date_cen[2];

    let mut op_writer = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            config_param.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };

    let exchange_rate_file = match new_buf_rdr(config_param.exchange_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_bloom_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in exchange_rate_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_bloom_file_path(),
                line_num + 1,
                error
            ),
        };
        let ex_rt_split: Vec<&str> = line.split("|").collect();
        ex_rt.insert(
            ex_rt_split[0].to_string() + ex_rt_split[1],
            ex_rt_split[2].parse::<f64>().unwrap_or(0.0),
        );
    }

    let mut bloomdata: Vec<Vec<String>> = Vec::new();
    let bloomberg_file = match new_buf_rdr(config_param.input_bloom_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_bloom_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in bloomberg_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_bloom_file_path(),
                line_num + 1,
                error
            ),
        };

        let bloom_data_split: Vec<&str> = line.split(",").collect();
        let bloom_string_data: Vec<String> = bloom_data_split
            .iter()
            .map(|data| data.to_string())
            .collect();
        bloomdata.push(bloom_string_data);
    }

    let mut final_bloom_tt: Vec<Vec<String>> = transpose2(bloomdata);

    for record in final_bloom_tt.clone() {
        if bloom_header_flag == true {
            bloom_header_flag = false;
            continue;
        }

        let rate = *ex_rt
            .get(&(record[5].to_owned() + config_param.defalut_ccy()))
            .unwrap_or(&0.0);
        let mut open_price: Vec<f64> = record[14..record.len()]
            .to_owned()
            .iter()
            .filter_map(|i| i.parse::<f64>().ok())
            .collect();
        open_price.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let output_acc = AccFieldNames {
            AsOnDate: *config_param.as_on_date(),
            RptItem: record[0].to_string(),
            RptDesc: record[1].to_string() + " " + &record[11],
            FaceAmt: { rate * (*config_param.face_value()) },
            IssueDate: {
                let issue_date_split: Vec<&str> = record[7].split("-").collect();
                let new_issue_dt = issue_date_split[0].to_string()
                    + "-"
                    + issue_date_split[1]
                    + "-"
                    + &century.to_owned()
                    + &issue_date_split[2][issue_date_split[2].len() - 2..];
                date_parser.parse_opt(&new_issue_dt).unwrap_or(
                    NaiveDate::parse_from_str(&new_issue_dt, "%d-%m-%Y")
                        .unwrap_or(*config_param.as_on_date()),
                )
            },
            OutsdngAmt: record[6].parse::<f64>().unwrap_or(1.0) * rate,
            MatDate: {
                let mat_date_split: Vec<&str> = record[8].split("-").collect();
                let new_mat_dt = mat_date_split[0].to_string()
                    + "-"
                    + mat_date_split[1]
                    + "-"
                    + &century.to_owned()
                    + &mat_date_split[2][mat_date_split[2].len() - 2..];
                date_parser.parse_opt(&new_mat_dt).unwrap_or(
                    NaiveDate::parse_from_str(&new_mat_dt, "%d-%m-%Y")
                        .unwrap_or(*config_param.as_on_date()),
                )
            },
            Coupons: {
                if record[9] == "#N/A"
                    || record[9] == "N/A"
                    || record[9] == "null"
                    || record[9] == ""
                {
                    0.0
                } else {
                    record[9].parse::<f64>().unwrap_or(0.0)
                }
            },
            OpenPrice: record[13].parse::<f64>().unwrap_or(0.0) * rate,
            HighPrice: open_price[open_price.len() - 1] * rate,
            LowPrice: open_price[0] * rate,
            ClosePrice: record[record.len() - 1].parse::<f64>().unwrap_or(0.0) * rate,
        };

        if record[1]
            .to_string()
            .contains(config_param.transaction_type())
        {
            output_data_mtn.push(output_acc);
        } else {
            output_data_other.push(output_acc);
        }
    }
    output_data_mtn.sort_by_key(|data| data.IssueDate);
    output_data_other.sort_by_key(|data| data.IssueDate);

    for record in output_data_mtn {
        let mut op_value = format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            record.AsOnDate.format("%d-%m-%Y"),
            record.RptItem,
            record.RptDesc,
            record.FaceAmt,
            record.IssueDate.format("%d-%m-%Y"),
            record.OutsdngAmt,
            record.MatDate.format("%d-%m-%Y"),
            record.Coupons,
            record.OpenPrice,
            record.HighPrice,
            record.LowPrice,
            record.ClosePrice,
            count,
        );
        op_value.push_str("\n");
        count += 1;
        writer::write_data(&mut op_writer, op_value, log);
    }
    for record in output_data_other {
        let mut op_value = format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            record.AsOnDate.format("%d-%m-%Y"),
            record.RptItem,
            record.RptDesc,
            record.FaceAmt,
            record.IssueDate.format("%d-%m-%Y"),
            record.OutsdngAmt,
            record.MatDate.format("%d-%m-%Y"),
            record.Coupons,
            record.OpenPrice,
            record.HighPrice,
            record.LowPrice,
            record.ClosePrice,
            count,
        );
        op_value.push_str("\n");
        count += 1;
        writer::write_data(&mut op_writer, op_value, log);
    }
}

fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
