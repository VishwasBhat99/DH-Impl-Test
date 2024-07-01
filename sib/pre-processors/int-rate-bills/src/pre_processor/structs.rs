use rbdate::NaiveDate;

#[derive(Debug)]
pub struct FeiData {
    pub dc_ref_num: String,
    pub issu_bank_code: String,
    pub bank_name: String,
    pub issu_branch_code: String,
    pub other_bank_ref_num: String,
}
impl Default for FeiData {
    fn default() -> FeiData {
        FeiData {
            dc_ref_num: "".to_string(),
            issu_bank_code: "".to_string(),
            bank_name: "".to_string(),
            issu_branch_code: "".to_string(),
            other_bank_ref_num: "".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FbhKey {
    pub sol_id: String,
    pub bill_id: String,
}

#[derive(Debug)]
pub struct FbhVal {
    pub count: i64,
    pub vfd_bod_date: NaiveDate,
}

impl Default for FbhVal {
    fn default() -> FbhVal {
        FbhVal {
            count: 0,
            vfd_bod_date: NaiveDate::from_ymd(1970, 1, 1),
        }
    }
}

impl FbhVal {
    pub fn add_fbh_value(
        &mut self,
        bill_func: String,
        fd_bod_date: NaiveDate,
        entity_cre_flag: String,
        as_on_date: NaiveDate,
    ) {
        if bill_func == *"K" && fd_bod_date <= as_on_date {
            self.count += 1;
        }
        if bill_func == *"P" && entity_cre_flag == *"Y" && fd_bod_date >= self.vfd_bod_date {
            self.vfd_bod_date = fd_bod_date;
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct FaeKey {
    pub bill_id: String,
    pub sol_id: String,
}
#[derive(Debug, Clone)]
pub struct IdtValue {
    pub int_rate: f64,
    pub int_amt: f64,
    pub int_type: String,
}

impl Default for IdtValue {
    fn default() -> IdtValue {
        IdtValue {
            int_rate: 0.0,
            int_amt: 0.0,
            int_type: "".to_string(),
        }
    }
}
