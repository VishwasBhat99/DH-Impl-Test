use cp::CP;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_agg_rules_adj::agg_rules::AggRules_adj;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use stamp_ftp::cfinput::AccFieldNames;
use stamp_ftp::read_adjustments::AdjKey;
use std::collections::{HashMap, HashSet};

use stamp_ftp::restructured_op::additional_struct::AmbData;

pub struct FtpParameters {
    pub cp: CP,
    pub log: Logger,
    pub diag_log: Logger,
    pub input_data: reader::Reader,
    pub input_reader: reader::Reader,
    pub input_field_names: AccFieldNames,
    pub m_rules: AggRules,
    pub bc_rules: AggRules,
    pub fix_adj_rules: AggRules_adj,
    pub var_adj_rules: AggRules_adj,
    pub adj_rates: HashMap<AdjKey, f64>,
    pub amb_map: HashMap<String, AmbData>,
    pub config_map: HashMap<String, HashSet<String>>,
}
