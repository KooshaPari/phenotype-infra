//! Command history

pub struct History {
    entries: Vec<String>,
}

impl History {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, entry: String) {
        self.entries.push(entry);
    }

    pub fn all(&self) -> &[String] {
        &self.entries
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
