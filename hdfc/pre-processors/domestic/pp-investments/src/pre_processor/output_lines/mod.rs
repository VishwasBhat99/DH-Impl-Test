pub struct OutputLines {
    pub processed_lines: String,
    pub concat_lines: Vec<String>,
}

impl OutputLines {
    pub fn new() -> OutputLines {
        OutputLines {
            processed_lines: String::new(),
            concat_lines: Vec::new(),
        }
    }
}
