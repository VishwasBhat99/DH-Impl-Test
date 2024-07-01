use sdb_io::buf_file_wrtr;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
#[derive(Debug)]
pub struct Writers {
    pub borr_cf_writer: BufWriter<File>,
    pub borr_aip_writer: BufWriter<File>,
    pub lend_cf_writer: BufWriter<File>,
    pub lend_air_writer: BufWriter<File>,
    pub concat_writer: BufWriter<File>,
}

impl Writers {
    pub fn create_writer(
        output_path_borr: &str,
        output_path_lend: &str,
        output_concat: &str,
    ) -> Writers {
        let borr_cf = output_path_borr.to_string() + "_cf_output.txt";
        let lend_cf = output_path_lend.to_string() + "_cf_output.txt";
        let borr_aip = output_path_borr.to_string() + "_aip_output.txt";
        let lend_air = output_path_lend.to_string() + "_air_output.txt";
        let concat = output_concat.to_string();

        let writer_borr_cf = match buf_file_wrtr(borr_cf.as_str(), None) {
            Ok(file) => file,
            Err(error) => panic!(
                "Unable to create output file `{}` on location `{}` : {}",
                borr_cf,
                current_dir()
                    .expect("Unable to get current directory path.")
                    .display(),
                error
            ),
        };
        let writer_borr_aip = match buf_file_wrtr(borr_aip.as_str(), None) {
            Ok(file) => file,
            Err(error) => panic!(
                "Unable to create output file `{}` on location `{}` : {}",
                borr_cf,
                current_dir()
                    .expect("Could not create Borrowings AIP file.")
                    .display(),
                error
            ),
        };
        let writer_lend_cf = match buf_file_wrtr(lend_cf.as_str(), None) {
            Ok(file) => file,
            Err(error) => panic!(
                "Unable to create output file `{}` on location `{}` : {}",
                borr_cf,
                current_dir()
                    .expect("Could not create Lendings Cashflow file.")
                    .display(),
                error
            ),
        };
        let writer_lend_air = match buf_file_wrtr(lend_air.as_str(), None) {
            Ok(file) => file,
            Err(error) => panic!(
                "Unable to create output file `{}` on location `{}` : {}",
                borr_cf,
                current_dir()
                    .expect("Could not create Lendings AIR file.")
                    .display(),
                error
            ),
        };
        let writer_concat = match buf_file_wrtr(concat.as_str(), None) {
            Ok(file) => file,
            Err(error) => panic!(
                "Unable to create concat file `{}` on location `{}` : {}",
                concat,
                current_dir()
                    .expect("Could not create concat file.")
                    .display(),
                error
            ),
        };

        Writers {
            borr_cf_writer: writer_borr_cf,
            borr_aip_writer: writer_borr_aip,
            lend_cf_writer: writer_lend_cf,
            lend_air_writer: writer_lend_air,
            concat_writer: writer_concat,
        }
    }
}

pub fn write_cf(
    fields: &[&str],
    writer: &mut BufWriter<File>,
    source_gl: &str,
    alm_concat: &str,
    product_concat: &str,
    division: &str,
    alm_line: &str,
    ia_line: &str,
    oper_typ: &str,
    sma_data: &HashMap<String, String>,
) {
    let mut int_amt = 0.0;
    let mut prin_amt = 0.0;
    let amt = fields[18].parse::<f64>().unwrap_or(0.0);
    if fields[19] == "PRINCIPAL" {
        int_amt = 0.0;
        if oper_typ == "BORROW" && amt < 0.0 {
            prin_amt = amt.abs();
        } else if oper_typ == "LEND" && amt > 0.0 {
            prin_amt = amt;
        }
    } else if fields[19] == "INTEREST" {
        prin_amt = 0.0;
        int_amt = amt.abs();
    }
    for (index, value) in fields.iter().enumerate() {
        if index == 18 {
            write!(writer, "{}|{}|", prin_amt, int_amt)
                .expect("Error while writing principal and interest amount.");
            continue;
        }
        write!(writer, "{}|", value).expect("Error while writing principal and interest amount.");
    }
    write!(
        writer,
        "{}|{}|{}|{}|{}|{}|{}",
        source_gl,
        product_concat,
        alm_concat,
        division,
        alm_line,
        ia_line,
        &sma_data
            .get(&fields[0].to_string())
            .unwrap_or(&"P".to_string())
            .to_string(),
    )
    .expect("Error while writing derived fields.");
    writeln!(writer).expect("Error while writing output.");
}

pub fn write_air_aip(
    fields: &[&str],
    writer: &mut BufWriter<File>,
    source_gl: &str,
    alm_concat: &str,
    product_concat: &str,
    division: &str,
    alm_line: &str,
    ia_line: &str,
    sma_data: &HashMap<String, String>,
) {
    let int_amt = 0.0;
    let prin_amt = fields[30].parse::<f64>().unwrap_or(0.0);
    for (index, value) in fields.iter().enumerate() {
        if index == 18 {
            write!(writer, "{}|{}|", prin_amt, int_amt)
                .expect("Error while writing principal and interest amount.");
            continue;
        } else if (18..=29).contains(&index) || (31..=39).contains(&index) {
            write!(writer, "|").expect("Error while writing empty fields in air/aip file.");
            continue;
        }
        write!(writer, "{}|", value).expect("Error while writing empty fields in air/aip file.");
    }
    writeln!(
        writer,
        "{}|{}|{}|{}|{}|{}|{}",
        source_gl,
        product_concat,
        alm_concat,
        division,
        alm_line,
        ia_line,
        &sma_data
            .get(&fields[0].to_string())
            .unwrap_or(&"P".to_string())
            .to_string(),
    )
    .expect("Error while writing derived fields.");
    writeln!(writer).expect("Error while writing air/aip output.");
}
