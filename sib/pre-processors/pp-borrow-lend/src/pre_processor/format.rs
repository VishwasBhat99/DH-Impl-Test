use rbdate::NaiveDate;

#[derive(Debug, Clone)]
pub struct CashflowData {
    pub cf_type: String,
    pub cf_sub_type: String,
    pub cf_amount: f64,
    pub currency: String,
    pub cf_date: NaiveDate,
}

#[derive(Debug, Clone)]
pub struct MappingMaster {
    pub c_d: String,
    pub group: String,
    pub llg: String,
    pub other_llg_classification: String,
}

pub fn get_cashflow_data(fields: &[&str]) -> CashflowData {
    CashflowData {
        cf_type: fields[2].to_string(),
        cf_sub_type: fields[3].to_string(),
        cf_amount: fields[4].parse::<f64>().unwrap_or(0.0),
        currency: fields[5].to_string(),
        cf_date: NaiveDate::parse_from_str(fields[6], "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd(1900, 1, 1)),
    }
}

pub fn get_op_line(
    fields: &[&str],
    cf: &CashflowData,
    gl_code: String,
    mapping_val: MappingMaster,
) -> String {
    let ost_amt = fields[4].parse::<f64>().unwrap_or(0.0);
    let oustanding_diff =if fields[2]=="B" && fields[6].to_uppercase()=="TRI PARTY REPO"{
        ost_amt.to_owned() - fields[26].parse::<f64>().unwrap_or(0.0)
    }else{
        ost_amt
    };
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        NaiveDate::parse_from_str(fields[0], "%d-%m-%Y").unwrap_or(NaiveDate::from_ymd(1900, 1, 1)).format("%d-%m-%Y"),
        fields[1],
        fields[2],
        fields[3],
        ost_amt,
        fields[5],
        fields[6],
        fields[7],
        fields[8],
        fields[9],
        NaiveDate::parse_from_str(fields[10], "%d-%m-%Y").unwrap_or(NaiveDate::from_ymd(1900, 1, 1)).format("%d-%m-%Y"),
        NaiveDate::parse_from_str(fields[11], "%d-%m-%Y").unwrap_or(NaiveDate::from_ymd(1900, 1, 1)).format("%d-%m-%Y"),
        fields[12],
        fields[13],
        fields[14],
        fields[15],
        NaiveDate::parse_from_str(fields[16], "%d-%m-%Y").unwrap_or(NaiveDate::from_ymd(1900, 1, 1)).format("%d-%m-%Y"),
        NaiveDate::parse_from_str(fields[17], "%d-%m-%Y").unwrap_or(NaiveDate::from_ymd(1900, 1, 1)).format("%d-%m-%Y"),
        fields[18],
        NaiveDate::parse_from_str(fields[19], "%d-%m-%Y").unwrap_or(NaiveDate::from_ymd(1900, 1, 1)).format("%d-%m-%Y"),
        fields[20],
        fields[21],
        fields[22],
        fields[23],
        fields[24],
        gl_code,
        fields[26],
        fields[27],
        fields[28],
        fields[29],
        fields[30],
        fields[31],
        cf.cf_type,
        cf.cf_sub_type,
        cf.cf_amount,
        cf.cf_date.format("%d-%m-%Y"),
        cf.currency,
        oustanding_diff,
        mapping_val.group,
        mapping_val.llg,
        mapping_val.other_llg_classification,
    )
}
