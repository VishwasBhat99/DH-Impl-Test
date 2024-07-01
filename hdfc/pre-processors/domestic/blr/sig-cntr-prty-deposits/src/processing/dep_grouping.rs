use config_params::ConfigurationParameters;
use processing::io::{extract_liab_line, extract_lines, get_writer, output_writer, read_file};
use processing::structs::InputAccount;
use slog::Logger;

pub fn get_liability_bal(
    config_params: &ConfigurationParameters,
    log: &Logger,
    diag_log: &Logger,
) -> f64 {
    info!(
        diag_log,
        "Start of getting liability balance from file {}.",
        config_params.liability_bal_file()
    );
    let mut liability_bal: f64 = 0.0;
    let mut input_reader = read_file(config_params.liability_bal_file());
    for (line_num, lines) in input_reader.deserialize().enumerate() {
        liability_bal = extract_liab_line(line_num, lines, config_params.input_file(), log);
        break;
    }
    info!(
        diag_log,
        "End of getting liability balance from file {}.",
        config_params.liability_bal_file()
    );

    liability_bal
}

pub fn read_file_and_create_op_data(
    config_params: &ConfigurationParameters,
    liability_bal: f64,
    ttl_accnts: &mut usize,
    ttl_suc_accnts: &mut usize,
    ttl_bal_ip: &mut f64,
    ttl_bal_op: &mut f64,
    log: &Logger,
    diag_log: &Logger,
) -> String {
    info!(
        diag_log,
        "Start of reading & creting output data of pre-processed deposits file."
    );
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

        if input_account.tot_bal_lcy != 0.0
            && ((input_account.tot_bal_hcy / liability_bal) * 100.0) > config_params.sig_perc()
        {
            *ttl_bal_ip += input_account.tot_bal_lcy;
            *ttl_bal_op += input_account.tot_bal_lcy;

            if rec_count == config_params.top_n_sig_count() {
                break;
            } else {
                op_line.push_str(&input_account.print());
                rec_count += 1;
            }
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
        "Start of creating & writing significant counterparty deposits output file."
    );
    let mut op_writer = get_writer(config_params.output_file());
    output_writer(&mut op_writer, op_line, config_params.output_file());
    info!(
        diag_log,
        "End of creating & writing significant counterparty deposits output file."
    );
}
