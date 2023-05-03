use std::io::{stdout, self};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;

#[derive(Debug, Default)]
pub struct Editor { }


impl Editor {
    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        for key in io::stdin().keys() {
            match key {
                Err(err) => panic!("deeznutz"),
                Ok(key) => match key {
                    Key::Ctrl('q')=> break,
                    Key::Char(c) if c.is_control() => println!("{:?} \r", c as u8 ),
                    _ =>  println!("{:?}\r", key)
                }
            }
        }
    }
}
