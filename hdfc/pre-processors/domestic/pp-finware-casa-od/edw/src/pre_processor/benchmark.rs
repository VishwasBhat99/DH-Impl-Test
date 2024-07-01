use statics::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Benchmark {
    pub amt: f64,
    pub bm_id: String,
}

pub fn get_benchmark<'a>(
    bm_id_map: &mut HashMap<String, Benchmark>,
    acc_no: &str,
    req_amt: &str,
    mut roi_typ: &str,
) {
    if roi_typ == "" {
        roi_typ = "FIXED";
    }
    if bm_id_map.contains_key(&acc_no.to_string().trim_matches('"').to_uppercase()) {
        let map_amt: f64 = bm_id_map
            .get(&acc_no.to_string().trim_matches('"').to_uppercase())
            .unwrap()
            .amt;
        if req_amt.parse::<f64>().unwrap_or(DEFAULT_FLOAT) > map_amt {
            bm_id_map.insert(
                acc_no.to_string().trim_matches('"').to_uppercase(),
                Benchmark {
                    amt: req_amt.parse::<f64>().unwrap_or(DEFAULT_FLOAT),
                    bm_id: roi_typ.to_string().trim_matches('"').to_uppercase(),
                },
            );
        }
    } else {
        bm_id_map.insert(
            acc_no.to_string().trim_matches('"').to_uppercase(),
            Benchmark {
                amt: req_amt.parse::<f64>().unwrap_or(0.0),
                bm_id: roi_typ.to_string().trim_matches('"').to_uppercase(),
            },
        );
    }
}

impl Default for Benchmark {
    fn default() -> Benchmark {
        Benchmark {
            amt: DEFAULT_FLOAT,
            bm_id: "FIXED".to_string(),
        }
    }
}
