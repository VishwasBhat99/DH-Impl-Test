#[derive(Debug, Clone)]
pub struct InputAccount {
    pub gl_code: String,
    pub out_bal: f64,
    pub ccy: String,
    pub out_bal_lcy: f64,
    pub branch_code: String,
    pub dr_bal: f64,
    pub cr_bal: f64,
}

impl InputAccount {
    pub fn new(
        gl_code: String,
        out_bal: String,
        ccy: String,
        out_bal_lcy: String,
        branch_code: String,
        dr_bal: String,
        cr_bal: String,
    ) -> InputAccount {
        InputAccount {
            gl_code: gl_code.to_string(),
            out_bal: out_bal.to_string().parse::<f64>().unwrap_or(0.0),
            ccy: ccy.to_string(),
            out_bal_lcy: out_bal_lcy.to_string().parse::<f64>().unwrap_or(0.0),
            branch_code: branch_code.to_string(),
            dr_bal: dr_bal.to_string().parse::<f64>().unwrap_or(0.0),
            cr_bal: cr_bal.to_string().parse::<f64>().unwrap_or(0.0),
        }
    }

    pub fn append_data(&mut self, new_data: Self) {
        self.gl_code = new_data.gl_code;
        self.out_bal += new_data.out_bal;
        self.ccy = new_data.ccy;
        self.out_bal_lcy += new_data.out_bal_lcy;
        self.branch_code = new_data.branch_code;
        self.dr_bal += new_data.dr_bal;
        self.cr_bal += new_data.cr_bal;
    }
}

#[derive(Debug, Clone)]
pub struct FinMap {
    pub gl_code: String,
    pub code_desc: String,
    pub group2: String,
    pub group3: String,
    pub line: String,
}

impl FinMap {
    pub fn new(
        gl_code: String,
        code_desc: String,
        group2: String,
        group3: String,
        line: String,
    ) -> FinMap {
        FinMap {
            gl_code: gl_code.to_string(),
            code_desc: code_desc.to_string(),
            group2: group2.to_string(),
            group3: group3.to_string(),
            line: line.to_string(),
        }
    }
}

impl FinMap {
    pub fn init() -> FinMap {
        FinMap {
            gl_code: "".to_string(),
            code_desc: "".to_string(),
            group2: "".to_string(),
            group3: "".to_string(),
            line: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MocAccount {
    pub gl_code: String,
    pub branch_code: String,
    pub dr_bal: f64,
    pub cr_bal: f64,
    pub net_bal: f64,
    pub ccy: String,
    pub alm_line: String,
}

impl MocAccount {
    pub fn new(
        gl_code: String,
        branch_code: String,
        dr_bal: String,
        cr_bal: String,
        net_bal: String,
        ccy: String,
        alm_line: String,
    ) -> MocAccount {
        MocAccount {
            gl_code: gl_code.to_string(),
            branch_code: branch_code.to_string(),
            dr_bal: dr_bal.to_string().parse::<f64>().unwrap_or(0.0),
            cr_bal: cr_bal.to_string().parse::<f64>().unwrap_or(0.0),
            net_bal: net_bal.to_string().parse::<f64>().unwrap_or(0.0),
            ccy: ccy.to_string(),
            alm_line: alm_line.to_string(),
        }
    }
}
