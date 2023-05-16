use crate::row::DualRow;
use std::{fs, io};

#[derive(Clone)]
pub struct Document {
    rows: Vec<DualRow>,
}

impl Document {
    pub fn open(filename: &str) -> io::Result<Self> {
        let text = fs::read_to_string(filename)?;
        let rows: Vec<DualRow> = text.lines().map(DualRow::from).collect();
        Ok(Self { rows })
    }

    pub fn row(&self, index: usize) -> Option<&DualRow> {
        self.rows.get(index)
    }

    pub fn row_mut(&mut self, index: usize) -> Option<&mut DualRow> {
        self.rows.get_mut(index)
    }

    pub fn max_char(&self, index: usize) -> usize {
        self.row(index).map_or(0, |row| row.len().saturating_sub(1))
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}
