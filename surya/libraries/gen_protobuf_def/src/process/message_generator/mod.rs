use super::*;

pub fn generate_message(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    protoc_rust::Codegen::new()
        .out_dir("/Surya/gen_protobuf_def/test-bed")
        .inputs(&[config_params.output_proto_path()])
        .run()
        .expect("unable to generate rust file from the proto file");
}
