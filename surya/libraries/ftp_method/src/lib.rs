mod structs;
mod tests;

pub use structs::*;

use chrono::NaiveDate;

impl<'a> Method<'a> {
    pub fn new(
        id: i32,
        curve_pick_date: NaiveDate,
        tenor_start_date: NaiveDate,
        tenor_end_date: NaiveDate,
    ) -> Self {
        Method {
            id,
            name: get_method_name(id),
            curve_pick_date,
            tenor_start_date,
            tenor_end_date,
        }
    }
}
