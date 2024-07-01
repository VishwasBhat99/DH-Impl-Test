use super::account_field_names::AccFieldNames;
use configuration_parameters::ConfigurationParameters;
use rbdate::{NaiveDate, NaiveDateTime};
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Serialize, Deserialize, Debug)]
pub struct OutputData {
    pub as_on_date: String,
    pub src_file_name: String,
    pub account_number: String,
    pub branch: String,
    pub product: String,
    pub sub_product_code: String,
    pub customer_id: String,
    pub customer_grp_id: String,
    pub customer_name: String,
    pub customer_title: String,
    pub gl_code_1: String,
    pub gl_code_2: String,
    pub gl_code_3: String,
    pub gl_code_4: String,
    pub loan_subtype: String,
    pub repayment_frequency: String,
    pub guarantor_id: String,
    pub guarantor_name: String,
    pub loan_sanction_date: i64,
    pub account_value_date: i64,
    pub loan_disbursement_date: i64,
    pub account_maturity_date: i64,
    pub currency: String,
    pub ost_bal_ccy: String,
    pub ost_bal_lcy: String,
    pub exchange_rate: f64,
    pub purpose_of_loan: String,
    pub ltv: String,
    pub is_restruct: String,
    pub last_restructured_date: i64,
    pub rating: String,
    pub internal_rating: String,
    pub external_rating_agenecy: String,
    pub external_rating: String,
    pub pd: String,
    pub cust_category: String,
    pub sector: String,
    pub industry: String,
    pub cust_class_1: String,
    pub cust_class_2: String,
    pub sys_claim_rule_id: String,
    pub sys_classified_claim_id: String,
    pub sys_sub_claim_rule_id: String,
    pub sys_classified_sub_claim_id: String,
    pub sys_rw_rule_id: String,
    pub sys_rw: String,
    pub final_classified_claim_id: String,
    pub final_rw: f64,
    pub rw_out_bal_lcy: String,
    pub rw_out_bal_ccy: String,
    pub prov_amt: String,
    pub tot_col_amt_lcy: String,
    pub crm_amt_lcy: String,
    pub final_rw_amt: String,
    pub iis_amt: String,
    pub final_rw_amt_after_iis: f64,
}

impl Display for OutputData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{:.3}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.as_on_date,
            self.account_number,
            self.src_file_name,
            self.sys_classified_sub_claim_id,
            self.ost_bal_ccy,
            self.ost_bal_lcy,
            self.currency,
            self.exchange_rate,
            self.gl_code_1,
            self.gl_code_2,
            self.gl_code_3,
            self.customer_title,
            self.customer_id,
            self.customer_name,
            self.customer_grp_id,
            self.customer_name,
            self.branch,
            naivedate_from_timestamp(self.loan_sanction_date).format("%d-%m-%Y").to_string(),
            naivedate_from_timestamp(self.account_value_date).format("%d-%m-%Y").to_string(),
            naivedate_from_timestamp(self.account_maturity_date).format("%d-%m-%Y").to_string(),
            self.rating,
            self.internal_rating,
            self.external_rating_agenecy,
            self.external_rating,
            self.purpose_of_loan,
            self.product,
            self.sub_product_code,
            self.sys_claim_rule_id,
            self.sys_sub_claim_rule_id,
            self.sys_rw_rule_id,
            self.cust_class_1,
            self.cust_class_2,
            self.sys_classified_claim_id,
            self.final_classified_claim_id,
            self.sys_rw,
            self.industry,
            self.loan_subtype,
            self.repayment_frequency,
            self.guarantor_id,
            self.guarantor_name,
            self.sys_sub_claim_rule_id,
            self.sys_rw_rule_id,
            self.final_rw,
            self.rw_out_bal_lcy,
            self.rw_out_bal_ccy,
            self.prov_amt,
            self.tot_col_amt_lcy,
            self.crm_amt_lcy,
            self.final_rw_amt,
            self.iis_amt,
            self.final_rw_amt_after_iis,
            "",
            "",
            "",
            naivedate_from_timestamp(self.loan_disbursement_date).format("%d-%m-%Y").to_string(),
            naivedate_from_timestamp(self.last_restructured_date).format("%d-%m-%Y").to_string(),
            "01-01-1970",
            "01-01-1970",
            "01-01-1970",
            self.ltv,
            self.is_restruct,
            self.pd,
            self.cust_category,
            self.sector,
        )
    }
}

