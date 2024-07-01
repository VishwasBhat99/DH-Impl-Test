use super::output_lines::OutputLines;
use macros;
use rbdate::NaiveDate;
use slog::Logger;
use std::collections::HashMap;

pub fn get_op_line(
    rec: &mut [&str],
    alm_line: &mut HashMap<String, String>,
    ia_line: &mut HashMap<String, String>,
    ora_gl: &mut HashMap<String, String>,
    ora_cat: &mut HashMap<String, String>,
    prod_code: &mut HashMap<String, String>,
    as_on_dt: NaiveDate,
    log: &Logger,
    sma_data: &HashMap<String, String>,
) -> OutputLines {
    let mut output_lines = OutputLines::new();
    let dt = as_on_dt.format("%d-%m-%Y");

    // TODO: Missing compmis1, prod, glsl
    let alm_concat = get_alm_concat(
        rec[27],
        prod_code
            .entry(rec[9].to_string())
            .or_insert_with(|| "".to_string()),
        ora_gl
            .entry(rec[9].to_string())
            .or_insert_with(|| "".to_string()),
        ora_cat
            .entry(rec[9].to_string())
            .or_insert_with(|| "".to_string()),
    );

    let alm_line = alm_line
        .entry(alm_concat.to_string())
        .or_insert("NONE".to_string());
    let ia_line = ia_line
        .entry(alm_concat.to_string())
        .or_insert("NONE".to_string());

    log_debug!(
        log,
        "account: `{}`, alm_concat: `{}`, alm_line: `{}`.",
        rec[4],
        alm_concat,
        alm_line,
    );
    output_lines.processed_lines = get_line(rec, alm_line, ia_line, &dt.to_string(), sma_data);

    if alm_line == "NONE" {
        output_lines
            .concat_lines
            .push(get_concat_line(rec[0], rec[9], &alm_concat));
    }

    output_lines
}

fn get_concat_line(deal_no: &str, gl: &str, alm_concat: &str) -> String {
    let mut op_line = String::new();
    op_line.push_str("SecLoans|");
    op_line.push_str(deal_no);
    op_line.push('|');
    op_line.push_str(gl);
    op_line.push('|');
    op_line.push_str(alm_concat);
    op_line
}

fn get_line(
    rec: &mut [&str],
    alm_line: &mut str,
    ia_line: &mut str,
    dt: &str,
    sma_data: &HashMap<String, String>,
) -> String {
    let mut output_line = String::new();
    output_line.push_str(rec[0]);
    output_line.push('|');
    output_line.push_str(rec[1]);
    output_line.push('|');
    output_line.push_str(rec[3]);
    output_line.push('|');
    output_line.push_str(rec[6]);
    output_line.push('|');
    output_line.push_str(rec[4]);
    output_line.push('|');
    output_line.push_str(rec[5]);
    output_line.push('|');
    output_line.push_str(rec[9]);
    output_line.push('|');
    output_line.push_str(rec[11]);
    output_line.push('|');
    output_line.push_str(rec[21]);
    output_line.push_str("||");

    output_line.push_str("||");
    // TODO: Missing spread, rate flag

    output_line.push_str("018||");
    output_line.push_str(rec[23]);
    output_line.push('|');
    output_line.push('0');
    output_line.push('|');
    output_line.push('0');
    output_line.push('|');

    // TODO: Misssing mis3
    output_line.push_str(rec[27]);
    output_line.push('|');
    output_line.push_str(rec[28]);
    output_line.push('|');

    output_line.push_str("|INR|");
    output_line.push_str(rec[23]);
    output_line.push('|');
    output_line.push_str(rec[24]);
    output_line.push('|');
    output_line.push_str(rec[25]);
    output_line.push('|');
    output_line.push_str(rec[12]);
    output_line.push('|');
    output_line.push_str(rec[13]);
    output_line.push('|');
    output_line.push_str(rec[14]);
    output_line.push('|');
    output_line.push_str(rec[15]);
    output_line.push('|');
    output_line.push_str(rec[16]);
    output_line.push('|');
    output_line.push_str(rec[17]);
    output_line.push('|');
    output_line.push_str(dt);
    output_line.push('|');
    output_line.push_str(rec[20]);
    output_line.push('|');
    output_line.push_str(rec[22]);
    output_line.push('|');
    output_line.push_str(alm_line);
    output_line.push('|');
    output_line.push_str(rec[26]);
    output_line.push('|');
    output_line.push_str(rec[2]);
    output_line.push('|');
    output_line.push_str(ia_line);
    output_line.push('|');
    output_line.push_str(
        &sma_data
            .get(&rec[0].to_string())
            .unwrap_or(&"P".to_string())
            .to_string(),
    );
    output_line.push('\n');
    output_line
}

fn get_alm_concat(ora_mis1: &str, prod_code: &str, ora_gl: &str, ora_cat: &str) -> String {
    let mut alm_concat: String = String::new();
    alm_concat.push('1');
    alm_concat.push_str(ora_mis1);
    alm_concat.push('_');
    alm_concat.push_str(prod_code);
    alm_concat.push('_');
    alm_concat.push_str(ora_gl);
    alm_concat.push('_');
    alm_concat.push_str(ora_cat);
    alm_concat
}
