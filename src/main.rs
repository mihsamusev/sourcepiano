mod config;
mod document;
mod editor;
mod row;
mod row_iterator;
mod terminal;
use editor::Editor;
use std::{env, io};
use terminal::Terminal;

fn main() -> io::Result<()> {
    if let Some(maybe_filename) = env::args().nth(1) {
        Editor::with_file(&maybe_filename)?.run();
    }
    Ok(())
}
