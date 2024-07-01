use self::account::format_output;
use self::account::{
    BenposData, BenposMap, BorrUpdateMap, Cashflow, FloatMap, InputAccount, OutputAccount,
};
use self::get_output::get_op_data;
use self::get_prod_type::store_prod_type;
use self::io::get_writer;
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use calamine::{open_workbook_auto, Reader};
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;
use std::io::Write;

mod account;
mod get_output;
mod get_prod_type;
mod io;

pub fn process(config_params: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let mut tot_acc_encntrd = 0;
    let mut tot_cf_acc = 0;
    let mut tot_amt_in_ip = 0.0;
    let mut tot_amt_in_op = 0.0;
    let mut skip_rec_count = 0;
    let date_parser = rbdate::DateParser::new("%d-%b-%Y".to_string(), false);

    //Two Output(Borrowings and Borrowings-Amortized)
    let op_path_1 = format!("{}.txt", &config_params.output_file_path());
    let op_path_2 = format!("{}_amortized.txt", &config_params.output_file_path());
    let mut op_writer1 = get_writer(&op_path_1);
    let mut op_writer2 = get_writer(&op_path_2);

    //stores all the required product-types
    let mut prod_type_map = store_prod_type();

    //Reading Input Master File
    let input = File::open(&config_params.input_file()).expect("Could Not Read Input File");
    let input_reader = BufReader::new(input);
    let mut input_map: HashMap<String, InputAccount> = HashMap::new();
    for (line_no, line) in input_reader.lines().enumerate() {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                skip_rec_count += 1;
                log_error!(
                    log,
                    "Cannot read line {} from input file: {:?}",
                    line_no,
                    error
                );
                continue;
            }
        };
        tot_acc_encntrd += 1;
        let input_fields: Vec<&str> = acc_info.split('|').collect();
        let acc_id = input_fields[0].to_string();
        if input_fields.len() < 19 {
            skip_rec_count += 1;
            log_error!(log,
                "Cannot read line {} from input file for acc_no: {:?} due to incorrect column count {:?}",
                line_no,
                acc_id,
                input_fields.len());
            continue;
        }
        let inp_acc = InputAccount::new(input_fields);
        input_map.insert(acc_id, inp_acc);
    }

    //Reading Input Cashflow File
    let cashflow = File::open(&config_params.input_cashflow_file())
        .expect("Could Not Read Input Cashflow File");
    let cashflow_reader = BufReader::new(cashflow);
    let mut cf_map: HashMap<String, Vec<Cashflow>> = HashMap::new();
    for (line_no, line) in cashflow_reader.lines().enumerate() {
        let cf_info: String = match line {
            Ok(cf_info) => cf_info,
            Err(error) => {
                log_error!(
                    log,
                    "Cannot read line {} from cashflow file: {:?}",
                    line_no,
                    error
                );
                continue;
            }
        };
        if cf_info.matches('|').count() < 4 {
            error!(
                log,
                "Cannot read line {} from cashflow file at line_no: {:?}", cf_info, line_no
            );
            continue;
        }
        let cf_fields: Vec<&str> = cf_info.split('|').collect();
        let mut acc_id = cf_fields[0].to_string();
        if cf_fields[1].trim() != "" {
            acc_id = cf_fields[1].to_string();
        }
        let cf_acc = Cashflow::new(cf_fields);
        cf_map
            .entry(acc_id)
            .and_modify(|cashflow| cashflow.push(cf_acc.to_owned()))
            .or_insert_with(|| vec![cf_acc.to_owned()]);
    }

    //Reading Benpos Data File
    let mut benpos_data = open_workbook_auto(config_params.benpos_data_file())
        .expect("Could Not Read Benpos Data File");
    let mut benpos_data_map: HashMap<String, Vec<BenposData>> = HashMap::new();
    if let Some(Ok(reader)) = benpos_data.worksheet_range(config_params.benpos_data_sheet()) {
        for benpos_data in reader.rows().skip(1) {
            if benpos_data.len() != *config_params.benpos_column_count() {
                log_error!(log,
                    "Cannot read line {:?} from benpos data file for acc_no: {:?} due to incorrect column count {:?}",
                    benpos_data,
                    benpos_data[7],
                    benpos_data.len());
                continue;
            }
            let isin = benpos_data[58].to_string();
            let benpos_data_acc = BenposData::new(benpos_data);
            if !isin.is_empty() {
                benpos_data_map
                    .entry(isin)
                    .and_modify(|data| data.push(benpos_data_acc.to_owned()))
                    .or_insert_with(|| vec![benpos_data_acc.to_owned()]);
            }
        }
    }

    //Reading Benpos Mapping File
    let mut benpos_map_reader = open_workbook_auto(config_params.benpos_mapping_file())
        .expect("Could Not Read Benpos Mapping File");
    let mut benpos_mapping: HashMap<String, BenposMap> = HashMap::new();
    if let Some(Ok(reader)) =
        benpos_map_reader.worksheet_range(config_params.benpos_mapping_sheet())
    {
        for benpos_map in reader.rows().skip(1) {
            if benpos_map.len() < 4 {
                log_error!(log,
                    "Cannot read line {:?} from benpos mapping file due to incorrect column count {:?}",
                    benpos_map,
                    benpos_map.len());
                continue;
            }
            let pangir1 = benpos_map[0].to_string();
            let benpos_map_acc = BenposMap::new(benpos_map);
            benpos_mapping.insert(pangir1, benpos_map_acc);
        }
    }

    //Reading Float Mapping File
    let mut float_map_reader = open_workbook_auto(config_params.floating_mapping_file())
        .expect("Could Not Read Floating Mapping File");
    let mut float_mapping: HashMap<String, FloatMap> = HashMap::new();
    if let Some(Ok(reader)) =
        float_map_reader.worksheet_range(config_params.floating_mapping_sheet())
    {
        for floating_map in reader.rows().skip(1) {
            if floating_map.len() < 4 {
                log_error!(log,
                    "Cannot read line {:?} from floating mapping file due to incorrect column count {:?}",
                    floating_map,
                    floating_map.len());
                continue;
            }
            let isin = floating_map[0].to_string();
            let floating_map_acc = FloatMap::new(floating_map);
            float_mapping.insert(isin, floating_map_acc);
        }
    }

    //Reading Borrowing UpdateType Master File
    let mut borr_update_type_master_reader =
        open_workbook_auto(config_params.borrowing_update_type_master())
            .expect("Could Not Read Borrowing UpdateType Master File");
    let mut borr_update_type_master: HashMap<String, BorrUpdateMap> = HashMap::new();
    if let Some(Ok(reader)) = borr_update_type_master_reader
        .worksheet_range(config_params.borrowing_update_type_master_sheet())
    {
        for borr_update_type_map in reader.rows().skip(1) {
            if borr_update_type_map.len() < 3 {
                log_error!(log,
                    "Cannot read line {:?} from Borrowing Update Type Master file due to incorrect column count {:?}",
                    borr_update_type_map,
                    borr_update_type_map.len());
                continue;
            }
            let update_type = borr_update_type_map[0].to_string();
            let borr_update_type_map_acc = BorrUpdateMap::new(borr_update_type_map);
            borr_update_type_master.insert(update_type, borr_update_type_map_acc);
        }
    }

    for (acc_id, inp_acc) in input_map.iter() {
        if inp_acc.instrument_type == "10B" || inp_acc.instrument_type == "10C" {
            continue;
        }
        let mut op_data: OutputAccount =
            get_op_data(inp_acc, &mut prod_type_map, &config_params, log);
        tot_amt_in_ip += op_data.prin_ost_bal.parse::<f64>().unwrap_or(0.0);

        let mut sum = 0.0;
        match benpos_data_map.get(&inp_acc.isin.to_string()) {
            Some(benpos_data_set) => {
                for data in benpos_data_set {
                    sum += data.nfacevn * data.position;
                }
                for benpos_data in benpos_data_set {
                    op_data.division = benpos_data.pangir1.to_string();
                    if inp_acc.prod_type == "Issue: LTD Installment Repay"
                        && inp_acc.instrument_type == "15E"
                    {
                        op_data.component = "PRINCIPAL".to_string();
                        op_data.amt_due = (benpos_data.nfacevn * benpos_data.position).to_string();
                        op_data.composition_percentage =
                            ((op_data.amt_due.parse::<f64>().unwrap_or(0.0) / sum) * 100.00)
                                .to_string();
                        op_data.bucket_category = benpos_mapping
                            .get(&benpos_data.pangir1.to_string())
                            .unwrap_or(&BenposMap::def())
                            .category
                            .to_string();
                        op_data.cust_name = benpos_data.name1.to_string();
                        for cf_data in cf_map.get(acc_id).unwrap_or(&Cashflow::def()) {
                            if float_mapping.contains_key(&inp_acc.isin.to_string())
                                && float_mapping
                                    .get(&inp_acc.isin.to_string())
                                    .unwrap_or(&FloatMap::def())
                                    .amt
                                    != 0.0
                            {
                                //With OIS
                                op_data.nxt_repricing_dt = rbdate::datevalue_to_naive_date(
                                    &float_mapping
                                        .get(&inp_acc.isin.to_string())
                                        .unwrap_or(&FloatMap::def())
                                        .reset_date,
                                )
                                .unwrap_or(*config_params.as_on_date())
                                .format("%d-%m-%Y")
                                .to_string();
                                op_data.cf_amt = ((op_data
                                    .composition_percentage
                                    .parse::<f64>()
                                    .unwrap_or(0.0)
                                    / 100.00)
                                    * float_mapping
                                        .get(&inp_acc.isin.to_string())
                                        .unwrap_or(&FloatMap::def())
                                        .amt)
                                    .to_string();
                                op_data.ucid = "with OIS".to_string();
                                tot_amt_in_op += op_data.cf_amt.parse::<f64>().unwrap_or(0.0);
                                let prin_cf = format_output(&op_data);
                                op_writer1
                                    .write_all(prin_cf.as_bytes())
                                    .expect("Error writing principal cf to output path 1!!");

                                //Without OIS
                                op_data.cf_amt = ((op_data
                                    .composition_percentage
                                    .parse::<f64>()
                                    .unwrap_or(0.0)
                                    / 100.00)
                                    * (inp_acc.outstanding_bal
                                        - float_mapping
                                            .get(&inp_acc.isin.to_string())
                                            .unwrap_or(&FloatMap::def())
                                            .amt))
                                    .to_string();
                                op_data.nxt_repricing_dt = date_parser
                                    .parse_opt(&inp_acc.mat_date)
                                    .unwrap_or(*config_params.as_on_date())
                                    .format("%d-%m-%Y")
                                    .to_string();
                                op_data.ucid = "without OIS".to_string();
                                op_data.due_dt = date_parser
                                    .parse_opt(&cf_data.cashflow_date)
                                    .unwrap_or_else(|| {
                                        rbdate::NaiveDate::parse_from_str(
                                            &inp_acc.mat_date,
                                            "%d-%b-%Y",
                                        )
                                        .unwrap_or(*config_params.as_on_date())
                                    })
                                    .format("%d-%m-%Y")
                                    .to_string();
                                tot_amt_in_op += op_data.cf_amt.parse::<f64>().unwrap_or(0.0);
                                let prin_cf = format_output(&op_data);
                                op_writer1
                                    .write_all(prin_cf.as_bytes())
                                    .expect("Error writing principal cf to output path 1!!");
                                if config_params.is_perf_diagnostics_enabled() {
                                    debug!(
                                            _diag_log,
                                            "Floating Data {:?} written for isin: {:?}  with amt: {:?} for product: {:?}",
                                            float_mapping
                                                .get(&inp_acc.isin.to_string())
                                                .unwrap_or(&FloatMap::def()),
                                            inp_acc.isin,
                                            float_mapping
                                                .get(&inp_acc.isin.to_string())
                                                .unwrap_or(&FloatMap::def())
                                                .amt,
                                                inp_acc.prod_type,
                                        );
                                }
                            } else {
                                let mut sign = 1.0;
                                let default_borr_data = BorrUpdateMap::default();
                                let borr_data = borr_update_type_master
                                    .get(&cf_data.cashflow_type.to_string())
                                    .unwrap_or(&default_borr_data);
                                if (borr_data.cf_type == "Principal".to_string()
                                    && borr_data.sign == "NEG".to_string())
                                {
                                    sign *= -1.0;
                                } else if (borr_data.cf_type == "Principal".to_string()
                                    && borr_data.sign == "POS".to_string())
                                {
                                    sign *= 1.0;
                                } else if (borr_data.cf_type == "".to_string()
                                    || borr_data.sign == "".to_string())
                                {
                                    log_error!(
                                            log,
                                            "Cannot get {} of Principal from Borrowings Update Type Master file",
                                            cf_data.cashflow_type
                                        );
                                }
                                op_data.due_dt = date_parser
                                    .parse_opt(&cf_data.cashflow_date)
                                    .unwrap_or_else(|| {
                                        rbdate::NaiveDate::parse_from_str(
                                            &inp_acc.mat_date,
                                            "%d-%b-%Y",
                                        )
                                        .unwrap_or(*config_params.as_on_date())
                                    })
                                    .format("%d-%m-%Y")
                                    .to_string();
                                op_data.cf_amt = ((op_data
                                    .composition_percentage
                                    .parse::<f64>()
                                    .unwrap_or(0.0)
                                    / 100.00)
                                    * cf_data.cashflow_amount
                                    * sign)
                                    .to_string();
                                tot_amt_in_op += op_data.cf_amt.parse::<f64>().unwrap_or(0.0);
                                let prin_cf = format_output(&op_data);
                                op_writer1
                                    .write_all(prin_cf.as_bytes())
                                    .expect("Error writing principal cf to output path 1!!");
                                if config_params.is_perf_diagnostics_enabled() {
                                    debug!(
                                            _diag_log,
                                            "The BENPOS Data of isin: {:?}  for product-type: {:?} -> {:?}",
                                            inp_acc.isin,
                                            inp_acc.prod_type,
                                            benpos_data
                                        );
                                    debug!(
                                        _diag_log,
                                        "The BENPOS Mapping for pangir1: {:?} -> {:?}\n",
                                        benpos_data.pangir1,
                                        benpos_mapping
                                            .get(&benpos_data.pangir1.to_string())
                                            .unwrap_or(&BenposMap::def())
                                    );
                                }
                            }
                        }
                    } else if inp_acc.prod_type != "Issue: Zero Coupon Par Premium"
                        && inp_acc.prod_type != "Issue: Commercial Paper"
                        && (inp_acc.prod_type != "Issue: LTD Installment Repay"
                            && inp_acc.instrument_type != "15E")
                    {
                        op_data.component = "PRINCIPAL".to_string();
                        op_data.amt_due = (benpos_data.nfacevn * benpos_data.position).to_string();
                        op_data.cf_amt = op_data.amt_due.to_string();
                        op_data.composition_percentage =
                            ((op_data.amt_due.parse::<f64>().unwrap_or(0.0) / sum) * 100.00)
                                .to_string();
                        op_data.bucket_category = benpos_mapping
                            .get(&benpos_data.pangir1.to_string())
                            .unwrap_or(&BenposMap::def())
                            .category
                            .to_string();
                        op_data.cust_name = benpos_data.name1.to_string();
                        if float_mapping.contains_key(&inp_acc.isin.to_string())
                            && float_mapping
                                .get(&inp_acc.isin.to_string())
                                .unwrap_or(&FloatMap::def())
                                .amt
                                != 0.0
                        {
                            //With OIS
                            op_data.nxt_repricing_dt = rbdate::datevalue_to_naive_date(
                                &float_mapping
                                    .get(&inp_acc.isin.to_string())
                                    .unwrap_or(&FloatMap::def())
                                    .reset_date,
                            )
                            .unwrap_or(*config_params.as_on_date())
                            .format("%d-%m-%Y")
                            .to_string();
                            op_data.cf_amt =
                                ((op_data.composition_percentage.parse::<f64>().unwrap_or(0.0)
                                    / 100.00)
                                    * float_mapping
                                        .get(&inp_acc.isin.to_string())
                                        .unwrap_or(&FloatMap::def())
                                        .amt)
                                    .to_string();
                            op_data.ucid = "with OIS".to_string();
                            tot_amt_in_op += op_data.cf_amt.parse::<f64>().unwrap_or(0.0);
                            let prin_cf = format_output(&op_data);
                            op_writer1
                                .write_all(prin_cf.as_bytes())
                                .expect("Error writing principal cf to output path 1!!");

                            //Without OIS
                            op_data.cf_amt =
                                ((op_data.composition_percentage.parse::<f64>().unwrap_or(0.0)
                                    / 100.00)
                                    * (inp_acc.outstanding_bal
                                        - float_mapping
                                            .get(&inp_acc.isin.to_string())
                                            .unwrap_or(&FloatMap::def())
                                            .amt))
                                    .to_string();
                            op_data.nxt_repricing_dt = date_parser
                                .parse_opt(&inp_acc.mat_date)
                                .unwrap_or(*config_params.as_on_date())
                                .format("%d-%m-%Y")
                                .to_string();
                            op_data.ucid = "without OIS".to_string();
                            tot_amt_in_op += op_data.cf_amt.parse::<f64>().unwrap_or(0.0);
                            let prin_cf = format_output(&op_data);
                            op_writer1
                                .write_all(prin_cf.as_bytes())
                                .expect("Error writing principal cf to output path 1!!");
                            if config_params.is_perf_diagnostics_enabled() {
                                debug!(
                                    _diag_log,
                                    "Floating Data {:?} written for isin: {:?}  with amt: {:?}",
                                    float_mapping
                                        .get(&inp_acc.isin.to_string())
                                        .unwrap_or(&FloatMap::def()),
                                    inp_acc.isin,
                                    float_mapping
                                        .get(&inp_acc.isin.to_string())
                                        .unwrap_or(&FloatMap::def())
                                        .amt,
                                );
                            }
                        } else if float_mapping.contains_key(&inp_acc.isin.to_string())
                            && float_mapping
                                .get(&inp_acc.isin.to_string())
                                .unwrap_or(&FloatMap::def())
                                .amt
                                == 0.0
                        {
                            op_data.nxt_repricing_dt = rbdate::datevalue_to_naive_date(
                                &float_mapping
                                    .get(&inp_acc.isin.to_string())
                                    .unwrap_or(&FloatMap::def())
                                    .reset_date,
                            )
                            .unwrap_or_else(|_| {
                                rbdate::NaiveDate::parse_from_str(
                                    &op_data.nxt_repricing_dt,
                                    "%d-%b-%Y",
                                )
                                .unwrap_or(*config_params.as_on_date())
                            })
                            .format("%d-%m-%Y")
                            .to_string();
                            tot_amt_in_op += op_data.cf_amt.parse::<f64>().unwrap_or(0.0);
                            let prin_cf = format_output(&op_data);
                            op_writer1
                                .write_all(prin_cf.as_bytes())
                                .expect("Error writing principal cf to output path 1!!");
                            if config_params.is_perf_diagnostics_enabled() {
                                debug!(
                                    _diag_log,
                                    "Floating Data {:?} written for isin: {:?}  with amt: {:?}",
                                    float_mapping
                                        .get(&inp_acc.isin.to_string())
                                        .unwrap_or(&FloatMap::def()),
                                    inp_acc.isin,
                                    float_mapping
                                        .get(&inp_acc.isin.to_string())
                                        .unwrap_or(&FloatMap::def())
                                        .amt,
                                );
                            }
                        } else {
                            tot_amt_in_op += op_data.cf_amt.parse::<f64>().unwrap_or(0.0);
                            let prin_cf = format_output(&op_data);
                            op_writer1
                                .write_all(prin_cf.as_bytes())
                                .expect("Error writing principal cf to output path 1!!");
                            if config_params.is_perf_diagnostics_enabled() {
                                debug!(
                                    _diag_log,
                                    "The BENPOS Data of isin: {:?}  for product-type: {:?} -> {:?}",
                                    inp_acc.isin,
                                    inp_acc.prod_type,
                                    benpos_data
                                );
                                debug!(
                                    _diag_log,
                                    "The BENPOS Mapping for pangir1: {:?} -> {:?}\n",
                                    benpos_data.pangir1,
                                    benpos_mapping
                                        .get(&benpos_data.pangir1.to_string())
                                        .unwrap_or(&BenposMap::def())
                                );
                            }
                        }
                    } else {
                        op_data.component = "PRINCIPAL".to_string();
                        op_data.amt_due = (benpos_data.nfacevn * benpos_data.position).to_string();
                        op_data.composition_percentage =
                            ((op_data.amt_due.parse::<f64>().unwrap_or(0.0) / sum) * 100.00)
                                .to_string();
                        op_data.cf_amt =
                            ((op_data.composition_percentage.parse::<f64>().unwrap_or(0.0)
                                / 100.00)
                                * op_data.amt_setld.parse::<f64>().unwrap_or(0.0))
                            .to_string();
                        if config_params.is_perf_diagnostics_enabled() {
                            debug!(
                                _diag_log,
                                "The BENPOS Data of isin: {:?}  for product-type: {:?} -> {:?}",
                                inp_acc.isin,
                                inp_acc.prod_type,
                                benpos_data
                            );
                            debug!(
                                _diag_log,
                                "The BENPOS Mapping for pangir1: {:?} -> {:?}\n",
                                benpos_data.pangir1,
                                benpos_mapping
                                    .get(&benpos_data.pangir1.to_string())
                                    .unwrap_or(&BenposMap::def())
                            );
                        }
                        op_data.bucket_category = benpos_mapping
                            .get(&benpos_data.pangir1.to_string())
                            .unwrap_or(&BenposMap::def())
                            .category
                            .to_string();
                        op_data.cust_name = benpos_data.name1.to_string();
                        tot_amt_in_op += op_data.cf_amt.parse::<f64>().unwrap_or(0.0);
                        let prin_cf1 = format_output(&op_data);
                        op_writer1
                            .write_all(prin_cf1.as_bytes())
                            .expect("Error writing principal cf to output path 1!!");
                    }
                }
            }
            None => {
                let default_acc_data = format_output(&op_data);
                tot_amt_in_op += op_data.cf_amt.parse::<f64>().unwrap_or(0.0);
                warn!(
                    log,
                    "From Benpos-file: Default account written as cust-no: {:?} with reference: {:?}",
                    op_data.cust_no,
                    op_data.reference
                );
                op_writer1
                    .write_all(default_acc_data.as_bytes())
                    .expect("Error writing default account data to output path 1!!");
            }
        };

        if inp_acc.prod_type != "Issue: Zero Coupon Par Premium"
            && inp_acc.prod_type != "Issue: Commercial Paper"
            && inp_acc.prod_type != "Term Loans"
            && inp_acc.prod_type != "WCDL"
        {
            match cf_map.get(acc_id) {
                Some(cf_data_set) => {
                    for cf_data in cf_data_set {
                        tot_cf_acc += 1;
                        if cf_data.cashflow_type == "Borrowing / Nominal Interest"
                            || cf_data.cashflow_type == "Interest Amount"
                            || cf_data.cashflow_type == "Interest on Debenture"
                            || cf_data.cashflow_type == "Interest on NCD with Instal Repay"
                            || cf_data.cashflow_type == "Interest on Partly Paid Debn"
                            || cf_data.cashflow_type == "Interest"
                        {
                            op_data.component = "INTEREST".to_string();
                            op_data.due_dt = date_parser
                                .parse_opt(&cf_data.cashflow_date)
                                .unwrap_or(*config_params.as_on_date())
                                .format("%d-%m-%Y")
                                .to_string();
                            let mut sign = 1.0;
                            let default_borr_data = BorrUpdateMap::default();
                            let borr_data = borr_update_type_master
                                .get(&cf_data.cashflow_type.to_string())
                                .unwrap_or(&default_borr_data);
                            if (borr_data.cf_type == "Interest".to_string()
                                && borr_data.sign == "NEG".to_string())
                            {
                                sign *= -1.0;
                            } else if (borr_data.cf_type == "Interest".to_string()
                                && borr_data.sign == "POS".to_string())
                            {
                                sign *= 1.0;
                            } else if (borr_data.cf_type == "".to_string()
                                || borr_data.sign == "".to_string())
                            {
                                log_error!(
                                    log,
                                    "Cannot get {} of Interest from Borrowings Update Type Master file",
                                    cf_data.cashflow_type
                                );
                            }
                            let cf_amount_temp = cf_data.cashflow_amount * sign;
                            op_data.cf_amt = cf_amount_temp.to_string();
                            let int_cf = format_output(&op_data);
                            op_writer1
                                .write_all(int_cf.as_bytes())
                                .expect("Error writing interest cf to output path!!");
                        }
                    }
                }
                None => {
                    op_data.component = "INTEREST".to_string();
                    op_data.cf_amt = inp_acc.outstanding_bal.to_string();
                    let default_acc_data = format_output(&op_data);
                    tot_amt_in_op += op_data.cf_amt.parse::<f64>().unwrap_or(0.0);
                    warn!(
                        log,
                        "From cashflow-file: Default account written as cust-no: {:?} with reference: {:?}",
                        op_data.cust_no,
                        op_data.reference
                    );
                    op_writer1
                        .write_all(default_acc_data.as_bytes())
                        .expect("Error writing default account data to output path 1!!");
                }
            };
        }
        if inp_acc.prod_type != "Issue: Zero Coupon Par Premium"
            && inp_acc.prod_type != "Issue: Commercial Paper"
        {
            op_data.amt_due = inp_acc.outstanding_bal.to_string();
            op_data.cust_name = inp_acc.lender_name.to_string();
            op_data.due_dt = inp_acc.next_in_payout_date.to_string();
            op_data.composition_percentage = "".to_string();
            op_data.cf_amt = ((op_data.amt_setld.parse::<f64>().unwrap_or(0.0))
                - (op_data.prin_ost_bal.parse::<f64>().unwrap_or(0.0)))
            .to_string();
            tot_amt_in_op += op_data.cf_amt.parse::<f64>().unwrap_or(0.0);
            let prin_cf2 = format_output(&op_data);
            op_writer2
                .write_all(prin_cf2.as_bytes())
                .expect("Error writing principal cf to output path 2!!");
        }
    }
    log_info!(
        log,
        "Number of records read from master file: {}",
        tot_acc_encntrd
    );
    // Flush Output Writers
    op_writer1
        .flush()
        .expect("Error while flushing data from writer buffer 1!!");
    op_writer2
        .flush()
        .expect("Error while flushing data from writer buffer 2!!");
    // Generate Health Check Report
    let health_report = health_report::HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skip_rec_count,
        skip_rec_count,
        tot_amt_in_ip,
        tot_amt_in_op,
        tot_cf_acc,
    );
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file_path());
}
