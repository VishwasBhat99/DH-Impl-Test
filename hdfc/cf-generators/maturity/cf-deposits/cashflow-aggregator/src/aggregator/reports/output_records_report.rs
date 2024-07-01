use serde::ser::{Serialize, Serializer, SerializeStruct};

pub struct OutputRecordsWrittenReport{
    pub report: [u32; 6]
}

impl OutputRecordsWrittenReport {
    pub fn new(report: [u32; 6]) -> OutputRecordsWrittenReport {
        OutputRecordsWrittenReport {
            report
        }
    }
}

impl Serialize for OutputRecordsWrittenReport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut s = serializer.serialize_struct("outputRecordsCount", 6)?;
        s.serialize_field("file0", &self.report[0])?;
        s.serialize_field("file1", &self.report[1])?;
        s.serialize_field("file2", &self.report[2])?;
        s.serialize_field("file3", &self.report[3])?;
        s.serialize_field("file4", &self.report[4])?;
        s.serialize_field("file5", &self.report[5])?;
        s.end()
    }
}