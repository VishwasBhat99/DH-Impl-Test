use calamine::{open_workbook_auto, Reader};
use rbdate::timestamp;
use sdb_dyn_proto_rdr::reader;
use serde_derive::{Deserialize, Serialize};
use slog::Logger;
mod account_with_cashflows;
mod account_with_cashflows_writer;
use cashflow_generator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use statics::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::time::SystemTime;
extern crate chrono;
extern crate rbdate;
extern crate sdb_dyn_proto_rdr;
extern crate sdb_io;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use crate::cashflow_generator::account_with_cashflows::{AccountWithCashflows, Cashflow};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub account_number: String,
    pub gl_code: String,
    pub rate_flag: String,
    pub repricing_index: String,
    pub customer_id: String,
    pub maturity_date: String,
}

impl AccFieldNames {
    pub fn new_from_path(_path: &str) -> AccFieldNames {
        let mut file =
            sdb_io::open_file_read(_path).expect("Cannot open the account metadata file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: AccFieldNames = serde_json::from_str(&buf[..])
            .expect("Account metadata json file was not well-formatted");
        req_fields
    }
}

#[derive(Serialize, Deserialize)]
struct Field {
    name: String,
    position: usize,
    typ: String,
}
#[derive(Serialize, Deserialize)]
struct Fields {
    fields: Vec<Field>,
}

enum Data {
    Float(f64),
    Integer(i64),
    String(String),
}
impl Data {
    // method to get the float value, returns None if the variant is not Float
    fn as_float(&self) -> Option<f64> {
        match *self {
            Data::Float(f) => Some(f),
            _ => None,
        }
    }

    // method to get the integer value, returns None if the variant is not Integer
    fn as_integer(&self) -> Option<i64> {
        match *self {
            Data::Integer(i) => Some(i),
            _ => None,
        }
    }

