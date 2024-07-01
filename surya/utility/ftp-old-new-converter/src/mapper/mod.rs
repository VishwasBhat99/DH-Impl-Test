use self::io::*;
use self::structs::*;
use crate::configuration_parameters::ConfigurationParameters;
use crate::statics::*;
use health_report::HealthReport;
use std::default::Default;

mod io;
mod structs;

pub fn process(config_param: ConfigurationParameters) {
    let mut op_line: String = String::new();
    let mut tot_rec = DEFAULT_INT;
    let tot_amt = DEFAULT_FLOAT;
    let skp_rec = DEFAULT_INT;

    let mut input_reader = read_file(config_param.input_file_path());
    for (line_num, lines) in input_reader.deserialize().enumerate() {
        let old_acc_smry: OldAccountSummary =
            extract_lines(line_num, lines, config_param.input_file_path());
        tot_rec += 1;
        let mut new_acc_smry: NewAccountSummary = NewAccountSummary::new();
        new_acc_smry.mapped(&config_param, old_acc_smry);
        op_line.push_str(&new_acc_smry.print());
    }

    let mut op_writer = get_writer(config_param.output_file_path());
    output_writer(&mut op_writer, op_line, config_param.output_file_path());

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    health_report.gen_health_rpt(&config_param.output_file_path());
}
