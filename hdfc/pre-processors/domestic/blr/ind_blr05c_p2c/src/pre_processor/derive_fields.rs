use super::{BLRInput};

pub fn get_op_line(blr_data: BLRInput, as_on_date : String) -> String {
    let mut op_line = String::new();
    op_line.push_str(&blr_data.print(as_on_date));
    
    op_line
}
