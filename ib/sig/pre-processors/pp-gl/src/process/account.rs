use crate::configuration_parameters::ConfigurationParameters;

#[derive(Debug, Clone, Default)]
pub struct Account {
    pub rpt_dt: String,
    pub gl_acct_id: String,
    pub cust_id: String,
    pub gl_acct_desc: String,
    pub currency_cd: String,
    pub gl_acct_val_amt: f64,
    pub gl_acct_val_amt_sgd: f64,
    pub gl_acct_type_cd: String,
    pub value_date: String,
    pub mat_date: String,
    pub brca: String,
    pub acct_suffix: String,
    pub cust_type: String,
    pub acct_type: String,

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
        let date_parser = rbdate::DateParser::new("%d/%m/%Y".to_string(), false);
        Account {
            rpt_dt: input_acc[0].to_string(),
            gl_acct_id: input_acc[1].to_string(),
            cust_id: input_acc[2].to_string(),
            gl_acct_desc: input_acc[3].to_string(),
            currency_cd: input_acc[4].to_string(),
            gl_acct_val_amt: input_acc[5].to_string().parse::<f64>().unwrap_or(0.0),
            gl_acct_val_amt_sgd: input_acc[6].to_string().parse::<f64>().unwrap_or(0.0),
            gl_acct_type_cd: input_acc[7].to_string(),
            value_date: date_parser
                .parse_opt(input_acc[8])
                .unwrap_or(*config_params.as_on_date())
                .format("%d-%m-%Y")
                .to_string(),
            mat_date: date_parser
                .parse_opt(input_acc[9])
                .unwrap_or(*config_params.as_on_date())
                .format("%d-%m-%Y")
                .to_string(),
            brca: input_acc[10].to_string(),
            acct_suffix: input_acc[11].to_string(),
            cust_type: input_acc[12].to_string(),
            acct_type: input_acc[13].to_string(),
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
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        account.rpt_dt,
        account.gl_acct_id,
        account.cust_id,
        account.gl_acct_desc,
        account.currency_cd,
        account.gl_acct_val_amt,
        account.gl_acct_val_amt_sgd,
        account.gl_acct_type_cd,
        account.value_date,
        account.mat_date,
        account.brca,
        account.acct_suffix,
        account.cust_type,
        account.acct_type,
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
