use crate::configuration_parameters::{self, ConfigurationParameters};
use rbdate::NaiveDate;

#[derive(Debug, Clone, Default)]
pub struct AdfFxdeals {
    pub as_on_date: NaiveDate,
    pub deal_number: String,
    pub deal_date: String,
    pub product_type: String,
    pub deal_ref: String,
    pub transaction_type: String,
    pub portfolio: String,
    pub counter_party: String,
    pub counterparty_category: String,
    pub internal_external_deal_type: String,
    pub maturity_date: NaiveDate,
    pub crncy1: String,
    pub crncy2: String,
    pub deal_rate: String,
    pub crncy1_amt: String,
    pub crncy2_amt: String,
    pub reval_rate: String,
    pub reval_profit: String,
    pub reval_loss: String,
    pub profit_and_loss_amount: String,
    pub m_duration: String,
    pub treasury_gl_code: String,
}

impl AdfFxdeals {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        input_acc: &[&str],
        row: usize,
    ) -> AdfFxdeals {
        AdfFxdeals {
            as_on_date: get_date(config_params, input_file, input_acc, 0, row),
            deal_number: get_str(input_file, input_acc, 1, row),
            deal_date: get_str(input_file, input_acc, 2, row),
            product_type: get_str(input_file, input_acc, 3, row),
            deal_ref: get_str(input_file, input_acc, 4, row),
            transaction_type: get_str(input_file, input_acc, 5, row),
            portfolio: get_str(input_file, input_acc, 6, row),
            counter_party: get_str(input_file, input_acc, 7, row),
            counterparty_category: get_str(input_file, input_acc, 8, row),
            internal_external_deal_type: get_str(input_file, input_acc, 9, row),
            maturity_date: get_date(config_params, input_file, input_acc, 10, row),
            crncy1: get_str(input_file, input_acc, 11, row),
            crncy2: get_str(input_file, input_acc, 12, row),
            deal_rate: get_str(input_file, input_acc, 13, row),
            crncy1_amt: get_str(input_file, input_acc, 14, row),
            crncy2_amt: get_str(input_file, input_acc, 15, row),
            reval_rate: get_str(input_file, input_acc, 16, row),
            reval_profit: get_str(input_file, input_acc, 17, row),
            reval_loss: get_str(input_file, input_acc, 18, row),
            profit_and_loss_amount: get_str(input_file, input_acc, 19, row),
            m_duration: get_str(input_file, input_acc, 20, row),
            treasury_gl_code: get_str(input_file, input_acc, 21, row),
        }
    }
}

pub fn get_str(input_file: &str, data: &[&str], index: usize, row: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` in row-no: `{:?}` from File: {}",
                index + 1,
                row,
                input_file,
            )
        })
        .trim()
        .trim_matches(|pat| pat == ' ' || pat == '"')
        .to_string()
}
pub fn get_date(
    config_params: &ConfigurationParameters,
    input_file: &str,
    data: &[&str],
    index: usize,
    row: usize,
) -> NaiveDate {
    let date_parser = rbdate::DateParser::new("%d-%m-%Y".to_string(), false);
    date_parser
        .parse_opt(
            &data
                .get(index)
                .unwrap_or_else(|| {
                    panic!(
                        "Could not get data at column-no: `{}` in row-no: `{:?}` from File: {}",
                        index + 1,
                        row,
                        input_file,
                    )
                })
                .replace('.', ""),
        )
        .unwrap_or(*config_params.as_on_date())
}
