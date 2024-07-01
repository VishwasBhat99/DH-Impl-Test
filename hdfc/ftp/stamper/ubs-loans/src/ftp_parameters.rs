use cp::CP;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_agg_rules_adj::agg_rules::AggRulesAdj;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use stamp_ftp::amb_file_reader::AmbVal;
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::read_adjustments::Adj_key;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;

pub struct FtpParameters {
    pub cp: CP,
    pub log: Logger,
    pub diag_log: Logger,
    pub m_rules: AggRules,
    pub bc_rules: AggRules,
    pub fix_adj_rules: AggRulesAdj,
    pub var_adj_rules: AggRulesAdj,
    pub input_data: reader::Reader,
    pub input_reader: reader::Reader,
    pub input_field_names: AccFieldNames,
    pub ftp_rates: HashMap<String, Vec<f64>>,
    pub lock_adjs: HashMap<i32, String>,
    pub adj_rates: HashMap<Adj_key, f64>,
    pub avg_bal: HashMap<String, AmbVal>,
    pub spread_writer: BufWriter<File>,
}
