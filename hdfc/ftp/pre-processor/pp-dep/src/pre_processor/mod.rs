use self::derive_fields::append_alm_balm_ia_line;
use self::derive_fields::append_as_on_date;
use self::derive_fields::append_currency;
use self::derive_fields::append_current_book_balance;
use self::derive_fields::append_gl_acc;
use self::derive_fields::append_int_rate;
use self::derive_fields::cost_center;
use self::recon::ReconKey;
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::DateParser;
use rbdate::NaiveDate;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::time::SystemTime;

mod derive_fields;
mod recon;

#[derive(Debug)]
pub struct ConcatFields {
    pub two_point_concat: String,
    pub alm_concat: String,
}

#[derive(Debug, Clone)]
pub struct WithdrawAccData {
    pub diff_amount: f64,
    pub withdraw_date: NaiveDate,
}

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_ref_time = SystemTime::now();
    let mut ref_mis1: String = "".to_string();
    let input_file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(input_file) => input_file,
        Err(error) => panic!("{}", error),
    };

    let mut ref_excel1: Xlsx<_> =
        open_workbook(config_param.ref_file_path_1()).expect("Error while opening `R1` file.");
    let mut ref_map1: HashMap<String, ConcatFields> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel1.worksheet_range("Sheet1") {
        for row in reader.rows() {
            let mut two_point_concat = String::new();
            two_point_concat.push_str(&row[4].to_string());
            two_point_concat.push_str("_");
            two_point_concat.push_str(&row[1].to_string());

            let mut alm_concat = String::new();
            alm_concat.push_str(&row[4].to_string());
            alm_concat.push_str("_");
            alm_concat.push_str(&row[1].to_string());
            alm_concat.push_str("_");
            alm_concat.push_str(&row[5].to_string());

            let concat_fields = ConcatFields {
                two_point_concat: two_point_concat,
                alm_concat: alm_concat,
            };
            ref_map1.insert(row[0].to_string(), concat_fields);
            ref_mis1 = row[2].to_string();
        }
    }
    let mut ref_excel2: Xlsx<_> = open_workbook(config_param.ref_file_path_2())
        .expect("Error while opening `ALM Mater File`.");
    let mut alm_llg: HashMap<String, String> = HashMap::new();
    let mut ia_llg: HashMap<String, String> = HashMap::new();
    let mut balm_llg: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel2.worksheet_range(config_param.alm_master_sheet_name()) {
        for row in reader.rows() {
            alm_llg.insert(row[0].to_string(), row[6].to_string());
            ia_llg.insert(row[0].to_string(), row[7].to_string());
            balm_llg.insert(row[0].to_string(), row[9].to_string());
        }
    }
    let mut ref_excel3: Xlsx<_> =
        open_workbook(config_param.ref_file_path_3()).expect("Error while opening `R3`.");
    let mut ref_map3: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel3.worksheet_range("Sheet1") {
        for row in reader.rows() {
            let cost_center = row[1].to_string();
            ref_map3.insert(row[0].to_string(), cost_center);
        }
    }

    let def_withdraw_dt = NaiveDate::parse_from_str("31-12-2099", "%d-%m-%Y")
        .expect("unable to parse default withdraw date.");
    let mut td_ref_map: HashMap<String, WithdrawAccData> = HashMap::new();
    let ref_td_file = match new_buf_rdr(config_param.ref_file_path_5()) {
        Ok(ref_td_file) => ref_td_file,
        Err(error) => panic!("{}", error),
    };
    let ref_td = BufReader::new(ref_td_file);
    for (line_num, line) in ref_td.lines().enumerate().skip(1) {
        let td_info = match line {
            Ok(td_info) => td_info.trim().to_string(),
            Err(error) => {
                panic!(
                    "Cannot read line-no:{} from ref-td file: {:?}",
                    line_num + 1,
                    error
                );
            }
        };
        let td_fields: Vec<&str> = td_info.split("|").collect();
        let acc_no = td_fields[1].trim();
        let date = td_fields[17].trim();
        let diff_amount = td_fields[6]
            .parse::<f64>()
            .expect("Unable to parse `last_withdraw_rate`.")
            - td_fields[7]
                .parse::<f64>()
                .expect("Unable to parse `Org_int_rate`.");
        let date_parser = DateParser::new("%d-%b-%Y".to_string(), false);

        let check_dt_last_wthdrw = NaiveDate::parse_from_str(date, "%d-%b-%Y");
        if check_dt_last_wthdrw.is_err() {
            if config_param.is_perf_diagnostics_enabled() {
                info!(log, "Invalid date-last-withdraw(column-18): {} found in lin-num: {}, in td-ref file.", date, line_num+1);
            }
        }
        let dt_last_wthdrw = date_parser.parse_opt(date).unwrap_or(def_withdraw_dt);
        let withdraw_data = WithdrawAccData {
            diff_amount: diff_amount,
            withdraw_date: dt_last_wthdrw,
        };
        if !td_ref_map.contains_key(acc_no) && diff_amount < 1.0 {
            td_ref_map.insert(acc_no.to_string(), withdraw_data);
        }
    }
    info!(
        log,
        "Inserted {} records from td-ref file to HashMap",
        td_ref_map.len()
    );
    if config_param.is_perf_diagnostics_enabled() {
        for (k, v) in td_ref_map.iter() {
            info!(
                log,
                "Account-no: {} mapped with date-last-withdraw: {} in td-ref map",
                k,
                v.withdraw_date
            );
        }
    }

    let mut currency_codes: HashMap<String, String> = HashMap::new();
    let mut is_header: bool = true;
    let mut ref_excel4: Xlsx<_> =
        open_workbook(config_param.ref_file_path_4()).expect("Error while opening `R4`.");
    if let Some(Ok(reader)) = ref_excel4.worksheet_range("Sheet1") {
        for row in reader.rows() {
            if is_header {
                is_header = false;
                continue;
            }
            currency_codes.insert(row[0].to_string(), row[1].to_string());
        }
    }
    let end_read_ref_time = SystemTime::now();
    let total_duration = end_read_ref_time
        .duration_since(start_read_ref_time)
        .expect("Could not calculate total read reference duration.");
    info!(
        diag_log,
        "Reading Reference Total Duration: {:?}", total_duration
    );
    let start_process_time = SystemTime::now();
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{}", error),
    };
    let rec_output_file = match buf_file_wrtr(config_param.rec_output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{}", error),
    };
    let reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);
    let mut recon_writer = BufWriter::new(rec_output_file);
    let mut output_line = String::new();
    let mut recon_map: HashMap<ReconKey, f64> = HashMap::new();
    let mut tot_acc_encntrd = DEFAULT_INT;
    let mut skp_acc = DEFAULT_INT;
    let mut tot_amt = DEFAULT_FLOAT;
    let mut concats: Vec<String> = Vec::new();
    let header = "account_number|accrued_interest|deposit_type|maturity_date|rat_acct_int|rat_acct_int_var|next_compound_date|next_payment_date|account_start_date|currency_code|customer_id|original_balance|origination_date|previous_roll_over_date|description|client_name|tname|as_on_date|bank_num|branch|rate_flag|cost_centre_ftp|int_pay_freq|institution|new_gl|int_rate|concat|ia_llg|balm_llg|current_book_balance|cost_center|comp_freq|fin_cost_ftp|date_last_withdraw|prepayment_less_than_7_days|two_point_concat|four_point_concat\n";
    output_line.push_str(header);
    for (line_num, line) in reader.lines().enumerate() {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
        let mut fields: Vec<&str> = acc_info.split("~#~").collect();

        tot_acc_encntrd += 1;

        if fields.len() != 38 && fields.len() != 39 {
            skp_acc += 1;
            continue;
        }

        if fields[1].parse::<f64>().is_err() {
            continue;
        }

        let bal_principal_lcy: &str = fields[7];
        let rat_acct_int: &str = fields[12];
        let rat_acct_int_var: &str = fields[13];
        let rat_prod_var: &str;
        if fields.len() == 38 {
            rat_prod_var = "0.0";
        } else {
            rat_prod_var = fields[38];
        }
        let bal_int_comp_lcy: &str = fields[25];
        let cod_gl_regular_dep: &str = fields[28];
        let mis1: &str = fields[37];
        let curr: &str = fields[17];
        let cost_cen_ftp: &str = fields[34];
        let comp_freq: &str = fields[30];
        let cip_gl: &str = fields[36];
        let cod_gl: &str = fields[28];

        fields[0] = fields[0].trim();

        output_line.push_str(fields[0]);
        output_line.push('|');
        output_line.push_str(fields[1]);
        output_line.push('|');
        output_line.push_str(fields[5]);
        output_line.push('|');

        let mat_dt = match NaiveDate::parse_from_str(fields[8], "%d-%b-%Y") {
            Ok(dt) => dt.format("%d-%m-%Y").to_string(),
            Err(_) => "".to_string(),
        };
        output_line.push_str(&mat_dt);
        output_line.push_str("|");

        output_line.push_str(fields[12]);
        output_line.push('|');
        output_line.push_str(fields[13]);
        output_line.push('|');

        let nxt_comp_dt = match NaiveDate::parse_from_str(fields[14], "%d-%b-%Y") {
            Ok(dt) => dt.format("%d-%m-%Y").to_string(),
            Err(_) => "".to_string(),
        };
        output_line.push_str(&nxt_comp_dt);
        output_line.push_str("|");

        let nxt_pay_dt = match NaiveDate::parse_from_str(fields[16], "%d-%b-%Y") {
            Ok(dt) => dt.format("%d-%m-%Y").to_string(),
            Err(_) => "".to_string(),
        };
        output_line.push_str(&nxt_pay_dt);
        output_line.push_str("|");

        let acc_st_dt = match NaiveDate::parse_from_str(fields[15], "%d-%b-%Y") {
            Ok(dt) => dt.format("%d-%m-%Y").to_string(),
            Err(_) => "".to_string(),
        };
        output_line.push_str(&acc_st_dt);
        output_line.push_str("|");

        output_line.push_str(fields[17]);
        output_line.push('|');
        output_line.push_str(fields[18]);
        output_line.push('|');
        output_line.push_str(fields[20]); //amt_initl_dep_lcy
        output_line.push('|');

        let org_dt = match NaiveDate::parse_from_str(fields[23], "%d-%b-%Y") {
            Ok(dt) => dt.format("%d-%m-%Y").to_string(),
            Err(_) => "".to_string(),
        };
        output_line.push_str(&org_dt);
        output_line.push_str("|");
        let prev_roll_over_dt = match NaiveDate::parse_from_str(fields[23], "%d-%b-%Y") {
            Ok(dt) => dt.format("%d-%m-%Y").to_string(),
            Err(_) => "".to_string(),
        };
        output_line.push_str(&prev_roll_over_dt);
        output_line.push_str("|");

        output_line.push_str(fields[27]);
        output_line.push('|');
        output_line.push_str(fields[31]);
        output_line.push_str("|FWTD|");
        append_as_on_date(&mut output_line, config_param.as_on_date);
        output_line.push_str("000|");
        output_line.push_str(fields[4]);
        output_line.push_str("|F|");
        output_line.push_str(fields[11]);
        output_line.push('|');
        output_line.push_str(fields[34]);
        output_line.push('|');
        let currency = append_currency(&mut output_line, curr, &mut currency_codes);

        let cip_recon_key = ReconKey::new(
            currency.to_string(),
            "CIPGL".to_string(),
            cip_gl.to_string(),
        );
        let cod_recon_key =
            ReconKey::new(currency.clone(), "PRDGL".to_string(), cod_gl.to_string());
        append_gl_acc(&mut output_line, cod_gl_regular_dep);
        append_int_rate(
            &mut output_line,
            rat_acct_int,
            rat_acct_int_var,
            rat_prod_var,
        );
        let cost_center = cost_center(&ref_map3, cod_gl_regular_dep, cost_cen_ftp);
        let gl = cod_gl.to_string() + "#" + &cip_gl.to_string();
        concats.push(append_alm_balm_ia_line(
            &mut output_line,
            &ref_map1,
            &alm_llg,
            &ia_llg,
            &balm_llg,
            cod_gl_regular_dep,
            &cost_center,
            mis1,
            &ref_mis1,
            log,
            fields[0],
            &gl,
        ));
        tot_amt += append_current_book_balance(
            &mut output_line,
            bal_principal_lcy,
            bal_int_comp_lcy,
            cip_recon_key,
            cod_recon_key,
            &mut recon_map,
        );
        output_line.push_str(&cost_center);
        output_line.push('|');
        output_line.push_str(&comp_freq);
        output_line.push('|');
        if mis1.is_empty() {
            output_line.push_str("999");
        } else {
            output_line.push_str(mis1);
        }
        output_line.push('|');
        if td_ref_map.contains_key(fields[0].trim()) {
            let withdraw_data = &td_ref_map
                .get(fields[0].trim())
                .expect("Unable to get withdraw data");

            let dt_last = withdraw_data.withdraw_date;
            output_line.push_str(&dt_last.format("%d-%m-%Y").to_string());
            output_line.push('|');
            if !org_dt.is_empty() {
                let diff;
                let dt_org = NaiveDate::parse_from_str(&org_dt, "%d-%m-%Y").unwrap();
                if dt_last < dt_org {
                    diff = rbdate::num_days_start_to_end(dt_last, dt_org);
                } else {
                    diff = rbdate::num_days_start_to_end(dt_org, dt_last);
                }
                if config_param.is_perf_diagnostics_enabled() {
                    info!(
                        log,
                        "date got from the map:{}; date got from input:{}", dt_last, dt_org
                    );
                    info!(log, "diff: {}", diff);
                }
                if curr == "1" {
                    if diff < 7 {
                        output_line.push('Y');
                    } else {
                        output_line.push('N');
                    }
                } else {
                    output_line.push('N');
                }
            } else {
                if config_param.is_perf_diagnostics_enabled() {
                    info!(log, "Found invalid value date(column-24): {} in line-no: {} in input file, must be in DD-MMM-YYYY format", org_dt, line_num+1);
                }
                output_line.push('N');
            }
        } else {
            if config_param.is_perf_diagnostics_enabled() {
                info!(log, "Can not find account-no(column-1): {} in td-ref map, line-no: {} in input file", fields[0], line_num+1);
            }
            output_line.push_str(&"31-03-2099".to_string());
            output_line.push('|');
            output_line.push('N');
        }
        let two_point_concat: String = match ref_map1.get(cip_gl) {
            Some(val) => val.two_point_concat.to_string(),
            None => {
                log_debug!(log, "Cannot get two point concat for {}.", cip_gl);
                "".to_string()
            }
        };
        let four_point_concat = get_four_point_concat(
            &ref_map1,
            cod_gl_regular_dep,
            &cost_center,
            mis1,
            &ref_mis1,
            log,
            fields[0],
        );
        output_line.push_str("|");
        output_line.push_str(&two_point_concat);
        output_line.push_str("|");
        output_line.push_str(&four_point_concat);
        output_line.push_str("\n");
    }
    let end_process_time = SystemTime::now();
    let duration = end_process_time
        .duration_since(start_process_time)
        .expect("Could not calculate total process duration.");
    info!(diag_log, "Process Total Duration: {:?}.", duration);
    let start_writer_time = SystemTime::now();
    let mut recon_output_line = String::new();
    for (key, value) in recon_map {
        let op = format!(
            "{}|{}|{}|{}|{}|{}",
            config_param.as_on_date().format("%d-%m-%Y"),
            "TD353",
            key.gl_type,
            key.gl_code,
            value,
            key.currency,
        );
        recon_output_line.push_str(&op[..]);
        recon_output_line.push_str("\n");
    }
    match recon_writer.write_all(recon_output_line.as_bytes()) {
        Ok(val) => val,
        Err(error) => {
            panic!("Error writing reconciliation report: {:?}", error);
        }
    }
    match writer.write_all(output_line.as_bytes()) {
        Ok(val) => val,
        Err(error) => {
            panic!("Error writing processed data: {:?}", error);
        }
    }

    let mut concat_lines = String::new();
    let mut concat_writer = match buf_file_wrtr(config_param.concat_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create concat file: `{}` on location `{}` : {}",
            config_param.concat_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    concats.sort();
    concats.dedup();
    for concat in concats.drain(..) {
        concat_lines.push_str(&concat);
        concat_lines.push('\n');
    }
    match concat_writer.write_all(concat_lines.as_bytes()) {
        Ok(_) => println!("Successfully written concats for missing alm lines."),
        Err(error) => panic!(
            "Unable to write concat lines to the file `{}`: {}.",
            config_param.concat_file_path(),
            error,
        ),
    }
    let end_writer_time = SystemTime::now();
    let duration = end_writer_time
        .duration_since(start_writer_time)
        .expect("Could not calculate total write process duration.");
    info!(diag_log, "Write Process Total Duration: {:?}.", duration);

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skp_acc,
        skp_acc,
        tot_amt,
        tot_amt,
        DEFAULT_INT,
    );
    health_report.gen_health_rpt(&config_param.output_file_path());
}

