use std::ops::AddAssign;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use sdb_dyn_proto_rdr::compound_types::Cashflow;

#[derive(Debug, Copy, Clone)]
pub struct InputReport {
    pub accounts_count: i32,
    pub cashflows_count: i32,
    pub total_principal_amount: f64,
    pub total_interest_amount: f64,
}

impl InputReport {
    pub fn new() -> InputReport {
        InputReport {
            accounts_count: 0,
            cashflows_count: 0,
            total_principal_amount: 0.0,
            total_interest_amount: 0.0
        }
    }

    pub fn add_cf_values(&mut self, cf: &Cashflow) {
        self.cashflows_count += 1;
        self.total_principal_amount += cf.principal_amount;
        self.total_interest_amount += cf.interest_amount;
    }

    pub fn add_account_totals(
        &mut self,
        account_amount_total: InputReport
    ) {
        self.accounts_count += 1;   // New account being added
        self.cashflows_count += account_amount_total.cashflows_count;
        self.total_principal_amount += account_amount_total.total_principal_amount;
        self.total_interest_amount += account_amount_total.total_interest_amount;
    }
}

impl AddAssign for InputReport {
    fn add_assign(&mut self, other: InputReport) {
        self.accounts_count += other.accounts_count;
        self.cashflows_count += other.cashflows_count;
        self.total_principal_amount += other.total_principal_amount;
        self.total_interest_amount += other.total_interest_amount;
    }
}

impl Serialize for InputReport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut s = serializer.serialize_struct("input", 5)?;
        s.serialize_field("accountsCount", &self.accounts_count)?;
        s.serialize_field("cashflowsCount", &self.cashflows_count)?;
        s.serialize_field("totalPrincipalAmount", &self.total_principal_amount)?;
        s.serialize_field("totalInterestAmount", &self.total_interest_amount)?;
        s.end()
    }
}