use crate::{
    document::{Document, Row},
    Terminal,
};
use std::{io, ops::Sub};
use termion::event::Key;
use termion::color;

const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);

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
    should_quit: bool,
    terminal: Terminal,
    document_pos: Position,
    offset_to_document_pos: Position,
    document: Document,
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
    pub fn with_args(args: &[String]) -> Self {
        let document = match args.get(1) {
            Some(maybe_filename) => Document::open(maybe_filename).unwrap_or_default(),
            _ => Document::default(),
        };

        Self {
            should_quit: false,
            terminal: Terminal::try_new().expect("failed to get terminal size="),
            document_pos: Position::default(),
            document,
            offset_to_document_pos: Position::default(),
        }
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
            Key::Up | Key::Down | Key::Left | Key::Right | Key::PageUp | Key::PageDown => {
                self.move_cursor(pressed_key)
            }
            _ => (),
        }
        self.scroll();
        Ok(())
    }


    fn move_cursor(&mut self, key: Key) {
        let Position { x: x_old, y: y_old } = self.document_pos;
        let (mut x_new, mut y_new) = (x_old, y_old);
        
        let height = self.document.len();
        let x_max = self.document.max_char(y_old);
        
        match key {
            Key::Up => {
                y_new = y_old.saturating_sub(1);
                x_new = self.document.max_char(y_new).min(x_new);
            },
            Key::Down => {
                y_new = height.min(y_old.saturating_add(1));
                x_new = self.document.max_char(y_new).min(x_new);
            },
            Key::Left => {
                if x_old == 0 {
                    y_new = y_old.saturating_sub(1);
                    x_new = self.document.max_char(y_new); 
                } else {
                    x_new = x_old.saturating_sub(1);
                }
            }
            Key::Right => {
                if x_old == x_max {
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

    fn refresh_screen(&self) -> io::Result<()> {
        Terminal::cursor_hide();
        Terminal::cursor_position(Position::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("bye homie\r");
        } else {
            self.draw_rows();
            self.draw_message_bar();
            self.draw_status_bar();
            Terminal::cursor_position(self.document_pos - self.offset_to_document_pos);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        let start = self.offset_to_document_pos.x;
        let end = start + self.terminal.size().width;

        for terminal_row in 0..height {
            Terminal::clear_current_line();
            let document_row = terminal_row + self.offset_to_document_pos.y;
            if let Some(row) = self
                .document
                .row(document_row)
            {
                if document_row == self.document_pos.y {
                    println!("{}\r", row.render(start, end));
                } else {
                    println!("{}\r", row.render(start, end));
                }
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
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

    fn draw_row(&self, row: &Row) {
        let start = self.offset_to_document_pos.x;
        let end = start + self.terminal.size().width;
        println!("{}\r", row.render(start, end))
    }

    fn draw_welcome_message(&self) {
        let mut message = String::from("Deez nutz editor 0.1.0");
        let message_width = message.len();
        let width = self.terminal.size().width;
        let padding = width.saturating_sub(message_width) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        message = format!("~{}{}", spaces, message);
        message.truncate(width);
        println!("{}\r", message);
    }

    fn draw_status_bar(&self) {
        let Position { x, y } = self.document_pos;
        let position = format!(
            "char {}/{}, line {}/{}",
            x + 1,
            self.document.row(y).map_or(0, |row| row.len()),
            y + 1,
            self.document.len()
        );
        let width = self.terminal.size().width;
        let spaces = " ".repeat(width - position.len());
        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{}{}\r", spaces, position);
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
    }
}

fn die(error: io::Error) {
    Terminal::clear_screen();
    panic!("dunno what happened, {:?}", error);
}
