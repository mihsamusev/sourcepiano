use crate::{config::Config, document::Document, row::DualRow, row_iterator::DiffPart, Terminal};
use std::{io, ops::Sub};
use termion::event::Key;

#[derive(Default, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Sub<Position> for Position {
    type Output = Position;
    fn sub(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}
pub struct Editor {
    config: Config,
    should_quit: bool,
    terminal: Terminal,
    document_pos: Position,
    offset_to_document_pos: Position,
    document: Document,
    current_char: Option<char>,
}

fn clamp_add(value: usize, step: usize, max_value: usize) -> usize {
    let mut value = value;
    if value + step < max_value {
        value += step
    } else {
        value = max_value
    }
    value
}

impl Editor {
    pub fn with_file(filename: &str) -> io::Result<Self> {
        let document = Document::open(filename)?;

        Ok(Self {
            config: Config::default(),
            should_quit: false,
            terminal: Terminal::try_new().expect("failed to get terminal size="),
            document_pos: Position::default(),
            document,
            offset_to_document_pos: Position::default(),
            current_char: None,
        })
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen().unwrap_or_else(die);
            if self.should_quit {
                break;
            }
            self.process_keypress().unwrap_or_else(die);
        }
    }

    fn process_keypress(&mut self) -> io::Result<()> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => {
                self.should_quit = true;
            }
            Key::Backspace => {
                if let Some(row) = self.document.row_mut(self.document_pos.y) {
                    row.pop_char();
                }
                self.move_cursor(Key::Left);
            }
            Key::Char(c) if c.is_ascii() && !c.is_control() => {
                self.current_char = Some(c);
                if let Some(row) = self.document.row_mut(self.document_pos.y) {
                    row.push_char(c);
                    self.move_cursor(Key::Right)
                }
            }
            Key::Char(c) if c == '\n' => {
                if let Some(row) = self.document.row(self.document_pos.y) {
                    if self.document_pos.x == row.len() {
                        self.move_cursor(Key::Right);
                    }
                }
            }
            _ => (),
        }
        self.scroll();
        Ok(())
    }

    fn refresh_screen(&self) -> io::Result<()> {
        Terminal::cursor_hide();
        Terminal::cursor_position(Position::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("bye homie\r");
        } else {
            self.draw_rows();
            self.draw_status_bar();
            Terminal::cursor_position(self.document_pos - self.offset_to_document_pos);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            let document_row = terminal_row + self.offset_to_document_pos.y;
            if let Some(row) = self.document.row(document_row) {
                if self.document_pos.y >= document_row {
                    self.draw_row_diff(row);
                } else {
                    self.draw_row(row);
                }
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_row_diff(&self, row: &DualRow) {
        let mut colored = String::with_capacity(2 * row.len());
        row.diff_parts().for_each(|part| match part {
            DiffPart::Match(s) => {
                colored.push_str(&self.config.fg_color_match);
                colored.push_str(s);
            }
            DiffPart::Mismatch(s) => {
                colored.push_str(&self.config.fg_color_mismatch);
                colored.push_str(s);
            }
            DiffPart::Untouched(s) => {
                colored.push_str(&self.config.fg_color_default);
                colored.push_str(s);
            }
        });
        colored.push_str(&self.config.fg_color_default);
        println!("{}\r", colored);
    }

    fn draw_row(&self, row: &DualRow) {
        let start = self.offset_to_document_pos.x;
        let end = start + self.terminal.size().width;
        println!("{}\r", row.render(start, end));
    }

    fn draw_status_bar(&self) {
        let Position { x, y } = self.document_pos;
        let position = format!(
            "current char '{}' at {}/{}, line {}/{}",
            self.current_char.unwrap_or_default(),
            x,
            self.document.row(y).map_or(0, |row| row.len()),
            y,
            self.document.len()
        );
        let width = self.terminal.size().width;
        let spaces = " ".repeat(width - position.len());
        println!(
            "{}{}{}{}{}{}\r",
            self.config.bg_color_status_bar,
            self.config.fg_color_status_bar,
            spaces,
            position,
            self.config.fg_color_default,
            self.config.bg_color_default
        );
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { x: x_old, y: y_old } = self.document_pos;
        let (mut x_new, mut y_new) = (x_old, y_old);

        let height = self.document.len();
        let x_max = self.document.row_len(y_old);

        match key {
            Key::Up => {
                y_new = y_old.saturating_sub(1);
                x_new = self.document.row_len(y_new).min(x_new);
            }
            Key::Down => {
                y_new = height.min(y_old.saturating_add(1));
                x_new = self.document.row_len(y_new).min(x_new);
            }
            Key::Left => {
                if x_old == 0 && y_old != 0 {
                    y_new = y_old.saturating_sub(1);
                    x_new = self.document.row_len(y_new);
                } else {
                    x_new = x_old.saturating_sub(1);
                }
            }
            Key::Right => {
                if x_old == x_max && y_old != height {
                    x_new = 0;
                    y_new = height.min(y_old.saturating_add(1));
                } else {
                    x_new = clamp_add(x_old, 1, x_max);
                }
            }
            _ => (),
        };
        self.document_pos = Position::new(x_new, y_new);
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.document_pos;
        let terminal_width = self.terminal.size().width;
        let terminal_height = self.terminal.size().height;
        let mut offset = &mut self.offset_to_document_pos;

        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(terminal_height) {
            offset.y = y.saturating_sub(terminal_height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(terminal_width) {
            offset.x = x.saturating_sub(terminal_width).saturating_add(1);
        }
    }
}

fn die(error: io::Error) {
    Terminal::clear_screen();
    panic!("dunno what happened, {:?}", error);
}
