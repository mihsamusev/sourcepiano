use std::io::{self, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::color;
use termion::cursor;

use crate::editor::Position;

pub struct Terminal {
    pub width: u16,
    pub height: u16,
    _stdout: RawTerminal<io::Stdout>,
}

impl Terminal {
    pub fn try_new() -> io::Result<Self> {
        let size = termion::terminal_size()?;
        Ok(Self {
            width: size.0,
            height: size.1.saturating_sub(2),
            _stdout: io::stdout().into_raw_mode()?,
        })
    }

    pub fn read_key() -> Result<Key, io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All)
    }
    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine)
    }

    pub fn flush() -> io::Result<()> {
        io::stdout().flush()
    }

    pub fn cursor_position(position: Position) {
        let x = position.x.saturating_add(1) as u16;
        let y = position.y.saturating_add(1) as u16;
        print!("{}", termion::cursor::Goto(x, y))
    }

    pub fn cursor_hide() {
        print!("{}", cursor::Hide);
    }
    pub fn cursor_show() {
        print!("{}", cursor::Show);
    }

    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color))
    }

    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset))
    }

    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color))
    }

    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset))
    }
}
