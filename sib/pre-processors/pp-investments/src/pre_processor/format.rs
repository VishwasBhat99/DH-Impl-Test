use std::collections::HashMap;

use rbdate::NaiveDate;

#[derive(Debug, Clone)]
pub struct CashflowData {
    pub cashflow_type: String,
    pub cashflow_amount: f64,
    pub cashflow_currency: String,
    pub cf_date: NaiveDate,
}

#[derive(Debug, Clone)]
pub struct MappingMaster {
    pub group: String,
    pub llg: String,
    pub other_llg_classification: String,
}

pub fn get_cashflow_data(fields: Vec<&str>) -> CashflowData {
    CashflowData {
        cashflow_type: fields[2].to_string(),
        cashflow_amount: fields[3].parse::<f64>().unwrap_or(0.0),
        cashflow_currency: fields[4].to_string(),
        cf_date: NaiveDate::parse_from_str(fields[5], "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).expect("Error in parsing Date")),
    }
}

pub fn get_op_line(
    concat_id: &String,
    fields: &[&str],
    cf: &CashflowData,
    gl_code: String,
    mapping_val: MappingMaster,
    duration_map: &HashMap<(String, String), String>,
) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        NaiveDate::parse_from_str(fields[0], "%d-%m-%Y").unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).expect("Error in parsing Date")).format("%d-%m-%Y"),
        fields[1],
        fields[2],
        fields[3],
        concat_id,
        fields[4],
        fields[5],
        fields[6],
        fields[7],
        fields[8],
        fields[9],
        fields[10],
        fields[11],
        fields[12],
        fields[13],
        fields[14],
        fields[15],
        fields[16],
        NaiveDate::parse_from_str(fields[17], "%d-%m-%Y").unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).expect("Error in parsing Date")).format("%d-%m-%Y"),
        fields[18],
        fields[19],
        fields[20],
        fields[21],
        fields[22],
        fields[23],
        fields[24],
        NaiveDate::parse_from_str(fields[25], "%d-%m-%Y").unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).expect("Error in parsing Date")).format("%d-%m-%Y"),
        NaiveDate::parse_from_str(fields[26], "%d-%m-%Y").unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).expect("Error in parsing Date")).format("%d-%m-%Y"),
        NaiveDate::parse_from_str(fields[27], "%d-%m-%Y").unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).expect("Error in parsing Date")).format("%d-%m-%Y"),
        NaiveDate::parse_from_str(fields[28], "%d-%m-%Y").unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).expect("Error in parsing Date")).format("%d-%m-%Y"),
        NaiveDate::parse_from_str(fields[29], "%d-%m-%Y").unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).expect("Error in parsing Date")).format("%d-%m-%Y"),
        NaiveDate::parse_from_str(fields[30], "%d-%m-%Y").unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).expect("Error in parsing Date")).format("%d-%m-%Y"),
        NaiveDate::parse_from_str(fields[31], "%d-%m-%Y").unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).expect("Error in parsing Date")).format("%d-%m-%Y"),
        fields[32],
        fields[33],
        fields[34],
        fields[35],
        fields[36],
        fields[37],
        fields[38],
        fields[39],
        fields[40],
        fields[41],
        fields[42],
        fields[43],
        fields[44],
        fields[45],
        fields[46],
        NaiveDate::parse_from_str(fields[47], "%d-%m-%Y").unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).expect("Error in parsing Date")).format("%d-%m-%Y"),
        fields[48],
        duration_map.get(&(fields[7].trim().to_string(), fields[2].trim().to_string())).unwrap_or(&fields[49].to_string()),
        fields[50],
        fields[51],
        fields[52],
        fields[53],
        NaiveDate::parse_from_str(fields[54], "%d-%m-%Y").unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).expect("Error in parsing Date")).format("%d-%m-%Y"),
        fields[55],
        gl_code,
        mapping_val.group,
        mapping_val.llg,
        mapping_val.other_llg_classification,
        cf.cashflow_type,
        cf.cashflow_amount,
        cf.cashflow_currency,
        cf.cf_date.format("%d-%m-%Y"),
    )
}