pub fn get_four_point_concat(
    ref_map1: &HashMap<String, ConcatFields>,
    cod_gl_regular_dep: &str,
    cost_center: &str,
    mis1: &str,
    ref_mis1: &str,
    log: &Logger,
    acc_num: &str,
) -> String {
    let mut temp_concat: String = "".to_string();
    let alm_concat: String = match ref_map1.get(cod_gl_regular_dep) {
        Some(val) => val.alm_concat.to_string(),
        None => {
            log_debug!(
                log,
                "Cannot get alm concat for the account number :- {}.",
                acc_num
            );
            "".to_string()
        }
    };
    if mis1 != "" {
        temp_concat.push_str("1");
        temp_concat.push_str(&mis1);
        temp_concat.push_str("_");
        temp_concat.push_str(&alm_concat);
    } else {
        let concat_fields: Vec<&str> = alm_concat.split('_').collect();
        let prod_code: i64 = concat_fields[0]
            .parse()
            .expect("Cannot convert product code to integer.");
        if prod_code == 120_415
            || prod_code == 120_416
            || prod_code == 120_417
            || prod_code == 120_418
        {
            temp_concat.push_str("1");
            temp_concat.push_str(&cost_center);
            temp_concat.push_str("_");
            temp_concat.push_str(&alm_concat);
        } else {
            temp_concat.push_str(&ref_mis1);
            temp_concat.push_str("_");
            temp_concat.push_str(&alm_concat);
        }
    }

    temp_concat[..].to_string()
}
