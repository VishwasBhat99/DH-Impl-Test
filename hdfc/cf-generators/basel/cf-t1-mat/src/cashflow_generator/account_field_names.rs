use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub acc_id: String,
    pub cust_id: String,
    pub prod_code: String,
    pub int_rate: String,
    pub currency: String,
    pub str1: String,
    pub str2: String,
    pub str3: String,
    pub str4: String,
    pub str5: String,
    pub int1: String,
    pub int2: String,
    pub int3: String,
    pub int4: String,
    pub int5: String,
    pub float1: String,
    pub float2: String,
    pub float3: String,
    pub float4: String,
    pub float5: String,
    pub cashflows: String,
}

impl AccFieldNames {
    pub fn new_from_path(_path: &str) -> AccFieldNames {
        let mut file =
            sdb_io::open_file_read(_path).expect("Cannot open the account metadata file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: AccFieldNames = serde_json::from_str(&buf[..])
            .expect("Account metadata json file was not well-formatted");
        req_fields
    }
}
