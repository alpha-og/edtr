#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn load(file_path: &str) -> Result<Self, std::io::Error> {
        let file_contents = std::fs::read_to_string(file_path)?;
        let mut vec: Vec<String> = Vec::new();
        for line in file_contents.lines() {
            vec.push(line.to_string());
        }
        Ok(Self { lines: vec })
    }
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
