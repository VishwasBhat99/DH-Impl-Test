use super::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct MethodDates {
    pub curve_pick_date: String,
    pub tenor_start_date: String,
    pub tenor_end_date: String,
}

pub type MethodMap = HashMap<i32, MethodDates>;
pub fn get_method_config(keys: &MethodFields) -> MethodMap {
    let mut method_map: MethodMap = HashMap::new();

    for key in &keys.method_fields {
        let id: i32 = key
            .id
            .parse()
            .expect("Error while getting method id from method config file.");

        method_map.insert(
            id,
            MethodDates {
                curve_pick_date: key.curve_pick_date.to_string(),
                tenor_start_date: key.tenor_start_date.to_string(),
                tenor_end_date: key.tenor_end_date.to_string(),
            },
        );
    }

    method_map
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParsedMethod {
    pub id: i32,
    pub curve_pick_date: NaiveDate,
    pub tenor_start_date: NaiveDate,
    pub tenor_end_date: NaiveDate,
}

impl ParsedMethod {
    fn default(as_on_date: rbdate::NaiveDate) -> Self {
        Self {
            id: DEFAULT_INT as i32,
            curve_pick_date: as_on_date,
            tenor_start_date: as_on_date,
            tenor_end_date: as_on_date,
        }
    }
}

impl ParsedMethod {
    pub fn new(id: i32, keys: &MethodMap, account: &AccountWithCFs, as_on_date: i64) -> Self {
        if let Some(key) = keys.get(&id) {
            let mut parsed_method = Self {
                id,
                curve_pick_date: date_from_timestamp(
                    account
                        .get_i64_for_key(&key.curve_pick_date)
                        .unwrap_or(as_on_date),
                ),
                tenor_start_date: date_from_timestamp(
                    account
                        .get_i64_for_key(&key.tenor_start_date)
                        .unwrap_or(as_on_date),
                ),
                tenor_end_date: date_from_timestamp(
                    account
                        .get_i64_for_key(&key.tenor_end_date)
                        .unwrap_or(as_on_date),
                ),
            };
            parsed_method.apply_def_dates(&date_from_timestamp(as_on_date));
            parsed_method
        } else {
            Self::default(naivedate_from_timestamp(as_on_date))
        }
    }

    fn apply_def_dates(&mut self, as_on_date: &NaiveDate) {
        get_def_date(&mut self.curve_pick_date, as_on_date);
        get_def_date(&mut self.tenor_start_date, as_on_date);
        get_def_date(&mut self.tenor_end_date, as_on_date);
    }
}

pub fn naivedate_from_timestamp(t: i64) -> rbdate::NaiveDate {
    if t == 0 {
        rbdate::NaiveDate::from_ymd(1900, 1, 1)
    } else {
        let naive_date_time = rbdate::NaiveDateTime::from_timestamp(t, 0);
        naive_date_time.date()
    }
}
