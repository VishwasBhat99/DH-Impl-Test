use super::*;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AdjKey {
    pub date: NaiveDate,
    pub id: i32,
}

impl AdjKey {
    pub fn new(date: i64, id: i32) -> Self {
        Self {
            date: date_from_timestamp(date),
            id,
        }
    }
}

#[derive(Debug)]
pub struct Adjustments {
    pub adjs: AdjMap,
}

pub type AdjMap = HashMap<AdjKey, f64>;
impl Adjustments {
    pub fn new(file_path: &str) -> Self {
        let mut adjs: AdjMap = HashMap::new();
        let reader = read_file(file_path);
        for (line_num, lines) in reader.lines().enumerate() {
            let line = extract_lines(line_num, lines, file_path);
            let fields: Vec<&str> = line.split('|').collect();
            if fields.len() == 3 {
                adjs.insert(
                    AdjKey {
                        date: parse_date(fields[0]),
                        id: parse_i32(fields[1]),
                    },
                    parse_f64(fields[2].trim()),
                );
            }
        }
        Self { adjs }
    }
}
