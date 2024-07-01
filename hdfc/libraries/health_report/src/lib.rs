extern crate sdb_io;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
use std::io::Write;

#[derive(Debug, Serialize, Copy, Clone)]
pub struct HealthReport {
    tot_accounts: i64,
    acc_read_succ: i64,
    acc_read_fail: i64,
    tot_amt_ip: f64,
    tot_amt_op: f64,
    tot_no_cf: i64,
}

impl HealthReport {
    pub fn gen_health_rpt(self, op_path: &str) {
        let mut out_path = op_path.to_string();
        out_path = out_path.replace(".txt", "").replace(".csv", "").replace(".xlsx","").replace(".xls","") + "-health-check-report.json";

        let report_json = serde_json::to_string_pretty(&self).expect("json error");
        let mut wrtr = match sdb_io::buf_file_wrtr(&out_path, None) {
            Ok(writer) => writer,
            Err(error) => panic!(
                "Cannot write to file at path: '{}'. Error: {}",
                out_path, error
            ),
        };

        wrtr.write(&report_json.as_bytes()).expect("writer error");
        wrtr.flush()
            .expect("Unable to flush report writer contents");
    }

    pub fn new(
        tot_accounts: i64,
        acc_read_succ: i64,
        acc_read_fail: i64,
        tot_amt_ip: f64,
        tot_amt_op: f64,
        tot_no_cf: i64,
    ) -> HealthReport {
        HealthReport {
            tot_accounts: tot_accounts,
            acc_read_succ: acc_read_succ,
            acc_read_fail: acc_read_fail,
            tot_amt_ip: tot_amt_ip,
            tot_amt_op: tot_amt_op,
            tot_no_cf: tot_no_cf,
        }
    }

    pub fn display(&self) -> String {
        format!(
            "Accounts encountered: {}\n\
             Accounts proccessed suceessfully: {}\n\
             Accounts failed to process: {}\n\
             Total amount in input: {}\n\
             Total amount in output: {}\n\
             Total no. of cashflows: {}",
            self.tot_accounts,
            self.acc_read_succ,
            self.acc_read_fail,
            self.tot_amt_ip,
            self.tot_amt_op,
            self.tot_no_cf,
        )
    }
}
