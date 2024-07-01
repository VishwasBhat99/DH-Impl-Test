use std::collections::HashMap;

pub fn store_prod_type() -> HashMap<&'static str, &'static str> {
    let mut prod_type_map: HashMap<&str, &str> = HashMap::new();
    prod_type_map.insert("10A", "Cash Credit");
    prod_type_map.insert("10D", "ECB FC");
    prod_type_map.insert("10E", "ECB FC INR swap");
    prod_type_map.insert("10F", "ECB INR");
    prod_type_map.insert("10G", "NHB Loans");
    prod_type_map.insert("12A", "ICB");
    prod_type_map.insert("15A", "CP");
    prod_type_map.insert("15B", "ZCD");
    prod_type_map.insert("15C", "ZCD Par Premium");
    prod_type_map.insert("15D", "Long Term Debt");
    prod_type_map.insert("15E", "LTD Inst Repayment");
    prod_type_map.insert("15F", "LTD Variable Int");
    prod_type_map.insert("15G", "Tier 2 Debt");
    prod_type_map.insert("15H", "Perpetual Debt");
    prod_type_map.insert("15I", "MLD");
    prod_type_map.insert("15J", "Partly Paid Debt");
    prod_type_map.insert("45A", "LTD  Foreign Exchange");
    prod_type_map.insert("45B", "LTD  Cross Currency Swap");

    prod_type_map
}

pub fn get_prod_type<'a>(
    prod_type_map: &mut std::collections::HashMap<&str, &'a str>,
    key: &str,
) -> &'a str {
    prod_type_map
        .get(&key.to_uppercase().trim())
        .unwrap_or(&"Invalid Product Type")
}
