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
        ora_mis1,
        ora_prod,
        ora_gl,
        ora_catogery,
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
