#[derive(Debug, Clone)]
///FTPAccIEAmt Structure {Value}
pub struct FTPAccIEAmt {
    pub source: String,
    pub interest_income: f64,
    pub interest_expense: f64,
}

impl FTPAccIEAmt {
    pub fn new(ftp_acc_ie: Vec<&str>) -> FTPAccIEAmt {
        FTPAccIEAmt {
            source: ftp_acc_ie[1].to_string(),
            interest_income: ftp_acc_ie[2].to_string().parse::<f64>().unwrap_or(0.0),
            interest_expense: ftp_acc_ie[3].to_string().parse::<f64>().unwrap_or(0.0),
        }
    }
}
