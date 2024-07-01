use aggregate::cfinput::AccFieldNames;
use cp::CP;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;

pub struct FtpParameters {
    pub cp: CP,
    pub log: Logger,
    pub diag_log: Logger,
    pub input_data: reader::Reader,
    pub input_field_names: AccFieldNames,
}
