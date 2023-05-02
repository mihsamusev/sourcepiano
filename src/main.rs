use std::io::{Read, stdout, self};
use termion::raw::IntoRawMode;

fn with_ctrl(c: char) -> u8 {
    (c as u8) & 0b0001_0001
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let b = b.expect("bs");
        match b {
            b if b == with_ctrl('q') => break,
            b if (b as char).is_control() => println!("{:?} \r", b),
            _ =>  println!("{:?} ({})\r", b, b as char),
        }
    }
}
