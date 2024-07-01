use self::io::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
use self::account::ProdGLData;
use calamine::{open_workbook_auto, Reader};
use chrono::Datelike;
use health_report::HealthReport;
use macros;
use rbdate::DateParser;
use rbdate::NaiveDate;
use rbdate::{decr_dt_by_mon_presrv_eom, get_month_end_date};
use sdb_cf_gen::get_freq;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
mod account;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;
    let mut excess_writer = get_writer(config_params.excess_output_file_path());
    let mut overdue_writer = get_writer(config_params.overdue_output_file_path());
    let mut singleeistructure_writer =
        get_writer(config_params.single_eistructure_output_file_path());
    let mut excess_emimultiple_writer =
        get_writer(config_params.excess_emimultiple_output_file_path());
    let mut overdueemimultiple_writer =
        get_writer(config_params.overdue_emimultiple_output_file_path());
    let mut multipleemiaccounts_writer =
        get_writer(config_params.multipleemiaccounts_output_file_path());
    let mut multiplepmiaccounts_writer =
        get_writer(config_params.multiplepmiaccounts_output_file_path());

    //Reading Prod-GL Master File
    let mut prod_gl_data = open_workbook_auto(config_params.prod_gl_master_file())
        .expect("Could Not Read Prod-GL Master File");
    let mut prod_gl_map: HashMap<String, ProdGLData> = HashMap::new();
    println!(
        "Sheets present in Prod-GL-Master-File: `{:?}`",
        prod_gl_data.sheet_names()
    );
    if !prod_gl_data
        .sheet_names()
        .contains(&config_params.prod_gl_master_sheet().to_string())
    {
        panic!(
            "Sheet passed: `{}` as config-param not present in Prod-GL-Master File: `{}`",
            config_params.prod_gl_master_sheet(),
            config_params.prod_gl_master_file()
        );
    }
    println!(
        "Reading Sheet: `{}` from Prod-GL-Master-File",
        config_params.prod_gl_master_sheet()
    );
    if let Some(Ok(reader)) = prod_gl_data.worksheet_range(config_params.prod_gl_master_sheet()) {
        for product_gls in reader.rows().skip(1) {
            let prod_code = product_gls[0].to_string();
            let prod_gls_accs = ProdGLData::new(product_gls);
            if !prod_code.is_empty() {
                prod_gl_map.insert(prod_code, prod_gls_accs);
            }
        }
    }

    //Reading SMA FILE
    let mut sma_map: HashMap<String, String> = HashMap::new();
    let sma_file_reader =
        std::fs::read_to_string(config_params.sma_file_path()).expect("Could not read SMA File");
    for (line_no, line) in sma_file_reader.lines().enumerate() {
        let sma_data_vec: Vec<&str> = line.split(',').collect::<Vec<&str>>();
        let data_src_name_1 = get_str(config_params.sma_file_path(), &sma_data_vec, 1, line_no);
        if data_src_name_1.to_uppercase() == config_params.data_src_name().trim().to_uppercase() {
            sma_map.insert(
                get_str(config_params.sma_file_path(), &sma_data_vec, 2, line_no),
                get_str(config_params.sma_file_path(), &sma_data_vec, 14, line_no),
            );
        }
    }

    let master_file = File::open(config_params.input_master_file()).expect("Could Not Read File");
    let master_reader = BufReader::new(master_file);
    let date_parser = DateParser::new("%d/%m/%Y".to_string(), false);
    let header = "ACC_NO|DISBURSED_AMT|OS_LOAN_BAL_LOCAL_CURRENCY|CURR_APPLICABLE_INTEREST_RATE|EI_AMOUNT_CURRENT|INTEREST_TYPE|OS_P_BAL_DUE_LOCAL_CURRENCY|OS_I_BAL_DUE_LOCAL_CURRENCY|EI_AMT_PAID_ADVANCE_LOCAL_CURR|PRE_EI_BAL_LOCAL_CURR|ACCOUNT_OPEN_VALUE_DATE|MATURITY_DATE|EI_START_DATE_CURRENT|EI_END_DATE_CURRENT|EI_PAYMENT_FREQUENCY_CURRENT|EMI_LAST_PAID_DATE_CURRENT|EI_PAYMENT_DAY|EI_ORGINAL_TERM|EI_BALANCE_TERM|REPRICING_BENCHMARK|SPREAD|LAST_REPRICING_DATE|NEXT_REPRICING_DATE|REPRICING_FREQUENCY|NUMBER_EI_STRUCTURES|NPA_CLASSIFICATION|REMARK|MONTHS_OS_COMB|MORATORIUM_TYPE|FROM_MORATORIUM_DATE|TO_MORATORIUM_DATE|RECALCULATE_EI_AMOUNT_FLAG|MORATORIUM_INTEREST_CALCULATION|BULLET_PAYMENT_FLAG|RESTRUCTURED_FLAG|RESIDENTIAL_MORTGAGE|RISK_WEIGHT|INTERNAL_RATING|EXTERNAL_RATING|CONTRACTUAL_TENOR|RESIDUAL_TENOR|CUSTOMER_CONSTITUTION_CODE|PRODUCT_CODE|P_GL_CODE|M_NPA_CLASSIFICATION|ACCRUED_INTEREST|CUSTOMER_ID|CUSTOMER_NAME|GROUP_ID|GROUP_NAME|BRANCH_CODE|SECTOR|INDUSTRY|LTV|OVERDUE_ACCOUNT|EXCESS_ACCOUNT|LOAN_TYPE|RESIDUAL_INTEREST|CURRENCY|HDFC_LTD_PERCENTAGE|SECURITIZATION_PERCENTAGE|OVERDUE_TYPE|ALM_LINE|EMI_OVERDUE_GL_CD|PRE_EMI_OVERDUE_GL_CD|EXCESS_EMI_GL_CD|EXCESS_PRE_EMI_GL_CD|SMA_FLAG";
    for (index, line) in master_reader.lines().enumerate() {
        tot_acc_encntrd += 1;
        let line = line.expect("Could Not Read Line").to_string();
        let master_fields: Vec<&str> = line.split('|').collect();
        let mut final_op_writer = HashMap::new();
        acc_pro_suc += 1;
        //key 1= excess writer, key 2= overdue writer, key 3= singleEIStructure writer
        //key 4= ExcesseEMIMultiple writer, key 5= OverdueEMIMultiple writer
        //key 6=MultipleEMIAccounts writer, key 7=MultiplePMIAccounts writer
        if master_fields[47] <= "1" {
            if master_fields[26].parse::<f32>().unwrap_or(0.0) > 0.0 {
                final_op_writer.insert("1", &mut excess_writer);
            }
            if master_fields[22].parse::<f32>().unwrap_or(0.0) > 0.0
                || master_fields[24].parse::<f32>().unwrap_or(0.0) > 0.0
            {
                final_op_writer.insert("2", &mut overdue_writer);
            }
            final_op_writer.insert("3", &mut singleeistructure_writer);
        } else {
            if master_fields[26].parse::<f32>().unwrap_or(0.0) > 0.0 {
                final_op_writer.insert("4", &mut excess_emimultiple_writer);
            }
            if master_fields[22].parse::<f32>().unwrap_or(0.0) > 0.0
                || master_fields[24].parse::<f32>().unwrap_or(0.0) > 0.0
            {
                final_op_writer.insert("5", &mut overdueemimultiple_writer);
            }
            if master_fields[74].to_lowercase() == "emi"
                || master_fields[74].to_lowercase().replace(' ', "") == "emipmiboth"
            {
                final_op_writer.insert("6", &mut multipleemiaccounts_writer);
            }
            if master_fields[74].to_lowercase() == "pmi"
                || master_fields[74].to_lowercase().replace(' ', "") == "emipmiboth"
            {
                final_op_writer.insert("7", &mut multiplepmiaccounts_writer);
            }
        }
        let remark = if master_fields[74].to_lowercase() == "emi" || master_fields[74].is_empty() {
            "N"
        } else {
            "P"
        };
        let mut def_gl_codes = ProdGLData::def(config_params);
        let gl_codes = get_gl_codes(&prod_gl_map, &master_fields, logger, &mut def_gl_codes);

        for (key, writer) in final_op_writer {
            if index == 0 {
                writeln!(writer, "{}", header).expect("Error Writing Headers");
            }
            writeln!(writer,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}||L|||{}|||{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|||{}|{}|{}||{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            master_fields[0],
            master_fields[17],
            //key 1= excess, key 2= overdue, key 3= singleEIStructure 
            //key 4= ExcesseEMIMultiple, key 5= OverdueEMIMultiple
            //key 6=MultipleEMIAccounts, key 7=MultiplePMIAccounts
            if key == "1" || key == "2" || key == "4" || key == "5" || key == "7" {
                "0"
            }
            else{
                master_fields[20]
            },
            master_fields[35],
            master_fields[49],
            master_fields[34],
            if key == "1" || key == "3" || key == "4" || key == "6" || key == "7" {
                "0"
            }
            else{
                master_fields[22]
            },
            if key == "1" || key == "3" || key == "4" || key == "6" || key == "7" {
                "0"
            }
            else{
                master_fields[24]
            },
            if key == "2" || key == "3" || key == "5" || key == "6" || key == "7" {
                "0"
            }
            else{
                master_fields[26]
            },
            if key == "1" || key == "2" || key == "4" || key == "5" || key == "6" {
                "0"
            }
            else{
                master_fields[28]
            },
            date_parser.parse_opt(master_fields[10]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            get_maturity_date(config_params, master_fields.clone(), &date_parser, logger),
            date_parser.parse_opt(master_fields[51]).unwrap_or(rbdate::get_month_end_date(*config_params.as_on_date())).format("%d-%m-%Y"),
            date_parser.parse_opt(master_fields[52]).unwrap_or(date_parser.parse_opt(master_fields[11]).unwrap_or(config_params.as_on_date().succ_opt().expect("Error while incrementing date"))).format("%d-%m-%Y"),
            master_fields[48],
            if master_fields[53].is_empty() {
                let as_on_date = config_params.as_on_date();
                get_month_end_date((decr_dt_by_mon_presrv_eom(*as_on_date, 1)).expect("Error in decrementing date")).format("%d-%m-%Y").to_string()
            }else{
                date_parser.parse_opt(master_fields[53]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y").to_string()
            },
            if master_fields[53].is_empty() {
                let as_on_date = config_params.as_on_date();
                get_month_end_date((decr_dt_by_mon_presrv_eom(*as_on_date, 1)).expect("Error in decrementing date")).day().to_string()
            }else{
                master_fields[53][..2].to_string()
            },
            master_fields[12],
            master_fields[14],
            master_fields[37],
            master_fields[38],
            date_parser.parse_opt(master_fields[39]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            date_parser.parse_opt(master_fields[40]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            master_fields[41],
            master_fields[47],
            master_fields[70],
            master_fields[74],
            master_fields[75],
            remark,
            if remark == "N"{
                date_parser.parse_opt(master_fields[10]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y").to_string()
            }
            else{
                config_params.as_on_date().format("%d-%m-%Y").to_string()
            },
            if remark == "N"{
                date_parser.parse_opt(master_fields[10]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y").to_string()
            }
            else{
                rbdate::incr_dt_by_mon_presrv_eom(*config_params.as_on_date(),3).expect("Data is not in correct format").format("%d-%m-%Y").to_string()
            },
            if remark == "N"
            {
                remark
            }
            else
            {
                "Y"
            },
            master_fields[61],
            if master_fields[11].is_empty(){
                1
            }
            else if master_fields[10].is_empty() {
                0
            }else{
                rbdate::num_days_start_to_end(date_parser.parse(master_fields[10]), date_parser.parse(master_fields[11]))
            },
            if master_fields[11].is_empty(){
                1
            }else{
                rbdate::num_days_start_to_end(*config_params.as_on_date(), date_parser.parse(master_fields[11]))
            },
            master_fields[60],
            master_fields[8],
            gl_codes.gl_code,
            master_fields[70],
            master_fields[24],
            master_fields[3],
            master_fields[4],
            master_fields[6],
            master_fields[7],
            master_fields[1],
            master_fields[63],
            if master_fields[22].parse::<f64>().unwrap_or(0.0) <= 0.0 && master_fields[24].parse::<f64>().unwrap_or(0.0) <= 0.0 {
                "NO"
            }else{
                "YES"
            },
            if master_fields[26].parse::<f64>().unwrap_or(0.0) <= 0.0{
                "NO"
            }else{
                "YES"
            },
            master_fields[66],
            master_fields[18],
            master_fields[33],
            master_fields[32],
            //Overdue Type
            if master_fields[75].parse::<f64>().unwrap_or(0.0) == 0.0 {
                ""
            }
            else if master_fields[75].parse::<f64>().unwrap_or(0.0) == 1.0{
                "Overdue 1"
            }
            else if master_fields[75].parse::<f64>().unwrap_or(0.0) > 1.0 && master_fields[75].parse::<f64>().unwrap_or(0.0) < 7.0{
                "Overdue 2"
            }
            else if master_fields[75].parse::<f64>().unwrap_or(0.0) >= 7.0 && master_fields[75].parse::<f64>().unwrap_or(0.0) < 12.0{
                "Overdue 3"
            }
            else{
                "NPA"
            },
            master_fields[31],
            gl_codes.emi_overdue_gl_code,
            gl_codes.pre_emi_overdue_gl_code,
            gl_codes.emi_excess_gl_code,
            gl_codes.pre_emi_excess_gl_code,
            sma_map.get(&get_str(
                config_params.input_master_file(),
                &master_fields,
                0,
                index + 1,
            )).unwrap_or(&"P".to_string()),
        ).expect("Error while writing data");
        }
    }
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
        0.0,
        0.0,
        0,
    );
    create_hcrs(&health_report, config_params);
}

pub fn get_maturity_date(
    config_params: &ConfigurationParameters,
    input_acc: Vec<&str>,
    date_parser: &DateParser,
    logger: &Logger,
) -> String {
    let emi_last_paid_date = input_acc[53].to_string();
    let mut is_ei_paid_on_month_end = false;
    let mut maturity_date = date_parser
        .parse_opt(input_acc[11])
        .unwrap_or(*config_params.as_on_date());
    if maturity_date < *config_params.as_on_date() {
        log_debug!(logger,"Maturity Date less than AsOnDate for Account: {} \n Stamping AsOnDate as Maturity-Date ",input_acc[0]);
        maturity_date = *config_params.as_on_date();
    }
    let mat_date = if !input_acc[11].to_string().is_empty() {
        maturity_date.format("%d-%m-%Y").to_string()
    } else {
        let orig_term = input_acc[12].to_string();
        let ei_period = input_acc[48].to_string();
        let freq = get_freq(&ei_period);
        let months_to_be_added = freq * orig_term.to_string().parse::<usize>().unwrap_or(0);
        if orig_term.is_empty() || ei_period.is_empty() {
            log_debug!(logger,"Orig-Term/EI-Period is Empty for Account: {} \n Stamping AsOnDate as Maturity-Date ",input_acc[0]);
            config_params.as_on_date().format("%d-%m-%Y").to_string()
        } else {
            let maturity_day = if !emi_last_paid_date.is_empty() {
                let emi_date = date_parser
                    .parse_opt(&emi_last_paid_date)
                    .expect("Error while reading EMI-Last-Paid-Date");
                is_ei_paid_on_month_end = rbdate::is_month_end_date(emi_date);
                emi_date.day()
            } else {
                rbdate::get_month_end_date(
                    NaiveDate::from_ymd_opt(
                        config_params.as_on_date().year(),
                        config_params.as_on_date().month(),
                        1,
                    )
                    .unwrap_or(*config_params.as_on_date()),
                )
                .day()
            };
            let mon_year_date = rbdate::increment_date_by_months(
                *config_params.as_on_date(),
                months_to_be_added as u16,
            );
            if maturity_day <= get_month_end_date(mon_year_date).day() && !is_ei_paid_on_month_end {
                NaiveDate::from_ymd_opt(mon_year_date.year(), mon_year_date.month(), maturity_day)
            } else {
                NaiveDate::from_ymd_opt(
                    mon_year_date.year(),
                    mon_year_date.month(),
                    get_month_end_date(mon_year_date).day(),
                )
            }
            .expect("Error getting mat-date")
            .format("%d-%m-%Y")
            .to_string()
        }
    };
    mat_date
}

pub fn get_gl_codes(
    prod_gl_map: &HashMap<String, ProdGLData>,
    input_acc: &[&str],
    logger: &Logger,
    def_gl_codes: &mut ProdGLData,
) -> ProdGLData {
    let prod_code = input_acc[66].to_string();
    let mut gl_codes: ProdGLData = prod_gl_map.get(&prod_code).unwrap_or(def_gl_codes).clone();

    match !input_acc[31].is_empty() {
        true => gl_codes.gl_code = input_acc[31].to_string(),
        false => {
            log_warn!(
                logger,
                "GL-Code not found in Input File for Account: {}",
                input_acc[0]
            );
            log_debug!(
                logger,
                "Stamping GL-Code: {} from Prod-GL-Master for Prod-Code: {}",
                gl_codes.gl_code,
                prod_code
            );
        }
    }
    match !input_acc[76].is_empty() {
        true => gl_codes.emi_overdue_gl_code = input_acc[76].to_string(),
        false => {
            log_warn!(
                logger,
                "EMI-Overdue-GL-Code not found in Input File for Account: {}",
                input_acc[0]
            );
            log_debug!(
                logger,
                "Stamping EMI-Overdue-GL-Code: {} from Prod-GL-Master for Prod-Code: {}",
                gl_codes.emi_overdue_gl_code,
                prod_code
            );
        }
    }
    match !input_acc[77].is_empty() {
        true => gl_codes.pre_emi_overdue_gl_code = input_acc[77].to_string(),
        false => {
            log_warn!(
                logger,
                "PreEMI-Overdue-GL-Code not found in Input File for Account: {}",
                input_acc[0]
            );
            log_debug!(
                logger,
                "Stamping PreEMI-Overdue-GL-Code: {} from Prod-GL-Master for Prod-Code: {}",
                gl_codes.pre_emi_overdue_gl_code,
                prod_code
            );
        }
    }
    match !input_acc[78].is_empty() {
        true => gl_codes.emi_excess_gl_code = input_acc[78].to_string(),
        false => {
            log_warn!(
                logger,
                "EMI-Excess-GL-Code not found in Input File for Account: {}",
                input_acc[0]
            );
            log_debug!(
                logger,
                "Stamping EMI-Excess-GL-Code: {} from Prod-GL-Master for Prod-Code: {}",
                gl_codes.emi_excess_gl_code,
                prod_code
            );
        }
    }
    match !input_acc[79].is_empty() {
        true => gl_codes.pre_emi_excess_gl_code = input_acc[79].to_string(),
        false => {
            log_warn!(
                logger,
                "PreEMI-Excess-GL-Code not found in Input File for Account: {}",
                input_acc[0]
            );
            log_debug!(
                logger,
                "Stamping PreEMI-Excess-GL-Code: {} from Prod-GL-Master for Prod-Code: {}",
                gl_codes.pre_emi_excess_gl_code,
                prod_code
            );
        }
    }

    gl_codes
}

pub fn get_str(input_file: &str, data: &[&str], index: usize, row: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` in row-no: `{:?}` from File: {}",
                index + 1,
                row,
                input_file,
            )
        })
        .trim()
        .trim_matches(|pat| pat == ' ' || pat == '"')
        .to_string()
}

pub fn create_hcrs(health_report: &HealthReport, config_params: &ConfigurationParameters) {
    health_report.gen_health_rpt(config_params.excess_output_file_path());
    health_report.gen_health_rpt(config_params.overdue_output_file_path());
    health_report.gen_health_rpt(config_params.single_eistructure_output_file_path());
    health_report.gen_health_rpt(config_params.excess_emimultiple_output_file_path());
    health_report.gen_health_rpt(config_params.overdue_emimultiple_output_file_path());
    health_report.gen_health_rpt(config_params.multipleemiaccounts_output_file_path());
    health_report.gen_health_rpt(config_params.multiplepmiaccounts_output_file_path());
}
