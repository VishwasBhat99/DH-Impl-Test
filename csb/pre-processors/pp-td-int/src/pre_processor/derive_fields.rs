use super::{macros, InputAccount, Logger};
pub use chrono::NaiveDateTime;
use chrono::{Datelike, NaiveDate};

pub fn get_op_line(
    acc: &mut InputAccount,
    as_on_date: NaiveDate,
    int_rt: &str,
    log: &Logger,
) -> String {
    let mut op_line = String::new();
    let mut uniq_acc_no = String::new();
    uniq_acc_no.push_str(&acc.acc_no);
    uniq_acc_no.push('-');
    uniq_acc_no.push_str(&acc.cntrct_num);
    acc.acc_no = uniq_acc_no;

    let acc_open_dt = get_date(&acc.acc_no, &acc.acc_open_dt, "account_open_date", log);
    let effc_dt = get_date(&acc.acc_no, &acc.effc_dt, "effective_date", log);
    let mat_dt = get_date(&acc.acc_no, &acc.mat_dt, "maturity_date", log);
    let lst_int_acr_dt = get_date(
        &acc.acc_no,
        &acc.lst_int_acr_dt,
        "last_interest_accrued_date",
        log,
    );

    let mut is_overdue: String = String::from("NO");
    let mut max_date = get_max_date(acc_open_dt, lst_int_acr_dt);
    let mut resid_days: i64 = num_days_start_to_end(max_date, as_on_date);
    if let Ok(over_dt) = NaiveDate::parse_from_str(&acc.over_dt, "%d-%m-%Y") {
        if over_dt <= as_on_date {
            is_overdue = String::from("YES");
            max_date = over_dt;
            resid_days = num_days_start_to_end(over_dt, as_on_date);
        }
    } else if max_date == acc_open_dt {
        resid_days = num_days_start_to_end(max_date, as_on_date) + 1;
    }

    acc.acc_open_dt = get_formatted_date(acc_open_dt);
    acc.effc_dt = get_formatted_date(effc_dt);
    acc.mat_dt = get_formatted_date(mat_dt);
    acc.as_on = get_formatted_date(get_date(&acc.acc_no, &acc.as_on, "as_on_date", log));

    op_line.push_str(&acc.print());

    op_line.push_str(&is_overdue);
    op_line.push('|');
    op_line.push_str(&get_formatted_date(max_date));
    op_line.push('|');
    op_line.push_str(&resid_days.to_string());
    op_line.push('|');
    op_line.push_str(int_rt);
    op_line.push('\n');

    op_line
}

fn get_date(acc_no: &str, date: &str, field_name: &str, log: &Logger) -> NaiveDate {
    match NaiveDate::parse_from_str(date, "%d-%m-%Y") {
        Ok(dt) => dt,
        Err(error) => {
            log_error!(
                log,
                "`{}` is not well-formatted as `DD-MM-YYYY` for account: `{}` : `{}`.",
                field_name,
                acc_no,
                error
            );
            NaiveDate::from_ymd(1970, 01, 01)
        }
    }
}

fn get_formatted_date(dt: NaiveDate) -> String {
    dt.format("%d-%m-%Y").to_string()
}

pub fn num_days_start_to_end(start: NaiveDate, end: NaiveDate) -> i64 {
    if start > end {
        return -1;
    }

    i64::from(end.num_days_from_ce() - start.num_days_from_ce())
}

pub fn get_max_date(first_date: NaiveDate, second_date: NaiveDate) -> NaiveDate {
    if first_date > second_date {
        first_date
    } else {
        second_date
    }
}
