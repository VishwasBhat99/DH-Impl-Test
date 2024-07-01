use crate::calculation::OutputData;
use std::fmt;
use std::fs::File;
use std::io::Write;

impl fmt::Display for OutputData {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.id,
            self.from_year,
            self.from_months,
            self.from_days,
            self.to_year,
            self.to_months,
            self.to_days,
            self.from_amt,
            self.to_amt,
            self.rate_index
        )
    }
}

pub fn write_file(out: OutputData, out_file: &mut File) {
    writeln!(out_file, "{}", out).expect("Error while writing into output file");
}
