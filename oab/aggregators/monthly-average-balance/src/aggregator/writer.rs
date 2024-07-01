use super::{AggregateData, HashMap};
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn get_op_line(
    no_of_days: f64,
    mut ttl_amt: &mut f64,
    writer: &mut BufWriter<File>,
    bal_org: &mut HashMap<String, AggregateData>,
    recon_writer: &mut BufWriter<File>,
    is_avg_bal_absolute: bool,
    source: String,
) {
    let mut op_line = String::new();
    let mut recon_op_line = String::new();
    recon_op_line
        .push_str("AccID|Avg Balance|Int rate|Source Avg Bal Picked|System Generated bal|Flag|Source|AcrIntAmtCCY|AcrIntAmtHCY\n");
    append_to_string(
        &mut op_line,
        no_of_days,
        &mut ttl_amt,
        &mut recon_op_line,
        bal_org,
        is_avg_bal_absolute,
        source,
    );
    write!(writer, "{}", op_line).expect("Error while writing to the output file.");
    write!(recon_writer, "{}", recon_op_line).expect("Error while writing to the recon file.");
}

fn append_to_string(
    op_line: &mut String,
    no_of_days: f64,
    ttl_amt: &mut f64,
    recon_op_line: &mut String,
    bal_org: &mut HashMap<String, AggregateData>,
    is_avg_bal_absolute: bool,
    source: String,
) {
    for (acc_no, data) in bal_org.iter_mut() {
        op_line.push_str(&acc_no);
        op_line.push_str("|");
        let final_avg_bal: f64;
        let int_rate: f64;
        if data.calc_flag == "From Source".to_string() {
            final_avg_bal = (data.input_avg_bal * data.days) / no_of_days;
            int_rate = data.int_rate;
        } else {
            final_avg_bal = data.calculated_avg_bal / no_of_days;
            int_rate = data.int_rate / no_of_days;
        };

        *ttl_amt += final_avg_bal;
        recon_op_line.push_str(&acc_no);
        recon_op_line.push_str("|");
        if is_avg_bal_absolute {
            op_line.push_str(&final_avg_bal.abs().to_string());
            recon_op_line.push_str(&final_avg_bal.abs().to_string());
        } else {
            op_line.push_str(&final_avg_bal.to_string());
            recon_op_line.push_str(&final_avg_bal.to_string());
        }
        op_line.push_str("|");
        op_line.push_str(&int_rate.to_string());
        op_line.push_str("\n");
        recon_op_line.push_str("|");
        recon_op_line.push_str(&(data.int_rate / no_of_days).to_string());
        recon_op_line.push_str("|");
        recon_op_line.push_str(&data.input_avg_bal.to_string());
        recon_op_line.push_str("|");
        recon_op_line.push_str(&(data.calculated_avg_bal / no_of_days).to_string());
        recon_op_line.push_str("|");
        recon_op_line.push_str(&data.calc_flag);
        recon_op_line.push_str("|");
        recon_op_line.push_str(&source);
        recon_op_line.push_str("|");
        recon_op_line.push_str(&data.acr_int_amt_ccy.to_string());
        recon_op_line.push_str("|");
        recon_op_line.push_str(&data.acr_int_amt_hcy.to_string());
        recon_op_line.push_str("\n");
    }
}
