use super::NaiveDate;

#[derive(Debug, Clone)]
pub struct Method<'a> {
    pub id: i32,
    pub name: &'a str,
    pub curve_pick_date: NaiveDate,
    pub tenor_start_date: NaiveDate,
    pub tenor_end_date: NaiveDate,
}

// Function that return Method Name corresponding to given Method Id
pub fn get_method_name<'a>(id: i32) -> &'a str {
    match id {
        1001 => "Matched Term 1",
        1002 => "Matched Term 2",
        1003 => "Matched Term 3",
        1011 => "Cashflow 1",
        1012 => "Cashflow 2",
        1013 => "Cashflow 3",
        1014 => "Cashflow 4",
        1015 => "Cashflow 5",
        1021 => "Assign Rate 1",
        1022 => "Assign Rate 2",
        1023 => "Assign Rate 3",
        1031 => "Assign Rate with Lock 1",
        1032 => "Assign Rate with Lock 2",
        1033 => "Assign Rate with Lock 3",
        1034 => "Reprice Term with Lock",
        1035 => "Cashflow Lock",
        1036 => "Reprice Term with Lock 2",
        1041 => "Margin Method 1",
        1042 => "Margin Method 2",
        1043 => "Margin Method 3",
        _ => "NA",
    }
}
