mod document;
mod editor;
mod terminal;
use editor::Editor;
use std::{env};
use terminal::Terminal;


fn main() {
    let args: Vec<String> = env::args().collect();
    Editor::with_args(&args).run();
}
