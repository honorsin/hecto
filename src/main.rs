#![warn(clippy::all, clippy::pedantic)]
mod document;
mod editor;
mod row;
mod terminal;
mod highlighting;
mod filetype;
use editor::Editor;

pub use document::Document;
pub use editor::Position;

pub use editor::SearchDirection;
pub use filetype::FileType;
pub use filetype::HighlightingOptions;
pub use row::Row;
pub use terminal::Terminal;
fn main() {
    Editor::default().run();
}
