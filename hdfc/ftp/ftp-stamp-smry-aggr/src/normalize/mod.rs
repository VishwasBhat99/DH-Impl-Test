use self::derive_acc_fields::*;
use self::io::*;
use self::structs::*;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use normalize::account_field_names::AccFieldNames;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use statics::DEFAULT_FLOAT;
use std::collections::HashMap;
use std::io::Write;
use std::time::SystemTime;

mod account_field_names;
mod derive_acc_fields;
mod io;
mod structs;

pub fn normalizing(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let start_time = SystemTime::now();
    let mut tot_acc_encntrd = 0;
    let skp_acc = 0;
    let mut ttl_amt: f64 = 0.0;
    let keys = AccFieldNames::new_from_path(config_params.known_fields_file_path());
    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );

    let mut smry_aggr_data: HashMap<String, AggregatedValue> = HashMap::new();
    let mut smry_aggr_data_report2: HashMap<String, AggregatedValue_Report2> = HashMap::new();
    let mut header = String::new();
    let mut header2 = String::new();
    header.push_str("Org_month|");
    for field in keys.aggr_keys.iter() {
        header.push_str(&field);
        header.push_str("|");
    }
    for field in keys.aggr_keys_report2.iter() {
        header2.push_str(&field);
        header2.push_str("|");
    }
    header.push_str("gr_ofs_gl_amt|ui_ofs_gl_amt|re_ofs_gl_amt|is_ofs_gl_amt|int_income_gl_amt|Overdue_int_gl_amt|Int_on_Cancellation_gl_amt|W/off_gl_amt|Total_AMB|Total Accr_int|Total PSL_Amt|Total_final_FTP_Amt|Total_FTP_without_PSL|Total_additional_SMF|Total_crnt_ostd_td|Total_base_tpr|Total_adjs|Total_Margin\n");
    header2.push_str("Weighted_Yield|Weighted_Base_FTP_Rate|Weighted_PSL_Rate|Weighted_Final_FTP_Rate|Weighted_Total_Spread\n");

    let mut input_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    
    for account in account_reader.iter() {
        tot_acc_encntrd += 1;
        get_acc_fields(
            &mut input_reader,
            &mut smry_aggr_data,
            &mut smry_aggr_data_report2,
            &account,
            &keys,
            *config_params.as_on_date(),
        );
        ttl_amt += 0.0;
    }

    let op_writer_path = format!("{}.txt",config_params.output_file_path());
    let mut op_writer = get_writer(&op_writer_path);
    let op_writer2_path = format!("{}_report2.txt",config_params.output_file_path());
    let mut op_writer2 = get_writer(&op_writer2_path);

    op_writer
        .write(&header.as_bytes())
        .expect("unable to write the header to summary file");
    for (key, value) in smry_aggr_data.drain() {
        write!(op_writer, "{}|{}\n", key, value).expect("Unable to generate summary file.");
    }

    op_writer2
        .write(&header2.as_bytes())
        .expect("Unable to write the header to summary file 2nd report.");
    for (key, value) in smry_aggr_data_report2.drain() {
        write!(op_writer2, "{}|{}\n", key, value).expect("Unable to generate summary file report 2.");
    }

    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skp_acc,
        skp_acc,
        ttl_amt,
        ttl_amt,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}
