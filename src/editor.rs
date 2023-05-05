use std::io::{stdout, self, Write};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;

#[derive(Debug, Default)]
pub struct Editor {
    should_quit: bool
}


impl Editor {
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.refresh_screen() {
                die(error)
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
            if self.should_quit {
                println!("bye homie {}", termion::cursor::Goto(1, 1));
                break;
            }

        }
    }

    fn process_keypress(&mut self) -> io::Result<()> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => {self.should_quit = true;} 
            _ => ()
        }
        Ok(())
    }

    fn refresh_screen(& self) -> io::Result<()> {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        io::stdout().flush()
    }
}

fn read_key() -> Result<Key, io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn die(error: io::Error) {
    panic!("dunno what happened, {:?}", error) 
}