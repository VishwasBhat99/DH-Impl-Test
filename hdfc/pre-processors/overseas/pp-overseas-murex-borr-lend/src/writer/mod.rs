use sdb_io::buf_file_wrtr;
use std::env::current_dir;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use crate::pre_processor::alm_concat::ALMMasterFields;
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
    alm_fields: &ALMMasterFields,
    funding_source: &str,
) {
    let mut int_amt = 0.0;
    let mut prin_amt = 0.0;
    if fields[19] == "PRINCIPAL" {
        int_amt = 0.0;
        prin_amt = fields[18].parse::<f64>().unwrap_or(0.0);
    } else if fields[19] == "INTEREST" {
        prin_amt = 0.0;
        int_amt = fields[18].parse::<f64>().unwrap_or(0.0);
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
        "{}|{}|{}|{}|{}|{}|{}|{}",
        source_gl,
        product_concat,
        alm_concat,
        division,
        alm_fields.alm_line,
        alm_fields.ia_line,
        alm_fields.balm_l2,
        funding_source
    )
    .expect("Error while writing derived fields.");
    writeln!(writer, "").expect("Error while writing output.");
}

pub fn write_air_aip(
    fields: &[&str],
    writer: &mut BufWriter<File>,
    source_gl: &str,
    alm_concat: &str,
    product_concat: &str,
    division: &str,
    alm_fields: &ALMMasterFields,
    funding_source: &str,
) {
    let int_amt = 0.0;
    let prin_amt = fields[30].parse::<f64>().unwrap_or(0.0);
    for (index, value) in fields.iter().enumerate() {
        if index == 18 {
            write!(writer, "{}|{}|", prin_amt, int_amt)
                .expect("Error while writing principal and interest amount.");
            continue;
        } else if (index >= 18 && index <= 29) || (index >= 31 && index <= 39) {
            write!(writer, "|").expect("Error while writing empty fields in air/aip file.");
            continue;
        }
        write!(writer, "{}|", value).expect("Error while writing empty fields in air/aip file.");
    }
    write!(
        writer,
        "{}|{}|{}|{}|{}|{}|{}|{}",
        source_gl,
        product_concat,
        alm_concat,
        division,
        alm_fields.alm_line,
        alm_fields.ia_line,
        alm_fields.balm_l2,
        funding_source
    )
    .expect("Error while writing derived fields.");
    writeln!(writer, "").expect("Error while writing air/aip output.");
}
