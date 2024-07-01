use reader::types::Type;
use sdb_io;
use serde_json;
use std::collections::HashMap;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
struct PropertyDescriptor {
    name: String,
    typ: String,
    position: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct AccountMetadata {
    #[serde(rename = "fields")]
    field_descriptors: Vec<PropertyDescriptor>,
}

pub fn get_metadata_from_path(
    _path: &str,
) -> (HashMap<String, usize>, HashMap<u32, (String, Type)>) {
    let mut file = sdb_io::open_file_read(_path).expect("Cannot open the account metadata file");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Cannot read input as string");
    let account_metadata: AccountMetadata =
        serde_json::from_str(&buf[..]).expect("Account metadata json file was not well-formatted");
    let mut map = HashMap::new();
    let mut rev_map = HashMap::new();

    for property in account_metadata.field_descriptors {
        if !map.contains_key(&property.name) {
            map.insert(property.name.clone(), property.position);
        } else {
            panic!(
                "Property already exists in account metadata: {:#?}",
                property
            );
        }
        if !rev_map.contains_key(&(property.position as u32)) {
            match &property.typ[..] {
                "I64" => rev_map.insert(property.position as u32, (property.name, Type::I64)),
                "I32" => rev_map.insert(property.position as u32, (property.name, Type::I32)),
                "F64" => rev_map.insert(property.position as u32, (property.name, Type::F64)),
                "F32" => rev_map.insert(property.position as u32, (property.name, Type::F32)),
                "String" => rev_map.insert(property.position as u32, (property.name, Type::String)),
                "Cashflows" => {
                    rev_map.insert(property.position as u32, (property.name, Type::Cashflows))
                }
                _ => panic!(
                    "Invalid property type encountered in account metadata: {:#?}",
                    property
                ),
            };
        } else {
            panic!(
                "Property already exists in account metadata: {:#?}",
                property
            );
        }
    }

    return (map, rev_map);
}
