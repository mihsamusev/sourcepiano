use std::{fs, io};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default, Clone)]
pub struct Row {
    text: String,
    len: usize
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let text = String::from(slice);
        let len = text.graphemes(true).count();
        Row { text, len }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = self.text.len().min(end);
        let start = self.text.len().min(start);
        self.text.graphemes(true).skip(start).take(end - start).collect::<String>()
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
#[derive(Default, Clone)]
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
        let rows: Vec<Row> = text.lines().map(Row::from).collect();
        Ok(Self { rows })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn max_char(&self, index: usize) -> usize {
        self.row(index).map_or(0, |row| row.len().saturating_sub(1))
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}
