use crate::row::DualRow;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiffPart<'a> {
    Same(&'a str),
    New(&'a str),
    Ref(&'a str)
}

pub struct DiffParts<'a> {
    before: &'a str,
    after: &'a str,
    counter: usize,
}

impl<'a> DiffParts<'a> {
    pub fn new(before: &'a str, after: &'a str) -> Self {
        Self {
            before,
            after,
            counter: 0,
        }
    }
}

impl<'a> Iterator for DiffParts<'a> {
    type Item = DiffPart<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut iter = self
            .before
            .chars()
            .zip(self.after.chars())
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
                        Some(DiffPart::Same(&self.before[start..end]))
                    } else {
                        None
                    }
                } else {
                    let size = iter.take_while(|(r, n)| r != n).count();
                    if size > 0 {
                        let start = self.counter;
                        let end = self.counter + size;
                        self.counter += size;
                        Some(DiffPart::New(&self.after[start..end]))
                    } else {
                        None
                    }
                }
            }
            None => {
                if self.counter < self.before.len() {
                    let start = self.counter;
                    self.counter = self.before.len();
                    Some(DiffPart::Ref(&self.before[start..]))
                    
                } else {
                    None
                }
            },
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    fn diff_parts<'a>(before: &'a str, after: &'a str) -> Vec<DiffPart<'a>> {
        let mut intervals = Vec::with_capacity(before.len());
        let mut substrings = DiffParts::new(before, after);
        while let Some(result) = substrings.next() {
            intervals.push(result);
        }
        intervals
    }

    #[test]
    fn before_and_after_are_equal_size() {
        let before = "hello kitty, its me mario";
        let after = "helwo kitty, its me   rio";
        let parts = diff_parts(before, after);
        assert_eq!(
            &parts,
            &[
                DiffPart::Same("hel"),
                DiffPart::New("w"),
                DiffPart::Same("o kitty, its me "),
                DiffPart::New("  "),
                DiffPart::Same("rio")
            ]
        )
    }

    #[test]
    fn no_after_string() {
        let before = "hello kitty";
        let after = "";
        let parts = diff_parts(before, after);
        assert_eq!(
            &parts,
            &[
                DiffPart::Ref("hello kitty"),
            ]
        )
    }
    #[test]
    fn after_string_is_smaller() {
        let before = "hello kitty, its me mario";
        let after = "helwo kitty,";
        let parts = diff_parts(before, after);
        assert_eq!(
            &parts,
            &[
                DiffPart::Same("hel"),
                DiffPart::New("w"),
                DiffPart::Same("o kitty,"), 
                DiffPart::Ref(" its me mario"),
            ]
        )
    }
}