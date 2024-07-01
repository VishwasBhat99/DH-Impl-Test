use calamine::DataType;

///Format in which Output is to be writtens
pub struct SolDimData {
    pub solline_id: String,
    pub sol_name: String,
    pub sol_type: String,
    pub sol_cat1: String,
    pub sol_cat2: String,
    pub sol_cat3: String,
    pub sol_cat4: String,
    pub sol_cat5: String,
    pub hl_ho: String,
    pub hl_ro: String,
    pub hl_ad1: String,
    pub hl_ad2: String,
    pub hl_ad3: String,
}

impl SolDimData {
    pub fn new(sol_data: &[DataType], soltype_mapper: &String) -> SolDimData {
        SolDimData {
            solline_id: sol_data[1].to_string(),
            sol_name: sol_data[2].to_string().trim().to_string(),
            sol_type: soltype_mapper.to_string(),
            sol_cat1: "NA".to_string(),
            sol_cat2: "NA".to_string(),
            sol_cat3: "NA".to_string(),
            sol_cat4: "NA".to_string(),
            sol_cat5: "NA".to_string(),
            hl_ho: "NA".to_string(),
            hl_ro: "NA".to_string(),
            hl_ad1: "NA".to_string(),
            hl_ad2: "NA".to_string(),
            hl_ad3: "NA".to_string(),
        }
    }
}

pub fn format_output(output_rec: &SolDimData) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        output_rec.solline_id,
        output_rec.sol_name,
        output_rec.sol_type,
        output_rec.sol_cat1,
        output_rec.sol_cat2,
        output_rec.sol_cat3,
        output_rec.sol_cat4,
        output_rec.sol_cat5,
        output_rec.hl_ho,
        output_rec.hl_ro,
        output_rec.hl_ad1,
        output_rec.hl_ad2,
        output_rec.hl_ad3,
    )
}
