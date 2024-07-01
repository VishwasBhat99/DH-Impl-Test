use crate::configuration_parameters::ConfigurationParameters;

#[derive(Debug, Clone)]
///Structure which defines InputData
pub struct Account {
    //Input-Fields
    pub card_no: String,
    pub card_status: String,
    pub prd_code: String,
    pub acc_no: String,
    pub outstanding_bal: f64,
    pub emi_amt: f64,
    pub in_date: String,
    pub tenurs: f64,
    pub del_cnt: String,
    pub cif_no: String,
    pub pan_no: String,

    //Fields to be derived
    pub int_rate: f64,
    pub maturity_date: String,
    pub bgl: String,
    pub cgl: String,
    pub branch_code: String,
    pub int_amount: f64,
    pub due_date: String,
    pub group: String,
    pub llg: String,
    pub currency: String,
}

impl Account {
    pub fn new(input_acc: Vec<&str>, config_params: &ConfigurationParameters) -> Account {
        Account {
            card_no: input_acc[0].to_string(),
            card_status: input_acc[1].to_string(),
            prd_code: input_acc[2].to_string(),
            acc_no: input_acc[3].to_string(),
            outstanding_bal: input_acc[4].parse().unwrap_or(0.0),
            emi_amt: input_acc[5].parse().unwrap_or(0.0),
            in_date: rbdate::NaiveDate::parse_from_str(input_acc[6], "%d-%b-%y")
                .expect("Error reading IN-Date expected format DD-MON-YY)")
                .format("%d-%m-%Y")
                .to_string(),
            tenurs: input_acc[7].parse().unwrap_or(0.0),
            del_cnt: input_acc[8].to_string(),
            cif_no: input_acc[9].to_string(),
            pan_no: {
                if input_acc.len() < 11 {
                    "NA".to_string()
                } else {
                    input_acc[10].to_string()
                }
            },
            int_rate: *config_params.int_rate(),
            maturity_date: "NA".to_string(),
            //TODO: Get bgl and cgl from BGL-CGL Data
            bgl: "NA".to_string(),
            cgl: "NA".to_string(),
            branch_code: config_params.branch_code().to_string(),
            int_amount: 0.0,
            due_date: config_params.as_on_date().format("%d-%m-%Y").to_string(),
            //TODO: Get group and llg from Master-Data
            group: "NA".to_string(),
            llg: "NA".to_string(),
            currency: config_params.currency().to_string(),
        }
    }

    pub fn append_data(&mut self, new_data: Self) {
        self.outstanding_bal += new_data.outstanding_bal;
        self.emi_amt += new_data.emi_amt;
    }
}

pub fn format_output(output_rec: &Account) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        output_rec.card_status,
        output_rec.prd_code,
        output_rec.acc_no,
        output_rec.outstanding_bal,
        output_rec.emi_amt,
        output_rec.in_date,
        output_rec.tenurs,
        output_rec.del_cnt,
        output_rec.cif_no,
        output_rec.pan_no,
        output_rec.int_rate,
        output_rec.maturity_date,
        output_rec.bgl,
        output_rec.cgl,
        output_rec.branch_code,
        output_rec.int_amount,
        output_rec.due_date,
        output_rec.group,
        output_rec.llg,
        output_rec.currency
    )
}

#[derive(Debug, Clone, Default)]
///Structure which defines MasterData
pub struct MasterData {
    pub grp: String,
    pub llg: String,
}

impl MasterData {
    pub fn new(grp: String, llg: String) -> Self {
        Self { grp, llg }
    }
    pub fn def() -> MasterData {
        ::std::default::Default::default()
    }
}
