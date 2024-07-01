use chrono::NaiveDate;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Data {
    pub zone: String,
    pub long_pos: f64,
    pub short_pos: f64,
    pub as_on_date: NaiveDate,
}

impl Data {
    pub fn new(fields: &[&str], as_on_date: NaiveDate) -> Data {
        Data {
            zone: fields[8].to_string(),
            long_pos: 0.0,
            short_pos: 0.0,
            as_on_date,
        }
    }

    pub fn append_data(&mut self, fields: Vec<&str>, as_on_date: NaiveDate) {
        if fields[61].to_uppercase() == "LONG" {
            self.long_pos += fields[45].parse::<f64>().unwrap_or(0.0);
        } else if fields[61].to_uppercase() == "SHORT" {
            self.short_pos += fields[45].parse::<f64>().unwrap_or(0.0);
        }
        self.as_on_date = NaiveDate::parse_from_str(fields[0], "%d-%m-%Y").unwrap_or(as_on_date);
    }
}
