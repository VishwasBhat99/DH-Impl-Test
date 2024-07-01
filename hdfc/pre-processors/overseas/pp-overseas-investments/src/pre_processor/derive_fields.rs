use calamine::DataType;
use macros;
use rbdate::{date_from_timestamp, NaiveDate};
use slog::Logger;

pub fn get_op_line(rec: &[DataType], as_on_dt: NaiveDate, log: &Logger) -> String {
    log_debug!(log, "account: `{}`, alm_line: `{}`.", rec[5], rec[3],);
    get_line(rec, as_on_dt, log)
}

fn get_line(val: &[DataType], as_on_dt: NaiveDate, log: &Logger) -> String {
    let cal_put_dt: String = if val[10].to_string().parse::<f64>().is_err() {
        get_date(&val[10], "put/call date", &val[5], log)
    } else {
        datevalue_to_date(val[10].to_string())
    };

    let mat_dt: String = if val[9].to_string().parse::<f64>().is_err() {
        get_date(&val[9], "maturity_date", &val[5], log)
    } else {
        datevalue_to_date(val[9].to_string())
    };

    let lst_cpn_dt: String = if val[11].to_string().parse::<f64>().is_err() {
        get_date(&val[11], "last_coupon_date", &val[5], log)
    } else {
        datevalue_to_date(val[11].to_string())
    };

    let as_on_dt = as_on_dt.format("%d-%m-%Y").to_string();

    let mut output_line = String::new();
    output_line.push_str(&val[5].to_string());
    output_line.push('|');
    output_line.push_str(&val[4].to_string());
    output_line.push_str("||");
    output_line.push_str(&cal_put_dt);
    output_line.push('|');
    output_line.push_str(&cal_put_dt);
    output_line.push_str("||");
    output_line.push_str(&val[2].to_string());
    output_line.push('|');
    output_line.push_str(&val[7].to_string());
    output_line.push('|');
    output_line.push_str(&val[16].to_string());
    output_line.push('|');
    output_line.push_str(&val[25].to_string());
    output_line.push_str("||");
    output_line.push_str(&val[29].to_string());
    output_line.push_str("|||||");

    let os_cost_val = val[28].to_string().parse::<f64>().unwrap_or(0.0)
        / val[27].to_string().parse::<f64>().unwrap_or(1.0);

    output_line.push_str(&os_cost_val.to_string());
    output_line.push_str("||");
    output_line.push_str(&val[7].to_string());
    output_line.push('|');
    output_line.push_str(&val[12].to_string());
    output_line.push_str("||");
    output_line.push_str(&mat_dt);
    output_line.push_str("||");
    output_line.push_str(&val[1].to_string());
    output_line.push_str("||");
    output_line.push_str(&lst_cpn_dt);
    output_line.push_str("|||||||");
    output_line.push_str(&val[6].to_string());
    output_line.push_str("||||");
    output_line.push_str(&val[3].to_string());
    output_line.push_str("|||||||||");
    output_line.push_str(&val[7].to_string());
    output_line.push('|');
    output_line.push_str(&as_on_dt);
    output_line.push_str("\n");
    output_line
}

fn datevalue_to_date(date: String) -> String {
    match date.parse::<f64>() {
        Ok(timestamp) => date_from_timestamp(((timestamp as i64) - 25569) * 86400)
            .format("%d-%m-%Y")
            .to_string(),
        Err(_) => "".to_string(),
    }
}

fn get_date(val: &DataType, name: &str, acc: &DataType, log: &Logger) -> String {
    if let Ok(dt) = NaiveDate::parse_from_str(&val.to_string(), "%d-%b-%Y") {
        dt.format("%d-%m-%Y").to_string()
    } else {
        log_error!(
            log,
            "`{}`: `{}` is not well-formatted for account: `{}`, expected format: `DD-MMM-YYYY`.",
            name,
            val,
            acc
        );
        "".to_string()
    }
}
