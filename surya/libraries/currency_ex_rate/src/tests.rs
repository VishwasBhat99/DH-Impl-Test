use super::*;

#[test]
fn check_create_ccy_converter() {
    create_currency_converter("USD".to_string(),"INR".to_string(),"src/input.txt");
}

/*This test fails as currency_conversion_file_path is not found
#[test]
fn check_create_ccy_converter_fail() {
    create_currency_converter("USD".to_string(),"INR".to_string(),"src/other.txt");
}*/

#[test]
fn check_ccy_converter() {
    let mut exchange_rate: HashMap<CurrencyExchange, f64> = HashMap::new();
    let ex_key = CurrencyExchange {
            from_ccy: "USD".to_string(),
            to_ccy: "INR".to_string(),
        };
    exchange_rate.insert(
            ex_key,
            75.00,
        );      
    CurrencyConverter::new("USD".to_string(),"INR".to_string(),exchange_rate);
}

#[test]
fn check_ccy_converter_1() {
    let mut exchange_rate: HashMap<CurrencyExchange, f64> = HashMap::new();
    let ex_key = CurrencyExchange {
            from_ccy: "INR".to_string(),
            to_ccy: "IN1".to_string(),
        };
    exchange_rate.insert(
            ex_key,
            1.00,
        );      
    CurrencyConverter::new("INR".to_string(),"IN1".to_string(),exchange_rate);
}
