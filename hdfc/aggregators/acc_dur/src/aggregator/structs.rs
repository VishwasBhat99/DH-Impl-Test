use sdb_io;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct RequiredFields {
    pub acc_no: String,
    pub ccy: String,
    pub amt_ccy: String,
    pub duration: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct Currency {
    pub from_ccy: String,
    pub to_ccy: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct LLGVal {
    pub tot_bal_ccy: String,
    pub tot_bal_hcy: String,
    pub weighted_dur: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct LLGKey {
    pub llg: String,
    pub ccy: String,
}

impl RequiredFields {
    pub fn new_from_path(_path: &str) -> RequiredFields {
        let mut file =
            sdb_io::open_file_read(_path).expect("Cannot open the account metadata file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: RequiredFields =
            serde_json::from_str(&buf).expect("Account metadata json file was not well-formatted");
        req_fields
    }
}

impl Currency {
    pub fn new(exrt_file_path: &str) -> HashMap<Currency, f64> {
        let exrt_file = fs::read_to_string(exrt_file_path).expect("cannot read exchange rate file");
        let mut currency_map: HashMap<Currency, f64> = HashMap::new();
        for line in exrt_file.lines() {
            let fields: Vec<&str> = line.split("|").collect();
            let currencies: Currency = Currency {
                from_ccy: fields[0].to_string(),
                to_ccy: fields[1].to_string(),
            };
            let conversion_rate: f64 = fields[2].parse::<f64>().expect(&format!(
                "Cannot fetch exchange rate for {} to {}",
                fields[0], fields[1]
            ));
            currency_map.insert(currencies, conversion_rate);
        }
        currency_map
    }
    pub fn get_key(from_ccy: &str, to_ccy: &str) -> Currency {
        Currency {
            from_ccy: from_ccy.to_string(),
            to_ccy: to_ccy.to_string(),
        }
    }
}

impl LLGKey {
    pub fn get_key(llg: i32, ccy: &str) -> LLGKey {
        return LLGKey {
            llg: llg.to_string(),
            ccy: ccy.to_string(),
        };
    }
}

impl LLGVal {
    pub fn get_val(amt_ccy: &f64, amt_hcy: &f64, weighted_dur: &f64) -> LLGVal {
        return LLGVal {
            tot_bal_ccy: amt_ccy.to_string(),
            tot_bal_hcy: amt_hcy.to_string(),
            weighted_dur: weighted_dur.to_string(),
        };
    }
}
