use crate::configuration_parameters::ConfigurationParameters;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputData {
    pub acc_no: String,
    pub as_on_dt: String,
    pub src: String,
    pub ccy: String,
    pub value_dt: String,
    pub prod_code: String,
    pub scheme_id: String,
    pub mis_1: String,
    pub mis_2: String,
    pub mis_3: String,
    pub raw_bm: String,
    pub final_bm: String,
    pub concat: String,
    pub npa_flag: String,
    pub division: String,
    pub alm_line: String,
    pub ia_line: String,
    pub org_tenor: String,
    pub alco_map: String,
    pub psl_code: String,
    pub custom1: String,
    pub rate_bucket: String,
    pub tot_amt: String,
    pub yield_rate: String,
}

pub fn format_output(output_record: OutputData) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        output_record.acc_no,
        output_record.as_on_dt,
        output_record.src,
        output_record.ccy,
        output_record.value_dt,
        output_record.prod_code,
        output_record.scheme_id,
        output_record.mis_1,
        output_record.mis_2,
        output_record.mis_3,
        output_record.raw_bm,
        output_record.final_bm,
        output_record.concat,
        output_record.npa_flag,
        output_record.division,
        output_record.alm_line,
        output_record.ia_line,
        output_record.org_tenor,
        output_record.alco_map,
        output_record.psl_code,
        output_record.custom1,
        output_record.rate_bucket,
        output_record.tot_amt,
        output_record.yield_rate,
    )
}
pub fn get_writer(file_path: &str) -> std::io::BufWriter<std::fs::File> {
    match sdb_io::buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!("Unable to create file `{}` due to: {}", file_path, error),
    }
}

impl OutputData {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        data: &[&str],
        row: usize,
    ) -> OutputData {
        OutputData {
            acc_no: get_str(input_file, data, 0, row),
            as_on_dt: get_str(input_file, data, 1, row),
            src: get_str(input_file, data, 2, row),
            ccy: get_str(input_file, data, 3, row),
            value_dt: get_str(input_file, data, 4, row),
            prod_code: get_str(input_file, data, 5, row),
            scheme_id: get_str(input_file, data, 6, row),
            mis_1: get_str(input_file, data, 7, row),
            mis_2: get_str(input_file, data, 8, row),
            mis_3: get_str(input_file, data, 9, row),
            raw_bm: get_str(input_file, data, 10, row),
            final_bm: get_str(input_file, data, 11, row),
            concat: get_str(input_file, data, 12, row),
            npa_flag: get_str(input_file, data, 13, row),
            division: get_str(input_file, data, 14, row),
            alm_line: get_str(input_file, data, 15, row),
            ia_line: get_str(input_file, data, 16, row),
            org_tenor: get_str(input_file, data, 17, row),
            alco_map: get_str(input_file, data, 18, row),
            psl_code: get_str(input_file, data, 19, row),
            custom1: get_str(input_file, data, 20, row),
            rate_bucket: get_str(input_file, data, 21, row),
            tot_amt: get_str(input_file, data, 22, row),
            yield_rate: get_str(input_file, data, 23, row),
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
