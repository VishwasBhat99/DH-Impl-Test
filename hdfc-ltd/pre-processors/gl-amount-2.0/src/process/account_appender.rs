use crate::macros;
use crate::process::cf_writer::account::*;
use crate::slog::Logger;

pub fn create_cf_acc(output_fields: Vec<&str>, log: &Logger) -> Account {
    let mut out_acc = Account::new();

    if output_fields.len() != 14 {
        log_error!(log, "Incorrect output for gl `{}`", output_fields[0]);
    }
    out_acc.gl_code = output_fields[0].to_string();
    out_acc.branch_code = output_fields[1].to_string();
    out_acc.dr_bal = output_fields[2].to_string().parse::<f64>().unwrap_or(0.0);
    out_acc.cr_bal = output_fields[3].to_string().parse::<f64>().unwrap_or(0.0);
    out_acc.net_bal = output_fields[4].to_string().parse::<f64>().unwrap_or(0.0);
    out_acc.cf_type = output_fields[5].to_string();
    out_acc.curr = output_fields[6].to_string();
    out_acc.is_gl = output_fields[7].to_string();
    out_acc.alm_line = output_fields[8].to_string();
    out_acc.code_desc = output_fields[9].to_string();
    out_acc.group_2 = output_fields[10].to_string();
    out_acc.group_3 = output_fields[11].to_string();
    out_acc.line = output_fields[12].to_string();
    out_acc.prefix = output_fields[13].to_string();

    out_acc
}
