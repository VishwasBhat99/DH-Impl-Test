use self::derive_fields::*;
use self::io::*;
use self::structs::input_fields::*;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::io::BufRead;
use std::time::SystemTime;

mod derive_fields;
mod io;
mod structs;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let st_tm_read = SystemTime::now();
    let mut op_line: String = String::new();
    let mut tot_rec = DEFAULT_INT;
    let mut skp_rec = DEFAULT_INT;

    let mut col_mkt_rate_map: HashMap<String, f64> = HashMap::new();
    let col_mkt_rate_rdr = match sdb_io::new_buf_rdr(config_param.col_mkt_rate_file()) {
        Ok(r) => r,
        Err(e) => panic!(format!(
            "Cannot read file at path: '{}', Error: '{}'",
            config_param.col_mkt_rate_file, e
        )),
    };
    for (line_num, lines) in col_mkt_rate_rdr.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read rules file at line number: `{}` : {}",
                line_num + 1,
                error
            ),
        };
        let line_info: Vec<&str> = line.split('|').collect();
        let col_type_cd = line_info[0].to_string();
        let mkt_rate: f64 = line_info[1].parse().unwrap_or(0.0);
        col_mkt_rate_map.insert(col_type_cd, mkt_rate);
    }

    let mut tot_amt = DEFAULT_FLOAT;
    let elg_col_type_cds = config_param.elg_col_type_cd();
    let mut input_reader = read_file(config_param.input_file_path());
    for (line_num, lines) in input_reader.deserialize().enumerate().skip(1) {
        tot_rec += 1;
        let mut input_fields: InputFields =
            extract_lines(line_num, lines, config_param.input_file_path(), log);
        if let Some(mkt_rate) = col_mkt_rate_map.get(&input_fields.col_type_cd) {
            let net_weight: f64 = input_fields.net_weight.parse().unwrap_or(0.0);
            input_fields.tot_mk_val_of_col = net_weight * mkt_rate;
        }
        if !elg_col_type_cds.contains(&input_fields.col_type_cd.as_str())
            || input_fields.tot_mk_val_of_col == 0.0
        {
            skp_rec += 1;
            log_debug!(
                log,
                    "Account Id: `{}` skipped either because of `collateral_type_code`: `{}` or `total_market_value_of_collateral`: `{}`.",
                input_fields.acc_id,
                input_fields.col_type_cd,
                input_fields.tot_mk_val_of_col,
        );
            continue;
        }
        op_line.push_str(&get_op_line(input_fields, &mut tot_amt));
    }

    let ed_tm_read = SystemTime::now();
    let duration = ed_tm_read
        .duration_since(st_tm_read)
        .expect("Could not calculate total read process duration.");
    debug!(diag_log, "Read Process Total Duration: {:?}.", duration);

    let st_tm_writer = SystemTime::now();
    let mut op_writer = get_writer(config_param.output_file_path());
    output_writer(&mut op_writer, op_line, config_param.output_file_path());

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_param.output_file_path());

    let ed_tm_writer = SystemTime::now();
    let duration = ed_tm_writer
        .duration_since(st_tm_writer)
        .expect("Could not calculate total duration for write process.");
    debug!(
        diag_log,
        "Writing Collateral Non-Security Data, Total Duration: {:?}.", duration
    );
}
