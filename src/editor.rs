use crate::{document::{Document, Row}, Terminal};
use std::io;
use termion::event::Key;

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
}

impl Editor {
    pub fn with_args(args: &[String]) -> Self {
        let document = match args.get(1) {
            Some(maybe_filename) => Document::open(maybe_filename).unwrap_or_default(),
            _ => Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::try_new().expect("failed to get terminal size="),
            cursor_position: Position::default(),
            document,
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error)
            }
            if let Err(error) = self.process_keypress() {
                // is blocking resize
                die(error);
            }
            if self.should_quit {
                Terminal::clear_screen();
                println!("bye homie\r");
                break;
            }
        }
    }

    fn process_keypress(&mut self) -> io::Result<()> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => {
                self.should_quit = true;
            }
            Key::Up | Key::Down | Key::Left | Key::Right => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        let height = self.terminal.height.saturating_sub(1) as usize;
        let width = self.terminal.width.saturating_sub(1) as usize;
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => y = height.min(y.saturating_add(1)),
            Key::Left => x = x.saturating_sub(1),
            Key::Right => x = width.min(x.saturating_add(1)),
            _ => (),
        }
        self.cursor_position = Position::new(x, y);
    }

    fn refresh_screen(&self) -> io::Result<()> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::new(0, 0));
        self.draw_rows();
        Terminal::cursor_position(&self.cursor_position);
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_rows(&self) {
        let height = self.terminal.height as usize;
        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_row(&self, row: &Row) {
        let start = 0;
        let end = self.terminal.width as usize;
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
}

fn die(error: io::Error) {
    Terminal::clear_screen();
    panic!("dunno what happened, {:?}", error);
}
