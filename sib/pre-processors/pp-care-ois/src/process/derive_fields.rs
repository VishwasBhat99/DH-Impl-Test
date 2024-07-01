use super::structs::TimeBandData;
use chrono::NaiveDate;

pub fn get_op_line(
    fields: Vec<&str>,
    res_tenor: i64,
    cashflow_date: NaiveDate,
    value_date: NaiveDate,
    time_band_val: TimeBandData,
) -> String {
    let (src_cd, position) = match fields[5].to_uppercase().as_str() {
        "PAY" => ("OIS-PAY", "Short"),
        "RECEIVE" => ("OIS-RECEIVE", "Long"),
        _ => ("NA", "NA"),
    };
    let isin = format!("{}_{}", fields[1], fields[5]);
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|0|0|{}|{}|{}|{}|{}\n",
        fields[1],
        value_date.format("%d-%m-%Y"),
        src_cd,
        cashflow_date.format("%d-%m-%Y"),
        fields[8].parse::<f64>().unwrap_or(0.0),
        fields[6].parse::<f64>().unwrap_or(0.0),
        fields[7],
        isin,
        fields[9].parse::<f64>().unwrap_or(0.0),
        fields[10].parse::<f64>().unwrap_or(0.0),
        fields[11].parse::<f64>().unwrap_or(0.0),
        res_tenor as f64,
        fields[12].parse::<f64>().unwrap_or(0.0),
        time_band_val.timeband,
        time_band_val.period,
        time_band_val.zone,
        position,
    )
}
