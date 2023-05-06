mod document;
mod editor;
mod terminal;
use editor::Editor;
use terminal::Terminal;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    Editor::with_args(&args).run();
}
