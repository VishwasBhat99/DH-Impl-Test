use super::get_data;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct NPAInput {
    pub sl_no: String,
    pub branch_code: String,
    pub client_id: String,
    pub branch_name: String,
    pub zone: String,
    pub area: String,
    pub account_no: String,
    pub name: String,
    pub pan: String,
    pub gl_head: String,
    pub asset_code: String,
    pub npa_dt: String,
    pub account_balance: String,
    pub pwo: String,
    pub written_off_dt: String,
    pub ho_balance: String,
    pub npa_provision: String,
    pub ho_provision: String,
    pub suspence_bal: String,
    pub suspence_writeoff: String,
    pub ho_suspence: String,
    pub claim: String,
    pub primary: String,
    pub collateral: String,
    pub total_security: String,
    pub primary_valuation_dt: String,
    pub collateral_valuation_dt: String,
    pub gold_deficit: String,
    pub fraud: String,
    pub wilful_default: String,
    pub subsidy: String,
    pub priority: String,
    pub priority_type: String,
    pub main_sector: String,
    pub sub_sector: String,
    pub activity: String,
    pub industry: String,
    pub category_of_borrower: String,
    pub org_gl_head: String,
}

#[derive(Debug)]
pub struct NPAData {
    pub asset_code: String,
    pub npa_dt: String,
    pub account_balance: String,
    pub pwo: String,
    pub ho_balance: String,
    pub npa_provision: String,
    pub ho_provision: String,
    pub suspence_bal: String,
    pub suspence_writeoff: String,
    pub ho_suspence: String,
    pub claim: String,
    pub primary: String,
    pub collateral: String,
    pub priority: String,
    pub main_sector: String,
    pub industry: String,
}

impl Default for NPAData {
    fn default() -> Self {
        NPAData {
            asset_code: String::from("NA"),
            npa_dt: String::from("NA"),
            account_balance: String::from("NA"),
            pwo: String::from("NA"),
            ho_balance: String::from("0"),
            npa_provision: String::from("0"),
            ho_provision: String::from("0"),
            suspence_bal: String::from("0"),
            suspence_writeoff: String::from("0"),
            ho_suspence: String::from("0"),
            claim: String::from("0"),
            primary: String::from("NA"),
            collateral: String::from("NA"),
            priority: String::from("NA"),
            main_sector: String::from("NA"),
            industry: String::from("NA"),
        }
    }
}

impl NPAData {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            get_data(&self.asset_code),
            get_data(&self.npa_dt),
            get_data(&self.account_balance),
            get_data(&self.pwo),
            get_data(&self.ho_balance),
            get_data(&self.npa_provision),
            get_data(&self.ho_provision),
            get_data(&self.suspence_bal),
            get_data(&self.suspence_writeoff),
            get_data(&self.ho_suspence),
            get_data(&self.claim),
            get_data(&self.primary),
            get_data(&self.collateral),
            get_data(&self.priority),
            get_data(&self.main_sector),
            get_data(&self.industry),
        )
    }

    pub fn new() -> Self {
        NPAData {
            ..Default::default()
        }
    }

    pub fn insert(&mut self, npa_input: NPAInput) {
        self.asset_code = npa_input.asset_code;
        self.npa_dt = npa_input.npa_dt;
        self.account_balance = npa_input.account_balance;
        self.pwo = npa_input.pwo;
        self.ho_balance = npa_input.ho_balance;
        self.npa_provision = npa_input.npa_provision;
        self.ho_provision = npa_input.ho_provision;
        self.suspence_bal = npa_input.suspence_bal;
        self.suspence_writeoff = npa_input.suspence_writeoff;
        self.ho_suspence = npa_input.ho_suspence;
        self.claim = npa_input.claim;
        self.primary = npa_input.primary;
        self.collateral = npa_input.collateral;
        self.priority = npa_input.priority;
        self.main_sector = npa_input.main_sector;
        self.industry = npa_input.industry;
    }
}

#[derive(Debug, Default)]
pub struct NPAMap {
    pub store: HashMap<String, NPAData>,
}

impl NPAMap {
    pub fn new() -> Self {
        NPAMap {
            store: HashMap::new(),
        }
    }
}
