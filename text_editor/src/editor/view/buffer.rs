use super::line::Line;

#[derive(Default)]
pub struct Buffer {
    pub data: Vec<Line>,
}

impl Buffer {
    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn push(&mut self, string: &str) {
        self.data.push(Line::from(string));
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    pub fn get_number_of_lines(&self) -> usize {
        self.data.len()
    }
}
