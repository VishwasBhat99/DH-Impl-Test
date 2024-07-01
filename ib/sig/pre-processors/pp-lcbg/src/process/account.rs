use crate::configuration_parameters::ConfigurationParameters;

#[derive(Debug, Clone, Default)]
pub struct Account {
    pub rpt_date: String,
    pub reference_number: String,
    pub account_id: String,
    pub customer_id: String,
    pub crncy_code: String,
    pub bill_pur_dt: String,
    pub bill_due_dt: String,
    pub bank_code: String,
    pub branch_code: String,
    pub zone_code: String,
    pub sanctioned_limit: f64,
    pub outstanding_balance: f64,
    pub util_amount: f64,
    pub schm_type: String,
    pub schm_code: String,
    pub beneficiary_name: String,
    pub issued_on_behalf_of: String,
    pub prd_cd: String,
    pub gl_code: String,
    pub country_code: String,
    pub credit_status_cd: String,
    pub devolvement_lc: String,
    pub devolvement_lc_amount: f64,
    pub alm_cd: String,
    pub bsr_cd: String,
    pub rbi_cd: String,
    pub avg_book_balance: f64,
    pub rediscount_flag: String,
    pub cgl_number: String,
    pub interest_rate: f64,
    pub cash_margin: String,
    pub margin_amount: f64,
    pub close_flag: String,
    pub true_cur: String,
    pub type_of_bill: String,
    pub cust_type: String,
    pub branch: String,
    pub acod: String,
    pub ref_type: String,

    //Fields Derived in Output(Not present in Input)
    pub as_on_date: String,
    pub a1: f64,
    pub a2: f64,
    pub a3: f64,
    pub a4: f64,
    pub a5: String,
    pub a6: String,
    pub a7: String,
    pub a8: String,
    pub a9: String,
    pub a10: String,
}

