use crate::configuration_parameters::ConfigurationParameters;

#[derive(Debug, Clone, Default)]
pub struct Account {
    pub rpt_dt: String,
    pub branch: String,
    pub counter_party: String,
    pub deal_type: String,
    pub deal_ref: String,
    pub major_ccy: String,
    pub rate: f64,
    pub premium_disc: f64,
    pub ccy: String,
    pub ccy_amt: f64,
    pub ccy1: String,
    pub ccy1_amt: f64,
    pub amt: f64,
    pub amt_in_sgd: f64,
    pub deal_date: String,
    pub value_date: String,
    pub ccy_pair: String,
    pub cust_type: String,
    pub port: String,
    pub gl_code: String,
    pub cust: String,
    pub purchase_acct_suffix: String,
    pub sales_acct_suffix: String,

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
        let date_parser =
            rbdate::DateParser::new(config_params.input_date_format().to_string(), false);
        Account {
            rpt_dt: date_parser
                .parse_opt(input_acc[0])
                .unwrap_or(*config_params.as_on_date())
                .format("%d-%m-%Y")
                .to_string(),
            branch: input_acc[1].to_string(),
            counter_party: input_acc[2].to_string(),
            deal_type: input_acc[3].to_string(),
            deal_ref: input_acc[4].to_string(),
            major_ccy: input_acc[5].to_string(),
            rate: input_acc[6].to_string().parse::<f64>().unwrap_or(0.0),
            premium_disc: input_acc[7].to_string().parse::<f64>().unwrap_or(0.0),
            ccy: input_acc[8].to_string(),
            ccy_amt: input_acc[9].to_string().parse::<f64>().unwrap_or(0.0),
            ccy1: input_acc[10].to_string(),
            ccy1_amt: input_acc[11].to_string().parse::<f64>().unwrap_or(0.0),
            amt: input_acc[12].to_string().parse::<f64>().unwrap_or(0.0),
            amt_in_sgd: input_acc[13].to_string().parse::<f64>().unwrap_or(0.0),
            deal_date: date_parser
                .parse_opt(input_acc[14])
                .unwrap_or(*config_params.as_on_date())
                .format("%d-%m-%Y")
                .to_string(),
            value_date: date_parser
                .parse_opt(input_acc[15])
                .unwrap_or(*config_params.as_on_date())
                .format("%d-%m-%Y")
                .to_string(),
            ccy_pair: input_acc[16].to_string(),
            cust_type: input_acc[17].to_string(),
            port: input_acc[18].to_string(),
            gl_code: input_acc[19].to_string(),
            cust: input_acc[20].to_string(),
            purchase_acct_suffix: input_acc[21].to_string(),
            sales_acct_suffix: input_acc[22].to_string(),
            as_on_date: config_params.as_on_date().format("%d-%m-%Y").to_string(),
            a1: input_acc[11].to_string().parse::<f64>().unwrap_or(0.0),
            a2: input_acc[9].to_string().parse::<f64>().unwrap_or(0.0),
            a3: 0.0,
            a4: 0.0,
            a5: config_params.as_on_date().format("%d-%m-%Y").to_string(),
            a6: config_params.as_on_date().format("%d-%m-%Y").to_string(),
            a7: input_acc[10].to_string(),
            a8: input_acc[8].to_string(),
            a9: "NA".to_string(),
            a10: "NA".to_string(),
        }
    }
}

pub fn format_output(account: &Account) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        account.rpt_dt,
        account.branch,
        account.counter_party,
        account.deal_type,
        account.deal_ref,
        account.major_ccy,
        account.rate,
        account.premium_disc,
        account.ccy,
        account.ccy_amt,
        account.ccy1,
        account.ccy1_amt,
        account.amt,
        account.amt_in_sgd,
        account.deal_date,
        account.value_date,
        account.ccy_pair,
        account.cust_type,
        account.port,
        account.gl_code,
        account.cust,
        account.purchase_acct_suffix,
        account.sales_acct_suffix,
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
