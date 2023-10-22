mod editor;

use editor::Editor;
mod terminal;
mod document;
pub use document::Document;
mod row;
pub use row::Row;

fn main() {
    Editor::default().run();
}
