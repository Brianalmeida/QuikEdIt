#![warn(clippy::all, clippy::pedantic)]
mod document;
mod editor;
mod row;
mod terminal;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
pub use document::Document;

use editor::Editor;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    Editor::default();
    Editor::default().run();
}

// https://www.philippflenker.com/hecto-chapter-3/
