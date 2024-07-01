#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GlMapInput {
    pub treasury_gl_code: String,
    pub cbs_gl_code: String,
}
