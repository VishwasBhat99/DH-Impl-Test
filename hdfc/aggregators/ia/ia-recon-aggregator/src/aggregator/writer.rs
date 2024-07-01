use super::{AggregatedMap,Write};
use std::io::BufWriter;
use std::fs::File;

pub fn write_output(aggr_map: &mut AggregatedMap,op_writer: &mut BufWriter<File>) {
    for (key, val) in aggr_map.store.drain() {
        write!(op_writer, "{}|{}\n", key.print(), val.print())
            .expect("Unable to generate summary file.");
    }
}
