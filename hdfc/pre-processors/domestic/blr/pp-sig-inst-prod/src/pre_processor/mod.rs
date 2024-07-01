use self::derive_fields::*;
use self::io::*;
use self::structs::{aggregation::*, input_record::*, product_description::*};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::default::Default;
use std::io::Write;
use std::time::SystemTime;

mod derive_fields;
mod io;
mod structs;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let st_tm_read = SystemTime::now();
    let mut tot_rec = DEFAULT_INT;
    let mut skp_rec = DEFAULT_INT;

    let liability_bal = get_liability_bal(&config_param, log, diag_log);

    let mut input_reader = read_file(config_param.prod_map_master());
    let mut prod_desc_map: HashMap<i64, ProdDescOutput> = HashMap::new();
    for (line_num, lines) in input_reader.deserialize().enumerate().skip(1) {
        let prod_desc: ProdDescInput =
            extract_lines(line_num, lines, config_param.prod_map_master(), log);

        prod_desc_map.insert(
            prod_desc.llg_id,
            ProdDescOutput {
                prod_cd: prod_desc.prod_cd,
                prod_name: prod_desc.prod_name,
            },
        );
    }

    let mut input_reader = read_file(config_param.input_file_path());
    let mut tot_amt = 0.0;
    let mut smry_aggr_data: HashMap<AggregatedKey, AggregatedValue> = HashMap::new();
    for (line_num, lines) in input_reader.deserialize().enumerate() {
        let mut input_record: InputRecord =
            extract_lines(line_num, lines, config_param.input_file_path(), log);
        tot_rec += 1;

        if input_record.lcy_amt != 0.0
            && ((input_record.lcy_amt / liability_bal) * 100.0) < config_param.sig_perc()
        {
            skp_rec += 1;
        }
        tot_amt += input_record.lcy_amt;
        get_op_line(&mut input_record, &mut prod_desc_map, &mut smry_aggr_data);
    }

    let ed_tm_read = SystemTime::now();
    let duration = ed_tm_read
        .duration_since(st_tm_read)
        .expect("Could not calculate total read process duration.");
    debug!(diag_log, "Read Process Total Duration: {:?}.", duration);

    let st_tm_writer = SystemTime::now();
    let mut op_writer = get_writer(config_param.output_file_path());
    for (key, value) in smry_aggr_data.drain() {
        write!(op_writer, "{}|{}", key, value).expect("Unable to generate summary file.");
    }

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
        "Writing Significant Instruements and Products, Total Duration: {:?}.", duration
    );
}
