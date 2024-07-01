use self::structure::*;
use chrono::Datelike;
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use pre_processor::output_field_name::AccFieldNames;
use pre_processor::trade_fields_name::TradeFieldNames;
use rbdate::DateParser;
use sdb_io::buf_file_wrtr;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
mod output_field_name;
mod structure;
mod trade_fields_name;
mod writer;

pub fn process(config_param: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let mut outstdng_map: OutstandingStruct = OutstandingStruct::new();
    let mut issue_date_map: HashMap<String, Vec<NaiveDate>> = HashMap::new();
    let mut outstdng_amt_map: OutstandingStruct = OutstandingStruct::new();
    let mut coupon_map: PriceStruct = PriceStruct::new();

    let date_parser = DateParser::new("%d-%b-%Y".to_string(), false);

    let ason_date_year = config_param.as_on_date().year().to_string();
    let ason_date_cen: Vec<&str> = ason_date_year.split("").collect();
    let century = ason_date_cen[1].to_owned() + ason_date_cen[2];

    let outstdng_file = match new_buf_rdr(config_param.input_outstanding_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_outstanding_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in outstdng_file.lines().enumerate().skip(1) {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_outstanding_file_path(),
                line_num + 1,
                error
            ),
        };

        let out_data: Vec<&str> = line.split(",").collect();
        if outstdng_map.data.contains_key(&out_data[0].to_string()) {
            let mut data = match outstdng_map.data.get_mut(&out_data[0].to_string()) {
                Some(val) => val,
                None => panic!(
                    "Error in fetching outstanding vec for key : {}",
                    out_data[0]
                ),
            };

            data.push(line.to_owned());

            let mut issue_date = match issue_date_map.get_mut(&out_data[0].to_string()) {
                Some(val) => val,
                None => panic!("Error in fetching issue date vec for key : {}", out_data[0]),
            };

            let issue_date_split: Vec<&str> = out_data[2].split("-").collect();
            let new_issue_dt = issue_date_split[0].to_string()
                + "-"
                + issue_date_split[1]
                + "-"
                + &century.to_owned()
                + &issue_date_split[2][issue_date_split[2].len() - 2..];

            issue_date.push(
                date_parser.parse_opt(&new_issue_dt).unwrap_or(
                    NaiveDate::parse_from_str(&new_issue_dt, "%d-%m-%Y")
                        .unwrap_or(*config_param.as_on_date()),
                ),
            );

            let mut outstdng_amt = match outstdng_amt_map.data.get_mut(&out_data[0].to_string()) {
                Some(val) => val,
                None => panic!(
                    "Error in fetching outstanding amount vec for key : {}",
                    out_data[0]
                ),
            };
            outstdng_amt.push(out_data[6].to_owned());

            let mut coupan = match coupon_map.data.get_mut(&out_data[0].to_string()) {
                Some(val) => val,
                None => panic!("Error in fetching coupon vec for key : {}", out_data[0]),
            };
            coupan.push(out_data[8].parse::<f64>().unwrap_or(0.0).to_owned());
        } else {
            outstdng_map
                .data
                .insert(out_data[0].to_string(), vec![line.to_owned()]);

            let issue_date_split: Vec<&str> = out_data[2].split("-").collect();
            let new_issue_dt = issue_date_split[0].to_string()
                + "-"
                + issue_date_split[1]
                + "-"
                + &century.to_owned()
                + &issue_date_split[2][issue_date_split[2].len() - 2..];

            let iss_dt = date_parser.parse_opt(&new_issue_dt).unwrap_or(
                NaiveDate::parse_from_str(&new_issue_dt, "%d-%m-%Y")
                    .unwrap_or(*config_param.as_on_date()),
            );
            issue_date_map.insert(out_data[0].to_string(), vec![iss_dt]);
            outstdng_amt_map
                .data
                .insert(out_data[0].to_string(), vec![out_data[6].to_owned()]);
            coupon_map.data.insert(
                out_data[0].to_string(),
                vec![out_data[8].parse::<f64>().unwrap_or(0.0).to_owned()],
            );
        }
    }

    let mut trade_map: OutstandingStruct = OutstandingStruct::new();
    let mut trade_price_map: PriceStruct = PriceStruct::new();

    let trade_file = match new_buf_rdr(config_param.input_trade_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_trade_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in trade_file.lines().enumerate().skip(1) {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_trade_file_path(),
                line_num + 1,
                error
            ),
        };

        let trade_data: Vec<&str> = line.split(",").collect();
        if trade_map.data.contains_key(&trade_data[1].to_string()) {
            let mut data = match trade_map.data.get_mut(&trade_data[1].to_string()) {
                Some(val) => val,
                None => panic!("Error in fetching trade vec for key : {}", trade_data[0]),
            };
            data.push(line.to_owned());

            let mut trade_price = match trade_price_map.data.get_mut(&trade_data[1].to_string()) {
                Some(val) => val,
                None => panic!(
                    "Error in fetching trade price vec for key : {}",
                    trade_data[0]
                ),
            };
            trade_price.push(trade_data[8].parse::<f64>().unwrap_or(0.0).to_owned());
        } else {
            trade_map
                .data
                .insert(trade_data[1].to_string(), vec![line.to_owned()]);
            trade_price_map.data.insert(
                trade_data[1].to_string(),
                vec![trade_data[8].parse::<f64>().unwrap_or(0.0).to_owned()],
            );
        }
    }

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

    let mut count = 1;
    let mut output_data: Vec<AccFieldNames> = Vec::new();
    for (key, value) in outstdng_map.data.iter() {
        if trade_map.data.contains_key(key) {
            let mut issue_date = match issue_date_map.get_mut(key) {
                Some(val) => val,
                None => panic!("Error in fetching issue date vec for key : {}", key),
            };

            let outstdng_amt = match outstdng_amt_map.data.get_mut(key) {
                Some(val) => val,
                None => panic!("Error in fetching outstanding amount vec for key : {}", key),
            };

            let mut coupon = match coupon_map.data.get_mut(key) {
                Some(val) => val,
                None => panic!("Error in fetching issue date vec for key : {}", key),
            };

            coupon.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let mut trade_price_val = match trade_price_map.data.get_mut(key) {
                Some(val) => val,
                None => panic!("Error in fetching issue date vec for key : {}", key),
            };

            trade_price_val.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let outstdng_value = match outstdng_map.data.get(key) {
                Some(val) => val,
                None => panic!("Error in fetching outstanding vec for key : {}", key),
            };
            let trade_value = match trade_map.data.get(key) {
                Some(val) => val,
                None => panic!("Error in fetching trade value vec for key : {}", key),
            };

            let out_val_split: Vec<&str> = outstdng_value[0].split(",").collect();
            let trade_val_split: Vec<&str> = trade_value[0].split(",").collect();
            let mut output_acc = AccFieldNames {
                AsOnDate: *config_param.as_on_date(),
                RptItem: key.to_string(),
                RptDesc: "CD ".to_string() + &trade_val_split[2].to_owned(),
                FaceAmt: *config_param.face_value(),
                IssueDate: {
                    issue_date.sort();
                    issue_date[0].to_owned()
                },
                OutsdngAmt: outstdng_amt
                    .iter()
                    .filter_map(|i| i.parse::<f64>().ok())
                    .sum(),
                MatDate: {
                    let mat_date_split: Vec<&str> = out_val_split[3].split("-").collect();

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
                Coupons: { coupon[coupon.len() - 1] },
                OpenPrice: {
                    let sort_trade = sort_vec_by_deal_date_and_time(
                        trade_value,
                        &century,
                        &date_parser,
                        &config_param,
                    );
                    sort_trade[0].TradePrice.parse::<f64>().unwrap_or(0.0)
                },
                HighPrice: { trade_price_val[trade_price_val.len() - 1] },
                LowPrice: { trade_price_val[0] },
                ClosePrice: {
                    let sort_trade = sort_vec_by_deal_date_and_time(
                        trade_value,
                        &century,
                        &date_parser,
                        &config_param,
                    );
                    sort_trade[sort_trade.len() - 1]
                        .TradePrice
                        .parse::<f64>()
                        .unwrap_or(0.0)
                },
            };

            output_data.push(output_acc);
        }
    }
    output_data.sort_by_key(|data| data.IssueDate);

    for record in output_data {
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
fn sort_vec_by_deal_date_and_time(
    trade_value: &Vec<String>,
    century: &str,
    date_parser: &DateParser,
    config_param: &ConfigurationParameters,
) -> Vec<TradeFieldNames> {
    let mut td_inp_vec: Vec<TradeFieldNames> = Vec::new();

    for record in trade_value {
        let td_val: Vec<&str> = record.split(",").collect();
        let td_inp = TradeFieldNames {
            MarketType: td_val[0].to_string(),
            Isin: td_val[1].to_string(),
            IsinDescription: td_val[2].to_string(),
            MatDate: td_val[3].to_string(),
            DealDate: {
                let deal_date_split: Vec<&str> = td_val[4].split("-").collect();

                let new_deal_dt = deal_date_split[0].to_string()
                    + "-"
                    + deal_date_split[1]
                    + "-"
                    + &century.to_owned()
                    + &deal_date_split[2][deal_date_split[2].len() - 2..];

                date_parser
                    .parse_opt(&new_deal_dt)
                    .unwrap_or(
                        NaiveDate::parse_from_str(&new_deal_dt, "%d-%m-%Y")
                            .unwrap_or(*config_param.as_on_date()),
                    )
                    .to_string()
            },
            SettType: td_val[5].to_string(),
            SettDate: td_val[6].to_string(),
            TradeAmt: td_val[7].to_string(),
            TradePrice: td_val[8].to_string(),
            TradeYield: td_val[9].to_string(),
            Wap: td_val[10].to_string(),
            Way: td_val[11].to_string(),
            DealTime: td_val[12].to_string(),
        };

        td_inp_vec.push(td_inp);
    }
    td_inp_vec.sort();
    td_inp_vec
}
