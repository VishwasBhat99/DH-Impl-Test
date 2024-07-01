use slog::Logger;
use std::io::Read;

#[derive(Serialize, Clone, Debug, Deserialize, Default)]
pub struct Fields {
    #[serde(default)]
    pub p1_int_1: i64,
    #[serde(default)]
    pub p2_int_2: i64,
    #[serde(default)]
    pub p3_int_3: i64,
    #[serde(default)]
    pub p4_int_4: i64,
    #[serde(default)]
    pub p5_int_5: i64,
    #[serde(default)]
    pub p6_flt_1: i64,
    #[serde(default)]
    pub p7_flt_2: i64,
    #[serde(default)]
    pub p8_flt_3: i64,
    #[serde(default)]
    pub p9_flt_4: i64,
    #[serde(default)]
    pub p10_flt_5: i64,
    #[serde(default)]
    pub p11_str_1: i64,
    #[serde(default)]
    pub p12_str_2: i64,
    #[serde(default)]
    pub p13_str_3: i64,
    #[serde(default)]
    pub p14_str_4: i64,
    #[serde(default)]
    pub p15_str_5: i64,
}

impl Fields {
    pub fn new_from_path(_path: &str) -> Fields {
        let mut file = sdb_io::open_file_read(_path).expect("Cannot open the required fields file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input json as String");
        let req_fields: Fields = serde_json::from_str(&buf[..])
            .expect("Required fields json file was not well-formatted");
        req_fields
    }
    pub fn iterate(&self) -> Vec<i64> {
        let struct_vec = vec![
            self.p1_int_1,
            self.p2_int_2,
            self.p3_int_3,
            self.p4_int_4,
            self.p5_int_5,
            self.p6_flt_1,
            self.p7_flt_2,
            self.p8_flt_3,
            self.p9_flt_4,
            self.p10_flt_5,
            self.p11_str_1,
            self.p12_str_2,
            self.p13_str_3,
            self.p14_str_4,
            self.p15_str_5,
        ];
        struct_vec
    }
}
