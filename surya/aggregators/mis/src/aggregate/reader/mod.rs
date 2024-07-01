use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::Reader;

pub fn read_file(file_path: &str, metadata_file_path: &str) -> Reader {
    reader::Reader::new_at_path(metadata_file_path, file_path)
}
