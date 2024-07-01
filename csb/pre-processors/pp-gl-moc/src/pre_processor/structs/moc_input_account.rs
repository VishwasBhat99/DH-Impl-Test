use calamine::DataType;
#[derive(Debug, Default)]
pub struct MocInputAccount {
    pub gl_cd: String,
    pub dr_bal: String,
    pub cr_bal: String,
    pub amt: String,
    pub ccy: String,
    pub br_cd: String,
    pub typ: String,
    pub remarks: String,
}

impl MocInputAccount {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|",
            self.gl_cd, self.dr_bal, self.cr_bal, self.amt, self.ccy, self.br_cd, self.typ,
        )
    }
}

impl MocInputAccount {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn insert(&mut self, row: &[DataType]) {
        fn get_data(data: &DataType) -> String {
            data.to_string().replace("\u{a0}", "")
        }

        self.gl_cd = get_data(&row[0]);
        self.dr_bal = get_data(&row[2]);
        self.cr_bal = get_data(&row[3]);
        self.amt = get_data(&row[4]);
        self.ccy = get_data(&row[5]);
        self.br_cd = get_data(&row[6]);
        self.typ = get_data(&row[8]);
    }
}
