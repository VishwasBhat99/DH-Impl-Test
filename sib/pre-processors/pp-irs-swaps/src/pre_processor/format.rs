use rbdate::NaiveDate;

#[derive(Debug, Clone)]
pub struct CashflowData {
    pub as_on_date: NaiveDate,
    pub deal_number: String,
    pub cf_type: String,
    pub cf_sub_type: String,
    pub cf_amount: f64,
    pub currency: String,
    pub cf_date: NaiveDate,
}
#[derive(Debug, Clone)]
pub struct InputData {
    pub as_on_date: NaiveDate,
    pub deal_ref: String,
    pub branch_code: i64,
    pub deal_date: NaiveDate,
    pub pay_recv: String,
    pub cparty: String,
    pub curr: String,
    pub deal_amt: f64,
    pub int_rate: f64,
    pub benchmark_code: String,
    pub int_freq: String,
    pub int_basis: String,
    pub due_date: NaiveDate,
    pub reval_date: NaiveDate,
    pub profit_and_loss_amt: f64,
    pub treasury_gl_code: i64,
}

pub fn get_cashflow_data(fields: Vec<&str>) -> CashflowData {
    CashflowData {
        as_on_date: NaiveDate::parse_from_str(fields[0], "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).expect("Error in parsing NaiveDate")),
        deal_number: fields[1].to_string(),
        cf_type: fields[2].to_string(),
        cf_sub_type: fields[3].to_string(),
        cf_amount: fields[4].parse::<f64>().unwrap_or(0.0),
        currency: fields[5].to_string(),
        cf_date: NaiveDate::parse_from_str(fields[6], "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).expect("Error in parsing NaiveDate")),
    }
}
pub fn get_input_data(fields: Vec<&str>) -> InputData {
    InputData {
        as_on_date: NaiveDate::parse_from_str(&get_input_data_field(&fields, 0), "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).expect("Error in parsing NaiveDate")),
        deal_ref: get_input_data_field(&fields, 1),
        branch_code: get_input_data_field(&fields, 2).parse::<i64>().unwrap_or(0),
        deal_date: NaiveDate::parse_from_str(&get_input_data_field(&fields, 3), "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).expect("Error in parsing NaiveDate")),
        pay_recv: get_input_data_field(&fields, 4),
        cparty: get_input_data_field(&fields, 5),
        curr: get_input_data_field(&fields, 6),
        deal_amt: get_input_data_field(&fields, 7)
            .parse::<f64>()
            .unwrap_or(0.0),
        int_rate: get_input_data_field(&fields, 8)
            .parse::<f64>()
            .unwrap_or(0.0),
        benchmark_code: get_input_data_field(&fields, 9),
        int_freq: get_input_data_field(&fields, 10),
        int_basis: get_input_data_field(&fields, 11),
        due_date: NaiveDate::parse_from_str(&get_input_data_field(&fields, 12), "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).expect("Error in parsing NaiveDate")),
        reval_date: NaiveDate::parse_from_str(&get_input_data_field(&fields, 13), "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).expect("Error in parsing NaiveDate")),
        profit_and_loss_amt: get_input_data_field(&fields, 14)
            .parse::<f64>()
            .unwrap_or(0.0),
        treasury_gl_code: get_input_data_field(&fields, 15)
            .parse::<i64>()
            .unwrap_or(0),
    }
}

fn get_input_data_field(data: &[&str], index: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "could not get data at column-no : {} for row :{:?}",
                index, data
            )
        })
        .to_string()
}

pub fn get_op_line(account: InputData, cf: &CashflowData) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        account.as_on_date.format("%d-%m-%Y"),
        account.deal_ref,
        account.branch_code,
        account.deal_date.format("%d-%m-%Y"),
        account.pay_recv,
        account.cparty,
        account.curr,
        account.deal_amt,
        account.int_rate,
        account.benchmark_code,
        account.int_freq,
        account.int_basis,
        account.due_date.format("%d-%m-%Y"),
        account.reval_date.format("%d-%m-%Y"),
        account.profit_and_loss_amt,
        account.treasury_gl_code,
        cf.cf_type,
        cf.cf_sub_type,
        cf.cf_amount,
        cf.currency,
        cf.cf_date.format("%d-%m-%Y"),
    )
}
