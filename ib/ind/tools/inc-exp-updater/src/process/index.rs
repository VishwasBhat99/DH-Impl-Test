use chrono::Datelike;

#[derive(Debug, Clone, Default)]
/// The structure in which the updation of indexes to be done
pub struct IndexData {
    pub income_master_index: u32,
    pub common_cap_index: u32,
    pub from_year: i32,
    pub to_year: i32,
}

pub fn get_indexes(date: &rbdate::NaiveDate) -> IndexData {
    let mut from_yr = date.year();
    let mut to_yr = date.year() + 1;
    if vec![1, 2, 3].contains(&date.month()) {
        to_yr = from_yr;
        from_yr -= 1;
    }
    let (income_index, common_index) = match date.month() {
        1 => (11, 10),
        2 => (12, 11),
        3 => (13, 12),
        _ => (date.month() - 2, date.month() - 3),
    };
    IndexData {
        income_master_index: income_index,
        common_cap_index: common_index,
        from_year: from_yr,
        to_year: to_yr,
    }
}
