use super::super::writer::writer_handle::AggregateWriterReport;
use aggregator::llg_key::LLGKey;
use aggregator::reports::input_report::InputReport;
use aggregator::reports::output_records_report::OutputRecordsWrittenReport;
use serde::ser::{Serialize, SerializeMap, SerializeStruct, Serializer};
use std::collections::HashMap;
use std::ops::AddAssign;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct PrincipalInterestAmountReport {
    pub principal_amount: f64,
    pub interest_amount: f64,
}

impl AddAssign for PrincipalInterestAmountReport {
    fn add_assign(&mut self, other: PrincipalInterestAmountReport) {
        self.principal_amount += other.principal_amount;
        self.interest_amount += other.interest_amount;
    }
}

impl PrincipalInterestAmountReport {
    pub fn new() -> PrincipalInterestAmountReport {
        PrincipalInterestAmountReport {
            principal_amount: 0.0,
            interest_amount: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct LLGsReport {
    pub report: HashMap<LLGKey, LLGReport>,
}

impl LLGsReport {
    pub fn new() -> LLGsReport {
        LLGsReport {
            report: HashMap::new(),
        }
    }

    pub fn add_account_totals_for_llg(&mut self, llg: &LLGKey, totals: InputReport) {
        let mut llg_report = LLGReport::new();
        llg_report.add_account_total(totals);
        self.report
            .entry(llg.clone())
            .and_modify(|l| l.add_account_total(totals))
            .or_insert(llg_report);
    }

    pub fn add_aggregate_writer_report(
        &mut self,
        report: AggregateWriterReport,
    ) -> OutputRecordsWrittenReport {
        let mut output_records_written_report = [0; 12];

        for r in report.into_iter() {
            let thread_no = r.thread_no as usize;
            output_records_written_report[thread_no] = r.total_records_written;
            for (llg, amount_report) in r.llg_amounts_report.into_iter() {
                self.report.get_mut(&llg)
                    .expect(
                        &format!("LLG {:?} doesn't exist in the report.\
                    It's expected you've added account totals at least once for an LLG before adding\
                    six group totals.", llg
                        )
                    ).add_at(thread_no, amount_report);
            }
        }

        OutputRecordsWrittenReport::new(output_records_written_report)
    }
}

pub type GroupAmountsReport = [PrincipalInterestAmountReport; 12];

#[derive(Debug)]
pub struct LLGReport {
    pub accounts_total_report: InputReport,
    pub six_file_amounts: GroupAmountsReport,
}

impl LLGReport {
    fn new() -> LLGReport {
        LLGReport {
            accounts_total_report: InputReport::new(),
            six_file_amounts: [PrincipalInterestAmountReport::new(); 12],
        }
    }

    fn add_account_total(&mut self, account_total: InputReport) {
        self.accounts_total_report += account_total;

        // This API is adding details for a new account.
        // Increment the account count for self.
        self.accounts_total_report.accounts_count += 1;
    }

    fn add_at(&mut self, index: usize, new: PrincipalInterestAmountReport) {
        self.six_file_amounts[index] += new;
    }
}

impl Serialize for LLGReport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("LLGReport", 8)?;
        s.serialize_field("accountsCount", &self.accounts_total_report.accounts_count)?;
        s.serialize_field(
            "cashflowsCount",
            &self.accounts_total_report.cashflows_count,
        )?;
        s.serialize_field(
            "totalPrincipalAmount",
            &self.accounts_total_report.total_principal_amount,
        )?;
        s.serialize_field(
            "totalInterestAmount",
            &self.accounts_total_report.total_interest_amount,
        )?;
        s.serialize_field("file0", &self.six_file_amounts[0])?;
        s.serialize_field("file1", &self.six_file_amounts[1])?;
        s.serialize_field("file2", &self.six_file_amounts[2])?;
        s.serialize_field("file3", &self.six_file_amounts[3])?;
        s.serialize_field("file4", &self.six_file_amounts[4])?;
        s.serialize_field("file5", &self.six_file_amounts[5])?;
        s.end()
    }
}

impl Serialize for LLGsReport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.report.len()))?;
        for (k, v) in &self.report {
            map.serialize_entry(&format!("{}", k), v)?;
        }
        map.end()
    }
}
