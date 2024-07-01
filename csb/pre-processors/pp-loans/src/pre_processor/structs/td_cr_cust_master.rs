use super::get_data;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TdCrCustMasterInput {
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
    pub t1: String,
    pub t2: String,
    pub t3: String,
    pub t4: String,
}

#[derive(Debug)]
pub struct TdCrCustMasterData {
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
    pub t1: String,
    pub t2: String,
    pub t3: String,
    pub t4: String,
}

impl Default for TdCrCustMasterData {
    fn default() -> Self {
        TdCrCustMasterData {
            dumy: String::from("NA"),
            clients_code: String::from("NA"),
            client_type: String::from("NA"),
            clients_name: String::from("NA"),
            clients_bsr_type_flg: String::from("NA"),
            clients_busdivn_code: String::from("NA"),
            clients_const_code: String::from("NA"),
            clients_cust_sub_catg: String::from("NA"),
            clients_group_code: String::from("NA"),
            clients_pan_gir_num: String::from("NA"),
            clients_risk_categorization: String::from("NA"),
            clients_risk_cntry: String::from("NA"),
            clients_segment_code: String::from("NA"),
            corpcl_client_name: String::from("NA"),
            corpcl_orgn_qualifier: String::from("NA"),
            corpcl_indus_code: String::from("NA"),
            corpcl_sub_indus_code: String::from("NA"),
            corpcl_nature_of_bus1: String::from("NA"),
            corpcl_nature_of_bus2: String::from("NA"),
            corpcl_nature_of_bus3: String::from("NA"),
            corpcl_scheduled_bank: String::from("NA"),
            corpcl_sovereign_flg: String::from("NA"),
            corpcl_type_of_sovereign: String::from("NA"),
            corpcl_cntry_code: String::from("NA"),
            corpcl_central_state_flg: String::from("NA"),
            corpcl_public_sector_flg: String::from("NA"),
            corpcl_primary_dlr_flg: String::from("NA"),
            corpcl_multilateral_bank: String::from("NA"),
            corpcl_connp_inv_num: String::from("NA"),
            corpcl_bc_gross_turnover: String::from("NA"),
            t1: String::from("NA"),
            t2: String::from("NA"),
            t3: String::from("NA"),
            t4: String::from("NA"),
        }
    }
}

impl TdCrCustMasterData {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            get_data(&self.dumy),
            get_data(&self.clients_code),
            get_data(&self.client_type),
            get_data(&self.clients_name),
            get_data(&self.clients_bsr_type_flg),
            get_data(&self.clients_busdivn_code),
            get_data(&self.clients_const_code),
            get_data(&self.clients_cust_sub_catg),
            get_data(&self.clients_group_code),
            get_data(&self.clients_pan_gir_num),
            get_data(&self.clients_risk_categorization),
            get_data(&self.clients_risk_cntry),
            get_data(&self.clients_segment_code),
            get_data(&self.corpcl_client_name),
            get_data(&self.corpcl_orgn_qualifier),
            get_data(&self.corpcl_indus_code),
            get_data(&self.corpcl_sub_indus_code),
            get_data(&self.corpcl_nature_of_bus1),
            get_data(&self.corpcl_nature_of_bus2),
            get_data(&self.corpcl_nature_of_bus3),
            get_data(&self.corpcl_scheduled_bank),
            get_data(&self.corpcl_sovereign_flg),
            get_data(&self.corpcl_type_of_sovereign),
            get_data(&self.corpcl_cntry_code),
            get_data(&self.corpcl_central_state_flg),
            get_data(&self.corpcl_public_sector_flg),
            get_data(&self.corpcl_primary_dlr_flg),
            get_data(&self.corpcl_multilateral_bank),
            get_data(&self.corpcl_connp_inv_num),
            self.corpcl_bc_gross_turnover,
            get_data(&self.t1),
            get_data(&self.t2),
            get_data(&self.t3),
            get_data(&self.t4),
        )
    }

    pub fn new() -> Self {
        TdCrCustMasterData {
            ..Default::default()
        }
    }

    pub fn insert(&mut self, cust_master_input: TdCrCustMasterInput) {
        self.dumy = cust_master_input.dumy;
        self.clients_code = cust_master_input.clients_code;
        self.client_type = cust_master_input.client_type;
        self.clients_name = cust_master_input.clients_name;
        self.clients_bsr_type_flg = cust_master_input.clients_bsr_type_flg;
        self.clients_busdivn_code = cust_master_input.clients_busdivn_code;
        self.clients_const_code = cust_master_input.clients_const_code;
        self.clients_cust_sub_catg = cust_master_input.clients_cust_sub_catg;
        self.clients_group_code = cust_master_input.clients_group_code;
        self.clients_pan_gir_num = cust_master_input.clients_pan_gir_num;
        self.clients_risk_categorization = cust_master_input.clients_risk_categorization;
        self.clients_risk_cntry = cust_master_input.clients_risk_cntry;
        self.clients_segment_code = cust_master_input.clients_segment_code;
        self.corpcl_client_name = cust_master_input.corpcl_client_name;
        self.corpcl_orgn_qualifier = cust_master_input.corpcl_orgn_qualifier;
        self.corpcl_indus_code = cust_master_input.corpcl_indus_code;
        self.corpcl_sub_indus_code = cust_master_input.corpcl_sub_indus_code;
        self.corpcl_nature_of_bus1 = cust_master_input.corpcl_nature_of_bus1;
        self.corpcl_nature_of_bus2 = cust_master_input.corpcl_nature_of_bus2;
        self.corpcl_nature_of_bus3 = cust_master_input.corpcl_nature_of_bus3;
        self.corpcl_scheduled_bank = cust_master_input.corpcl_scheduled_bank;
        self.corpcl_sovereign_flg = cust_master_input.corpcl_sovereign_flg;
        self.corpcl_type_of_sovereign = cust_master_input.corpcl_type_of_sovereign;
        self.corpcl_cntry_code = cust_master_input.corpcl_cntry_code;
        self.corpcl_central_state_flg = cust_master_input.corpcl_central_state_flg;
        self.corpcl_public_sector_flg = cust_master_input.corpcl_public_sector_flg;
        self.corpcl_primary_dlr_flg = cust_master_input.corpcl_primary_dlr_flg;
        self.corpcl_multilateral_bank = cust_master_input.corpcl_multilateral_bank;
        self.corpcl_connp_inv_num = cust_master_input.corpcl_connp_inv_num;
        self.corpcl_bc_gross_turnover = cust_master_input.corpcl_bc_gross_turnover;
        self.t1 = cust_master_input.t1;
        self.t2 = cust_master_input.t2;
        self.t3 = cust_master_input.t3;
        self.t4 = cust_master_input.t4;
    }
}

#[derive(Debug, Default)]
pub struct TdCrCustMasterMap {
    pub store: HashMap<String, TdCrCustMasterData>,
}

impl TdCrCustMasterMap {
    pub fn new() -> Self {
        TdCrCustMasterMap {
            store: HashMap::new(),
        }
    }
}
