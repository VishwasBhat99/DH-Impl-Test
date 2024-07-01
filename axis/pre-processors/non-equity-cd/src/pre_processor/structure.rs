use std::collections::HashMap;

pub struct OutstandingStruct {
    pub data: HashMap<String, Vec<String>>,
}
impl OutstandingStruct {
    pub fn new() -> OutstandingStruct {
        OutstandingStruct {
            data: HashMap::new(),
        }
    }
}

pub struct PriceStruct {
    pub data: HashMap<String, Vec<f64>>,
}

impl PriceStruct {
    pub fn new() -> PriceStruct {
        PriceStruct {
            data: HashMap::new(),
        }
    }
}
