#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProdDescInput {
    pub llg_id: i64,
    pub prod_cd: String,
    pub prod_name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProdDescOutput {
    pub prod_cd: String,
    pub prod_name: String,
}
