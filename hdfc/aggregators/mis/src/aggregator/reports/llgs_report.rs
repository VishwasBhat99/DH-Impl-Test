use aggregator::llg::llg_key::LLGKey;
use aggregator::reports::input_report::InputReport;
use serde::ser::{Serialize, SerializeMap, SerializeStruct, Serializer};
use std::collections::HashMap;
use std::ops::AddAssign;
#[derive(Clone, Copy, Debug, Serialize)]
pub struct OutstandingAmountReport {
    pub outstanding_amount: f64,
}

impl AddAssign for OutstandingAmountReport {
    fn add_assign(&mut self, other: OutstandingAmountReport) {
        self.outstanding_amount += other.outstanding_amount;
    }
}

impl OutstandingAmountReport {
    pub fn new() -> OutstandingAmountReport {
        OutstandingAmountReport {
            outstanding_amount: 0.0,
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
        if self.report.contains_key(llg) {
            self.report.get_mut(llg).unwrap().add_account_total(totals)
        } else {
            let mut llg_report = LLGReport::new();
            llg_report.add_account_total(totals);
            self.report.insert(llg.clone(), llg_report);
        }
    }
}

#[derive(Debug)]
pub struct LLGReport {
    pub accounts_total_report: InputReport,
}

impl LLGReport {
    fn new() -> LLGReport {
        LLGReport {
            accounts_total_report: InputReport::new(),
        }
    }

    fn add_account_total(&mut self, account_total: InputReport) {
        self.accounts_total_report += account_total;

        // This API is adding details for a new account.
        // Increment the account count for self.
        self.accounts_total_report.accounts_count += 1;
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
