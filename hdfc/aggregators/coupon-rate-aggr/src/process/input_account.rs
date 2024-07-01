use crate::configuration_parameters::ConfigurationParameters;
use calamine::DataType;
use rbdate::NaiveDate;

#[derive(Debug, Clone, Default)]
pub struct CouponData {
    pub llg_id: String,
    pub llg_desc: String,
    pub ccy: String,
    pub coupon_rate: f64,
}

impl CouponData {
    pub fn new_from_excel(master_data: &[DataType]) -> CouponData {
        CouponData {
            llg_id: get_str_from_excel(master_data, 0),
            llg_desc: get_str_from_excel(master_data, 1),
            ccy: get_str_from_excel(master_data, 2),
            coupon_rate: get_str_from_excel(master_data, 3)
                .parse::<f64>()
                .unwrap_or(0.0),
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct InputData {
    pub llg_id: String,
    pub as_on_date: NaiveDate,
    pub ccy: String,
    pub sls_irs: String,
    pub source: String,
    pub flow_type: String,
    pub amt: f64,
    pub coupon_rate: f64,
}

impl InputData {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        input_acc: &[&str],
        row: usize,
    ) -> InputData {
        InputData {
            llg_id: get_str(input_file, input_acc, 0, row),
            as_on_date: get_date(config_params, input_file, input_acc, 1, row),
            ccy: get_str(input_file, input_acc, 2, row),
            sls_irs: get_str(input_file, input_acc, 3, row),
            source: get_str(input_file, input_acc, 4, row),
            flow_type: get_str(input_file, input_acc, 5, row),
            amt: get_str(input_file, input_acc, 6, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            coupon_rate: get_str(input_file, input_acc, 7, row)
                .parse::<f64>()
                .unwrap_or(0.0),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ExchangeRate {
    pub from_ccy: String,
    pub to_ccy: String,
    pub val: f64,
}

impl ExchangeRate {
    pub fn new(input_file: &str, input_acc: &[&str], row: usize) -> ExchangeRate {
        ExchangeRate {
            from_ccy: get_str(input_file, input_acc, 0, row),
            to_ccy: get_str(input_file, input_acc, 1, row),
            val: get_str(input_file, input_acc, 2, row)
                .parse::<f64>()
                .unwrap_or(0.0),
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct AmountData {
    pub amt: f64,
    pub rate_amt: f64,
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
pub fn get_str_from_excel(data: &[DataType], index: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` for row: `{:?}`",
                index + 1,
                data
            )
        })
        .to_string()
        .replace('\n', " ")
        .replace('\r', " ")
        .trim()
        .to_string()
}
