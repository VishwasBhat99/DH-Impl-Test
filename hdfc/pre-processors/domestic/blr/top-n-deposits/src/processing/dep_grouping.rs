use config_params::ConfigurationParameters;
use processing::io::{extract_lines, get_writer, output_writer, read_file};
use processing::structs::InputAccount;
use slog::Logger;

pub fn read_file_and_create_op_data(
    config_params: &ConfigurationParameters,
    ttl_accnts: &mut usize,
    ttl_suc_accnts: &mut usize,
    ttl_bal_ip: &mut f64,
    ttl_bal_op: &mut f64,
    log: &Logger,
    diag_log: &Logger,
) -> String {
    let mut input_reader = read_file(config_params.input_file());

    let mut op_line: String = String::new();
    let mut rec_count: usize = 0;
    for (line_num, lines) in input_reader.deserialize().enumerate() {
        let input_account: InputAccount = extract_lines(
            line_num,
            lines,
            ttl_suc_accnts,
            config_params.input_file(),
            log,
        );
        *ttl_accnts += 1;

        *ttl_bal_ip +=
            input_account.sa_bal_lcy + input_account.ca_bal_lcy + input_account.td_bal_lcy;
        *ttl_bal_op +=
            input_account.sa_bal_lcy + input_account.ca_bal_lcy + input_account.td_bal_lcy;

        if rec_count == config_params.top_n_cust_count() {
            break;
        } else if !(input_account.ason_date.is_empty()
            || input_account.country_code.is_empty()
            || input_account.currency_id.is_empty())
        {
            op_line.push_str(&input_account.print());
            rec_count += 1;
        }
    }
    op_line.pop();
    info!(
        diag_log,
        "End of reading & creting output data of pre-processed deposits file."
    );

    op_line
}

pub fn write_output(config_params: &ConfigurationParameters, op_line: String, diag_log: &Logger) {
    info!(
        diag_log,
        "Start of creating & writing top n deposits output file."
    );
    let mut op_writer = get_writer(config_params.output_file());
    output_writer(&mut op_writer, op_line, config_params.output_file());
    info!(
        diag_log,
        "End of creating & writing top n deposits output file."
    );
}
