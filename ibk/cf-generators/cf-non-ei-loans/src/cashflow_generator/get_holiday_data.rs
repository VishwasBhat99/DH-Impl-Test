use chrono::Datelike;
use configuration_parameters::ConfigurationParameters;
use itertools::Itertools;
use rbdate::NaiveDate;
use std::collections::HashMap;

pub fn get_holiday_map(
    fields: &mut Vec<&str>,
    config_params: &ConfigurationParameters,
    holiday_map: &mut HashMap<NaiveDate, String>,
) {
    if fields[1].to_string().trim().to_uppercase() == config_params.currency() {
        let dates: Vec<&str> = fields[3].split('!').collect();
        //January to December
        let date_vec = vec![
            dates[0], dates[1], dates[2], dates[3], dates[4], dates[5], dates[6], dates[7],
            dates[8], dates[9], dates[10], dates[11],
        ];
        for month in 0..12 {
            let days = &date_vec[month]
                .chars()
                .chunks(1)
                .into_iter()
                .map(|chunk| chunk.collect::<String>())
                .collect::<Vec<_>>();
            let mut date = rbdate::NaiveDate::from_ymd(1970, 01, 01);
            for day in 0..28 {
                date = rbdate::NaiveDate::from_ymd(
                    fields[2]
                        .to_string()
                        .parse::<i32>()
                        .expect("Invalid Year from Res-File"),
                    (month + 1)
                        .to_string()
                        .parse::<u32>()
                        .expect("Invalid Month from Res-File"),
                    (day + 1)
                        .to_string()
                        .parse::<u32>()
                        .expect("Invalid Day from Res-File"),
                );
                holiday_map.insert(date, days[day].to_string());
            }
            match month {
                // For months Jan, Mar, May, Jul, Aug, Oct, Dec
                0 | 2 | 4 | 6 | 7 | 9 | 11 => {
                    holiday_map.insert(date.succ(), days[28].to_string());
                    holiday_map.insert(date.succ().succ(), days[29].to_string());
                    holiday_map.insert(date.succ().succ().succ(), days[30].to_string());
                }
                // For months having Apr, Jun, Sept, Nov
                3 | 5 | 8 | 10 => {
                    holiday_map.insert(date.succ(), days[28].to_string());
                    holiday_map.insert(date.succ().succ(), days[29].to_string());
                }
                // For Feb Month
                1 => {
                    if is_leap_year(
                        fields[2]
                            .to_string()
                            .parse::<i32>()
                            .expect("Invalid Year from Res-File"),
                    ) {
                        holiday_map.insert(date.succ(), days[28].to_string());
                    }
                }
                _ => {
                    panic!("Invalid Month");
                }
            }
        }
    }
}

pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}
