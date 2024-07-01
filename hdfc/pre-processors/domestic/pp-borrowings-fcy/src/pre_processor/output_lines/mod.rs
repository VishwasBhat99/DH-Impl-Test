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

    pub fn clear(&mut self) {
        self.processed_lines.clear();
        self.concat_lines.clear();
    }
}
