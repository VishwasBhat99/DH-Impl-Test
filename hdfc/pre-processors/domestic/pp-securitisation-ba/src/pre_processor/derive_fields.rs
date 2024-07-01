use super::{Balance, BalanceWithDate};
use calamine::DataType;
use rbdate::{date_from_timestamp, NaiveDate};

pub fn get_op_line(rec: &[DataType], data: &mut BalanceWithDate) -> f64 {
    let mut tot_bal: f64 = 0.0;
    for index in (0..90).step_by(3) {
        let op_bal = rec[index + 1].to_string().parse::<f64>().unwrap_or(0.0);
        let cl_bal = rec[index + 2].to_string().parse::<f64>().unwrap_or(0.0);
        tot_bal += op_bal;
        tot_bal += cl_bal;
        let mut def_bal = Balance::new();
        def_bal.insert(op_bal, cl_bal);
        data.store
            .entry(datevalue_to_date(rec[index].to_string()))
            .and_modify(|amts| amts.add_amts(op_bal, cl_bal))
            .or_insert(def_bal);
    }
    tot_bal
}

fn datevalue_to_date(date: String) -> NaiveDate {
    if let Ok(timestamp) = date.parse::<f64>() {
        date_from_timestamp(((timestamp as i64) - 25569) * 86400)
    } else {
        NaiveDate::from_ymd(1970, 01, 01)
    }
}
