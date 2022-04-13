#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
mod document;
mod row;
use editor::Editor;

pub use terminal::Terminal;
pub use editor::Position;
pub use document::Document;
pub use row::Row;
fn main() {
    dbg!("aaa".to_string().len());
    dbg!("äää".to_string().len());
    dbg!("y̆y̆y̆".to_string().len());
    dbg!("❤❤❤".to_string().len());
   // Editor::default().run();
}
