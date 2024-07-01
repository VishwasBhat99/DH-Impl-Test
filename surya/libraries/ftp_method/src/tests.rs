#[allow(unused)]
use super::*;

#[test]
fn test_get_method_name() {
    assert_eq!(get_method_name(1031), "Assign Rate with Lock 1");
}

#[test]
fn test_get_method_name_default() {
    assert_eq!(get_method_name(1), "NA");
}
