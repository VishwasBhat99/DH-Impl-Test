use chrono::Duration;
use rbdate::{increment_date_by_months_unchecked, NaiveDate};
use sdb_io;
use std::collections::HashMap;
use std::io::BufRead;
use std::path::Path;
use macros;
use slog::Logger;
pub fn read_distribution_rules(
    dist_rule_path: &str,
    as_on_date: &NaiveDate,
    log: &Logger,
) -> (HashMap<i64, NaiveDate>, HashMap<i64, f64>) {
    let mut dates: HashMap<i64, NaiveDate> = HashMap::new();
    let mut rules: HashMap<i64, f64> = HashMap::new();
    let mut first_day = true;

    if Path::new(&dist_rule_path).exists() {
        let reader = match sdb_io::new_buf_rdr(dist_rule_path) {
            Ok(r) => r,
            Err(e) => panic!(format!(
                "Cannot read distribution file at path: '{}', Error: '{}'",
                dist_rule_path, e
            )),
        };

        for line in reader.lines() {
            let mut line_components: Vec<String> = Vec::new();

            for component in line
                .expect("Error while parsing distribution rules line")
                .split('|')
            {
                line_components.push(component.to_string());
            }

            let period = line_components[3]
                .parse::<i64>()
                .expect("Error while parsing period");
            let uom = &line_components[4];

            if first_day {
                first_day = false;
                dates.insert(
                    line_components[0]
                        .parse::<i64>()
                        .expect("Error while parsing bucket id"),
                    *as_on_date,
                );
            } else {
                dates.insert(
                    line_components[0]
                        .parse::<i64>()
                        .expect("Error while parsing bucket id"),
                    get_date(*as_on_date, period, uom.to_string()),
                );
            }

            rules.insert(
                line_components[0]
                    .parse::<i64>()
                    .expect("Error while parsing bucket id"),
                line_components[5]
                    .parse::<f64>()
                    .expect("Error while parsing distribution rule %"),
            );
        }
    }
    else{
        log_error!(log,"Distribution rule file is not present!!");
        panic!("Distribution rule file is not present!!")
    }

    (dates, rules)
}

pub fn get_date(as_on: NaiveDate, period: i64, uom: String) -> NaiveDate {
    let resultant: NaiveDate;

    match uom.as_str() {
        "Days" => {
            resultant = as_on + Duration::days(period);
        }
        "Month" => {
            resultant = increment_date_by_months_unchecked(as_on, period as u16);
        }
        "Years" => {
            resultant = increment_date_by_months_unchecked(as_on, period as u16 * 12);
        }
        _ => resultant = as_on,
    }

    resultant
}
