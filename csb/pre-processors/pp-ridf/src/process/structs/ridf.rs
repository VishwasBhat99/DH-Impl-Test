use super::get_data;
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RIDFInput {
    pub sl_no: String,
    pub deposit_number: String,
    pub financial_year: String,
    pub demand_no: String,
    pub deposit_date: String,
    pub administering_inst: String,
    pub gl_code: String,
    pub nature_of_dep: String,
    pub dep_type: String,
    pub int_rate: String,
    pub tenor: String,
    pub tenor_unit: String,
    pub investment_amt: String,
    pub remarks: String,
    pub mat_date: String,
    pub closure_date: String,
    pub mat_amt: String,
    pub net_val: String,
}

#[derive(Debug)]
pub struct RIDFData {
    pub sl_no: String,
    pub deposit_number: String,
    pub financial_year: String,
    pub demand_no: String,
    pub deposit_date: String,
    pub administering_inst: String,
    pub gl_code: String,
    pub nature_of_dep: String,
    pub dep_type: String,
    pub int_rate: String,
    pub tenor: String,
    pub tenor_unit: String,
    pub investment_amt: String,
    pub remarks: String,
    pub mat_date: String,
    pub closure_date: String,
    pub mat_amt: String,
    pub currency: String,
    pub net_val: String,
}

impl RIDFData {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            get_data(&self.sl_no),
            get_data(&self.deposit_number),
            get_data(&self.financial_year),
            get_data(&self.demand_no),
            get_data(&self.deposit_date),
            get_data(&self.administering_inst),
            get_data(&self.gl_code),
            get_data(&self.nature_of_dep),
            get_data(&self.dep_type),
            get_data(&self.int_rate),
            get_data(&self.tenor),
            get_data(&self.tenor_unit),
            get_data(&self.investment_amt),
            get_data(&self.remarks),
            get_data(&self.mat_date),
            get_data(&self.closure_date),
            get_data(&self.mat_amt),
            get_data(&self.currency),
            get_data(&self.net_val),
        )
    }

    pub fn new() -> Self {
        RIDFData {
            ..Default::default()
        }
    }

    pub fn insert(&mut self, ridf_input: RIDFInput) {
        self.sl_no = ridf_input.sl_no;
        self.deposit_number = ridf_input.deposit_number;
        self.financial_year = ridf_input.financial_year;
        self.demand_no = ridf_input.demand_no;
        self.deposit_date = ridf_input.deposit_date;
        self.administering_inst = ridf_input.administering_inst;
        self.gl_code = ridf_input.gl_code;
        self.nature_of_dep = ridf_input.nature_of_dep;
        self.dep_type = ridf_input.dep_type;
        self.int_rate = ridf_input.int_rate;
        self.tenor = ridf_input.tenor;
        self.tenor_unit = ridf_input.tenor_unit;
        self.investment_amt = ridf_input.investment_amt;
        self.remarks = ridf_input.remarks;
        self.mat_date = ridf_input.mat_date;
        self.closure_date = ridf_input.closure_date;
        self.mat_amt = ridf_input.mat_amt;
        self.net_val = ridf_input.net_val;
    }
}

impl Default for RIDFData {
    fn default() -> Self {
        RIDFData {
            sl_no: String::from("NA"),
            deposit_number: String::from("NA"),
            financial_year: String::from("NA"),
            demand_no: String::from("NA"),
            deposit_date: String::from("NA"),
            administering_inst: String::from("NA"),
            gl_code: String::from("NA"),
            nature_of_dep: String::from("NA"),
            dep_type: String::from("NA"),
            int_rate: String::from("NA"),
            tenor: String::from("NA"),
            tenor_unit: String::from("NA"),
            investment_amt: String::from("NA"),
            remarks: String::from("NA"),
            mat_date: String::from("NA"),
            closure_date: String::from("NA"),
            mat_amt: String::from("NA"),
            currency: String::from("INR"),
            net_val: String::from("NA"),
        }
    }
}
