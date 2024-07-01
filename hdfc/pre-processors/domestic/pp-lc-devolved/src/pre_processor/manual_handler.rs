pub fn remove_comma(val: &str) -> String {
    let mut amt = val.to_string();
    amt.retain(|val| val != ',');
    String::from(amt.trim())
}
