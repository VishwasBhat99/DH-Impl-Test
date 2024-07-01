use super::datatype_lookup::get_datatype_equivalent;
use sdb_util::expand;
use serde_json::json;

pub fn generate_json_record(line: String) -> serde_json::Value {
    let mut record: Vec<String> = expand(
        line.replace("{", "")
            .replace("repeated", " ")
            .replace("=", " ")
            .replace(";", " ")
            .replace("}", "")
            .trim()
            .to_string(),
        ' ',
    );

    // Remove blank elements from a record
    let mut index = 0;
    while index < record.len() {
        if record[index].len() == 0 {
            record.remove(index);
        } else {
            index += 1;
        }
    }

    json!({
        "name": record[1].to_string(),
        "typ": get_datatype_equivalent(record[0].to_string()),
        "position": record[2].to_string().parse::<i64>().unwrap_or(0)
    })
}
