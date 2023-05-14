use std::{fs, io};
use crate::row::DualRow;

#[derive(Clone)]
pub struct Document {
    rows: Vec<DualRow>,
}

impl  Document {
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Match<'a> {
    Correct(&'a str),
    Wrong(&'a str)
}

struct Substrings<'a> {
    reference: &'a str,
    target: &'a str,
    counter: usize
}

impl<'a> Substrings<'a> {
    fn new(reference: &'a str, target: &'a str) -> Self {
        Self {
            reference,
            target,
            counter: 0
        }
    }
}

impl <'a> Iterator for Substrings<'a> {
   type Item = Match<'a>; 
   fn next(&mut self) -> Option<Self::Item> {
       let mut iter = self.reference.chars()
            .zip(self.target.chars())
            .skip(self.counter)
            .peekable();

        match iter.peek() {
            Some((ref_char, target_char)) => {
                if ref_char == target_char {
                    let size = iter.take_while(|(r, n)| r == n).count();
                    if size > 0 {
                        let start = self.counter;
                        let end = self.counter + size;
                        self.counter += size;
                        Some(Match::Correct(&self.reference[start..end]))
                    } else {
                        None
                    }
                } else {
                    let size = iter.take_while(|(r, n)| r != n).count();
                    if size > 0 {
                        let start = self.counter;
                        let end = self.counter + size;
                        self.counter += size;
                        Some(Match::Wrong(&self.target[start..end]))
                    } else {
                        None
                    }

                }
            },
            None => None
        }
   }
}
        

fn matching_substrings<'a>(reference: &'a str, new: &'a str) -> Vec<Match<'a>> {
    let mut intervals = Vec::with_capacity(reference.len());
    let mut substrings = Substrings::new(reference, new);
    while let Some(result) = substrings.next() {
        intervals.push(result);
    }
    intervals
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn exact_diff() {
        let reference_row = "hello kitty, its me mario";
        let written_row = "helwo kitty, its me   rio";
        let result = matching_substrings(reference_row, written_row);
        assert_eq!(&result, &[
            Match::Correct("hel"),
            Match::Wrong("w"),
            Match::Correct("o kitty, its me "),
            Match::Wrong("  "),
            Match::Correct("rio")
        ])
    }
}
