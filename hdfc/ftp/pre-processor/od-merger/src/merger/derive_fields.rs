use super::*;

pub fn get_op_line(
    acc: &mut InputAccount,
    non_core_map: &mut HashMap<String, InputAccount>,
) -> String {
    let mut op_line = String::new();

    if let Some(non_core_data) = non_core_map.get(&acc.account_number) {
        let mut avg_vals = AverageValues::new(&acc);
        avg_vals.weighted(&non_core_data);
        op_line.push_str(&acc.print_weighted(&avg_vals));
    } else {
        op_line.push_str(&acc.print());
    }

    op_line
}


pub fn get_op_line_add(
    acc: &mut InputAccountAdditional,
    non_core_map: &mut HashMap<String, InputAccountAdditional>,
) -> String {
    let mut op_line = String::new();
    if let Some(non_core_data) = non_core_map.get(&acc.acc_num) {
        let mut avg_vals = AverageValuesAdd::new(&acc);
        avg_vals.weighted(&non_core_data);
        op_line.push_str(&acc.print_weighted(&avg_vals));
    } else {
        op_line.push_str(&acc.print());
    }

    op_line
}