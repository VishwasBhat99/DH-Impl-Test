use self::derive_fields::*;
use self::io::*;
use self::structs::{biu_fields::*, casatd_fields::*};
use configuration_parameters::ConfigurationParameters;
use hashbrown::HashMap;
use health_report::HealthReport;
use macros;
use slog::Logger;
use statics::*;
use std::collections::HashSet;
use std::default::Default;
use std::io::BufRead;
use std::time::SystemTime;

mod derive_fields;
mod io;
mod structs;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let st_tm_read = SystemTime::now();
    let mut op_line: String = String::new();
    let mut op_line_hash = String::new();
    let mut tot_rec = DEFAULT_INT;
    let mut skp_rec = DEFAULT_INT;
    let const_desc_file_reader = read_file(config_param.const_desc_file_path());
    let mut const_desc_values: HashMap<String, bool> = HashMap::new();
    for (line_num, lines) in const_desc_file_reader.lines().enumerate() {
        let line = extract_lines(line_num, lines, config_param.const_desc_file_path());
        const_desc_values.insert(line.to_uppercase().trim().to_string(), true);
    }
    let prod_code_file_reader = read_file(config_param.prod_code_file_path());
    let mut prod_code_values: HashMap<String, bool> = HashMap::new();
    for (line_num, lines) in prod_code_file_reader.lines().enumerate() {
        let line = extract_lines(line_num, lines, config_param.prod_code_file_path());
        prod_code_values.insert(line.to_uppercase().trim().to_string(), true);
    }
    let adv_file_reader = read_file(config_param.adv_master_file_path());
    let mut adv_client_ids: HashMap<String, bool> = HashMap::new();
    for (line_num, lines) in adv_file_reader.lines().enumerate() {
        let line = extract_lines(line_num, lines, config_param.adv_master_file_path());
        adv_client_ids.insert(line, true);
    }

    let ops_acc_file_reader = read_file(config_param.ops_acc_data_file_path());
    let mut ops_client_ids: HashMap<String, bool> = HashMap::new();
    for (line_num, lines) in ops_acc_file_reader.lines().enumerate() {
        let line = extract_lines(line_num, lines, config_param.ops_acc_data_file_path());
        ops_client_ids.insert(line, true);
    }
    let mult_depo_file_reader = read_file(config_param.mult_depo_file_path());
    let mut mult_depo_cust_ids: HashMap<String, bool> = HashMap::new();
    for (line_num, lines) in mult_depo_file_reader.lines().enumerate() {
        let line = extract_lines(line_num, lines, config_param.mult_depo_file_path());
        mult_depo_cust_ids.insert(line, true);
    }
    let tot_amt = DEFAULT_FLOAT;
    let mut casatd_reader = read_file_struct(config_param.casatd_master_file_path());

    let mut mult_custid: HashMap<String, String> = HashMap::new();
    let mut written_custid: HashSet<String> = HashSet::new();
    for (line_num, lines) in casatd_reader.deserialize().enumerate() {
        tot_rec += 1;
        let mut flag = 0;
        let casatd_fields: CASATDFields =
            extract_struct_lines(line_num, lines, config_param.casatd_master_file_path(), log);
        if casatd_fields.prd_cd == *"Y".to_string() {
            if written_custid.contains(&casatd_fields.cust_id) {
                flag = 1;
                skp_rec += 1;
            }
            written_custid.insert(casatd_fields.cust_id.to_string());
        } else if casatd_fields.prd_cd == *"N".to_string() {
            if mult_custid.contains_key(&casatd_fields.cust_id) {
                flag = 1;
            } else {
                mult_custid.insert(
                    casatd_fields.cust_id.to_string(),
                    casatd_fields.prd_cd.to_string(),
                );
            }
            skp_rec += 1;
            flag = 1;
        }

        if flag == 0 {
            op_line.push_str(&get_op_line(
                casatd_fields,
                &adv_client_ids,
                &ops_client_ids,
                &mult_depo_cust_ids,
                &const_desc_values,
                &prod_code_values,
                &mult_custid,
                &written_custid,
            ));
        }
    }

    let ed_tm_read = SystemTime::now();
    let duration = ed_tm_read
        .duration_since(st_tm_read)
        .expect("Could not calculate total read process duration.");
    debug!(diag_log, "Read Process Total Duration: {:?}.", duration);

    let st_tm_writer = SystemTime::now();
    let mut op_writer = get_writer(config_param.output_file_path());
    output_writer(&mut op_writer, op_line, config_param.output_file_path());

    for (cust_id, prod) in &mult_custid {
        if written_custid.contains(cust_id) {
            continue;
        } else {
            op_line_hash.push_str(&get_op_line_hash_map_output(
                cust_id,
                prod,
                &adv_client_ids,
                &ops_client_ids,
                &mult_depo_cust_ids,
            ));
            skp_rec -= 1;
        }
    }
    output_writer(
        &mut op_writer,
        op_line_hash,
        config_param.output_file_path(),
    );

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_param.output_file_path());

    let ed_tm_writer = SystemTime::now();
    let duration = ed_tm_writer
        .duration_since(st_tm_writer)
        .expect("Could not calculate total duration for write process.");
    debug!(diag_log, "Writing BIU, Total Duration: {:?}.", duration);
}
