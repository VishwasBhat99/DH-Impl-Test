use super::{OutputLines, TotalBalance};
use config_params::ConfigurationParameters;
use processing::io::*;
use processing::structs::InputAccount;
use slog::Logger;

pub fn get_tot_bal(
    config_params: &ConfigurationParameters,
    log: &Logger,
    diag_log: &Logger,
) -> TotalBalance {
    info!(
        diag_log,
        "Start of getting tot balance from file {}.",
        config_params.tot_bal_file()
    );
    let mut tot_bal: TotalBalance = TotalBalance::new();
    let mut input_reader = read_file(config_params.tot_bal_file());
    for (line_num, lines) in input_reader.deserialize().enumerate() {
        tot_bal = extract_tot_line(line_num, lines, config_params.tot_bal_file(), log);
        break;
    }
    info!(
        diag_log,
        "End of getting tot balance from file {}.",
        config_params.tot_bal_file()
    );

    tot_bal
}

pub fn read_file_and_create_op_data(
    config_params: &ConfigurationParameters,
    tot_bal: TotalBalance,
    ttl_accnts: &mut usize,
    ttl_suc_accnts: &mut usize,
    ttl_bal_ip: &mut f64,
    ttl_bal_op: &mut f64,
    log: &Logger,
    diag_log: &Logger,
) -> OutputLines {
    info!(
        diag_log,
        "Start of reading & creting output data of pre-processed borrowings file."
    );
    let mut input_reader = read_file(config_params.input_file());

    let mut op_line: OutputLines = OutputLines::new();
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

        if input_account.borr_bal_lcy != 0.0
            && ((input_account.borr_bal_lcy / tot_bal.tot_liab_amt) * 100.0)
                > config_params.sig_perc()
        {
            *ttl_bal_ip += input_account.borr_bal_lcy;
            *ttl_bal_op += input_account.borr_bal_lcy;

            if rec_count == config_params.top_n_sig_count() {
                break;
            } else {
                op_line.liab_op_line.push_str(&input_account.print());
                rec_count += 1;
            }
        }
    }

    op_line
}

pub fn write_output(
    config_params: &ConfigurationParameters,
    op_line: OutputLines,
    diag_log: &Logger,
) {
    info!(
        diag_log,
        "Start of creating & writing significant counterparty borrowings output file."
    );
    let mut liab_file_path = String::from(config_params.output_file());
    liab_file_path = liab_file_path.replace(".txt", "").replace(".csv", "") + "-liab.txt";
    let mut op_writer = get_writer(&liab_file_path);
    output_writer(&mut op_writer, op_line.liab_op_line, &liab_file_path);
}
