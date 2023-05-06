use std::{fs, io};

#[derive(Default)]
pub struct Row {
    text: String,
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        Row {
            text: String::from(value),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = self.text.len().min(end);
        let start = self.text.len().min(start);
        self.text.get(start..end).unwrap_or_default().to_string()
    }
}
#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    pub fn default() -> Self {
        Self {
            rows: vec![Row::from("crikey mate!")],
        }
    }

    pub fn open(filename: &str) -> io::Result<Self> {
        let text = fs::read_to_string(filename)?;
        let rows: Vec<Row> = text.lines().into_iter().map(Row::from).collect();
        Ok(Self {rows})
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
