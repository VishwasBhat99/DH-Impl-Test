use self::defesance_logic::implemenation::apply_defesance;
use self::defesance_logic::split::split_by_defesance;
use self::defesance_logic::TradingAccount;
use self::derive_fields::get_op_line;
use self::gls::{get_gl, Gls};
use self::input_account::InputAccount;
use self::output_lines::OutputLines;
use self::reconcilation::ReconKey;
use calamine::{open_workbook, Reader, Xlsx};
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use csv::ReaderBuilder;
use health_report::HealthReport;
use macros;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::time::SystemTime;
mod defesance_logic;
mod derive_fields;
mod gls;
mod input_account;
mod output_lines;
mod reconcilation;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct FTPDates {
    pub nxt_rep_dt: NaiveDate,
    pub lst_rep_dt: NaiveDate,
}

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let mut tot_amt_ip = DEFAULT_FLOAT;
    let mut tot_amt_op = DEFAULT_FLOAT;
    let mut tot_acc_skpd = DEFAULT_INT;
    let mut ref_excel1: Xlsx<_> =
        open_workbook(config_param.ref_file_path_1()).expect("Unable to open MIS1_Desc.xlsx.");
    let mut div: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel1.worksheet_range("Sheet1") {
        for row in reader.rows() {
            div.insert(row[1].to_string(), row[2].to_string());
        }
    }
    let mut ref_excel2: Xlsx<_> = open_workbook(config_param.ref_file_path_2())
        .expect("Error while opening `Ora_GL.xlsx` file.");
    let mut t_ora_mis1: HashMap<String, String> = HashMap::new();
    let mut t_ora_prod: HashMap<String, String> = HashMap::new();
    let mut t_ora_gl: HashMap<String, String> = HashMap::new();
    let mut t_ora_cat: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel2.worksheet_range("Sheet1") {
        for row in reader.rows() {
            t_ora_mis1.insert(row[0].to_string(), row[2].to_string());
            t_ora_prod.insert(row[0].to_string(), row[4].to_string());
            t_ora_gl.insert(row[0].to_string(), row[1].to_string());
            t_ora_cat.insert(row[0].to_string(), row[5].to_string());
        }
    }

    let mut ref_excel3: Xlsx<_> = open_workbook(config_param.ref_file_path_3())
        .expect("Error while opening `ALM_Line_Master.xlsx` file.");
    let mut alm_line: HashMap<String, String> = HashMap::new();
    let mut ia_line: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel3.worksheet_range(config_param.alm_master_sheet_name()) {
        for row in reader.rows() {
            alm_line.insert(row[0].to_string(), row[6].to_string());
            ia_line.insert(row[0].to_string(), row[7].to_string());
        }
    }

    let mut ref_excel4: Xlsx<_> = open_workbook(config_param.ref_file_path_4())
        .expect("Error while opening `Etrsry.xlsx` file.");
    let mut o_sys_gl: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel4.worksheet_range("Sheet1") {
        for row in reader.rows() {
            o_sys_gl.insert(row[0].to_string(), row[4].to_string());
        }
    }

    let mut ref_excel5: Xlsx<_> = open_workbook(config_param.ref_file_path_5())
        .expect("Error while opening `Defeasance.xlsx` file.");
    let mut defeasance: HashMap<String, Vec<f64>> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel5.worksheet_range(config_param.ref_file_5_sheet_name()) {
        for row in reader.rows() {
            let mut def_amt: Vec<f64> = Vec::new();
            def_amt.push(row[2].to_string().parse::<f64>().expect("Not a number!"));
            def_amt.push(row[3].to_string().parse::<f64>().expect("Not a number!"));
            def_amt.push(row[4].to_string().parse::<f64>().expect("Not a number!"));
            def_amt.push(row[5].to_string().parse::<f64>().expect("Not a number!"));
            def_amt.push(row[6].to_string().parse::<f64>().expect("Not a number!"));
            def_amt.push(row[7].to_string().parse::<f64>().expect("Not a number!"));
            def_amt.push(row[8].to_string().parse::<f64>().expect("Not a number!"));
            def_amt.push(std::f64::MAX);
            defeasance.insert(row[1].to_string(), def_amt);
        }
    }

    let mut ref_excel6: Xlsx<_> =
        open_workbook(config_param.murex_inv_master()).expect("Unable to Murex_Inv_Master.xlsx.");
    let mut fv_gl: HashMap<String, String> = HashMap::new();
    let mut prem_gl: HashMap<String, String> = HashMap::new();
    let mut prem_amt: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel6.worksheet_range(config_param.murex_inv_sheet_name()) {
        for row in reader.rows() {
            let concat = row[5].to_string().as_str().replace("\u{a0}", " ");
            fv_gl.insert(concat.to_string(), row[6].to_string());
            prem_gl.insert(concat.to_string(), row[7].to_string());
            prem_amt.insert(
                concat.to_string(),
                row[9]
                    .to_string()
                    .to_uppercase()
                    .as_str()
                    .replace("\u{a0}", " "),
            );
        }
    }

    let def_date = NaiveDate::parse_from_str("01-01-1970", "%d-%m-%Y").unwrap();
    let date_parser = rbdate::DateParser::new("%d-%m-%Y".to_string(), false);
    //Read the Floating Bond master file to map nxt_rep_dt and lst_rep_dt
    let fb_master_file = File::open(&config_param.floating_bond_master())
        .expect("Could Not Read Floating Bond Master File");
    let fb_reader = BufReader::new(fb_master_file);
    let mut fb_master_map: HashMap<String, FTPDates> = HashMap::new();
    for (line_no, line) in fb_reader.lines().enumerate() {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                log_error!(
                    log,
                    "Cannot read line {} from floating bond file: {:?}",
                    line_no,
                    error
                );
                continue;
            }
        };
        let fb_fields: Vec<&str> = acc_info
            .split(config_param.floating_bond_delimiter())
            .collect();
        fb_master_map.insert(
            fb_fields[0].to_string(),
            FTPDates {
                lst_rep_dt: date_parser.parse_opt(fb_fields[1]).unwrap_or(def_date),
                nxt_rep_dt: date_parser.parse_opt(fb_fields[2]).unwrap_or(def_date),
            },
        );
    }
    let end_read_timer = SystemTime::now();
    let duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration for read timer.");
    debug!(
        diag_log,
        "Reading Reference Files, Total Duration: {:?}.", duration
    );

    let start_derive_timer = SystemTime::now();
    let mut is_first = true;
    let mut concats: Vec<String> = Vec::new();
    let mut op_line: String = String::new();
    //adding header to output
    let header = "deal_no|short_name|nxt_rep_dt|call_dt|put_dt|deal_dt|portfolio|deal_rt|org_face_val|os_face_val|org_cst_val|acrd_int|book_yield|int_basis|avg_os_vd|avg_os_dd|prin_amt|org_bal|coup_rt|nxt_coup_dt|gl|cf_dt|secu_desc|prod_desc|prod_cd|lst_coup_dt|call_dt1|coup_freq|val_dt|acrl_freq|lst_rep_dt|lst_put_dt|inst|org_term|acrl_basis|prod_concat|concat|div|alm_line|ia_line|cmpnd_freq|nxt_cmpnd_dt|rt_chng_freq|rt_flg|rep_idx|nxt_pay_dt|prev_rep_dt|int_pay_freq|int_rt|as_on_dt|port_typ|sec_grp|sec_type|sec_issuer|sec_guaranteed|mrkt|idx_label|bd_categ|bd_type|listed|npa_class|entity|desk|acc_sec_igaap|os_cv_before_amort|os_cv_after_amort|mat_dt|int_amt|flow_type|isin|wap_igaap|ost_bal|Contract Number|Instrid|Parent Code|Issuer Name|Rating|Taxstatus|SLR_NSLR|DealYTM|IntrAppFreq|CompoundFreq|IntrPrac|RateSpread|Asset Class|IntrTyp|Security Issuance Date|Coupon|LastIntrdt|NextIntrdt|AmortTillDate|ftp_lst_rep_date|ftp_nxt_rep_date\n";
    op_line.push_str(&header);
    let mut recon: HashMap<ReconKey, f64> = HashMap::new();
    let mut tot_acc_encntrd: i64 = 0;
    let mut trading_accounts: HashMap<String, Vec<TradingAccount>> = HashMap::new();
    let mut prev_acc_no = String::new();
    let mut ost_amt: f64 = DEFAULT_FLOAT;
    let mut prin_amt: f64;
    let mut int_amt: f64;
    let mut org_ost_amt = DEFAULT_FLOAT;
    let mut ost_bal = DEFAULT_FLOAT;
    let mut prev_acc: InputAccount = InputAccount::new();
    let mut prev_gls: Gls = Gls::new();
    let mut reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(config_param.input_file_path())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in reader.deserialize().enumerate() {
        let input_account: InputAccount = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.input_file_path(),
                    line_num + 1,
                    error
                );
                Default::default()
            }
        };
        tot_acc_encntrd += 1;

        if input_account.entity != config_param.entity() {
            tot_acc_skpd += 1;
            continue;
        }

        let gls = get_gl(&input_account, &mut fv_gl, &mut prem_gl, &mut prem_amt);
        let flow_amt = input_account
            .flow_amt
            .parse::<f64>()
            .unwrap_or(DEFAULT_FLOAT)
            .abs();
        let os_fv = input_account.os_fv.parse().unwrap_or(DEFAULT_FLOAT);
        let wap_igaap = input_account.wap_igaap.parse().unwrap_or(DEFAULT_FLOAT);
        //Calculate new amount field.
        ost_bal = (os_fv * wap_igaap) / 100.0;

        if prev_acc_no != input_account.deal_no {
            prev_acc_no = input_account.deal_no.to_string();
            let mut amt: f64 = input_account.os_fv.parse().unwrap_or(DEFAULT_FLOAT);

            let mut recon_key = ReconKey::new(
                "INR".to_string(),
                "face_value_gl".to_string(),
                gls.fv_gl.to_string(),
            );
            recon
                .entry(recon_key)
                .and_modify(|val| *val += amt)
                .or_insert(amt);

            amt = if gls.prem_amt_field == "BVBEFOREAMORT-FV" {
                input_account
                    .os_cv_before_amort
                    .parse()
                    .unwrap_or(DEFAULT_FLOAT)
                    - amt
            } else if gls.prem_amt_field == "BVAFTERAMORT-FV" {
                input_account
                    .os_cv_after_amort
                    .parse()
                    .unwrap_or(DEFAULT_FLOAT)
                    - amt
            } else {
                log_error!(
                    log,
                    "Invalid field for premium amount selection: `{}`, account: `{}`.",
                    gls.prem_amt_field,
                    input_account.deal_no,
                );
                DEFAULT_FLOAT
            };

            recon_key = ReconKey::new(
                "INR".to_string(),
                "premium_gl".to_string(),
                gls.prem_gl.to_string(),
            );
            recon
                .entry(recon_key)
                .and_modify(|val| *val += amt)
                .or_insert(amt);

            if org_ost_amt > 0.0 && !is_first {
                prin_amt = org_ost_amt;
                tot_amt_op += prin_amt;
                int_amt = DEFAULT_FLOAT;
                let mut processed_line = get_op_line(
                    &prev_acc,
                    &mut t_ora_mis1,
                    &mut t_ora_prod,
                    &mut t_ora_gl,
                    &mut t_ora_cat,
                    &prev_gls,
                    &mut div,
                    &mut alm_line,
                    &mut ia_line,
                    prin_amt,
                    int_amt,
                    ost_bal,
                    &mut fb_master_map,
                    def_date,
                    *config_param.as_on_date(),
                    &log,
                );
                if let Some(concat) = processed_line.concat_lines.pop() {
                    concats.push(concat);
                }
                if prev_acc.acc_sec_igaap.to_uppercase() == "HFT" {
                    let mut desc = prev_acc.short_name.to_string();
                    if prev_acc.prod_desc.to_uppercase() == "TBILL" {
                        desc = String::from("TBILL");
                    }
                    let mat_date = NaiveDate::parse_from_str(&prev_acc.vw_val_dt, "%d-%m-%Y")
                        .expect("Cannot parse cf dt string as Naivedate.");
                    if trading_accounts.contains_key(&desc) {
                        let value = trading_accounts
                            .get_mut(&desc)
                            .expect("Unexpected fail of unwrap while reading traiding accounts.");
                        let new_value = TradingAccount {
                            mat_dt: mat_date,
                            acc_pt: processed_line.processed_lines,
                        };
                        value.push(new_value);
                    } else {
                        let value = TradingAccount {
                            mat_dt: mat_date,
                            acc_pt: processed_line.processed_lines.to_string(),
                        };
                        let new_val = vec![value];
                        trading_accounts.insert(desc.to_string(), new_val);
                    }
                } else {
                    op_line.push_str(&processed_line.processed_lines);
                }
            }
            is_first = false;
            if org_ost_amt < 0.0 {
                prin_amt = org_ost_amt + ost_amt;
                tot_amt_op += prin_amt;
                int_amt = DEFAULT_FLOAT;
                let mut processed_line = get_op_line(
                    &prev_acc,
                    &mut t_ora_mis1,
                    &mut t_ora_prod,
                    &mut t_ora_gl,
                    &mut t_ora_cat,
                    &prev_gls,
                    &mut div,
                    &mut alm_line,
                    &mut ia_line,
                    prin_amt,
                    int_amt,
                    ost_bal,
                    &mut fb_master_map,
                    def_date,
                    *config_param.as_on_date(),
                    &log,
                );
                if let Some(concat) = processed_line.concat_lines.pop() {
                    concats.push(concat);
                }
                let fields: Vec<&str> = processed_line.processed_lines.split("|").collect();
                if fields[63].to_uppercase() == "HFT" {
                    let desc = &fields[69].replace("\n", "");
                    let mut desc = desc.as_str();
                    if fields[23].to_uppercase() == "TBILL" {
                        desc = "TBILL";
                    }
                    let mat_date = NaiveDate::parse_from_str(fields[21], "%d-%m-%Y")
                        .expect("Cannot parse mat dt string as Naivedate.");
                    if trading_accounts.contains_key(desc) {
                        let value = trading_accounts
                            .get_mut(desc)
                            .expect("Unexpected fail of unwrap while reading traiding accounts.");
                        let new_value = TradingAccount {
                            mat_dt: mat_date,
                            acc_pt: processed_line.processed_lines,
                        };
                        value.push(new_value);
                    } else {
                        let value = TradingAccount {
                            mat_dt: mat_date,
                            acc_pt: processed_line.processed_lines.to_string(),
                        };
                        let new_val = vec![value];
                        trading_accounts.insert(desc.to_string(), new_val);
                    }
                } else {
                    op_line.push_str(&processed_line.processed_lines);
                }
            }
            ost_amt = flow_amt;
            org_ost_amt = ost_bal;
            tot_amt_ip += org_ost_amt;
            if input_account.flow_type.to_uppercase() == "CAP" {
                if org_ost_amt == 0.0 {
                    continue;
                }
                if ost_amt <= org_ost_amt {
                    prin_amt = ost_amt;
                    tot_amt_op += prin_amt;
                    int_amt = DEFAULT_FLOAT;
                    let mut processed_line = get_op_line(
                        &input_account,
                        &mut t_ora_mis1,
                        &mut t_ora_prod,
                        &mut t_ora_gl,
                        &mut t_ora_cat,
                        &gls,
                        &mut div,
                        &mut alm_line,
                        &mut ia_line,
                        prin_amt,
                        int_amt,
                        ost_bal,
                        &mut fb_master_map,
                        def_date,
                        *config_param.as_on_date(),
                        &log,
                    );
                    if let Some(concat) = processed_line.concat_lines.pop() {
                        concats.push(concat);
                    }
                    let fields: Vec<&str> = processed_line.processed_lines.split("|").collect();
                    if fields[63].to_uppercase() == "HFT" {
                        let desc = &fields[69].replace("\n", "");
                        let mut desc = desc.as_str();
                        if fields[23].to_uppercase() == "TBILL" {
                            desc = "TBILL";
                        }
                        let mat_date = NaiveDate::parse_from_str(fields[21], "%d-%m-%Y")
                            .expect("Cannot parse mat dt string as Naivedate.");
                        if trading_accounts.contains_key(desc) {
                            let value = trading_accounts.get_mut(desc).expect(
                                "Unexpected fail of unwrap while reading traiding accounts.",
                            );
                            let new_value = TradingAccount {
                                mat_dt: mat_date,
                                acc_pt: processed_line.processed_lines,
                            };
                            value.push(new_value);
                        } else {
                            let value = TradingAccount {
                                mat_dt: mat_date,
                                acc_pt: processed_line.processed_lines.to_string(),
                            };
                            let new_val = vec![value];
                            trading_accounts.insert(desc.to_string(), new_val);
                        }
                    } else {
                        op_line.push_str(&processed_line.processed_lines);
                    }
                } else {
                    prin_amt = org_ost_amt;
                    tot_amt_op += prin_amt;
                    int_amt = DEFAULT_FLOAT;
                    let mut processed_line = get_op_line(
                        &input_account,
                        &mut t_ora_mis1,
                        &mut t_ora_prod,
                        &mut t_ora_gl,
                        &mut t_ora_cat,
                        &gls,
                        &mut div,
                        &mut alm_line,
                        &mut ia_line,
                        prin_amt,
                        int_amt,
                        ost_bal,
                        &mut fb_master_map,
                        def_date,
                        *config_param.as_on_date(),
                        &log,
                    );
                    if let Some(concat) = processed_line.concat_lines.pop() {
                        concats.push(concat);
                    }
                    let fields: Vec<&str> = processed_line.processed_lines.split("|").collect();
                    if fields[63].to_uppercase() == "HFT" {
                        let desc = &fields[69].replace("\n", "");
                        let mut desc = desc.as_str();
                        if fields[23].to_uppercase() == "TBILL" {
                            desc = "TBILL";
                        }
                        let mat_date = NaiveDate::parse_from_str(fields[21], "%d-%m-%Y")
                            .expect("Cannot parse mat dt string as Naivedate.");
                        if trading_accounts.contains_key(desc) {
                            let value = trading_accounts.get_mut(desc).expect(
                                "Unexpected fail of unwrap while reading traiding accounts.",
                            );
                            let new_value = TradingAccount {
                                mat_dt: mat_date,
                                acc_pt: processed_line.processed_lines,
                            };
                            value.push(new_value);
                        } else {
                            let value = TradingAccount {
                                mat_dt: mat_date,
                                acc_pt: processed_line.processed_lines.to_string(),
                            };
                            let new_val = vec![value];
                            trading_accounts.insert(desc.to_string(), new_val);
                        }
                    } else {
                        op_line.push_str(&processed_line.processed_lines);
                    }
                    org_ost_amt = 0.0;
                    ost_amt = 0.0;
                }
                org_ost_amt -= ost_amt;
            } else {
                prin_amt = DEFAULT_FLOAT;
                int_amt = flow_amt;
                let mut processed_line = get_op_line(
                    &input_account,
                    &mut t_ora_mis1,
                    &mut t_ora_prod,
                    &mut t_ora_gl,
                    &mut t_ora_cat,
                    &gls,
                    &mut div,
                    &mut alm_line,
                    &mut ia_line,
                    prin_amt,
                    int_amt,
                    ost_bal,
                    &mut fb_master_map,
                    def_date,
                    *config_param.as_on_date(),
                    &log,
                );
                if let Some(concat) = processed_line.concat_lines.pop() {
                    concats.push(concat);
                }
                let fields: Vec<&str> = processed_line.processed_lines.split("|").collect();
                if fields[63].to_uppercase() == "HFT" {
                    let desc = &fields[69].replace("\n", "");
                    let mut desc = desc.as_str();
                    if fields[23].to_uppercase() == "TBILL" {
                        desc = "TBILL";
                    }
                    let mat_date = NaiveDate::parse_from_str(fields[21], "%d-%m-%Y")
                        .expect("Cannot parse mat dt string as Naivedate.");
                    if trading_accounts.contains_key(desc) {
                        let value = trading_accounts
                            .get_mut(desc)
                            .expect("Unexpected fail of unwrap while reading traiding accounts.");
                        let new_value = TradingAccount {
                            mat_dt: mat_date,
                            acc_pt: processed_line.processed_lines,
                        };
                        value.push(new_value);
                    } else {
                        let value = TradingAccount {
                            mat_dt: mat_date,
                            acc_pt: processed_line.processed_lines.to_string(),
                        };
                        let new_val = vec![value];
                        trading_accounts.insert(desc.to_string(), new_val);
                    }
                } else {
                    op_line.push_str(&processed_line.processed_lines);
                }
            }
            prev_acc = input_account.clone();
            prev_gls = gls.clone();
        } else {
            if input_account.flow_type.to_uppercase() == "CAP" {
                if org_ost_amt == 0.0 {
                    continue;
                }
                ost_amt = flow_amt;
                if ost_amt <= org_ost_amt {
                    prin_amt = ost_amt;
                    tot_amt_op += prin_amt;
                    int_amt = DEFAULT_FLOAT;
                    let mut processed_line = get_op_line(
                        &input_account,
                        &mut t_ora_mis1,
                        &mut t_ora_prod,
                        &mut t_ora_gl,
                        &mut t_ora_cat,
                        &gls,
                        &mut div,
                        &mut alm_line,
                        &mut ia_line,
                        prin_amt,
                        int_amt,
                        ost_bal,
                        &mut fb_master_map,
                        def_date,
                        *config_param.as_on_date(),
                        &log,
                    );
                    if let Some(concat) = processed_line.concat_lines.pop() {
                        concats.push(concat);
                    }
                    let fields: Vec<&str> = processed_line.processed_lines.split("|").collect();
                    if fields[63].to_uppercase() == "HFT" {
                        let desc = &fields[69].replace("\n", "");
                        let mut desc = desc.as_str();
                        if fields[23].to_uppercase() == "TBILL" {
                            desc = "TBILL";
                        }
                        let mat_date = NaiveDate::parse_from_str(fields[21], "%d-%m-%Y")
                            .expect("Cannot parse mat dt string as Naivedate.");
                        if trading_accounts.contains_key(desc) {
                            let value = trading_accounts.get_mut(desc).expect(
                                "Unexpected fail of unwrap while reading traiding accounts.",
                            );
                            let new_value = TradingAccount {
                                mat_dt: mat_date,
                                acc_pt: processed_line.processed_lines,
                            };
                            value.push(new_value);
                        } else {
                            let value = TradingAccount {
                                mat_dt: mat_date,
                                acc_pt: processed_line.processed_lines.to_string(),
                            };
                            let new_val = vec![value];
                            trading_accounts.insert(desc.to_string(), new_val);
                        }
                    } else {
                        op_line.push_str(&processed_line.processed_lines);
                    }
                } else {
                    prin_amt = org_ost_amt;
                    tot_amt_op += prin_amt;
                    int_amt = DEFAULT_FLOAT;
                    let mut processed_line = get_op_line(
                        &input_account,
                        &mut t_ora_mis1,
                        &mut t_ora_prod,
                        &mut t_ora_gl,
                        &mut t_ora_cat,
                        &gls,
                        &mut div,
                        &mut alm_line,
                        &mut ia_line,
                        prin_amt,
                        int_amt,
                        ost_bal,
                        &mut fb_master_map,
                        def_date,
                        *config_param.as_on_date(),
                        &log,
                    );
                    if let Some(concat) = processed_line.concat_lines.pop() {
                        concats.push(concat);
                    }
                    let fields: Vec<&str> = processed_line.processed_lines.split("|").collect();
                    if fields[63].to_uppercase() == "HFT" {
                        let desc = &fields[69].replace("\n", "");
                        let mut desc = desc.as_str();
                        if fields[23].to_uppercase() == "TBILL" {
                            desc = "TBILL";
                        }
                        let mat_date = NaiveDate::parse_from_str(fields[21], "%d-%m-%Y")
                            .expect("Cannot parse mat dt string as Naivedate.");
                        if trading_accounts.contains_key(desc) {
                            let value = trading_accounts.get_mut(desc).expect(
                                "Unexpected fail of unwrap while reading traiding accounts.",
                            );
                            let new_value = TradingAccount {
                                mat_dt: mat_date,
                                acc_pt: processed_line.processed_lines,
                            };
                            value.push(new_value);
                        } else {
                            let value = TradingAccount {
                                mat_dt: mat_date,
                                acc_pt: processed_line.processed_lines.to_string(),
                            };
                            let new_val = vec![value];
                            trading_accounts.insert(desc.to_string(), new_val);
                        }
                    } else {
                        op_line.push_str(&processed_line.processed_lines);
                    }
                    org_ost_amt = 0.0;
                    ost_amt = 0.0;
                }
                org_ost_amt -= ost_amt;
            } else {
                prin_amt = DEFAULT_FLOAT;
                int_amt = flow_amt;
                let mut processed_line = get_op_line(
                    &input_account,
                    &mut t_ora_mis1,
                    &mut t_ora_prod,
                    &mut t_ora_gl,
                    &mut t_ora_cat,
                    &gls,
                    &mut div,
                    &mut alm_line,
                    &mut ia_line,
                    prin_amt,
                    int_amt,
                    ost_bal,
                    &mut fb_master_map,
                    def_date,
                    *config_param.as_on_date(),
                    &log,
                );
                if let Some(concat) = processed_line.concat_lines.pop() {
                    concats.push(concat);
                }
                let fields: Vec<&str> = processed_line.processed_lines.split("|").collect();
                if fields[63].to_uppercase() == "HFT" {
                    let desc = &fields[69].replace("\n", "");
                    let mut desc = desc.as_str();
                    if fields[23].to_uppercase() == "TBILL" {
                        desc = "TBILL";
                    }
                    let mat_date = NaiveDate::parse_from_str(fields[21], "%d-%m-%Y")
                        .expect("Cannot parse mat dt string as Naivedate.");
                    if trading_accounts.contains_key(desc) {
                        let value = trading_accounts
                            .get_mut(desc)
                            .expect("Unexpected fail of unwrap while reading traiding accounts.");
                        let new_value = TradingAccount {
                            mat_dt: mat_date,
                            acc_pt: processed_line.processed_lines,
                        };
                        value.push(new_value);
                    } else {
                        let value = TradingAccount {
                            mat_dt: mat_date,
                            acc_pt: processed_line.processed_lines.to_string(),
                        };
                        let new_val = vec![value];
                        trading_accounts.insert(desc.to_string(), new_val);
                    }
                } else {
                    op_line.push_str(&processed_line.processed_lines);
                }
            }
        }
    }

    if org_ost_amt > 0.0 {
        prin_amt = org_ost_amt;
        tot_amt_op += prin_amt;
        int_amt = DEFAULT_FLOAT;
        let mut processed_line = get_op_line(
            &prev_acc,
            &mut t_ora_mis1,
            &mut t_ora_prod,
            &mut t_ora_gl,
            &mut t_ora_cat,
            &prev_gls,
            &mut div,
            &mut alm_line,
            &mut ia_line,
            prin_amt,
            int_amt,
            ost_bal,
            &mut fb_master_map,
            def_date,
            *config_param.as_on_date(),
            &log,
        );
        if let Some(concat) = processed_line.concat_lines.pop() {
            concats.push(concat);
        }
        let fields: Vec<&str> = processed_line.processed_lines.split("|").collect();
        if fields[63].to_uppercase() == "HFT" {
            let desc = &fields[69].replace("\n", "");
            let mut desc = desc.as_str();
            if fields[23].to_uppercase() == "TBILL" {
                desc = "TBILL";
            }
            let mat_date = NaiveDate::parse_from_str(fields[21], "%d-%m-%Y")
                .expect("Cannot parse mat dt string as Naivedate.");
            if trading_accounts.contains_key(desc) {
                let value = trading_accounts
                    .get_mut(desc)
                    .expect("Unexpected fail of unwrap while reading traiding accounts.");
                let new_value = TradingAccount {
                    mat_dt: mat_date,
                    acc_pt: processed_line.processed_lines,
                };
                value.push(new_value);
            } else {
                let value = TradingAccount {
                    mat_dt: mat_date,
                    acc_pt: processed_line.processed_lines.to_string(),
                };
                let new_val = vec![value];
                trading_accounts.insert(desc.to_string(), new_val);
            }
        } else {
            op_line.push_str(&processed_line.processed_lines);
        }
    }
    if org_ost_amt < 0.0 {
        prin_amt = ost_amt + org_ost_amt;
        tot_amt_op += prin_amt;
        int_amt = DEFAULT_FLOAT;
        let mut processed_line = get_op_line(
            &prev_acc,
            &mut t_ora_mis1,
            &mut t_ora_prod,
            &mut t_ora_gl,
            &mut t_ora_cat,
            &prev_gls,
            &mut div,
            &mut alm_line,
            &mut ia_line,
            prin_amt,
            int_amt,
            ost_bal,
            &mut fb_master_map,
            def_date,
            *config_param.as_on_date(),
            &log,
        );
        if let Some(concat) = processed_line.concat_lines.pop() {
            concats.push(concat);
        }
        let fields: Vec<&str> = processed_line.processed_lines.split("|").collect();
        if fields[63].to_uppercase() == "HFT" {
            let desc = &fields[69].replace("\n", "");
            let mut desc = desc.as_str();
            if fields[23].to_uppercase() == "TBILL" {
                desc = "TBILL";
            }
            let mat_date = NaiveDate::parse_from_str(fields[21], "%d-%m-%Y")
                .expect("Cannot parse mat dt string as Naivedate.");
            if trading_accounts.contains_key(desc) {
                let value = trading_accounts
                    .get_mut(desc)
                    .expect("Unexpected fail of unwrap while reading traiding accounts.");
                let new_value = TradingAccount {
                    mat_dt: mat_date,
                    acc_pt: processed_line.processed_lines,
                };
                value.push(new_value);
            } else {
                let value = TradingAccount {
                    mat_dt: mat_date,
                    acc_pt: processed_line.processed_lines.to_string(),
                };
                let new_val = vec![value];
                trading_accounts.insert(desc.to_string(), new_val);
            }
        } else {
            op_line.push_str(&processed_line.processed_lines);
        }
    }
    for (desc, mut acc_infos) in trading_accounts.drain() {
        acc_infos.sort_by(|a, b| a.mat_dt.cmp(&b.mat_dt));
        let mut prod_desc = "";
        for account in &acc_infos {
            let fields: Vec<&str> = account.acc_pt.split("|").collect();
            prod_desc = fields[23];
        }
        let prod_desc = prod_desc.to_uppercase();
        if prod_desc == "GSEC"
            || prod_desc == "GSEC-FRB"
            || prod_desc == "TBILL"
            || prod_desc == "GILTS"
            || prod_desc == "GS-FRB"
            || prod_desc == "SDL"
        {
            apply_defesance(
                desc,
                acc_infos,
                defeasance.clone(),
                log,
                config_param.as_on_date,
                config_param.apply_defesance(),
                &mut op_line,
            );
        } else {
            for account in acc_infos {
                let fields: Vec<&str> = account.acc_pt.split("|").collect();
                let amt = fields[16].parse::<f64>().unwrap_or(DEFAULT_FLOAT);
                split_by_defesance(
                    &fields,
                    fields[0].to_string(),
                    amt,
                    account.mat_dt.format("%d-%m-%Y").to_string(),
                    account.mat_dt.format("%d-%m-%Y").to_string(),
                    &mut op_line,
                    config_param.as_on_date,
                    config_param.apply_defesance(),
                );
            }
        }
    }

    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);

    let start_write_timer = SystemTime::now();
    let mut writer = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            config_param.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };
    match writer.write_all(op_line.as_bytes()) {
        Ok(_) => println!("Successfully processed all accounts."),
        Err(error) => panic!(
            "Unable to write processed lines to the file `{}`: {}.",
            config_param.output_file_path(),
            error,
        ),
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

    let mut recon_writer = match buf_file_wrtr(config_param.rec_output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create reconcilation file: `{}` on location `{}` : {}",
            config_param.rec_output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    let mut recon_op_line = String::new();
    for (key, value) in recon {
        let op = format!(
            "{}|{}|{}|{}|{}|{}",
            config_param.as_on_date().format("%d-%m-%Y"),
            "SecurityComposition",
            key.gl_type,
            key.gl_code,
            value,
            key.currency,
        );
        recon_op_line.push_str(&op[..]);
        recon_op_line.push_str("\n");
    }
    match recon_writer.write_all(recon_op_line.as_bytes()) {
        Ok(_) => println!("Successfully written reconcilation file."),
        Err(error) => panic!(
            "Unable to write reconcilation lines to the file `{}`: {}.",
            config_param.rec_output_file_path(),
            error,
        ),
    };
    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing records and reconcilation file.");
    debug!(
        diag_log,
        "Writing Records and Reconcilation File, Total Duration: {:?}.", duration
    );
    let health_stat = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - tot_acc_skpd,
        tot_acc_skpd,
        tot_amt_ip,
        tot_amt_op,
        0,
    );
    info!(log, "{}", health_stat.display());
    println!("{}", health_stat.display());
    health_stat.gen_health_rpt(config_param.output_file_path());
}
