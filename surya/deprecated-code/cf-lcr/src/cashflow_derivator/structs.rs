#[derive(Debug, Clone, PartialEq)]
pub struct LCR {
    pub ca: f64,
    pub sa: f64,
    pub td_wd: f64,
    pub td_nwd: f64,
    pub rd: f64,
    pub tot_stable: f64,
    pub tot_less_stable: f64,
    pub ca_stable: f64,
    pub ca_less_stable: f64,
    pub sa_stable: f64,
    pub sa_less_stable: f64,
    pub casa_stable: f64,
    pub casa_less_stable: f64,
    pub stable_b1: f64,
    pub stable_b2: f64,
    pub stable_b3: f64,
    pub less_stable_b1: f64,
    pub less_stable_b2: f64,
    pub less_stable_b3: f64,
    pub nwd_b1: f64,
    pub nwd_b2: f64,
    pub nwd_b3: f64,
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct AggrKey {
    pub file_id: i32,
    pub currency: String,
}
