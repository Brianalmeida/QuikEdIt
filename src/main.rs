#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use editor::Editor;
pub use terminal::Terminal;

fn main() {
    Editor::default();
    Editor::default().run();
}

// https://www.philippflenker.com/hecto-chapter-3/
