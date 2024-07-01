use cp::CP;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use stamp_ftp::aggr_key::Customer;
use stamp_ftp::cfinput::AccFieldNames;
use std::collections::HashMap;

pub struct FtpParameters {
    pub cp: CP,
    pub log: Logger,
    pub diag_log: Logger,
    pub input_data: reader::Reader,
    pub input_field_names: AccFieldNames,
    pub aggr_bal: HashMap<Customer, f64>,
    pub avg_bal: HashMap<String, f64>,
}
