use chrono::NaiveDate;
use pre_processor::structs::fields_number::Field;
use rbdate::DateParser;

#[derive(Debug)]
pub struct Account {
    pub date: NaiveDate,
    pub symbol: String,
    pub series: String,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub last_traded_price: f64,
    pub close_price: f64,
    pub change_price: f64,
}

pub fn get_account_from_line(line: &str, fields: &Field, delimiter: &str) -> Account {
    let mut date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
    let line_vec: Vec<&str> = line.split(delimiter).collect();
    if line_vec[(fields.date - 1) as usize]
        .to_string()
        .contains('/')
    {
        date_parser = DateParser::new("%d/%m/%Y".to_string(), false);
    }
    Account {
        date: date_parser.parse(line_vec[(fields.date - 1) as usize]),
        symbol: line_vec[(fields.symbol - 1) as usize].to_string(),
        series: line_vec[(fields.series - 1) as usize].to_string(),
        open_price: line_vec[(fields.open_price - 1) as usize]
            .parse::<f64>()
            .unwrap_or(0f64),
        high_price: line_vec[(fields.high_price - 1) as usize]
            .parse::<f64>()
            .unwrap_or(0f64),
        low_price: line_vec[(fields.low_price - 1) as usize]
            .parse::<f64>()
            .unwrap_or(0f64),
        last_traded_price: line_vec[(fields.last_traded_price - 1) as usize]
            .parse::<f64>()
            .unwrap_or(0f64),
        close_price: line_vec[(fields.close_price - 1) as usize]
            .parse::<f64>()
            .unwrap_or(0f64),
        change_price: 0.0,
    }
}
