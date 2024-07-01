use rbdate::DateParser;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub file_id: i32,
    pub cust_id: String,
    pub ccy: String,
    pub ca: f64,
    pub sa: f64,
    pub td_wd: f64,
    pub td_nwd: f64,
    pub rd: f64,
    pub wd_b1: f64,
    pub wd_b2: f64,
    pub wd_b3: f64,
    pub nwd_b1: f64,
    pub nwd_b2: f64,
    pub nwd_b3: f64,
    pub rd_b1: f64,
    pub rd_b2: f64,
    pub rd_b3: f64,
    pub rd_td_wd_b1: f64,
    pub rd_td_wd_b2: f64,
    pub rd_td_wd_b3: f64,
    pub t1: String,
    pub t2: String,
    pub t3: String,
    pub tot_wd: f64,
    pub tot_nwd: f64,
    pub logic_type: String,
    pub tot_stable: f64,
    pub tot_less_stable: f64,
    pub ca_stable: f64,
    pub ca_less_stable: f64,
    pub sa_stable: f64,
    pub sa_less_stable: f64,
    pub casa_stable: f64,
    pub casa_less_stable: f64,
    pub stable_b1: f64,
    pub stable_b2: f64,
    pub stable_b3: f64,
    pub less_stable_b1: f64,
    pub less_stable_b2: f64,
    pub less_stable_b3: f64,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        _dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            file_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `file_id`.");
                }
            },
            cust_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_id`.");
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy`.");
                }
            },
            ca: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ca`.");
                }
            },
            sa: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `sa`.");
                }
            },
            td_wd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `td_wd`.");
                }
            },
            td_nwd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `td_nwd`.");
                }
            },
            rd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rd`.");
                }
            },
            wd_b1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `wd_b1`.");
                }
            },
            wd_b2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `wd_b2`.");
                }
            },
            wd_b3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `wd_b3`.");
                }
            },
            nwd_b1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `nwd_b1`.");
                }
            },
            nwd_b2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `nwd_b2`.");
                }
            },
            nwd_b3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `nwd_b3`.");
                }
            },
            rd_b1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rd_b1`.");
                }
            },
            rd_b2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rd_b2`.");
                }
            },
            rd_b3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rd_b3`.");
                }
            },
            rd_td_wd_b1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rd_td_wd_b1`.");
                }
            },
            rd_td_wd_b2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rd_td_wd_b2`.");
                }
            },
            rd_td_wd_b3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rd_td_wd_b3`.");
                }
            },
            t1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `t1`.");
                }
            },
            t2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `t2`.");
                }
            },
            t3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `t3`.");
                }
            },
            tot_wd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `tot_wd`.");
                }
            },
            tot_nwd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `tot_nwd`.");
                }
            },
            logic_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `logic_type`.");
                }
            },
            tot_stable: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `tot_stable`.");
                }
            },
            tot_less_stable: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `tot_less_stable`.");
                }
            },
            ca_stable: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ca_stable`.");
                }
            },
            ca_less_stable: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ca_less_stable`.");
                }
            },
            sa_stable: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `sa_stable`.");
                }
            },
            sa_less_stable: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `sa_less_stable`.");
                }
            },
            casa_stable: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `casa_stable`.");
                }
            },
            casa_less_stable: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `casa_less_stable`.");
                }
            },
            stable_b1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `stable_b1`.");
                }
            },
            stable_b2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `stable_b2`.");
                }
            },
            stable_b3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `stable_b3`.");
                }
            },
            less_stable_b1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `less_stable_b1`.");
                }
            },
            less_stable_b2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `less_stable_b2`.");
                }
            },
            less_stable_b3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `less_stable_b3`.");
                }
            },
        };
        Ok(input_account)
    }
}
