use super::*;

#[derive(Debug, Default)]
pub struct SpreadReader {
    pub ftp_rate: f64,
    pub curve_id_1: i32,
    pub curve_value_1: f32,
    pub curve_id_2: i32,
    pub curve_value_2: f32,
    pub curve_id_3: i32,
    pub curve_value_3: f32,
    pub curve_id_4: i32,
    pub curve_value_4: f32,
    pub curve_id_5: i32,
    pub curve_value_5: f32,
    pub curve_id_6: i32,
    pub curve_value_6: f32,
    pub curve_id_7: i32,
    pub curve_value_7: f32,
    pub int_rate: f64,
    pub spread: f64,
    pub method: i32,
}

pub type OldAccountMap = HashMap<String, SpreadReader>;
impl SpreadReader {
    pub fn new(file_path: &str) -> OldAccountMap {
        let mut old_acc_map: OldAccountMap = HashMap::new();
        let reader = read_file(file_path);
        for (line_num, lines) in reader.lines().enumerate().skip(1) {
            let line = extract_lines(line_num, lines, file_path);
            let fields: Vec<&str> = line.split('|').collect();
            if fields.len() == 19 {
                old_acc_map.insert(
                    fields[0].to_string(),
                    SpreadReader {
                        ftp_rate: parse_f64(fields[1]),
                        curve_id_1: parse_i32(fields[2]),
                        curve_value_1: parse_f32(fields[3]),
                        curve_id_2: parse_i32(fields[4]),
                        curve_value_2: parse_f32(fields[5]),
                        curve_id_3: parse_i32(fields[6]),
                        curve_value_3: parse_f32(fields[7]),
                        curve_id_4: parse_i32(fields[8]),
                        curve_value_4: parse_f32(fields[9]),
                        curve_id_5: parse_i32(fields[10]),
                        curve_value_5: parse_f32(fields[11]),
                        curve_id_6: parse_i32(fields[12]),
                        curve_value_6: parse_f32(fields[13]),
                        curve_id_7: parse_i32(fields[14]),
                        curve_value_7: parse_f32(fields[15]),
                        int_rate: parse_f64(fields[16]),
                        spread: parse_f64(fields[17]),
                        method: parse_i32(fields[18]),
                    },
                );
            }
        }
        old_acc_map
    }
}
