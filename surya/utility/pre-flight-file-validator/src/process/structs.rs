#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileCheckTypes {
    pub column_count: bool,
    pub row_count: bool,
    pub duplication_key: bool,
    pub data_type: bool,
    pub values_in: bool,
    pub values_not_in: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PanicFlags {
    pub duplicate_key: bool,
    pub row_count: bool,
    pub column_count: bool,
    pub values_in: bool,
    pub values_not_in: bool,
    pub data_type: bool,
    pub date_check: bool,
}
impl PanicFlags {
    pub fn new() -> PanicFlags {
        PanicFlags {
            duplicate_key: false,
            row_count: false,
            column_count: false,
            values_in: false,
            values_not_in: false,
            data_type: false,
            date_check: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColumnCount {
    pub row_no: usize,
    pub col_count: usize,
}
pub struct ValueStr {
    pub col_no: i64,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TypeCheck {
    pub integer: Vec<i64>,
    pub float: Vec<i64>,
}

impl TypeCheck {
    pub fn append_integer(&mut self, index: i64) {
        self.integer.push(index);
    }

    pub fn append_float(&mut self, index: i64) {
        self.float.push(index);
    }
}
