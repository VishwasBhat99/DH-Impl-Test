use aggregator::account_field_names::AccFieldNames;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use sdb_dyn_proto_rdr::reader::types::Type;
use sdb_dyn_proto_rdr::reader::Reader;

pub fn read_pass_through(
    account: &AccountWithCFs,
    keys: &AccFieldNames,
    reader: &Reader,
) -> String {
    let mut pt = String::new();
    let mut string_pt_value: Option<&str>;
    let mut f64_pt_value: Option<f64>;
    let mut i64_pt_value: Option<i64>;

    for pt_key in &keys.pass_through {
        string_pt_value = None;
        f64_pt_value = None;
        i64_pt_value = None;
        match account.get_string_for_key(pt_key) {
            Ok(val) => {
                string_pt_value = Some(val);
            }
            Err(_err) => match account.get_i64_for_key(pt_key) {
                Ok(val) => {
                    let field_type = reader
                        .get_field_type(pt_key)
                        .expect("Cannot determine type for some pass through.");
                    match field_type {
                        Type::String => {
                            i64_pt_value = None;
                        }
                        _ => {
                            i64_pt_value = Some(val);
                        }
                    }
                }
                Err(_err) => {
                    f64_pt_value = Some(
                        account
                            .get_f64_for_key(pt_key)
                            .expect("Error while reading pass through."),
                    );
                }
            },
        };
        if string_pt_value.is_some() {
            pt.push_str(string_pt_value.unwrap_or(""));
        } else if f64_pt_value.is_some() {
            pt.push_str(&f64_pt_value.unwrap_or(0.0).to_string());
        } else if i64_pt_value.is_some() {
            pt.push_str(&i64_pt_value.unwrap_or(0).to_string());
        } else {
            pt.push_str("");
        }
        pt.push_str("|");
    }
    pt
}