    // method to get the text value, returns None if the variant is not Text
    fn as_string(&self) -> Option<String> {
        match *self {
            Data::String(ref s) => Some(s.to_string()),
            _ => None,
        }
    }
}
pub fn generate(config_params: &ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let mut total_accounts_encountered: i64 = DEFAULT_INT;
    let mut total_accounts_with_cashflows: i64 = DEFAULT_INT;
    let mut total_cfs: usize = 0;
    let mut tot_prin_in_in = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let mut tot_int_in_op = DEFAULT_FLOAT;

    let start_generate_timer = SystemTime::now();
    let mut writer = create_io_workers(config_params.output_file_path(), log);
    let keys = AccFieldNames::new_from_path(config_params.req_fields_file_path());

    let mut metadata_file =
        File::open(config_params.metadata_file_path()).expect("failed to open metadata file");
    let mut contents = String::new();
    metadata_file
        .read_to_string(&mut contents)
        .expect("failed to read metadata file");

    let metadata: Fields = serde_json::from_str(&contents).expect("failed to parse metadata JSON");

    let mut account_reader = reader::Reader::new_at_path(
        config_params.metadata_file_path(),
        config_params.input_cf_file_path(),
    );
    let npa_class_map = get_npa_class_map(config_params.npa_class_file_path(), 2, 7, '|');
    let cust_id_map = get_cust_id_map(config_params.common_cust_file_path(), 1, 2, '|');
    let mut ora_gl_map: HashMap<String, String> = HashMap::new();
    let mut master_llg_map: HashMap<String, (String, String, String)> = HashMap::new();
    let mut ora_gl = open_workbook_auto(config_params.ora_gl_file_path())
        .expect("Could not read ora gl file path.");
    if !ora_gl
        .sheet_names()
        .contains(&config_params.ora_gl_sheet_name().to_string())
    {
        panic!(
            "sheet name {} is not present in {} : Available sheet names :{:?}",
            config_params.ora_gl_sheet_name(),
            config_params.ora_gl_file_path(),
            ora_gl.sheet_names()
        )
    }
    if let Some(Ok(reader)) = ora_gl.worksheet_range(config_params.ora_gl_sheet_name()) {
        for row in reader.rows() {
            ora_gl_map.insert(
                row[0].to_string(),
                format!("{}_{}_{}_{}", row[2], row[4], row[1], row[5]),
            );
        }
    }

    let mut master_llg = open_workbook_auto(config_params.master_llg_file_path())
        .expect("Could not read master llg file path.");
    if !master_llg
        .sheet_names()
        .contains(&config_params.master_llg_sheet_name().to_string())
    {
        panic!(
            "sheet name {} is not present in {} : Available sheet names :{:?}",
            config_params.master_llg_sheet_name(),
            config_params.master_llg_file_path(),
            ora_gl.sheet_names()
        )
    }
    if let Some(Ok(reader)) = master_llg.worksheet_range(config_params.master_llg_sheet_name()) {
        for row in reader.rows() {
            master_llg_map.insert(
                row[0].to_string(),
                (row[6].to_string(), row[7].to_string(), row[9].to_string()),
            );
        }
    }

    let rw_map = get_risk_weight_map(config_params.risk_weight_file_path(), '|');
    let resid_map = get_resid_map(config_params.resid_file_path(), '|');
    let rf_map = get_restructure_flag_map(config_params.restructure_flag_file_path(), '|');
    let rate_code_pos = RateCodeMasterFieldPosition {
        rate_code_pos: 1,
        interpretation_pos: 2,
        rate_flag_pos: 3,
        days_added_to_bus_dt_pos: 4,
        reset_freq_pos: 5,
        reset_month_pos: 6,
        reset_day_pos: 7,
        override_sys_reset_dt_pos: 8,
    };
    let finnone_master_map =
        get_rate_code_map(config_params.finnone_master_file_path(), rate_code_pos, '|');
    for mut account_with_cfs in account_reader.iter() {
        total_accounts_encountered += 1;
        let mut data_vec: Vec<Data> = Vec::new();
        let mut out_acc = AccountWithCashflows::new();
        for fields in &metadata.fields {
            let data_type = fields.typ.as_str();
            let mut float_data = Data::Float(0.0);
            let mut int_data = Data::Integer(0);
            let mut str_data = Data::String("".to_string());
            match data_type {
                "F64" => {
                    float_data = Data::Float(
                        (&account_with_cfs)
                            .get_f64_for_key(&fields.name)
                            .unwrap_or(0.0),
                    );
                    data_vec.push(float_data);
                }
                "I64" => {
                    int_data = Data::Integer(
                        (&account_with_cfs)
                            .get_i64_for_key(&fields.name)
                            .unwrap_or(0),
                    );
                    data_vec.push(int_data);
                }
                "I32" => {
                    int_data = Data::Integer(
                        (&account_with_cfs)
                            .get_i32_for_key(&fields.name)
                            .unwrap_or(0) as i64,
                    );
                    data_vec.push(int_data);
                }
                "String" => {
                    str_data = Data::String(
                        account_with_cfs
                            .get_string_for_key(&fields.name)
                            .unwrap_or(&"".to_string())
                            .to_string(),
                    );
                    data_vec.push(str_data);
                }
                "Cashflows" => {
                    total_accounts_with_cashflows += 1;
                    let mut cf_vec: Vec<Cashflow> = Vec::new();
                    match account_with_cfs.remove_cfs_for_key(&fields.name) {
                        Ok(cashflows) => {
                            for cf in cashflows {
                                total_cfs += 1;
                                tot_prin_in_in += cf.principal_amount;
                                tot_prin_in_op += cf.principal_amount;
                                tot_int_in_op += cf.interest_amount;
                                cf_vec.push(new_cashflow(
                                    cf.interest_amount,
                                    cf.principal_amount,
                                    cf.date,
                                ));
                            }
                        }
                        Err(_err) => {
                            log_debug!(
                                log,
                                "Account skipped due to empty cashflow {}",
                                data_vec[0].as_string().unwrap_or("".to_string())
                            );
                            continue;
                        }
                    };
                    out_acc.cashflows = protobuf::RepeatedField::from_vec(cf_vec);
                }
                _ => panic!("Unsupported data type"),
            };
        }

        let def_npa = "P".to_string();
        let gl_code: String = account_with_cfs
            .get_i64_for_key(&keys.gl_code)
            .unwrap_or(0)
            .to_string();
        let account_number = account_with_cfs
            .get_string_for_key(&keys.account_number)
            .unwrap_or(&"NONE".to_string())
            .to_string();
        let hdfcltd_custid = account_with_cfs
            .get_string_for_key(&keys.customer_id)
            .unwrap_or(&"NONE".to_string())
            .to_string();
        let npa_class = npa_class_map.get(&account_number).unwrap_or(&def_npa);
        let cust_id = cust_id_map
            .get(&hdfcltd_custid)
            .unwrap_or(&hdfcltd_custid)
            .to_string();
        let next_reprice_dt = config_params.as_on_date();
        let risk_weight = rw_map
            .get(&account_number)
            .unwrap_or(&"99.9".to_string())
            .to_string();
        let resid = resid_map
            .get(&account_number)
            .unwrap_or(&"N".to_string())
            .to_string();
        let restructural_flag = rf_map
            .get(&account_number)
            .unwrap_or(&"N".to_string())
            .to_string();
        let def_concat = "NONE".to_string();
        let ora_concat = ora_gl_map.get(&gl_code).unwrap_or(&def_concat);
        let def_llg_data = ("NONE".to_string(), "NONE".to_string(), "NONE".to_string());
        let master_llg_data = master_llg_map.get(ora_concat).unwrap_or(&def_llg_data);
        let alm_line = master_llg_data.0.to_owned();
        let ia_line = master_llg_data.1.to_owned();
        let balm_l2 = master_llg_data.2.to_owned();
        out_acc.accno = data_vec[0].as_string().unwrap_or("".to_string());
        out_acc.branchcode = data_vec[1].as_string().unwrap_or("".to_string());
        out_acc.custno = data_vec[2].as_string().unwrap_or("".to_string());
        out_acc.uccid = data_vec[3].as_string().unwrap_or("".to_string());
        out_acc.ccy = data_vec[4].as_string().unwrap_or("".to_string());
        out_acc.product = data_vec[5].as_string().unwrap_or("".to_string());
        out_acc.acc_date = data_vec[6].as_integer().unwrap_or(0);
        out_acc.gl_code = data_vec[7].as_integer().unwrap_or(0);
        out_acc.glcode_compounded_portion = data_vec[8].as_integer().unwrap_or(0);
        out_acc.glcode_int_accrued = data_vec[9].as_integer().unwrap_or(0);
        out_acc.deposit_date = data_vec[10].as_integer().unwrap_or(0);
        out_acc.initial_deposit_amount = data_vec[11].as_integer().unwrap_or(0);
        out_acc.initial_dep_amtlcy = data_vec[12].as_integer().unwrap_or(0);
        out_acc.current_outstanding_bal = data_vec[13].as_float().unwrap_or(0.0);
        out_acc.current_outstandingbal_lcy = data_vec[14].as_integer().unwrap_or(0);
        out_acc.cum_interest = data_vec[15].as_float().unwrap_or(0.0);
        out_acc.cum_interest_amt_lcy = data_vec[16].as_float().unwrap_or(0.0);
        out_acc.maturity_date = data_vec[17].as_integer().unwrap_or(0);
        out_acc.interest_type = data_vec[18].as_string().unwrap_or("".to_string());
        out_acc.interst_acrrual_basis = data_vec[19].as_string().unwrap_or("".to_string());
        out_acc.interest_accured_amount = data_vec[20].as_float().unwrap_or(0.0);
        out_acc.interest_compution_type = data_vec[21].as_string().unwrap_or("".to_string());
        out_acc.interest_rate = data_vec[22].as_float().unwrap_or(0.0);
        out_acc.interest_payment_freq = data_vec[23].as_float().unwrap_or(0.0);
        out_acc.next_int_payment_dt = data_vec[24].as_float().unwrap_or(0.0);
        out_acc.compounding_freq = data_vec[25].as_integer().unwrap_or(0);
        out_acc.next_compounding_dt = data_vec[26].as_integer().unwrap_or(0);
        out_acc.floating_rate_benchmark = data_vec[27].as_string().unwrap_or("".to_string());
        out_acc.spread = data_vec[28].as_integer().unwrap_or(0);
        out_acc.next_repricing_dt = data_vec[29].as_integer().unwrap_or(0);
        out_acc.repricing_frequency = data_vec[30].as_integer().unwrap_or(0);
        out_acc.non_withdrawable_flag = data_vec[31].as_string().unwrap_or("".to_string());
        out_acc.noticedays = data_vec[32].as_string().unwrap_or("".to_string());
        out_acc.lockin_till_dt = data_vec[33].as_integer().unwrap_or(0);
        out_acc.dep_pledged_against_loan_yn = data_vec[34].as_string().unwrap_or("".to_string());
        out_acc.customerconstitutioncode_1 = data_vec[35].as_string().unwrap_or("".to_string());
        out_acc.customerconstitutioncode_2 = data_vec[36].as_string().unwrap_or("".to_string());
        out_acc.period_months = data_vec[37].as_integer().unwrap_or(0);
        out_acc.period_days = data_vec[38].as_integer().unwrap_or(0);
        out_acc.intrest_craeted_upto = data_vec[39].as_integer().unwrap_or(0);
        out_acc.interest_accrued_upto = data_vec[40].as_integer().unwrap_or(0);
        out_acc.f_15hyear = data_vec[41].as_string().unwrap_or("".to_string());
        out_acc.customer_name = data_vec[42].as_string().unwrap_or("".to_string());
        out_acc.total_principal_balance = data_vec[43].as_float().unwrap_or(0.0);
        out_acc.alm_line = alm_line.to_owned();
        out_acc.npa_class = npa_class.to_string();
        out_acc.common_cust_id = cust_id;
        out_acc.risk_weight = risk_weight;
        out_acc.restructure_flag = restructural_flag;
        out_acc.resid = resid;
        out_acc.derived_next_reprice_date = timestamp(*next_reprice_dt);
        out_acc.ia_line = ia_line.to_owned();
        out_acc.balm_l2 = balm_l2.to_owned();
        out_acc.ora_concat = ora_concat.to_owned();
        out_acc.tot_balance = out_acc.current_outstanding_bal + out_acc.cum_interest;
        writer.write(out_acc);
    }
    writer.close();

    let end_generate_timer = SystemTime::now();
    let total_duration = end_generate_timer
        .duration_since(start_generate_timer)
        .expect("Could not calculate total duration for generate timer.");
    let report_string = format!(
        "Accounts Encountered: {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total Duration: {:.2?}\n\
         Total outstanding amount in input: {:.2} \n\
         Total outstanding amount in output: {:.2}\n\
         Total interest amount in output: {:.2}",
        total_accounts_encountered,
        total_accounts_with_cashflows,
        total_cfs,
        total_duration,
        tot_prin_in_in,
        tot_prin_in_op,
        tot_int_in_op
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);

    let health_stat = HealthReport::new(
        total_accounts_encountered,
        total_accounts_with_cashflows,
        0,
        tot_prin_in_in,
        tot_prin_in_op,
        total_cfs as i64,
    );
    health_stat.gen_health_rpt(config_params.output_file_path())
}

fn create_io_workers(output_path: &str, log: &Logger) -> AccountWithCashflowsWriter {
    let output_path_str = format!("{}", output_path);
    let writer = AccountWithCashflowsWriter::new(&output_path_str, log);

    writer
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;

    cf
}

pub fn get_risk_weight_map(
    risk_weight_file_path: &str,
    delimiter: char,
) -> HashMap<String, String> {
    let mut rw_map: HashMap<String, String> = HashMap::new();
    let file = File::open(risk_weight_file_path).expect("Could not open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let split: Vec<&str> = line.split(delimiter).collect();
        let account_number = split[0].to_string();
        let rw = split[1].to_string();
        rw_map.insert(account_number, rw);
    }
    rw_map
}

pub fn get_resid_map(resid_file_path: &str, delimiter: char) -> HashMap<String, String> {
    let mut resid_map: HashMap<String, String> = HashMap::new();
    let file = File::open(resid_file_path).expect("Could not open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let split: Vec<&str> = line.split(delimiter).collect();
        let account_number = split[0].to_string();
        let resid = split[1].to_string();
        resid_map.insert(account_number, resid);
    }
    resid_map
}

pub fn get_restructure_flag_map(
    restructure_flag_file_path: &str,
    delimiter: char,
) -> HashMap<String, String> {
    let mut rf_map: HashMap<String, String> = HashMap::new();
    let file = File::open(restructure_flag_file_path).expect("Could not open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let split: Vec<&str> = line.split(delimiter).collect();
        let account_number = split[0].to_string();
        let rf = split[1].to_string();
        rf_map.insert(account_number, rf);
    }
    rf_map
}

pub fn get_npa_class_map(
    npa_class_input_path: &str,
    account_number_pos: usize,
    asset_class_pos: usize,
    delimiter: char,
) -> HashMap<String, String> {
    let mut npa_map = HashMap::new();
    let file = File::open(npa_class_input_path).expect("Could not open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let split: Vec<&str> = line.split(delimiter).collect();
        let account_number = split[account_number_pos - 1].to_string();
        let asset_class = split[asset_class_pos - 1].to_string();
        npa_map.insert(account_number, asset_class);
    }
    npa_map
}

pub fn get_cust_id_map(
    cust_id_input_path: &str,
    hdfcltd_custid_pos: usize,
    hdfcbank_custid_pos: usize,
    delimiter: char,
) -> HashMap<String, String> {
    let mut alm_map = HashMap::new();
    let file = File::open(cust_id_input_path).expect("Could not open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let split: Vec<&str> = line.split(delimiter).collect();
        let hdfcltd_custid = split[hdfcltd_custid_pos - 1].to_string();
        let hdfcbank_custid = split[hdfcbank_custid_pos - 1].to_string();
        alm_map.insert(hdfcltd_custid, hdfcbank_custid);
    }
    alm_map
}

#[derive(Debug)]
pub struct RateCodeMaster {
    pub interpretation: String,
    pub rate_flag: String,
    pub days_added_to_bus_dt: String,
    pub reset_freq: String,
    pub reset_month: String,
    pub reset_day: String,
    pub override_sys_reset_dt: String,
}

#[derive(Debug)]
pub struct RateCodeMasterFieldPosition {
    pub rate_code_pos: usize,
    pub interpretation_pos: usize,
    pub rate_flag_pos: usize,
    pub days_added_to_bus_dt_pos: usize,
    pub reset_freq_pos: usize,
    pub reset_month_pos: usize,
    pub reset_day_pos: usize,
    pub override_sys_reset_dt_pos: usize,
}

pub fn get_rate_code_map(
    rate_code_master_input_path: &str,
    ratecodeposition: RateCodeMasterFieldPosition,
    delimiter: char,
) -> HashMap<String, RateCodeMaster> {
    let mut rate_code_map = HashMap::new();
    let file = File::open(rate_code_master_input_path).expect("Could not open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let split: Vec<&str> = line.split(delimiter).collect();
        let rate_code = split[ratecodeposition.rate_code_pos - 1].to_string();
        let rate_code_master = RateCodeMaster {
            interpretation: split[ratecodeposition.interpretation_pos - 1]
                .to_string()
                .trim_matches('"')
                .to_uppercase(),
            rate_flag: split[ratecodeposition.rate_flag_pos - 1].to_string(),
            days_added_to_bus_dt: split[ratecodeposition.days_added_to_bus_dt_pos - 1].to_string(),
            reset_freq: split[ratecodeposition.reset_day_pos - 1].to_string(),
            reset_month: split[ratecodeposition.reset_month_pos - 1].to_string(),
            reset_day: split[ratecodeposition.reset_day_pos - 1].to_string(),
            override_sys_reset_dt: split[ratecodeposition.override_sys_reset_dt_pos - 1]
                .to_string(),
        };
        rate_code_map.insert(rate_code, rate_code_master);
    }
    rate_code_map
}
