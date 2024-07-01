use calamine::{open_workbook_auto, Reader};
use rbdate::timestamp;
use sdb_dyn_proto_rdr::reader;
use serde_derive::{Deserialize, Serialize};
use slog::Logger;
mod account_with_cashflows;
mod account_with_cashflows_writer;
use cashflow_generator::account_with_cashflows::Cashflow;
use cashflow_generator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use statics::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::time::SystemTime;

use crate::cashflow_generator::account_with_cashflows::AccountWithCashflows;

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

    // method to get the String value, returns None if the variant is not Text
    fn as_string(&self) -> Option<String> {
        match *self {
            Data::String(ref s) => Some(s.to_string()),
            _ => None,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub account_number: String,
    pub concat: String,
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
    let npa_class_map = get_npa_class_map(config_params.npa_class_file_path(), 2, 7, ',');
    let cust_id_map = get_cust_id_map(config_params.common_cust_file_path(), 1, 2, '|');
    let mut balm_l2_map = HashMap::new();
    let mut ia_line_map = HashMap::new();
    let mut lacs_master = open_workbook_auto(config_params.lacs_master_file_path())
        .expect("Could not read lacs master file path.");
    if !lacs_master
        .sheet_names()
        .contains(&config_params.lacs_master_sheet_name().to_string())
    {
        panic!(
            "sheet name {} is not present in {} : Available sheet names :{:?}",
            config_params.lacs_master_sheet_name(),
            config_params.lacs_master_file_path(),
            lacs_master.sheet_names()
        )
    }
    if let Some(Ok(reader)) = lacs_master.worksheet_range(config_params.lacs_master_sheet_name()) {
        for row in reader.rows() {
            balm_l2_map.insert(row[0].to_string(), row[6].to_string());
            ia_line_map.insert(row[0].to_string(), row[5].to_string());
        }
    }
    let rw_map = get_risk_weight_map(config_params.risk_weight_file_path(), '|');
    let resid_map = get_resid_map(config_params.resid_file_path(), '|');
    let rf_map = get_restructure_flag_map(config_params.restructure_flag_file_path(), '|');
    let _rate_code_pos = RateCodeMasterFieldPosition {
        rate_code_pos: 1,
        interpretation_pos: 2,
        rate_flag_pos: 3,
        days_added_to_bus_dt_pos: 4,
        reset_freq_pos: 5,
        reset_month_pos: 6,
        reset_day_pos: 7,
        override_sys_reset_dt_pos: 8,
    };

    for mut account_with_cfs in account_reader.iter() {
        total_accounts_encountered += 1;
        let mut data_vec: Vec<Data> = Vec::new();
        let mut out_acc = AccountWithCashflows::new();
        for fields in &metadata.fields {
            let data_type = fields.typ.as_str();
            match data_type {
                "F64" => {
                    let float_data = Data::Float(
                        (account_with_cfs)
                            .get_f64_for_key(&fields.name)
                            .unwrap_or(0.0),
                    );
                    data_vec.push(float_data);
                }
                "I64" => {
                    let int_data = Data::Integer(
                        (account_with_cfs)
                            .get_i64_for_key(&fields.name)
                            .unwrap_or(0),
                    );
                    data_vec.push(int_data);
                }
                "I32" => {
                    let int_data = Data::Integer(
                        (account_with_cfs)
                            .get_i32_for_key(&fields.name)
                            .unwrap_or(0) as i64,
                    );
                    data_vec.push(int_data);
                }
                "String" => {
                    let str_data = Data::String(
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
        let def_str = "NONE".to_string();
        let def_rw = "99.9".to_string();
        let def_resid = "N".to_string();
        let account_number: String = account_with_cfs
            .get_string_for_key(&keys.account_number)
            .unwrap_or(&"NONE".to_string())
            .to_string();
        let concat: String = account_with_cfs
            .get_string_for_key(&keys.concat)
            .unwrap_or(&"NONE".to_string())
            .to_string();
        let hdfc_cust_id: String = account_with_cfs
            .get_string_for_key(&keys.customer_id)
            .unwrap_or(&"NONE".to_string())
            .to_string();
        let npa_class = npa_class_map.get(&account_number).unwrap_or(&def_npa);
        let cust_id = cust_id_map.get(&hdfc_cust_id).unwrap_or(&def_str);
        let next_reprice_dt = *config_params.as_on_date();
        let risk_weight = rw_map.get(&account_number).unwrap_or(&def_rw).to_string();
        let resid = resid_map
            .get(&account_number)
            .unwrap_or(&def_resid)
            .to_string();
        let restructural_flag = rf_map
            .get(&account_number)
            .unwrap_or(&def_resid)
            .to_string();
        let balm_l2 = balm_l2_map.get(&concat).unwrap_or(&def_str).to_string();
        let ia_line = ia_line_map.get(&concat).unwrap_or(&def_str).to_string();
        out_acc.acc_no = data_vec[0].as_string().unwrap_or("".to_string());
        out_acc.disbursed_amt = data_vec[1].as_float().unwrap_or(0.0);
        out_acc.os_loan_bal_lcy = data_vec[2].as_float().unwrap_or(0.0);
        out_acc.int_rate = data_vec[3].as_float().unwrap_or(0.0);
        out_acc.ei_amt_crnt = data_vec[4].as_float().unwrap_or(0.0);
        out_acc.int_type = data_vec[5].as_string().unwrap_or("".to_string());
        out_acc.os_p_bal_due_local_ccy = data_vec[6].as_float().unwrap_or(0.0);
        out_acc.os_i_bal_due_local_ccy = data_vec[7].as_float().unwrap_or(0.0);
        out_acc.ei_amt_paid_adv_lcy = data_vec[8].as_float().unwrap_or(0.0);
        out_acc.pre_ei_bal_lcy = data_vec[9].as_float().unwrap_or(0.0);
        out_acc.acc_open_value_date = data_vec[10].as_integer().unwrap_or(0);
        out_acc.maturity_date = data_vec[11].as_integer().unwrap_or(0);
        out_acc.ei_start_date_crnt = data_vec[12].as_integer().unwrap_or(0);
        out_acc.ei_end_date_crnt = data_vec[13].as_integer().unwrap_or(0);
        out_acc.ei_pay_freq_crnt = data_vec[14].as_string().unwrap_or("".to_string());
        out_acc.emi_last_paid_date_crnt = data_vec[15].as_integer().unwrap_or(0);
        out_acc.ei_pay_day = data_vec[16].as_integer().unwrap_or(0);
        out_acc.ei_orginal_term = data_vec[17].as_integer().unwrap_or(0);
        out_acc.ei_bal_term = data_vec[18].as_integer().unwrap_or(0);
        out_acc.rep_bm = data_vec[19].as_string().unwrap_or("".to_string());
        out_acc.spread = data_vec[20].as_string().unwrap_or("".to_string());
        out_acc.last_rep_date = data_vec[21].as_integer().unwrap_or(0);
        out_acc.next_rep_date = data_vec[22].as_integer().unwrap_or(0);
        out_acc.rep_freq = data_vec[23]
            .as_string()
            .unwrap_or("".to_string())
            .parse()
            .unwrap_or(0);
        out_acc.no_ei_structures = data_vec[24].as_integer().unwrap_or(0);
        out_acc.npa_class = data_vec[25].as_string().unwrap_or("".to_string());
        out_acc.remark = data_vec[26].as_string().unwrap_or("".to_string());
        out_acc.months_os_comb = data_vec[27].as_string().unwrap_or("".to_string());
        out_acc.mor_type = data_vec[28].as_string().unwrap_or("".to_string());
        out_acc.from_mor_date = data_vec[29].as_integer().unwrap_or(0);
        out_acc.to_mor_date = data_vec[30].as_integer().unwrap_or(0);
        out_acc.recalc_ei_amt_flag = data_vec[31].as_string().unwrap_or("".to_string());
        out_acc.mor_int_calc = data_vec[32].as_string().unwrap_or("".to_string());
        out_acc.bullet_pay_flag = data_vec[33].as_string().unwrap_or("".to_string());
        out_acc.restrct_flag = data_vec[34].as_string().unwrap_or("".to_string());
        out_acc.residential_mortgage = data_vec[35].as_string().unwrap_or("".to_string());
        out_acc.risk_weight = data_vec[36].as_string().unwrap_or("".to_string());
        out_acc.internal_rating = data_vec[37].as_string().unwrap_or("".to_string());
        out_acc.external_rating = data_vec[38].as_string().unwrap_or("".to_string());
        out_acc.contractual_tenor = data_vec[39].as_integer().unwrap_or(0);
        out_acc.residual_tenor = data_vec[40].as_integer().unwrap_or(0);
        out_acc.cust_constitution_code = data_vec[41].as_string().unwrap_or("".to_string());
        out_acc.prod_code = data_vec[42].as_string().unwrap_or("".to_string());
        out_acc.p_gl_code = data_vec[43].as_string().unwrap_or("".to_string());
        out_acc.p_gl_code = data_vec[43].as_string().unwrap_or("".to_string());
        out_acc.m_npaclass = data_vec[44].as_string().unwrap_or("".to_string());
        out_acc.acrd_int = data_vec[45].as_float().unwrap_or(0.0);
        out_acc.cust_id = data_vec[46].as_string().unwrap_or("".to_string());
        out_acc.cust_name = data_vec[47].as_string().unwrap_or("".to_string());
        out_acc.group_id = data_vec[48].as_string().unwrap_or("".to_string());
        out_acc.group_name = data_vec[49].as_string().unwrap_or("".to_string());
        out_acc.branch_code = data_vec[50].as_string().unwrap_or("".to_string());
        out_acc.sector = data_vec[51].as_string().unwrap_or("".to_string());
        out_acc.industry = data_vec[52].as_string().unwrap_or("".to_string());
        out_acc.ltv = data_vec[53].as_string().unwrap_or("".to_string());
        out_acc.overdue_acc = data_vec[54].as_string().unwrap_or("".to_string());
        out_acc.excess_acc = data_vec[55].as_string().unwrap_or("".to_string());
        out_acc.loan_type = data_vec[56].as_string().unwrap_or("".to_string());
        out_acc.resid_int = data_vec[57].as_float().unwrap_or(0.0);
        out_acc.ccy = data_vec[58].as_string().unwrap_or("".to_string());
        out_acc.hdfc_ltd_percent = data_vec[59].as_float().unwrap_or(0.0);
        out_acc.sec_percent = data_vec[60].as_float().unwrap_or(0.0);
        out_acc.overdue_type = data_vec[61].as_string().unwrap_or("".to_string());
        out_acc.alm_line = data_vec[62].as_string().unwrap_or("".to_string());
        out_acc.structure_number = data_vec[63].as_string().unwrap_or("".to_string());
        out_acc.memi = data_vec[64].as_float().unwrap_or(0.0);
        out_acc.ost_bal = data_vec[65].as_float().unwrap_or(0.0);
        out_acc.roi = data_vec[66].as_float().unwrap_or(0.0);
        out_acc.asondate = data_vec[67].as_integer().unwrap_or(0);
        out_acc.emi_overdue_gl_cd = data_vec[68].as_integer().unwrap_or(0);
        out_acc.pre_emi_overdue_gl_cd = data_vec[69].as_integer().unwrap_or(0);
        out_acc.excess_emi_gl_cd = data_vec[70].as_integer().unwrap_or(0);
        out_acc.excess_pre_emi_gl_cd = data_vec[71].as_integer().unwrap_or(0);
        out_acc.tot_prin_amt = data_vec[72].as_float().unwrap_or(0.0);
        out_acc.tot_int_amt = data_vec[73].as_float().unwrap_or(0.0);
        out_acc.balm_l2 = balm_l2;
        out_acc.derived_npa_class = npa_class.to_string();
        out_acc.common_cust_id = cust_id.to_string();
        out_acc.derived_risk_weight = risk_weight;
        out_acc.restructure_flag = restructural_flag;
        out_acc.resid = resid;
        out_acc.derived_next_reprice_date = timestamp(next_reprice_dt);
        out_acc.ia_line = ia_line;
        out_acc.sma_flag = data_vec[74].as_string().unwrap_or("P".to_string());
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
    AccountWithCashflowsWriter::new(&output_path_str, log)
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

pub fn _get_rate_code_map(
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

pub fn _get_alm_data_map(
    alm_input_path: &str,
    concat_pos: usize,
    alm_line_pos: usize,
    delimiter: char,
) -> HashMap<String, String> {
    let mut alm_map = HashMap::new();
    let file = File::open(alm_input_path).expect("Could not open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let split: Vec<&str> = line.split(delimiter).collect();
        let scheme_id = split[concat_pos - 1].to_string();
        let alm_line = split[alm_line_pos - 1].to_string();
        alm_map.insert(scheme_id, alm_line);
    }
    alm_map
}
