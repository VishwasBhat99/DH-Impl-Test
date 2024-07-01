pub fn expand(input_string: String, delimiter: char) -> Vec<String> {
    input_string
        .split(delimiter)
        .map(|s: &str| s.to_string())
        .collect()
}
