use super::*;

#[derive(Debug, Clone)]
pub struct BmMaster {
    pub vertex: i64,
    pub uom: String,
    pub rate: f64,
}

impl BmMaster {
    pub fn new(line: String) -> Result<BmMaster, String> {
        let mut value_iterator = line.split('|');

        Ok(BmMaster {
            vertex: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err(String::from("Could not parse property `vertex`."));
                }
            },
            uom: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err(String::from("Could not parse property `UOM`."));
                }
            },
            rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err(String::from("Could not parse property `rate`."));
                }
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntermediateBMPoint {
    pub vertex: i64,
    pub uom: String,
    pub rate: f64,
    pub days_diff: i64,
    pub month: i64,
}

impl<'a> IntermediateBMPoint {
    pub fn get_intermediate_bm_points(
        lst_bm: &mut Vec<BmMaster>,
        cpd: NaiveDate,
        lst_bm_inter: &mut IntermediateBMPoints,
    ) {
        for bm in lst_bm.iter_mut() {
            let inter_uom = &bm.uom;
            let inter_bm = IntermediateBMPoint {
                vertex: bm.vertex,
                uom: inter_uom.to_string(),
                rate: bm.rate,
                days_diff: get_days_diff(cpd, inter_uom.to_string(), bm.vertex).unwrap_or(0),
                month: (get_days_diff(cpd, inter_uom.to_string(), bm.vertex).unwrap_or(0) as f64
                    / 365.0
                    * 12.0)
                    .round() as i64,
            };

            lst_bm_inter.push(inter_bm);
        }
    }
}

pub fn get_days_diff(date: NaiveDate, uom: String, vertex: i64) -> Result<i64, Error> {
    let days_diff;

    match uom.as_ref() {
        "D" => {
            days_diff = vertex;
        }
        "M" => {
            let end_date = increment_date_by_months(date, vertex as u16);
            days_diff = num_days_start_to_end(date, end_date);
        }
        "Y" => {
            let end_date = increment_date_by_months(date, (vertex * 12) as u16);
            days_diff = num_days_start_to_end(date, end_date);
        }
        _ => days_diff = 0,
    }

    Ok(days_diff)
}
