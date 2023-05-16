mod document;
mod editor;
mod terminal;
mod row;
mod row_iterator;
use editor::Editor;
use std::{env, io};
use terminal::Terminal;

fn main() -> io::Result<()> {
    if let Some(maybe_filename) = env::args().skip(1).next() {
       Editor::with_file(&maybe_filename)?.run();
    }
    Ok(())
}
