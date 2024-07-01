extern crate serde;
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct AccKey {
    pub acc_num: String,
    pub cf_dt: NaiveDate,
}
impl AccKey {
    pub fn new() -> AccKey {
        AccKey {
            ..Default::default()
        }
    }
}

pub fn process(config_param: ConfigurationParameters, _log: &Logger, diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };

    let start_derive_timer = SystemTime::now();
    let mut tot_master_acc_encntrd: i64 = 0;
    let mut tot_cashflow_acc_encntrd: i64 = 0;
    let mut writer = BufWriter::new(output_file);

    let master_reader = fs::read_to_string(config_param.master_input_file())
        .expect("Could Not Read master file path");
    let cashflow_reader = fs::read_to_string(config_param.cashflow_input_file())
        .expect("Could Not Read cashflow file path");

    let mut master_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut date_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut acc_set: HashSet<String> = HashSet::new();
    let mut cf_map: BTreeMap<AccKey, Vec<String>> = BTreeMap::new();

    for (_line_num, lines) in master_reader.lines().enumerate().skip(1) {
        let fields = lines.split("~").collect::<Vec<&str>>();
        if fields.len() != 48 {
            continue;
        }
        let mut final_vec: Vec<String> = Vec::new();
        tot_master_acc_encntrd += 1;

        let mut index = 0;
        let mut due_date = fields[6].to_string();
        due_date.retain(|inp_char| inp_char != '"');
        let due_dt = NaiveDate::parse_from_str(&due_date, "%d-%b-%Y")
            .unwrap_or(config_param.as_on_date)
            .format("%d-%m-%Y")
            .to_string();

        let mut mat_date = fields[10].to_string();
        mat_date.retain(|inp_char| inp_char != '"');
        let mat_dt = NaiveDate::parse_from_str(&mat_date, "%d-%b-%Y")
            .unwrap_or(config_param.as_on_date)
            .format("%d-%m-%Y")
            .to_string();

        let mut org_date = fields[13].to_string();
        org_date.retain(|inp_char| inp_char != '"');
        let org_dt = NaiveDate::parse_from_str(&org_date, "%d-%b-%Y")
            .unwrap_or(config_param.as_on_date)
            .format("%d-%m-%Y")
            .to_string();

        let mut inst_strt_date = fields[24].to_string();
        inst_strt_date.retain(|inp_char| inp_char != '"');
        let inst_strt_dt = NaiveDate::parse_from_str(&inst_strt_date, "%d-%b-%Y")
            .unwrap_or(config_param.as_on_date)
            .format("%d-%m-%Y")
            .to_string();

        let mut first_inst_date = fields[27].to_string();
        first_inst_date.retain(|inp_char| inp_char != '"');
        let first_inst_dt = NaiveDate::parse_from_str(&first_inst_date, "%d-%b-%Y")
            .unwrap_or(config_param.as_on_date)
            .format("%d-%m-%Y")
            .to_string();

        let mut lst_inst_date = fields[30].to_string();
        lst_inst_date.retain(|inp_char| inp_char != '"');
        let lst_inst_dt = NaiveDate::parse_from_str(&lst_inst_date, "%d-%b-%Y")
            .unwrap_or(config_param.as_on_date)
            .format("%d-%m-%Y")
            .to_string();

        let date_vec = vec![
            due_dt,
            mat_dt,
            org_dt,
            inst_strt_dt,
            first_inst_dt,
            lst_inst_dt,
        ];

        for values in &fields {
            index += 1;
            if index == 7 || index == 11 || index == 14 || index == 25 || index == 28 || index == 31
            {
                final_vec.push("".to_string());
                continue;
            }

            let mut field_val = values.to_string();
            field_val.retain(|inp_char| inp_char != '"');
            field_val.retain(|inp_char| inp_char != '"');
            final_vec.push(field_val);
        }

        date_map.insert(fields[0].to_string(), date_vec);
        master_map.insert(fields[0].to_string(), final_vec);
        acc_set.insert(fields[0].to_string());
    }

    for (_line_num, lines) in cashflow_reader.lines().enumerate().skip(1) {
        let fields = lines.split("~#~").collect::<Vec<&str>>();
        if fields.len() != 4 {
            continue;
        }
        let mut op_line = String::new();
        tot_cashflow_acc_encntrd += 1;
        let acc_num = fields[0].to_string();
        let int_amt = fields[1].to_string();
        let prin_amt = fields[2].to_string();
        let cf_date = fields[3].to_string();
        let cf_dt = NaiveDate::parse_from_str(&cf_date.as_str(), "%d/%m/%Y")
            .unwrap_or(config_param.as_on_date)
            .format("%d-%m-%Y");
        if master_map.contains_key(&acc_num) {
            acc_set.remove(&acc_num);
            let final_vec = master_map.get(&acc_num).expect("Cannot get Master File Data");
            let dt_vec = date_map.get(&acc_num).expect("Cannnot get dates from date vector");
            op_line =  get_op_line(final_vec,dt_vec,int_amt,prin_amt,cf_dt.to_string());
            let mut key = AccKey::new();
            key.acc_num = final_vec[0].to_string();
            key.cf_dt = NaiveDate::parse_from_str(&cf_date.as_str(), "%d/%m/%Y")
                .unwrap_or(config_param.as_on_date);
            cf_map
                .entry(key)
                .and_modify(|vec| vec.push(op_line.to_owned()))
                .or_insert_with(|| vec![op_line.to_owned()]);
            op_line.clear();
        }
    }
    for (_key, val) in cf_map.iter() {
        let vec = val.to_owned();
        for lines in vec.iter() {
            writeln!(writer, "{}", lines).expect("Unable to merged file.");
        }
    }
    for val in acc_set.iter() {
        let final_vec = master_map.get(val).unwrap();
        let dt_vec = date_map.get(val).unwrap();
        let op_line = get_op_line(final_vec,dt_vec,"0.0".to_string(),"0.0".to_string(),config_param.as_on_date().format("%d-%m-%Y").to_string());
        writeln!(writer, "{}", op_line).expect("Unable to merged file.");
    }
    println!(
        "Total Accounts present in Master Input File: {}\n\
        Total Accounts present in Master Input File without Cashflow: {}\n\
        Total Accounts present in Cashflow Input File: {}",
        tot_master_acc_encntrd,
        acc_set.len(),
        tot_cashflow_acc_encntrd
    );
    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);
}


