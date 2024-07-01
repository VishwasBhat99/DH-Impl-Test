use slog::Logger;
use std::collections::BTreeMap;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetaData {
    pub fields: Vec<MetaDataFields>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetaDataFields {
    pub name: String,
    pub typ: String,
    pub position: u8,
}

impl MetaDataFields {
    pub fn new(metadata: MetaDataFields) -> MetaDataFields {
        MetaDataFields {
            name: metadata.name,
            typ: get_equivalent_proto_datatype(metadata.typ),
            position: metadata.position,
        }
    }
}

pub fn read_metadata(
    file_path: &str,
    _log: &Logger,
    _diag_log: &Logger,
    input_map: &mut BTreeMap<String, MetaDataFields>,
    column_count: &mut usize,
    date_fields_vec: &mut Vec<String>,
) {
    let mut file =
        sdb_io::open_file_read(file_path).expect("Cannot open the account required fields file");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Cannot read input as string");
    let metadata: MetaData = serde_json::from_str(&buf[..]).expect("");
    for data in metadata.fields {
        input_map.insert(
            data.name.to_owned(),
            MetaDataFields::new(data.to_owned()).to_owned(),
        );
        if data.typ.contains("Date") {
            date_fields_vec.push("Output.".to_string() + &data.name.to_owned());
        }
        *column_count += 1;
    }
}

pub fn get_equivalent_proto_datatype(input_type: String) -> String {
    match input_type.as_str() {
        "I32" => "int32",
        "I64" => "int64",
        "F32" => "float",
        "F64" => "double",
        "String" => "string",
        "Cashflows" => "Cashflow",
        _ => {
            panic!("Type : {} does not exist!", input_type);
        }
    }
    .to_string()
}
