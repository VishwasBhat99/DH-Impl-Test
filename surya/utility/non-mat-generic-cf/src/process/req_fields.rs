use crate::configuration_parameters::ConfigurationParameters;
use crate::process::read_metadata::*;
use slog::Logger;
use std::collections::HashMap;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccFieldNames {
    pub fields: Vec<ReqFields>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqFields {
    pub name: String,
    pub typ: String,
    pub position: u8,
}

pub fn get_req_fields(
    config_params: &mut ConfigurationParameters,
    _log: &Logger,
    _diag_log: &Logger,
    input_map: &mut HashMap<String, MetaDataFields>,
    output_map: &mut HashMap<String, i32>,
    proto_str: &mut String,
) {
    let mut file = sdb_io::open_file_read(config_params.output_metadata_file())
        .expect("Cannot open the account required fields file");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Cannot read input as string");
    let req_fields: AccFieldNames = serde_json::from_str(&buf[..])
        .expect("Account required fields json file was not well-formatted");
    let mut op_pos = 1;
    *proto_str = "message Output{\n".to_string();
    for field in req_fields.fields {
        proto_str.push_str(
            &input_map
                .get(&field.name)
                .unwrap_or_else(|| {
                    panic!(
                        "Input-Field(Req-Fields): `{}` not present in Input Metadata",
                        field.name
                    )
                })
                .typ
                .to_string(),
        );
        proto_str.push(' ');
        proto_str.push_str(&field.name);
        proto_str.push_str(" = ");
        proto_str.push_str(&op_pos.to_string());
        proto_str.push_str(";\n");
        op_pos += 1;
        output_map
            .entry("Output.".to_string() + &field.name.to_owned())
            .and_modify(|data| panic!("Repeated output-field in req-fields-file: {}", data))
            .or_insert_with(|| {
                input_map
                    .get(&field.name.to_owned())
                    .unwrap_or_else(|| {
                        panic!(
                            "Input-Field(Req-Fields): `{}` not present in Input Metadata",
                            field.name
                        )
                    })
                    .position
                    .to_string()
                    .parse::<i32>()
                    .unwrap_or_else(|_| {
                        panic!("Unable to parse Field-Position for : `{}`", field.name)
                    })
            });
    }
    proto_str.push_str("}\n");
}
