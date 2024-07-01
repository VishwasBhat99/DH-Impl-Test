use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug)]
pub struct BmMaster {
    pub vertex: i64,
    pub uom: String,
    pub rate: f64,
}

impl BmMaster {
    pub fn new(line: String) -> Result<BmMaster, String> {
        let mut value_iterator = line.split('|');

        let bm = BmMaster {
            vertex: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err(format!("Could not parse property `vertex`."));
                }
            },
            uom: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err(format!("Could not parse property `UOM`."));
                }
            },
            rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err(format!("Could not parse property `rate`."));
                }
            },
        };

        Ok(bm)
    }
}
