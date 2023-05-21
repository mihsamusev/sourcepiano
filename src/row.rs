use crate::row_iterator::DiffParts;

#[derive(Debug, Clone)]
pub struct DualRow {
    passive: String,
    active: String,
    length: usize,
    active_cursor: usize,
}

impl From<&str> for DualRow {
    fn from(slice: &str) -> Self {
        let length = slice.len();
        Self {
            passive: slice.into(),
            active: String::with_capacity(length),
            length,
            active_cursor: 0,
        }
    }
}
impl DualRow {
    pub fn push_char(&mut self, c: char) {
        if self.active_cursor < self.length {
            self.active.push(c);
            self.active_cursor += 1;
        }
    }

    pub fn pop_char(&mut self) {
        self.active.pop();
        self.active_cursor = self.active_cursor.saturating_sub(1);
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn render(&self, start: usize, end: usize) -> String {
        let end = self.len().min(end);
        let start = self.len().min(start);

        let active_start = self.active_cursor.min(start);
        let active_size = self
            .active_cursor
            .saturating_sub(active_start)
            .min(end - start);
        let passive_start = self.active_cursor.max(start);

        self.active
            .chars()
            .skip(active_start)
            .take(active_size)
            .chain(
                self.passive
                    .chars()
                    .skip(passive_start)
                    .take(end.saturating_sub(passive_start)),
            )
            .collect::<String>()
    }

    pub fn diff_parts(&self) -> DiffParts {
        DiffParts::new(&self.passive, &self.active)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_row() {
        let mut row = DualRow::from("Here is my row");
        row.push_char('a');
        row.push_char('b');

        assert_eq!(row.active, String::from("ab"))
    }

    #[test]
    fn active_part_capped_by_the_passive() {
        let mut row = DualRow::from("Hi");
        row.push_char('a');
        row.push_char('b');
        row.push_char('c');

        assert_eq!(row.active, String::from("ab"))
    }

    #[test]
    fn render_active_and_passive_parts() {
        let mut row = DualRow::from("Hello wørld");
        row.push_char('H');
        row.push_char('w');
        row.push_char('e');
        row.push_char('w');
        row.push_char('l');

        // cursor is in range
        assert_eq!(row.render(0, 12), String::from("Hwewl wørld"));
        assert_eq!(row.render(2, 9), String::from("ewl wør"));

        // cursor is before range start (only passive part captured)
        assert_eq!(row.render(6, 9), String::from("wør"));

        // cursor is after range (only active part captured)
        assert_eq!(row.render(0, 3), String::from("Hwe"));
    }
}
