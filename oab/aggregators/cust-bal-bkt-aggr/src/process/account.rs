///Cust-Bal-Aggr Output Structure Read as Input-File
#[derive(Debug, Clone, Default)]
pub struct Account {
    pub file_id: String,
    pub cust_id: i64,
    pub ccy: String,
    pub ca: f64,
    pub sa: f64,
    pub td_wd: f64,
    pub td_nwd: f64,
    pub rd: f64,
    pub wd_b1: f64,
    pub wd_b2: f64,
    pub wd_b3: f64,
    pub nwd_b1: f64,
    pub nwd_b2: f64,
    pub nwd_b3: f64,
    pub rd_b1: f64,
    pub rd_b2: f64,
    pub rd_b3: f64,
    pub rd_td_wd_b1: f64,
    pub rd_td_wd_b2: f64,
    pub rd_td_wd_b3: f64,
    pub t1: String,
    pub t2: String,
    pub t3: String,
    pub tot_wd: f64,
    pub tot_nwd: f64,
    pub logic_type: String,
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
    pub nwd_stable_b1: f64,
    pub nwd_stable_b2: f64,
    pub nwd_stable_b3: f64,
    pub nwd_less_stable_b1: f64,
    pub nwd_less_stable_b2: f64,
    pub nwd_less_stable_b3: f64,
    pub ca_weighted_rate: f64,
    pub sa_weighted_rate: f64,
    pub td_wd_weighted_rate: f64,
    pub td_nwd_weighted_rate: f64,
    pub rd_weighted_rate: f64,
}

impl Account {
    pub fn new(input_acc: &[&str]) -> Account {
        Account {
            file_id: get_str(input_acc, 0),
            cust_id: get_i64(input_acc, 1),
            ccy: get_str(input_acc, 2),
            ca: get_f64(input_acc, 3),
            sa: get_f64(input_acc, 4),
            td_wd: get_f64(input_acc, 5),
            td_nwd: get_f64(input_acc, 6),
            rd: get_f64(input_acc, 7),
            wd_b1: get_f64(input_acc, 8),
            wd_b2: get_f64(input_acc, 9),
            wd_b3: get_f64(input_acc, 10),
            nwd_b1: get_f64(input_acc, 11),
            nwd_b2: get_f64(input_acc, 12),
            nwd_b3: get_f64(input_acc, 13),
            rd_b1: get_f64(input_acc, 14),
            rd_b2: get_f64(input_acc, 15),
            rd_b3: get_f64(input_acc, 16),
            rd_td_wd_b1: get_f64(input_acc, 17),
            rd_td_wd_b2: get_f64(input_acc, 18),
            rd_td_wd_b3: get_f64(input_acc, 19),
            t1: get_str(input_acc, 20),
            t2: get_str(input_acc, 21),
            t3: get_str(input_acc, 22),
            tot_wd: get_f64(input_acc, 23),
            tot_nwd: get_f64(input_acc, 24),
            logic_type: get_str(input_acc, 25),
            tot_stable: get_f64(input_acc, 26),
            tot_less_stable: get_f64(input_acc, 27),
            ca_stable: get_f64(input_acc, 28),
            ca_less_stable: get_f64(input_acc, 29),
            sa_stable: get_f64(input_acc, 30),
            sa_less_stable: get_f64(input_acc, 31),
            casa_stable: get_f64(input_acc, 32),
            casa_less_stable: get_f64(input_acc, 33),
            stable_b1: get_f64(input_acc, 34),
            stable_b2: get_f64(input_acc, 35),
            stable_b3: get_f64(input_acc, 36),
            less_stable_b1: get_f64(input_acc, 37),
            less_stable_b2: get_f64(input_acc, 38),
            less_stable_b3: get_f64(input_acc, 39),
            nwd_stable_b1: get_f64(input_acc, 40),
            nwd_stable_b2: get_f64(input_acc, 41),
            nwd_stable_b3: get_f64(input_acc, 42),
            nwd_less_stable_b1: get_f64(input_acc, 43),
            nwd_less_stable_b2: get_f64(input_acc, 44),
            nwd_less_stable_b3: get_f64(input_acc, 45),
            ca_weighted_rate: get_f64(input_acc, 46),
            sa_weighted_rate: get_f64(input_acc, 47),
            td_wd_weighted_rate: get_f64(input_acc, 48),
            td_nwd_weighted_rate: get_f64(input_acc, 49),
            rd_weighted_rate: get_f64(input_acc, 50),
        }
    }
}
///Final Ret or NR Output Structure Read as Input-File
pub struct FinalAccount {
    pub file_id: String,
    pub acc_no: String,
    pub cust_id: i64,
    pub prod_type: String,
    pub currency: String,
    pub mis: String,
    pub mat_date: String,
    pub amount: f64,
    pub lcy_amount: f64,
    pub cust_type: String,
    pub residual_days: u32,
    pub is_nwd: String,
    pub is_nwd_final: String,
    pub bkt_id: String,
}

impl FinalAccount {
    pub fn new(final_input_acc: &[&str]) -> FinalAccount {
        FinalAccount {
            file_id: get_str(final_input_acc, 0),
            acc_no: get_str(final_input_acc, 1),
            cust_id: get_i64(final_input_acc, 2),
            prod_type: get_str(final_input_acc, 3),
            currency: get_str(final_input_acc, 4),
            mis: get_str(final_input_acc, 5),
            mat_date: get_str(final_input_acc, 6),
            amount: get_f64(final_input_acc, 7),
            lcy_amount: get_f64(final_input_acc, 8),
            cust_type: get_str(final_input_acc, 9),
            residual_days: get_str(final_input_acc, 10).parse().unwrap_or(0),
            is_nwd: get_str(final_input_acc, 11),
            is_nwd_final: get_str(final_input_acc, 12),
            bkt_id: get_str(final_input_acc, 13),
        }
    }
}

///LLG-Mapper Input Structure
#[derive(Debug, Clone, Default)]
pub struct LLGMapper {
    pub file_id: String,
    pub wd_stable_llgid: String,
    pub wd_less_stable_llgid: String,
    pub nwd_stable_llgid: String,
    pub nwd_less_stable_llgid: String,
}

impl LLGMapper {
    pub fn new(llg_mapper_data: &[&str]) -> LLGMapper {
        LLGMapper {
            file_id: get_str(llg_mapper_data, 0),
            wd_stable_llgid: get_str(llg_mapper_data, 1),
            wd_less_stable_llgid: get_str(llg_mapper_data, 2),
            nwd_stable_llgid: get_str(llg_mapper_data, 3),
            nwd_less_stable_llgid: get_str(llg_mapper_data, 4),
        }
    }
}

pub fn get_f64(data: &[&str], index: usize) -> f64 {
    data.get(index)
        .unwrap_or_else(|| panic!("Could Not Get {}th Column from Input File", index + 1))
        .to_string()
        .parse()
        .unwrap_or(0.0)
}

pub fn get_str(data: &[&str], index: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| panic!("Could Not Get {}th Column from Input File", index + 1))
        .to_string()
}

pub fn get_i64(data: &[&str], index: usize) -> i64 {
    data.get(index)
        .unwrap_or_else(|| panic!("Could Not Get {}th Column from Input File", index + 1))
        .to_string()
        .parse()
        .unwrap_or(0)
}
