#[derive(Debug, Clone, Default)]
pub struct CustFields {
    pub cust_basel_val_code: String,
    pub div: String,
    pub nat_of_bus: String,
    pub txt_desc: bool,
    pub sme_class: bool,
}

impl CustFields {
    pub fn new() -> Self {
        let def_str = String::from("NA");
        Self {
            cust_basel_val_code: def_str.to_string(),
            div: def_str.to_string(),
            nat_of_bus: def_str.to_string(),
            txt_desc: false,
            sme_class: false,
        }
    }

    pub fn update_cust_basel_val_code(&mut self, val: &str) {
        self.cust_basel_val_code = val.to_string();
    }

    pub fn update_div(&mut self, val: &str) {
        self.div = val.to_string();
    }

    pub fn update_bus_and_txt_desc(&mut self, vals: &Vec<&str>) {
        self.nat_of_bus = vals[1].to_string();
        if vals[2].contains("NBFC") || vals[2].contains("BROK") {
            self.txt_desc = true;
        };
    }

    pub fn update_sme_class(&mut self, val: &str) {
        if let Ok(val) = val.parse::<i32>() {
            if val == 5 {
                self.sme_class = true;
            }
        }
    }
}