impl Account {
    pub fn get_data(input_acc: Vec<&str>, config_params: &ConfigurationParameters) -> Account {
        let date_parser = rbdate::DateParser::new("%Y-%m-%d".to_string(), false);
        Account {
            rpt_date: date_parser
            .parse_opt(input_acc[0])
            .unwrap_or(*config_params.as_on_date())
            .format("%d-%m-%Y")
            .to_string(),
            reference_number: input_acc[1].to_string(),
            account_id: input_acc[2].to_string(),
            customer_id: input_acc[3].to_string(),
            crncy_code: input_acc[4].to_string(),
            bill_pur_dt: date_parser
            .parse_opt(input_acc[5])
            .unwrap_or(*config_params.as_on_date())
            .format("%d-%m-%Y")
            .to_string(),
            bill_due_dt: date_parser
            .parse_opt(input_acc[6])
            .unwrap_or(*config_params.as_on_date())
            .format("%d-%m-%Y")
            .to_string(),
            bank_code: input_acc[7].to_string(),
            branch_code: input_acc[8].to_string(),
            zone_code: input_acc[9].to_string(),
            sanctioned_limit: input_acc[10].to_string().parse::<f64>().unwrap_or(0.0),
            outstanding_balance: input_acc[11].to_string().parse::<f64>().unwrap_or(0.0),
            util_amount: input_acc[12].to_string().parse::<f64>().unwrap_or(0.0),
            schm_type: input_acc[13].to_string(),
            schm_code: input_acc[14].to_string(),
            beneficiary_name: input_acc[15].to_string(),
            issued_on_behalf_of: input_acc[16].to_string(),
            prd_cd: input_acc[17].to_string(),
            gl_code: input_acc[18].to_string(),
            country_code: input_acc[19].to_string(),
            credit_status_cd: input_acc[20].to_string(),
            devolvement_lc: input_acc[21].to_string(),
            devolvement_lc_amount: input_acc[22].to_string().parse::<f64>().unwrap_or(0.0),
            alm_cd: input_acc[23].to_string(),
            bsr_cd: input_acc[24].to_string(),
            rbi_cd: input_acc[25].to_string(),
            avg_book_balance: input_acc[26].to_string().parse::<f64>().unwrap_or(0.0),
            rediscount_flag: input_acc[27].to_string(),
            cgl_number: input_acc[28].to_string(),
            interest_rate: input_acc[29].to_string().parse::<f64>().unwrap_or(0.0),
            cash_margin: input_acc[30].to_string(),
            margin_amount: input_acc[31].to_string().parse::<f64>().unwrap_or(0.0),
            close_flag: input_acc[32].to_string(),
            true_cur: input_acc[33].to_string(),
            type_of_bill: input_acc[34].to_string(),
            cust_type: input_acc[35].to_string(),
            branch: input_acc[36].to_string(),
            acod: input_acc[37].to_string(),
            ref_type: input_acc[38].to_string(),
            as_on_date: config_params.as_on_date().format("%d-%m-%Y").to_string(),
            a1: 0.0,
            a2: 0.0,
            a3: 0.0,
            a4: 0.0,
            a5: config_params.as_on_date().format("%d-%m-%Y").to_string(),
            a6: config_params.as_on_date().format("%d-%m-%Y").to_string(),
            a7: "NA".to_string(),
            a8: "NA".to_string(),
            a9: "NA".to_string(),
            a10: "NA".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
///Fields used for stamping in output file
pub struct MasterData {
    pub acct_branch: String,
    pub acct_no: String,
    pub acct_suffix: String,
    pub acct_short_name: String,
    pub acct_type: String,
    pub cust_type: String,
    pub analysis_code: String,
    pub sundry_analysis_code: String,
    pub res_code: String,
    pub vs_param: String,
    pub vg_param: String,
    pub local_ccy: String,
    pub acct_ccy: String,
    pub bal_in_local_ccy: String,
    pub bal_in_actual_ccy: String,
    pub exrt_rate: String,
    pub date_of_extraction: String,
}

impl MasterData {
    pub fn new(master_data: Vec<&str>) -> MasterData {
        MasterData {
            acct_branch: master_data[0].to_string(),
            acct_no: master_data[1].to_string(),
            acct_suffix: master_data[2].to_string(),
            acct_short_name: master_data[3].to_string(),
            acct_type: master_data[4].to_string(),
            cust_type: master_data[5].to_string(),
            analysis_code: master_data[6].to_string(),
            sundry_analysis_code: master_data[7].to_string(),
            res_code: master_data[8].to_string(),
            vs_param: master_data[9].to_string(),
            vg_param: master_data[10].to_string(),
            local_ccy: master_data[11].to_string(),
            acct_ccy: master_data[12].to_string(),
            bal_in_local_ccy: master_data[13].to_string(),
            bal_in_actual_ccy: master_data[14].to_string(),
            exrt_rate: master_data[15].to_string(),
            date_of_extraction: master_data[16].to_string(),
        }
    }

    pub fn def() -> MasterData {
        MasterData {
            acct_branch: "NONE".to_string(),
            acct_no: "NONE".to_string(),
            acct_suffix: "NONE".to_string(),
            acct_short_name: "NONE".to_string(),
            acct_type: "NONE".to_string(),
            cust_type: "NONE".to_string(),
            analysis_code: "NONE".to_string(),
            sundry_analysis_code: "NONE".to_string(),
            res_code: "NONE".to_string(),
            vs_param: "NONE".to_string(),
            vg_param: "NONE".to_string(),
            local_ccy: "NONE".to_string(),
            acct_ccy: "NONE".to_string(),
            bal_in_local_ccy: "NONE".to_string(),
            bal_in_actual_ccy: "NONE".to_string(),
            exrt_rate: "NONE".to_string(),
            date_of_extraction: "NONE".to_string(),
        }
    }

    pub fn get_concat(master_data: MasterData) -> String {
        format!(
            "{}{}{}{}",
            master_data.acct_suffix.trim(),
            master_data.acct_no.trim(),
            master_data.acct_type.trim(),
            master_data.cust_type.trim(),
        )
    }
}

pub fn format_output(account: &Account) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        account.rpt_date,
        account.reference_number,
        account.account_id,
        account.customer_id,
        account.crncy_code,
        account.bill_pur_dt,
        account.bill_due_dt,
        account.bank_code,
        account.branch_code,
        account.zone_code,
        account.sanctioned_limit,
        account.outstanding_balance,
        account.util_amount,
        account.schm_type,
        account.schm_code,
        account.beneficiary_name,
        account.issued_on_behalf_of,
        account.prd_cd,
        account.gl_code,
        account.country_code,
        account.credit_status_cd,
        account.devolvement_lc,
        account.devolvement_lc_amount,
        account.alm_cd,
        account.bsr_cd,
        account.rbi_cd,
        account.avg_book_balance,
        account.rediscount_flag,
        account.cgl_number,
        account.interest_rate,
        account.cash_margin,
        account.margin_amount,
        account.close_flag,
        account.true_cur,
        account.type_of_bill,
        account.cust_type,
        account.branch,
        account.acod,
        account.ref_type,
        account.as_on_date,
        account.a1,
        account.a2,
        account.a3,
        account.a4,
        account.a5,
        account.a6,
        account.a7,
        account.a8,
        account.a9,
        account.a10,
    )
}
