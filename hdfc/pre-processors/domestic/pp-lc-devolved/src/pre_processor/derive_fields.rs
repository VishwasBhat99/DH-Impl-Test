use super::manual_handler::remove_comma;
use super::{Records, RecordsWithBalance};
use calamine::DataType;
use rbdate::{datevalue_to_naive_date, NaiveDate};

pub fn get_op_line(val: &[DataType], data: &mut RecordsWithBalance) -> f64 {
    let amt = remove_comma(&val[10].to_string());
    let dev_amt = amt.parse::<f64>().unwrap_or(0.0);
    let dev_dt: NaiveDate = match datevalue_to_naive_date(&val[9].to_string()) {
        Ok(dt) => dt,
        Err(error) => panic!(
            "Error while getting `devolvement date` for account: `{}`: `{}`.",
            val[1], error
        ),
    };

    let mut rec: Records = Records::new();
    rec.insert(val[2].to_string(), dev_dt);

    data.store
        .entry(rec)
        .and_modify(|amt| *amt += dev_amt)
        .or_insert(dev_amt);
    dev_amt
}
