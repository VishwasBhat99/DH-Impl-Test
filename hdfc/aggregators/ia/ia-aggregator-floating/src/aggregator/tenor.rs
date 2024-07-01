use chrono::Duration;
use chrono::{Datelike, NaiveDate, Weekday};
use rbdate::decr_dt_by_mon_presrv_eom;
use rbdate::increment_date_by_months;
use rbdate::NaiveDateTime;
pub fn get_tenor(file_path: &String, start_date: NaiveDateTime, end_date: NaiveDateTime) -> String {
    let mut yr: i64 = 0;
    let mut month: i64 = 0;
    let mut days: i64 = 0;
    let mut start = start_date.date();
    let end = end_date.date();

    let mut tenor_file: Vec<String> = Vec::new();
    for line in file_path.lines() {
        let each_line: Vec<&str> = line.split("|").collect();

        let col1_read = each_line[0];
        let col2_read = each_line[1];
        let mut col1_string = col1_read.to_string();
        let mut col2_string = col2_read.to_string();
        let tenor_value = each_line[2];
        let yr_str = "0Y";
        let m_str = "0M";
        let d_str = "0D";
        if col1_string.contains("D") && !col1_string.contains("M") && !col1_string.contains("Y") {
            col1_string = yr_str.to_string() + &m_str.to_string() + &col1_string;
        }
        if col2_string.contains("D") && !col2_string.contains("M") && !col2_string.contains("Y") {
            col2_string = yr_str.to_string() + &m_str.to_string() + &col2_string;
        }

        if col1_string.contains("M") && !col1_string.contains("Y") && !col1_string.contains("D") {
            col1_string = yr_str.to_string() + &col1_string + &d_str.to_string();
        }
        if col2_string.contains("M") && !col2_string.contains("Y") && !col2_string.contains("D") {
            col2_string = yr_str.to_string() + &col2_string + &d_str.to_string();
        }

        if col1_string.contains("D") && col1_string.contains("M") && !col1_string.contains("Y") {
            col1_string = yr_str.to_string() + &col1_string;
        }
        if col2_string.contains("D") && col2_string.contains("M") && !col2_string.contains("Y") {
            col2_string = yr_str.to_string() + &col2_string;
        }

        if !col1_string.contains("D") && !col1_string.contains("M") && col1_string.contains("Y") {
            col1_string = col1_string + &m_str.to_string() + &d_str.to_string();
        }
        if !col2_string.contains("D") && !col2_string.contains("M") && col2_string.contains("Y") {
            col2_string = col2_string + &m_str.to_string() + &d_str.to_string();
        }

        if col1_string.contains("D") && !col1_string.contains("M") && col1_string.contains("Y") {
            let split1: Vec<&str> = col1_string.split("Y").collect();

            col1_string = split1[0].to_string()
                + &"Y".to_string()
                + &m_str.to_string()
                + &split1[1].to_string();
        }
        if col2_string.contains("D") && !col2_string.contains("M") && col2_string.contains("Y") {
            let split2: Vec<&str> = col2_string.split("Y").collect();
            col2_string = split2[0].to_string()
                + &"Y".to_string()
                + &m_str.to_string()
                + &split2[1].to_string();
        }

        if !col1_string.contains("D") && col1_string.contains("M") && col1_string.contains("Y") {
            col1_string = col1_string + &d_str.to_string();
        }
        if !col2_string.contains("D") && col2_string.contains("M") && col2_string.contains("Y") {
            col2_string = col2_string + &d_str.to_string();
        }
        let concat = col1_string.to_string()
            + "|"
            + &col2_string.to_string()
            + "|"
            + &tenor_value.to_string();

        tenor_file.push(concat);
    }
    let mut change_value: i64 = 0;
    for file_line in tenor_file.clone() {
        let value: Vec<&str> = file_line.split("|").collect();
        let mut val1 = value[0].to_string();
        if val1.contains("0Y") && !val1.contains("0M") && !val1.contains("0D") {
            val1 = val1.replace("0Y", "");
            let temp: Vec<&str> = val1.split("M").collect();
            change_value = temp[0].parse::<i64>().expect("change value");
            break;
        }
    }
    while start <= end {
        start = increment_date_by_months(start, 12);
        yr += 1;
    }
    if start > end {
        start = decr_dt_by_mon_presrv_eom(start, 12).expect("connot decrement year");
        yr -= 1;
    }

    if yr <= 0 {
        let start_temp = start;
        while start <= end {
            start = increment_date_by_months(start, 1);
            month += 1;
        }
        if start > end {
            start = decr_dt_by_mon_presrv_eom(start, 1).expect("connot decrement month");
            month -= 1;
        }

        if month < change_value && yr == 0 {
            start = start_temp;
            month = 0;
        }
    }
    while start <= end {
        start += Duration::days(1);
        days += 1;
    }
    if start > end {
        start -= Duration::days(1);
        days -= 1;
    }
    let mut tenor: String = "".to_string();

    let duration = yr * 365 + month * 30 + days;

    for tenor_line in tenor_file {
        let columns: Vec<&str> = tenor_line.split("|").collect();

        let mut col1_temp = columns[0].to_string();

        col1_temp = col1_temp.replace("D", "");
        col1_temp = col1_temp.replace("M", "-");
        col1_temp = col1_temp.replace("Y", "-");

        let mut col2_temp = columns[1].to_string();
        col2_temp = col2_temp.replace("D", "");
        col2_temp = col2_temp.replace("M", "-");
        col2_temp = col2_temp.replace("Y", "-");

        let columns_col1: Vec<&str> = col1_temp.split("-").collect();

        let columns_col2: Vec<&str> = col2_temp.split("-").collect();

        let min = columns_col1[0].parse::<i64>().expect("") * 365
            + columns_col1[1].parse::<i64>().expect("") * 30
            + columns_col1[2].parse::<i64>().expect("");
        let max = columns_col2[0].parse::<i64>().expect("") * 365
            + columns_col2[1].parse::<i64>().expect("") * 30
            + columns_col2[2].parse::<i64>().expect("");
        if duration >= min && duration < max {
            tenor = columns[2].to_string();
            break;
        }
    }
    return tenor;
}
