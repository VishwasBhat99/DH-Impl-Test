use super::get_data;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GLMapInput {
    pub gl_cd: String,
    pub cbs_gl_cd: String,
}

#[derive(Debug)]
pub struct GLMapData {
    pub cbs_gl_cd: String,
}

impl Default for GLMapData {
    fn default() -> Self {
        GLMapData {
            cbs_gl_cd: String::from("NA"),
        }
    }
}

impl GLMapData {
    pub fn print(&self) -> String {
        format!("{}|", get_data(&self.cbs_gl_cd),)
    }

    pub fn new() -> Self {
        GLMapData {
            ..Default::default()
        }
    }

    pub fn insert(&mut self, gl_map_input: GLMapInput) {
        self.cbs_gl_cd = gl_map_input.cbs_gl_cd;
    }
}

#[derive(Debug, Default)]
pub struct GLMapMap {
    pub store: HashMap<String, GLMapData>,
}

impl GLMapMap {
    pub fn new() -> Self {
        GLMapMap {
            store: HashMap::new(),
        }
    }
}
