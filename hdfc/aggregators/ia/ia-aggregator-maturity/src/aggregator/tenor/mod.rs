use self::{rules_input::InputRules, tenor_input::InputTenor};
use super::{extract_lines, macros, read_file, Logger};
use std::collections::HashMap;

pub mod rules_input;
pub mod tenor_input;

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct Tenor {
    pub llg_id: i64,
    pub start_days: i64,
    pub end_days: i64,
    pub is_in_use: String,
}

pub fn get_tenor_map(rules_file: &str, tenor_file: &str, log: &Logger) -> HashMap<Tenor, i64> {
    let mut llg_pool: Vec<i64> = Vec::new();
    let mut rules_reader = read_file(rules_file);
    let mut used_llg: Vec<i64> = Vec::new();
    for (line_num, lines) in rules_reader.deserialize().enumerate() {
        let input_rules: InputRules = extract_lines(line_num, lines, rules_file, log);
        if input_rules.llg_id != 0 {
            llg_pool.push(input_rules.llg_id);
        }
    }

    let mut tenor_map: HashMap<Tenor, i64> = HashMap::new();
    let mut tenor_reader = read_file(tenor_file);
    for (line_num, lines) in tenor_reader.deserialize().enumerate() {
        let input_tenor: InputTenor = extract_lines(line_num, lines, tenor_file, log);
        if input_tenor.is_in_use.to_uppercase() == "N" && !used_llg.contains(&input_tenor.llg_id) {
            used_llg.push(input_tenor.llg_id);
            log_error!(
                log,
                "`is_in_use` flag is `N` for llg_id: `{}`.",
                input_tenor.llg_id
            );
        }

        if llg_pool.contains(&input_tenor.llg_id) && input_tenor.is_in_use.to_uppercase() == "Y" {
            let st_days = (input_tenor.tenor_st_yrs * 365)
                + (input_tenor.tenor_st_mons * 30)
                + input_tenor.tenor_st_days;
            let ed_days = (input_tenor.tenor_end_yrs * 365)
                + (input_tenor.tenor_end_mons * 30)
                + input_tenor.tenor_end_days;
            log_debug!(
                log,
                "llg_id: `{}`|start_days: `{}`|end_days: `{}`|tenor: `{}`",
                input_tenor.llg_id,
                st_days,
                ed_days,
                input_tenor.tenor_id
            );
            tenor_map.insert(
                Tenor {
                    llg_id: input_tenor.llg_id,
                    start_days: st_days,
                    end_days: ed_days,
                    is_in_use: String::from("Y"),
                },
                input_tenor.tenor_id,
            );
        }
    }

    tenor_map
}

pub fn get_tenor(
    tenor_map: &mut HashMap<Tenor, i64>,
    llg_id: i64,
    start_date: i64,
    end_date: i64,
) -> i64 {
    let duration = ((end_date - start_date) / 3600) / 24;
    let mut tenor_val: i64 = 1;
    for (tenor_struct, tenor) in tenor_map.iter() {
        if duration >= tenor_struct.start_days
            && duration < tenor_struct.end_days
            && llg_id == tenor_struct.llg_id
        {
            tenor_val = *tenor;
            break;
        }
    }
    tenor_val
}
