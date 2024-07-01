pub fn get_datatype_equivalent(input_type: String) -> String {
    match input_type.as_str() {
        "int32" => "I32",
        "int64" => "I64",
        "double" => "F64",
        "string" => "String",
        "Cashflow" => "Cashflows",
        _ => {
            panic!("Type : {} does not exist!", input_type);
        }
    }
    .to_string()
}
