extern crate sdb_dyn_proto_rdr;

use self::sdb_dyn_proto_rdr::reader;

#[test]
fn number_of_records_in_test_cf() {
    let mut reader_for_records = reader::Reader::new_at_path(
        "../test_input_resources/metadata.json",
        "../test_input_resources/test.cf",
    );
    let record_reader = reader_for_records.iter();
    let mut record_counter = 0;
    for _record in record_reader {
        record_counter += 1;
    }

    assert_eq!(4, record_counter);
}
