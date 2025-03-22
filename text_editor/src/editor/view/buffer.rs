#[derive(Default)]
pub struct Buffer {
    pub data: Vec<String>,
}

impl Buffer {
    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn push(&mut self, string: String) {
        self.data.push(string);
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
