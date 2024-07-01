pub fn remove_comma(val: &String) -> String {
    let mut amt = val.to_string();
    amt.retain(|val| val != ',');
    amt
}
