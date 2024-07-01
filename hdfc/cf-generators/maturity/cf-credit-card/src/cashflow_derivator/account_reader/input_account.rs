use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug)]
pub struct InputAccount {
    pub gl_number: String,
    pub gl_desc: String,
    pub pd: String,
    pub prod_type: String,
    pub i: String,
    pub day1_sum: f64,
    pub day1_count: i64,
    pub day2_sum: f64,
    pub day2_count: i64,
    pub day3_sum: f64,
    pub day3_count: i64,
    pub day4_sum: f64,
    pub day4_count: i64,
    pub day5_sum: f64,
    pub day5_count: i64,
    pub day6_sum: f64,
    pub day6_count: i64,
    pub day7_sum: f64,
    pub day7_count: i64,
    pub day8_14_sum: f64,
    pub day8_14_count: i64,
    pub day15_28_sum: f64,
    pub day15_28_count: i64,
    pub day29_30_sum: f64,
    pub day29_30_count: i64,
    pub m2_sum: f64,
    pub m2_count: i64,
    pub m3_sum: f64,
    pub m3_count: i64,
    pub m4_sum: f64,
    pub m4_count: i64,
    pub m5_sum: f64,
    pub m5_count: i64,
    pub m6_sum: f64,
    pub m6_count: i64,
    pub m7_sum: f64,
    pub m7_count: i64,
    pub m8_sum: f64,
    pub m8_count: i64,
    pub m10_sum: f64,
    pub m10_count: i64,
    pub m11_sum: f64,
    pub m11_count: i64,
    pub m12_sum: f64,
    pub m12_count: i64,
    pub y1_3_sum: f64,
    pub y1_3_count: i64,
    pub y3_5_sum: f64,
    pub y3_5_count: i64,
    pub y5_7_sum: f64,
    pub y5_7_count: i64,
    pub y7_8_sum: f64,
    pub y7_8_count: i64,
    pub y8_9_sum: f64,
    pub y8_9_count: i64,
    pub y9_10_sum: f64,
    pub y9_10_count: i64,
    pub y10_15_sum: f64,
    pub y10_15_count: i64,
    pub y15_sum: f64,
    pub y15_count: i64,
    pub total_pd_sum: f64,
    pub total_pd_count: i64,
}

impl<'a> InputAccount {
    pub fn new_from_line(line: String) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('~');

        let input_account = InputAccount {
            gl_number: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_number`.");
                }
            },
            gl_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property 'gl_desc'."),
            },
            pd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pd`.");
                }
            },
            prod_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property 'prod_type'."),
            },
            i: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property 'i'."),
            },
            day1_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `day1_sum`.");
                }
            },
            day1_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `day1_count`.");
                }
            },
            day2_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `day2_sum`.");
                }
            },
            day2_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `day2_count`.");
                }
            },
            day3_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `day3_sum`.");
                }
            },
            day3_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `day3_count`.");
                }
            },
            day4_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `day4_sum`.");
                }
            },
            day4_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `day4_count`.");
                }
            },
            day5_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `day5_sum`.");
                }
            },
            day5_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `day5_count`.");
                }
            },
            day6_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `day6_sum`.");
                }
            },
            day6_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `day6_count`.");
                }
            },
            day7_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `day7_sum`.");
                }
            },
            day7_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `day7_count`.");
                }
            },
            day8_14_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `day8_14_sum`.");
                }
            },
            day8_14_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `day8_14_count`.");
                }
            },
            day15_28_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `day15_28_sum`.");
                }
            },
            day15_28_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `day15_28_count`.");
                }
            },
            day29_30_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `day29_30_sum`.");
                }
            },
            day29_30_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `day29_30_count`.");
                }
            },
            m2_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `m2_sum`.");
                }
            },
            m2_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `m2_count`.");
                }
            },
            m3_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `m3_sum`.");
                }
            },
            m3_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `m3_count`.");
                }
            },
            m4_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `m4_sum`.");
                }
            },
            m4_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `m4_count`.");
                }
            },
            m5_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `m5_sum`.");
                }
            },
            m5_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `m5_count`.");
                }
            },
            m6_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `m6_sum`.");
                }
            },
            m6_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `m6_count`.");
                }
            },
            m7_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `m7_sum`.");
                }
            },
            m7_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `m7_count`.");
                }
            },
            m8_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `m8_sum`.");
                }
            },
            m8_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `m8_count`.");
                }
            },
            m10_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `m10_sum`.");
                }
            },
            m10_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `m10_count`.");
                }
            },
            m11_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `m11_sum`.");
                }
            },
            m11_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `m11_count`.");
                }
            },
            m12_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `m12_sum`.");
                }
            },
            m12_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `m12_count`.");
                }
            },
            y1_3_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `y1_3_sum`.");
                }
            },
            y1_3_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `y1_3_count`.");
                }
            },
            y3_5_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `y3_5_sum`.");
                }
            },
            y3_5_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `y3_5_count`.");
                }
            },
            y5_7_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `y5_7_sum`.");
                }
            },
            y5_7_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `y5_7_count`.");
                }
            },
            y7_8_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `y7_8_sum`.");
                }
            },
            y7_8_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `y7_8_count`.");
                }
            },
            y8_9_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `y8_9_sum`.");
                }
            },
            y8_9_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `y8_9_count`.");
                }
            },
            y9_10_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `y9_10_sum`.");
                }
            },
            y9_10_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `y9_10_count`.");
                }
            },
            y10_15_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `y10_15_sum`.");
                }
            },
            y10_15_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `y10_15_count`.");
                }
            },
            y15_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `y15_sum`.");
                }
            },
            y15_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `y15_count`.");
                }
            },
            total_pd_sum: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `total_pd_sum`.");
                }
            },
            total_pd_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `total_pd_count`.");
                }
            },
        };

        // println!("gl number  :{}", input_account.gl_number );
        // println!("total pd sum :{}", input_account.total_pd_sum );

        Ok(input_account)
    }
}
