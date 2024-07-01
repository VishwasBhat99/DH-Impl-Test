pub mod assign_rate;
pub mod margin_method;

pub fn get_out_type(aggr_bal: f64) -> String {
    let mut out_type: String = String::new();

    if aggr_bal >= 20000000.00 && aggr_bal < 50000000.00 {
        out_type = "two_to_five".to_string();
    } else if aggr_bal >= 50000000.00 {
        out_type = "greater_than_five".to_string();
    } else {
        out_type = "less_than_two".to_string();
    }

    out_type
}
