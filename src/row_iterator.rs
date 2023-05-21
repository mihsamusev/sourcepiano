#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiffPart<'a> {
    Match(&'a str),
    Mismatch(&'a str),
    Untouched(&'a str),
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
                        Some(DiffPart::Match(&self.before[start..end]))
                    } else {
                        None
                    }
                } else {
                    let size = iter.take_while(|(r, n)| r != n).count();
                    if size > 0 {
                        let start = self.counter;
                        let end = self.counter + size;
                        self.counter += size;
                        Some(DiffPart::Mismatch(&self.after[start..end]))
                    } else {
                        None
                    }
                }
            }
            None => {
                if self.counter < self.before.len() {
                    let start = self.counter;
                    self.counter = self.before.len();
                    Some(DiffPart::Untouched(&self.before[start..]))
                } else {
                    None
                }
            }
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
                DiffPart::Match("hel"),
                DiffPart::Mismatch("w"),
                DiffPart::Match("o kitty, its me "),
                DiffPart::Mismatch("  "),
                DiffPart::Match("rio")
            ]
        )
    }

    #[test]
    fn before_and_after_are_equal_size_utf8() {
        let before = "здарова пес";
        let after = "здорова ";
        let parts = diff_parts(before, after);
        assert_eq!(
            &parts,
            &[
                DiffPart::Match("зд"),
                DiffPart::Mismatch("о"),
                DiffPart::Match("рова "),
                DiffPart::Untouched(" пес")
            ]
        )
    }

    #[test]
    fn no_after_string() {
        let before = "hello kitty";
        let after = "";
        let parts = diff_parts(before, after);
        assert_eq!(&parts, &[DiffPart::Untouched("hello kitty"),])
    }
    #[test]
    fn after_string_is_smaller() {
        let before = "hello kitty, its me mario";
        let after = "helwo kitty,";
        let parts = diff_parts(before, after);
        assert_eq!(
            &parts,
            &[
                DiffPart::Match("hel"),
                DiffPart::Mismatch("w"),
                DiffPart::Match("o kitty,"),
                DiffPart::Untouched(" its me mario"),
            ]
        )
    }
}
