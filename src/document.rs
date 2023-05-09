use std::{fs, io};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
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

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
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
        Ok(Self { rows })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
