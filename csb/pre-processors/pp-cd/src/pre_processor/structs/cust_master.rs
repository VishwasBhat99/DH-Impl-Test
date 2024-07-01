use super::get_data;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CustMasterInput {
    pub dumy: String,
    pub clients_code: String,
    pub client_type: String,
    pub clients_name: String,
    pub clients_bsr_type_flg: String,
    pub clients_busdivn_code: String,
    pub clients_const_code: String,
    pub clients_cust_sub_catg: String,
    pub clients_group_code: String,
    pub clients_pan_gir_num: String,
    pub clients_risk_categorization: String,
    pub clients_risk_cntry: String,
    pub clients_segment_code: String,
    pub corpcl_client_name: String,
    pub corpcl_orgn_qualifier: String,
    pub corpcl_indus_code: String,
    pub corpcl_sub_indus_code: String,
    pub corpcl_nature_of_bus1: String,
    pub corpcl_nature_of_bus2: String,
    pub corpcl_nature_of_bus3: String,
    pub corpcl_scheduled_bank: String,
    pub corpcl_sovereign_flg: String,
    pub corpcl_type_of_sovereign: String,
    pub corpcl_cntry_code: String,
    pub corpcl_central_state_flg: String,
    pub corpcl_public_sector_flg: String,
    pub corpcl_primary_dlr_flg: String,
    pub corpcl_multilateral_bank: String,
    pub corpcl_connp_inv_num: String,
    pub corpcl_bc_gross_turnover: String,
}

#[derive(Debug)]
pub struct CustMasterData {
    pub client_type: String,
    pub clients_name: String,
    pub clients_bsr_type_flg: String,
    pub clients_busdivn_code: String,
    pub clients_const_code: String,
    pub clients_pan_gir_num: String,
    pub clients_risk_categorization: String,
    pub clients_risk_cntry: String,
    pub clients_segment_code: String,
    pub corpcl_orgn_qualifier: String,
    pub corpcl_indus_code: String,
    pub corpcl_nature_of_bus1: String,
    pub corpcl_central_state_flg: String,
    pub corpcl_public_sector_flg: String,
    pub corpcl_primary_dlr_flg: String,
    pub corpcl_multilateral_bank: String,
    pub corpcl_connp_inv_num: String,
    pub corpcl_bc_gross_turnover: String,
}

impl Default for CustMasterData {
    fn default() -> Self {
        CustMasterData {
            client_type: String::from("NA"),
            clients_name: String::from("NA"),
            clients_bsr_type_flg: String::from("NA"),
            clients_busdivn_code: String::from("NA"),
            clients_const_code: String::from("NA"),
            clients_pan_gir_num: String::from("NA"),
            clients_risk_categorization: String::from("NA"),
            clients_risk_cntry: String::from("NA"),
            clients_segment_code: String::from("NA"),
            corpcl_orgn_qualifier: String::from("NA"),
            corpcl_indus_code: String::from("NA"),
            corpcl_nature_of_bus1: String::from("NA"),
            corpcl_central_state_flg: String::from("NA"),
            corpcl_public_sector_flg: String::from("NA"),
            corpcl_primary_dlr_flg: String::from("NA"),
            corpcl_multilateral_bank: String::from("NA"),
            corpcl_connp_inv_num: String::from("NA"),
            corpcl_bc_gross_turnover: String::from("NA"),
        }
    }
}

impl CustMasterData {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            get_data(&self.client_type),
            get_data(&self.clients_name),
            get_data(&self.clients_bsr_type_flg),
            get_data(&self.clients_busdivn_code),
            get_data(&self.clients_const_code),
            get_data(&self.clients_pan_gir_num),
            get_data(&self.clients_risk_categorization),
            get_data(&self.clients_risk_cntry),
            get_data(&self.clients_segment_code),
            get_data(&self.corpcl_orgn_qualifier),
            get_data(&self.corpcl_indus_code),
            get_data(&self.corpcl_nature_of_bus1),
            get_data(&self.corpcl_central_state_flg),
            get_data(&self.corpcl_public_sector_flg),
            get_data(&self.corpcl_primary_dlr_flg),
            get_data(&self.corpcl_multilateral_bank),
            get_data(&self.corpcl_connp_inv_num),
            self.corpcl_bc_gross_turnover,
        )
    }

    pub fn new() -> Self {
        CustMasterData {
            ..Default::default()
        }
    }

    pub fn insert(&mut self, cust_master_input: CustMasterInput) {
        self.client_type = cust_master_input.client_type;
        self.clients_name = cust_master_input.clients_name;
        self.clients_bsr_type_flg = cust_master_input.clients_bsr_type_flg;
        self.clients_busdivn_code = cust_master_input.clients_busdivn_code;
        self.clients_const_code = cust_master_input.clients_const_code;
        self.clients_pan_gir_num = cust_master_input.clients_pan_gir_num;
        self.clients_risk_categorization = cust_master_input.clients_risk_categorization;
        self.clients_risk_cntry = cust_master_input.clients_risk_cntry;
        self.clients_segment_code = cust_master_input.clients_segment_code;
        self.corpcl_orgn_qualifier = cust_master_input.corpcl_orgn_qualifier;
        self.corpcl_indus_code = cust_master_input.corpcl_indus_code;
        self.corpcl_nature_of_bus1 = cust_master_input.corpcl_nature_of_bus1;
        self.corpcl_central_state_flg = cust_master_input.corpcl_central_state_flg;
        self.corpcl_public_sector_flg = cust_master_input.corpcl_public_sector_flg;
        self.corpcl_primary_dlr_flg = cust_master_input.corpcl_primary_dlr_flg;
        self.corpcl_multilateral_bank = cust_master_input.corpcl_multilateral_bank;
        self.corpcl_connp_inv_num = cust_master_input.corpcl_connp_inv_num;
        self.corpcl_bc_gross_turnover = cust_master_input.corpcl_bc_gross_turnover;
    }
}

#[derive(Debug, Default)]
pub struct CustMasterMap {
    pub store: HashMap<String, CustMasterData>,
}

impl CustMasterMap {
    pub fn new() -> Self {
        CustMasterMap {
            store: HashMap::new(),
        }
    }
}
