use statics::DEFAULT_FLOAT;

#[derive(Debug)]
pub struct TotalBalance {
    pub ttl_cr: f64,
    pub ttl_dr: f64,
    pub ttl_net: f64,
}

impl TotalBalance {
    pub fn add(&mut self, dr: f64, cr: f64, net: f64) {
        self.ttl_cr += cr;
        self.ttl_dr += dr;
        self.ttl_net += net;
    }

    pub fn new() -> TotalBalance {
        TotalBalance {
            ttl_cr: DEFAULT_FLOAT,
            ttl_dr: DEFAULT_FLOAT,
            ttl_net: DEFAULT_FLOAT,
        }
    }

    pub fn get_sum(&mut self, dr: f64, cr: f64, net: f64) -> TotalBalance {
        let mut ttl_bal = TotalBalance::new();
        ttl_bal.add(dr, cr, net);
        ttl_bal
    }
}

#[derive(Debug)]
pub struct IATotalBalance {
    pub ttl_cr: f64,
    pub ttl_dr: f64,
    pub ttl_net: f64,
}

impl IATotalBalance {
    pub fn add(&mut self, dr: f64, cr: f64, net: f64) {
        self.ttl_cr += cr;
        self.ttl_dr += dr;
        self.ttl_net += net;
    }

    pub fn new() -> IATotalBalance {
        IATotalBalance {
            ttl_cr: DEFAULT_FLOAT,
            ttl_dr: DEFAULT_FLOAT,
            ttl_net: DEFAULT_FLOAT,
        }
    }

    pub fn get_sum(&mut self, dr: f64, cr: f64, net: f64) -> IATotalBalance {
        let mut ttl_bal = IATotalBalance::new();
        ttl_bal.add(dr, cr, net);
        ttl_bal
    }
}
