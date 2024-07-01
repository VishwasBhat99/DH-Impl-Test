pub fn get_comp_result(left: f64, comp: &str, right: f64) -> bool {
    match comp {
        "==" => left == right,
        ">" => left > right,
        "<" => left < right,
        "<=" => left <= right,
        ">=" => left >= right,
        _ => panic!("Invalid Comparator!"),
    }
}
