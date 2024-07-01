use crate::configuration_parameters::ConfigurationParameters;
use calamine::DataType;
use rbdate::NaiveDate;

#[derive(Debug, Clone, Default)]
pub struct InputData {
    pub loanid: String,
    pub source_system_name: String,
    pub cust_name: String,
    pub maturity_date: NaiveDate,
    pub npa_date: NaiveDate,
    pub loanrepayment_frequency: String,
    pub currency: String,
    pub intereststartdate: NaiveDate,
    pub tenure: i64,
    pub noofinstallment: i64,
    pub balancetenure: i64,
    pub loaninterestratetype: String,
    pub emipaid: i64,
    pub interestrate: f64,
    pub totalinstalmentoverdue: f64,
    pub nextinstallmentduedate: NaiveDate,
    pub firstemidate: NaiveDate,
    pub lastemidate: NaiveDate,
    pub totalinterestaccrued: f64,
    pub originalemiamount: f64,
    pub overdueprincipal: f64,
    pub principalcomponent: f64,
    pub customertype: String,
    pub sourcesystemcustomerid: String,
    pub productname: String,
    pub productcode: String,
    pub glclasscode: String,
    pub disbursedloanamount: f64,
    pub businessdate: NaiveDate,
    pub de: i64,
}

impl InputData {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        data: &[&str],
        row: usize,
    ) -> InputData {
        InputData {
            loanid: get_str(input_file, data, 0, row),
            source_system_name: get_str(input_file, data, 1, row),
            cust_name: get_str(input_file, data, 2, row),
            maturity_date: get_date(config_params, input_file, data, 3, row),
            npa_date: get_date(config_params, input_file, data, 4, row),
            loanrepayment_frequency: get_str(input_file, data, 5, row),
            currency: get_str(input_file, data, 6, row),
            intereststartdate: get_date(config_params, input_file, data, 7, row),
            tenure: get_int(input_file, data, 8, row),
            noofinstallment: get_int(input_file, data, 9, row),
            balancetenure: get_int(input_file, data, 10, row),
            loaninterestratetype: get_str(input_file, data, 11, row),
            emipaid: get_int(input_file, data, 12, row),
            interestrate: get_float(input_file, data, 13, row),
            totalinstalmentoverdue: get_float(input_file, data, 14, row),
            nextinstallmentduedate: get_date(config_params, input_file, data, 15, row),
            firstemidate: get_date(config_params, input_file, data, 16, row),
            lastemidate: get_date(config_params, input_file, data, 17, row),
            totalinterestaccrued: get_float(input_file, data, 18, row),
            originalemiamount: get_float(input_file, data, 19, row),
            overdueprincipal: get_float(input_file, data, 20, row),
            principalcomponent: get_float(input_file, data, 21, row),
            customertype: get_str(input_file, data, 22, row),
            sourcesystemcustomerid: get_str(input_file, data, 23, row),
            productname: get_str(input_file, data, 24, row),
            productcode: get_str(input_file, data, 25, row),
            glclasscode: get_str(input_file, data, 26, row),
            disbursedloanamount: get_float(input_file, data, 27, row),
            businessdate: get_date(config_params, input_file, data, 28, row),
            de: get_int(input_file, data, 29, row),
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct FinnoneCashflows {
    pub loan_id: String,
    pub repayment_date: NaiveDate,
    pub principal_amt: f64,
    pub intrest_amt: f64,
}
impl FinnoneCashflows {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        data: &[&str],
        row: usize,
    ) -> FinnoneCashflows {
        FinnoneCashflows {
            loan_id: get_str(input_file, data, 3, row),
            repayment_date: get_date(config_params, input_file, data, 10, row),
            principal_amt: get_float(input_file, data, 13, row),
            intrest_amt: get_float(input_file, data, 14, row),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TcfslNpa {
    pub acct_no: String,
    pub asset_classification: String,
    pub schm_code: String,
}

impl TcfslNpa {
    pub fn new_from_excel(data: &[DataType]) -> TcfslNpa {
        TcfslNpa {
            acct_no: get_str_from_xlsx(data, 0),
            asset_classification: get_str_from_xlsx(data, 50),
            schm_code: get_str_from_xlsx(data, 12),
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct WriteOffMerged {
    pub source_system: String,
    pub asset_class: String,
    pub loan_id: String,
}

impl WriteOffMerged {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        data: &[&str],
        row: usize,
    ) -> WriteOffMerged {
        WriteOffMerged {
            source_system: get_str(input_file, data, 0, row),
            asset_class: get_str(input_file, data, 1, row),
            loan_id: get_str(input_file, data, 2, row),
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
    let date_parser = rbdate::DateParser::new("%Y-%m-%d %H:%M:%S".to_string(), false);
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
                .replace(".0000000", ""),
        )
        .unwrap_or(*config_params.as_on_date())
}
pub fn get_float(input_file: &str, data: &[&str], index: usize, row: usize) -> f64 {
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
        .parse::<f64>()
        .unwrap_or(0.0)
}
pub fn get_int(input_file: &str, data: &[&str], index: usize, row: usize) -> i64 {
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
        .parse::<i64>()
        .unwrap_or(0)
}
pub fn get_str_from_xlsx(data: &[DataType], index: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` for row: `{:?}`",
                index + 1,
                data
            )
        })
        .to_string()
        .replace("\n", " ")
        .trim()
        .to_string()
}
