#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct InputKeyEI {
    pub acid: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone)]
pub struct InputDataEI {
    pub shdl_num: String,
    pub num_of_dmds: i64,
    pub flow_start_date: rbdate::NaiveDate,
    pub flow_amt: i64,
    pub lr_freq_type: String,
    pub cf_code: String,
    pub num_of_flows: i64,
}

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct InputKeyNonEI {
    pub acid: String,
    pub cf_code: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone)]
pub struct InputDataNonEI {
    pub shdl_num: String,
    pub num_of_dmds: i64,
    pub flow_start_date: rbdate::NaiveDate,
    pub flow_amt: i64,
    pub lr_freq_type: String,
    pub cf_code: String,
    pub num_of_flows: i64,
}

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct OPKeyEI {
    pub acid: String,
    pub flow_start_date: rbdate::NaiveDate,
}

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone)]
pub struct OPDataEI {
    pub shdl_num: String,
    pub num_of_dmds: i64,
    pub flow_start_date: rbdate::NaiveDate,
    pub flow_amt: i64,
    pub lr_freq_type: String,
    pub cf_code: String,
    pub num_of_flows: i64,
}

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct OPKeyNonEI {
    pub acid: String,
    pub flow_start_date: rbdate::NaiveDate,
    pub cf_code: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone)]
pub struct OPDataNonEI {
    pub shdl_num: String,
    pub num_of_dmds: i64,
    pub flow_start_date: rbdate::NaiveDate,
    pub flow_amt: i64,
    pub lr_freq_type: String,
    pub cf_code: String,
    pub num_of_flows: i64,
}