pub fn get_op_line(final_vec: &Vec<String>, dt_vec: &Vec<String>, int_amt: String, prin_amt: String, cf_dt: String) -> String {
    format!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            final_vec[0],
            final_vec[1],
            final_vec[2],
            final_vec[3],
            final_vec[4],
            final_vec[5],
            dt_vec[0],
            final_vec[7],
            final_vec[8],
            final_vec[9],
            dt_vec[1],
            final_vec[11],
            final_vec[12],
            dt_vec[2],
            final_vec[14],
            final_vec[15],
            final_vec[16],
            final_vec[17],
            final_vec[18],
            final_vec[19],
            final_vec[20],
            final_vec[21],
            final_vec[22],
            final_vec[23],
            dt_vec[3],
            final_vec[25],
            final_vec[26],
            dt_vec[4],
            final_vec[28],
            final_vec[29],
            dt_vec[5],
            final_vec[31],
            final_vec[32],
            final_vec[33],
            final_vec[34],
            final_vec[35],
            final_vec[36],
            final_vec[37],
            final_vec[38],
            final_vec[39],
            final_vec[40],
            final_vec[41],
            final_vec[42],
            final_vec[43],
            final_vec[44],
            final_vec[45],
            final_vec[46],
            final_vec[47],
            int_amt,
            prin_amt,
            cf_dt
        )
}