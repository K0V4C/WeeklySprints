#![warn(clippy::all, clippy::pedantic)]
use editor::Editor;
mod editor;

fn main() {
    Editor::new().unwrap().run();
}
