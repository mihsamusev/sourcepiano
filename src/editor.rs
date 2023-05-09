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
        let Position { mut x, mut y } = self.document_pos;
        let height = self.document.len();
        let width = self.document.row(y).map_or(0, |row| row.len());

        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => y = height.min(y.saturating_add(1)),
            Key::Left => x = x.saturating_sub(1),
            Key::Right => x = width.min(x.saturating_add(1)),
            _ => (),
        }
        let width = self.document.row(y).map_or(0, |row| row.len());
        let x = width.min(x);
        self.document_pos = Position::new(x, y);
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
        let height = self.terminal.height as usize;
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self
                .document
                .row(terminal_row + self.offset_to_document_pos.y)
            {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.document_pos;
        let terminal_width = self.terminal.width as usize;
        let terminal_height = self.terminal.height as usize;
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
        let end = start + self.terminal.width as usize;
        println!("{}\r", row.render(start, end))
    }

    fn draw_welcome_message(&self) {
        let mut message = String::from("Deez nutz editor 0.1.0");
        let message_width = message.len();
        let width = self.terminal.width as usize;
        let padding = width.saturating_sub(message_width) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        message = format!("~{}{}", spaces, message);
        message.truncate(width);
        println!("{}\r", message);
    }

    fn draw_status_bar(&self) {
        let Position { x, y } = self.document_pos;
        let position = format!("offset y: {}, line {}/{}",  self.offset_to_document_pos.y, y + 1, self.document.len());
        let width = self.terminal.width as usize;
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
