use self::derive_fields::*;
use self::io::*;
use self::structs::{amb_account::*, calc_per_val, core_master::*, input_account::*};
use calamine::DataType;
use calamine::{open_workbook_auto, Reader};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;
use statics::*;
use std::collections::{HashMap, HashSet};
use std::default::Default;
use std::io::prelude::*;
use std::time::SystemTime;

mod derive_fields;
mod io;
mod structs;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let st_tm_read = SystemTime::now();
    let mut op_line_core: String = String::new();
    let mut op_line_non_core: String = String::new();
    let mut tot_rec = DEFAULT_INT;
    let skp_rec = DEFAULT_INT;

    let mut core_master: HashMap<String, CoreValues> = HashMap::new();
    let mut core_master_excel =
        open_workbook_auto(config_param.core_master()).expect("Unable to open Core Master File.");
    if let Some(Ok(reader)) =
        core_master_excel.worksheet_range(config_param.core_master_sheet_name())
    {
        for row in reader.rows() {
            core_master.insert(
                row[0].to_string(),
                CoreValues {
                    core: row[1].to_string().parse().unwrap_or(DEFAULT_FLOAT),
                    non_core: row[2].to_string().parse().unwrap_or(DEFAULT_FLOAT),
                },
            );
        }
    }

    let mut amb_data: HashMap<String, AMBData> = HashMap::new();
    let amb_file_delimeter = config_param.amb_file_delimeter();
    let amb_file_reader = read_file_del(config_param.amb_file_path());
    for (line_num, lines) in amb_file_reader.lines().enumerate().skip(1) {
        let line = extract_lines_del(line_num, lines, config_param.amb_file_path());
        let fields: Vec<&str> = line.split(amb_file_delimeter).collect();
        if fields.len() >= 8 {
            amb_data.insert(
                fields[0].to_string(),
                AMBData {
                    dr_avg_bal: fields[2].parse().unwrap_or(DEFAULT_FLOAT),
                    mis2: fields[7].to_string(),
                },
            );
        }
    }
    //Read mis1_desc excel file
    let mut mis1_desc_excel = open_workbook_auto(config_param.mis1_desc_file_path())
        .expect("Unable to open mis1 desc File.");
    let mut mis1_desc_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = mis1_desc_excel.worksheet_range(config_param.mis1_sheet_name()) {
        for row in reader.rows() {
            let sys_mis1 = get_str_from_xlsx(row, 0);
            let bdp_division = get_str_from_xlsx(row, 3);
            mis1_desc_map.insert(sys_mis1, bdp_division);
        }
    }
    //Reading BDP COA excel file
    let mut bdp_coa_excel =
        open_workbook_auto(config_param.bdp_coa_file_path()).expect("Unable to open bdp coa File.");
    let mut bdp_coa_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = bdp_coa_excel.worksheet_range(config_param.bdp_coa_sheet_name()) {
        for row in reader.rows() {
            let concat = get_str_from_xlsx(row, 0);
            let bdp_coa = get_str_from_xlsx(row, 1);
            bdp_coa_map.insert(concat, bdp_coa);
        }
    }

    let mut weaker_sec_master_file = open_workbook_auto(config_param.weaker_sec_master_path())
        .expect("Unable to open `Weaker_section_master.xlsx`.");
    let mut weaker_master: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) =
        weaker_sec_master_file.worksheet_range(&config_param.weaker_sec_sheet_name())
    {
        for row in reader.rows() {
            let weaker_sction = row[6].to_string();
            weaker_master.insert(row[0].to_string(), weaker_sction.to_owned());
        }
    }

    let mut ews_weaker_master_file = open_workbook_auto(config_param.ews_weaker_master_path())
        .expect("Unable to open `EWS_Weaker_master.xlsx`.");
    let mut ews_weaker_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) =
        ews_weaker_master_file.worksheet_range(&config_param.ews_master_sheet_name())
    {
        for row in reader.rows() {
            ews_weaker_map.insert(row[0].to_string(), row[7].to_string());
        }
    }

    let mut ora_gl_master_file = open_workbook_auto(config_param.ora_gl_master())
        .expect("Unable to open `OraGL Master.xlsx`.");
    let mut ora_gl_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ora_gl_master_file.worksheet_range(&config_param.ora_gl_sheet_name())
    {
        for row in reader.rows().skip(1) {
            let mut two_point_concat = String::new();
            two_point_concat.push_str(&row[4].to_string());
            two_point_concat.push('_');
            two_point_concat.push_str(&row[1].to_string());
            ora_gl_map.insert(row[0].to_string(), two_point_concat);
        }
    }

    let mut input_reader = read_file(config_param.input_file_path());
    let mut tot_amt = DEFAULT_FLOAT;
    let mut avg_bal: f64;
    for (line_num, lines) in input_reader.deserialize().enumerate() {
        let mut input_account: InputAccount =
            extract_lines(line_num, lines, config_param.input_file_path(), log);
        tot_rec += 1;
        let is_acc_weaker = if weaker_master.contains_key(&input_account.cod_acc_no.to_string()) {
            "Y"
        } else {
            "N"
        };
        let ews_weaker_value = if (match ews_weaker_map.get(&input_account.cod_acc_no.to_string()) {
            Some(val) => val,
            None => "Others",
        }) == "SEK_WK"
        {
            "SEK-WEAKER"
        } else {
            "Others"
        };
        let src_gl_code = if input_account.gl_acc_no.len() <= 4 {
            ""
        } else {
            &input_account.gl_acc_no[2..input_account.gl_acc_no.len() - 2]
        };
        let two_point_concat = match ora_gl_map.get(&src_gl_code.to_string()) {
            Some(val) => val,
            None => "",
        };
        let acct_id = input_account.cod_acc_no.clone();
        let empty_string = "".to_string();
        let weaker_desc = weaker_master.get(&acct_id).unwrap_or(&empty_string);
        let bdp_divison = mis1_desc_map
            .get(&input_account.mis1)
            .unwrap_or(&empty_string);
        let bdp_coa = bdp_coa_map
            .get(&two_point_concat.to_string())
            .unwrap_or(&empty_string);
        if let Some(amb_acc) = amb_data.get(&input_account.cod_acc_no) {
            if let Some(core_acc) = core_master.get(&input_account.mis1) {
                avg_bal = calc_per_val(amb_acc.dr_avg_bal, core_acc.core);
                op_line_core.push_str(&get_op_line(
                    &mut input_account,
                    &amb_acc.mis2,
                    avg_bal,
                    is_acc_weaker,
                    ews_weaker_value,
                    two_point_concat,
                    weaker_desc,
                    bdp_divison,
                    bdp_coa,
                ));
                tot_amt += avg_bal;

                avg_bal = calc_per_val(amb_acc.dr_avg_bal, 100.0 - core_acc.core);
                op_line_non_core.push_str(&get_op_line(
                    &mut input_account,
                    &amb_acc.mis2,
                    avg_bal,
                    is_acc_weaker,
                    ews_weaker_value,
                    two_point_concat,
                    weaker_desc,
                    bdp_divison,
                    bdp_coa,
                ));
                tot_amt += avg_bal;
            } else {
                avg_bal = amb_acc.dr_avg_bal;
                op_line_core.push_str(&get_op_line(
                    &mut input_account,
                    &amb_acc.mis2,
                    avg_bal,
                    is_acc_weaker,
                    ews_weaker_value,
                    two_point_concat,
                    weaker_desc,
                    bdp_divison,
                    bdp_coa,
                ));
                tot_amt += avg_bal;
            }
        } else {
            avg_bal = 0.0;
            op_line_core.push_str(&get_op_line(
                &mut input_account,
                "",
                avg_bal,
                is_acc_weaker,
                ews_weaker_value,
                two_point_concat,
                weaker_desc,
                bdp_divison,
                bdp_coa,
            ));
            tot_amt += avg_bal;
        }
    }

    let ed_tm_read = SystemTime::now();
    let duration = ed_tm_read
        .duration_since(st_tm_read)
        .expect("Could not calculate total read process duration.");
    debug!(diag_log, "Read Process Total Duration: {:?}.", duration);

    let st_tm_writer = SystemTime::now();
    let mut op_core_file_path = String::from(config_param.output_file_path());
    op_core_file_path.push_str("-core.txt");
    let mut op_writer_core = get_writer(&op_core_file_path);
    output_writer(&mut op_writer_core, op_line_core, &op_core_file_path);

    let mut op_non_core_file_path = String::from(config_param.output_file_path());
    op_non_core_file_path.push_str("-non-core.txt");
    let mut op_writer_non_core = get_writer(&op_non_core_file_path);
    output_writer(
        &mut op_writer_non_core,
        op_line_non_core,
        &op_non_core_file_path,
    );

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_param.output_file_path());

    let ed_tm_writer = SystemTime::now();
    let duration = ed_tm_writer
        .duration_since(st_tm_writer)
        .expect("Could not calculate total duration for write process.");
    debug!(diag_log, "Writing OD, Total Duration: {:?}.", duration);
}
pub fn get_str_from_xlsx(data: &[DataType], index: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` for row: `{:?}`",
                index + 1,
                data
            )
        })
        .to_string()
        .replace("\n", " ")
        .trim()
        .to_string()
}