pub fn get_op_data(
    account: &AccountWithCFs,
    acc_keys: &AccFieldNames,
    ost_bal_lcy: f64,
    ost_bal_ccy: f64,
    account_number: String,
    tot_col_amt_lcy: f64,
    crm_amt_lcy: f64,
    exchange_rate: f64,
    iis_amt: f64,
    rw_id: String,
    sub_id: String,
    prov_data_map: &mut HashMap<String, f64>,
    config_params: &ConfigurationParameters,
) -> OutputData {
    let claim_info = get_claim_info(config_params);
    let mut crm_amt = crm_amt_lcy;
    let final_rw: f64 = rw_id[4..8].to_string().parse().unwrap_or(0.0);
    // Using default value for prov amt as per program logic
    let prov_amt = prov_data_map.remove(&account_number).unwrap_or(0.0);
    let mut rw_out_bal_lcy = (ost_bal_lcy * config_params.ccf_prcnt()) / 100.0;
    let rw_out_bal_ccy = (ost_bal_ccy * config_params.ccf_prcnt()) / 100.0;
    rw_out_bal_lcy = if crm_amt > rw_out_bal_lcy && config_params.neg_crm_check() {
        crm_amt = rw_out_bal_lcy;
        0.0
    } else {
        rw_out_bal_lcy - crm_amt
    };
    rw_out_bal_lcy -= prov_amt;
    let final_rw_amt = (rw_out_bal_lcy * final_rw) / 100.0;

    let mut final_rw_amt_after_iis = 0.0;
    if (iis_amt < 0.0 && final_rw_amt == 0.0) || (iis_amt > final_rw_amt) {
        final_rw_amt_after_iis = 0.0;
    }
    else{
        final_rw_amt_after_iis = final_rw_amt - (iis_amt * (final_rw/100.0));
    }
    OutputData {
        as_on_date: config_params.as_on_date().format("%d-%m-%Y").to_string(),
        src_file_name: config_params.src_file_name().to_string(),
        account_number: if config_params.src_file_name().contains('~') {
            account_number + &config_params.src_file_name().replace("~", "-")
        } else {
            account_number
        },
        branch: match account.get_i64_for_key(&acc_keys.branch) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        product: match account.get_i64_for_key(&acc_keys.product) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        sub_product_code: match account.get_string_for_key(&acc_keys.sub_product_code) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        customer_id: match account.get_i64_for_key(&acc_keys.customer_id) {
            Ok(val) => val.to_string(),
            Err(_) => match account.get_string_for_key(&acc_keys.customer_id) {
                Ok(val) => val.to_string(),
                Err(_) => format!(""),
            },
        },
        customer_grp_id: match account.get_i64_for_key(&acc_keys.customer_grp_id) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        customer_name: match account.get_string_for_key(&acc_keys.customer_name) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        customer_title: match account.get_string_for_key(&acc_keys.customer_title) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        gl_code_1: match account.get_i64_for_key(&acc_keys.gl_code_1) {
            Ok(val) => val.to_string(),
            Err(_) => match account.get_string_for_key(&acc_keys.gl_code_1) {
                Ok(val) => val.to_string(),
                Err(_) => format!(""),
            },
        },
        gl_code_2: match account.get_i64_for_key(&acc_keys.gl_code_2) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        gl_code_3: match account.get_i64_for_key(&acc_keys.gl_code_3) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        gl_code_4: match account.get_i64_for_key(&acc_keys.gl_code_4) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        loan_subtype: match account.get_string_for_key(&acc_keys.loan_subtype) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        repayment_frequency: match account.get_string_for_key(&acc_keys.repayment_frequency) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        guarantor_id: match account.get_string_for_key(&acc_keys.guarantor_id) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        guarantor_name: match account.get_string_for_key(&acc_keys.guarantor_name) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        loan_sanction_date: match account.get_i64_for_key(&acc_keys.loan_sanction_date) {
            Ok(val) => val,
            Err(_) => 0,
        },
        account_value_date: match account.get_i64_for_key(&acc_keys.account_value_date) {
            Ok(val) => val,
            Err(_) => 0,
        },
        loan_disbursement_date: match account.get_i64_for_key(&acc_keys.loan_disbursement_date) {
            Ok(val) => val,
            Err(_) => 0,
        },
        account_maturity_date: match account.get_i64_for_key(&acc_keys.account_maturity_date) {
            Ok(val) => val,
            Err(_) => 0,
        },
        currency: match account.get_string_for_key(&acc_keys.currency) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        ost_bal_ccy: ost_bal_ccy.to_string(),
        ost_bal_lcy: ost_bal_lcy.to_string(),
        exchange_rate: exchange_rate,
        purpose_of_loan: match account.get_string_for_key(&acc_keys.purpose_of_loan) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        ltv: match account.get_f64_for_key(&acc_keys.ltv) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        is_restruct: match account.get_string_for_key(&acc_keys.is_restruct) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        last_restructured_date: match account.get_i64_for_key(&acc_keys.last_restructured_date) {
            Ok(val) => val,
            Err(_) => 0,
        },
        rating: match account.get_string_for_key(&acc_keys.rating) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        internal_rating: match account.get_string_for_key(&acc_keys.internal_rating) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        external_rating_agenecy: match account.get_string_for_key(&acc_keys.external_rating_agenecy)
        {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        external_rating: match account.get_string_for_key(&acc_keys.external_rating) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        pd: match account.get_string_for_key(&acc_keys.pd) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        cust_category: match account.get_string_for_key(&acc_keys.cust_category) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        sector: match account.get_string_for_key(&acc_keys.sector) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        industry: match account.get_string_for_key(&acc_keys.industry) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        cust_class_1: match account.get_string_for_key(&acc_keys.cust_class_1) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        cust_class_2: match account.get_string_for_key(&acc_keys.cust_class_2) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        iis_amt: iis_amt.to_string(),
        final_rw_amt_after_iis: final_rw_amt_after_iis,
        sys_claim_rule_id: claim_info[0..4].to_string(),
        sys_classified_claim_id: claim_info[4..8].to_string(),
        sys_sub_claim_rule_id: sub_id[0..4].to_string(),
        sys_classified_sub_claim_id: sub_id[4..8].to_string(),
        sys_rw_rule_id: rw_id[0..4].to_string(),
        sys_rw: rw_id[4..8].to_string(),
        final_classified_claim_id: claim_info[4..8].to_string(),
        final_rw: final_rw,
        tot_col_amt_lcy: tot_col_amt_lcy.to_string(),
        crm_amt_lcy: crm_amt.to_string(),
        rw_out_bal_lcy: rw_out_bal_lcy.to_string(),
        rw_out_bal_ccy: rw_out_bal_ccy.to_string(),
        prov_amt: prov_amt.to_string(),
        final_rw_amt: final_rw_amt.to_string(),
    }
}

pub fn get_claim_info(config_params: &ConfigurationParameters) -> String {
    let file_det: Vec<&str> = config_params.input_file_path().split('/').collect();
    let file_name = file_det[file_det.len() - 1].replace(".cf", "");
    let file_info: Vec<&str> = file_name.split('-').collect();
    let id = file_info[file_info.len() - 1].to_string();
    id
}

pub fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}
