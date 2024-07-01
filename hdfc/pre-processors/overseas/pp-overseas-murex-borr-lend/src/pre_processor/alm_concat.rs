#[derive(Debug, Clone, Default)]
pub struct Concat {
    pub ora_mis1: String,
    pub ora_prod: String,
    pub ora_gl: String,
    pub ora_catogery: String,
}

pub fn get_concat(
    ora_mis1: String,
    ora_prod: String,
    ora_gl: String,
    ora_catogery: String,
) -> Concat {
    Concat {
        ora_mis1: ora_mis1,
        ora_prod: ora_prod,
        ora_gl: ora_gl,
        ora_catogery: ora_catogery,
    }
}

pub fn get_concat_line(deal_no: &str, gl: &str, alm_concat: &str) -> String {
    let mut op_line = String::new();
    op_line.push_str("MurexBorrLend|");
    op_line.push_str(deal_no);
    op_line.push('|');
    op_line.push_str(gl);
    op_line.push('|');
    op_line.push_str(alm_concat);
    op_line
}

#[derive(Debug, Clone, Default)]
pub struct ALMMasterFields {
    pub alm_line: String,
    pub ia_line: String,
    pub balm_l2: String,
}

impl ALMMasterFields {
    pub fn new(alm_line: String, ia_line: String, balm_l2: String) -> Self {
        Self {
            alm_line,
            ia_line,
            balm_l2,
        }
    }

    pub fn default() -> Self {
        Self {
            alm_line: String::from("NONE"),
            ia_line: String::from("NONE"),
            balm_l2: String::from("NONE"),
        }
    }
}
