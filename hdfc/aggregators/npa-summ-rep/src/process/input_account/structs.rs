use super::statics::DEFAULT_FLOAT;

#[derive(Debug, Clone, Default)]
/// The structure to store all accs from NPA and Write-Off files
pub struct AccData {
    pub source: String,
    pub acc_no: String,
    pub bal_amt: f64,
}

impl AccData {
    pub fn new_npa(acc_data: &[calamine::DataType]) -> AccData {
        AccData {
            source: get_str(&acc_data[1]),
            acc_no: get_str(&acc_data[2]),
            bal_amt: acc_data[4]
                .to_string()
                .parse::<f64>()
                .unwrap_or(DEFAULT_FLOAT),
        }
    }
    pub fn new_writeoff(acc_data: &[calamine::DataType]) -> AccData {
        AccData {
            source: get_str(&acc_data[0]),
            acc_no: get_str(&acc_data[1]),
            bal_amt: acc_data[2]
                .to_string()
                .parse::<f64>()
                .unwrap_or(DEFAULT_FLOAT),
        }
    }
}
#[derive(Debug, Clone, Default)]
/// The structure in which the COA-Data for each product-code is expected in a excel file
pub struct COAData {
    pub prod_code: String,
    pub coa_mapping: String,
}

impl COAData {
    pub fn new(coa_data: &[calamine::DataType]) -> COAData {
        COAData {
            prod_code: get_str(&coa_data[0]),
            coa_mapping: get_str(&coa_data[1]),
        }
    }
    pub fn def() -> COAData {
        ::std::default::Default::default()
    }
}

#[derive(Debug, Clone, Default)]
/// The structure in which the NPA-Data for each acc_no is expected in a excel file
pub struct NPAData {
    pub source_system: String,
    pub acc_no: String,
    pub bal_amt: f64,
    pub specific_prov_amt: f64,
}

impl NPAData {
    pub fn new(npa_data: &[calamine::DataType]) -> NPAData {
        NPAData {
            source_system: get_str(&npa_data[1]),
            acc_no: get_str(&npa_data[2]),
            bal_amt: npa_data[4]
                .to_string()
                .parse::<f64>()
                .unwrap_or(DEFAULT_FLOAT),
            specific_prov_amt: npa_data[7]
                .to_string()
                .parse::<f64>()
                .unwrap_or(DEFAULT_FLOAT),
        }
    }
    pub fn def() -> NPAData {
        ::std::default::Default::default()
    }
}

#[derive(Debug, Clone, Default)]
/// The structure in which the Finnone-Product-to-Division-Mapping-Data for each product is expected in a excel file
pub struct FinnoneProdToDiv {
    pub prod_code: String,
    pub prod_desc: String,
    pub alm: String,
    pub coa: String,
    pub division: String,
    pub al_line: String,
    pub balm_l2: String,
    pub product_id: String,
}

impl FinnoneProdToDiv {
    pub fn new(finnone_data: &[calamine::DataType]) -> FinnoneProdToDiv {
        FinnoneProdToDiv {
            prod_code: get_str(&finnone_data[0]),
            prod_desc: get_str(&finnone_data[1]),
            alm: get_str(&finnone_data[2]),
            coa: get_str(&finnone_data[3]),
            division: get_str(&finnone_data[4]),
            al_line: get_str(&finnone_data[5]),
            balm_l2: get_str(&finnone_data[6]),
            product_id: get_str(&finnone_data[7]),
        }
    }
    pub fn def() -> FinnoneProdToDiv {
        ::std::default::Default::default()
    }
}

#[derive(Debug, Clone, Default)]
/// The structure in which the LNM-Alternative-Acc-Nos for each acc_no is expected in a excel file
pub struct LNMAlternateAccs {
    pub source: String,
    pub acc_no: String,
    pub ref3: String,
    pub acc_name: String,
    pub product_code: String,
    pub ref6: String,
}

impl LNMAlternateAccs {
    pub fn new(lnm_data: &[calamine::DataType]) -> LNMAlternateAccs {
        LNMAlternateAccs {
            source: get_str(&lnm_data[0]),
            acc_no: get_str(&lnm_data[1]),
            ref3: get_str(&lnm_data[2]),
            acc_name: get_str(&lnm_data[3]),
            product_code: get_str(&lnm_data[4]),
            ref6: get_str(&lnm_data[5]),
        }
    }
    pub fn def() -> LNMAlternateAccs {
        ::std::default::Default::default()
    }
}

#[derive(Debug, Clone, Default)]
/// The structure in which the Write-Off-Data for each acc_on is expected in a excel file
pub struct WriteOff {
    pub source: String,
    pub acc_no: String,
    pub amount: f64,
    pub tag: String,
    pub tagging: String,
    pub product: String,
    pub month: String,
    pub board_mapping: String,
}

impl WriteOff {
    pub fn new(write_data: &[calamine::DataType]) -> WriteOff {
        WriteOff {
            source: get_str(&write_data[0]),
            acc_no: get_str(&write_data[1]),
            amount: write_data[2]
                .to_string()
                .parse::<f64>()
                .unwrap_or(DEFAULT_FLOAT),
            tag: get_str(&write_data[3]),
            tagging: get_str(&write_data[4]),
            product: get_str(&write_data[5]),
            month: get_str(&write_data[6]),
            board_mapping: get_str(&write_data[7]),
        }
    }
    pub fn def() -> WriteOff {
        ::std::default::Default::default()
    }
}

#[derive(Debug, Clone, Default)]
/// The structure in which the Division-Mapping-Data for each sys_mis1 is expected in a excel file
pub struct DivisionMapping {
    pub sys_mis1: String,
    pub ora_mis1: String,
    pub mis1_desc: String,
}

impl DivisionMapping {
    pub fn new(div_data: &[calamine::DataType]) -> DivisionMapping {
        DivisionMapping {
            sys_mis1: get_str(&div_data[0]),
            ora_mis1: get_str(&div_data[1]),
            mis1_desc: get_str(&div_data[2]),
        }
    }
    pub fn def() -> DivisionMapping {
        ::std::default::Default::default()
    }
}

pub fn get_str(data: &calamine::DataType) -> String {
    data.to_string().trim().to_string()
}
