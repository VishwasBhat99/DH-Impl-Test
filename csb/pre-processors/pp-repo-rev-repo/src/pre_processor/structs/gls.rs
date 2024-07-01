use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GLMaster {
    pub treas_gl_cd: String,
    pub cbs_gl_cd: String,
}

#[derive(Debug, Default)]
pub struct GLMasterMap {
    pub store: HashMap<String, String>,
}

impl GLMasterMap {
    pub fn new() -> Self {
        GLMasterMap {
            store: HashMap::new(),
        }
    }
}
