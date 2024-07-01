use super::DEFAULT_FLOAT;
use calamine::DataType;

pub fn get_alm_master_data(row: &[DataType], op_line: &mut String, tot_amt: &mut f64) {
    fn get_data(data: &DataType) -> String {
        data.to_string().replace("\u{a0}", " ")
    }
    *tot_amt = *tot_amt + get_data(&row[2]).parse::<f64>().unwrap_or(DEFAULT_FLOAT);
    op_line.push_str(&get_data(&row[0]));
    op_line.push_str("|");
    op_line.push_str(&get_data(&row[1]));
    op_line.push_str("|");
    op_line.push_str(&get_data(&row[2]));
    op_line.push_str("|");
    op_line.push_str(&get_data(&row[3]));
    op_line.push_str("|");
    op_line.push_str(&get_data(&row[4]));
    op_line.push_str("|");
    op_line.push_str(&get_data(&row[5]));
    op_line.push_str("\n");
}
