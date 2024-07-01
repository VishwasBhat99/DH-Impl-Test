use crate::configuration_parameters::ConfigurationParameters;
use rbdate::NaiveDate;

#[derive(Debug, Clone, Default)]
pub struct MappingMaster {
    pub gl_account_number: String,
    pub description: String,
    pub classification: String,
    pub group: String,
    pub llg: String,
    pub other_llg_classification: String,
    pub logic: String,
}

impl MappingMaster {
    pub fn new(input_file: &str, input_acc: &[&str], row: usize) -> MappingMaster {
        MappingMaster {
            gl_account_number: get_str(input_file, input_acc, 0, row),
            description: get_str(input_file, input_acc, 1, row),
            classification: get_str(input_file, input_acc, 2, row),
            group: get_str(input_file, input_acc, 3, row),
            llg: get_str(input_file, input_acc, 4, row),
            other_llg_classification: get_str(input_file, input_acc, 5, row),
            logic: get_str(input_file, input_acc, 6, row),
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct GsttData {
    pub sol_id: String,
    pub gl_sub_head: String,
    pub crncy_code: String,
    pub tot_cr_bal: f64,
    pub tot_dr_bal: f64,
}

impl GsttData {
    pub fn new(input_file: &str, input_acc: &[&str], row: usize) -> GsttData {
        GsttData {
            sol_id: get_str(input_file, input_acc, 0, row),
            gl_sub_head: get_str(input_file, input_acc, 1, row),
            crncy_code: get_str(input_file, input_acc, 2, row),
            tot_cr_bal: get_str(input_file, input_acc, 3, row)
                .parse::<f64>()
                .unwrap_or(0.0),
            tot_dr_bal: get_str(input_file, input_acc, 4, row)
                .parse::<f64>()
                .unwrap_or(0.0),
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct ReconData {
    pub as_on_date: NaiveDate,
    pub source_name: String,
    pub llg: String,
    pub gl_code: String,
    pub ccy: String,
    pub lcy_aggr_amt: f64,
    pub hcy_aggr_amt: f64,
}

impl ReconData {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        input_acc: &[&str],
        row: usize,
    ) -> ReconData {
        let lcy_aggr_amt = get_str(input_file, input_acc, 5, row)
            .parse::<f64>()
            .unwrap_or(0.0);
        let hcy_aggr_amt = get_str(input_file, input_acc, 6, row)
            .parse::<f64>()
            .unwrap_or(0.0);
        let as_on_date = get_date(config_params, input_file, input_acc, 0, row);
        ReconData {
            as_on_date,
            source_name: get_str(input_file, input_acc, 1, row),
            llg: get_str(input_file, input_acc, 2, row),
            gl_code: get_str(input_file, input_acc, 3, row),
            ccy: get_str(input_file, input_acc, 4, row),
            lcy_aggr_amt,
            hcy_aggr_amt,
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
