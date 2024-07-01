mod editor;
use editor::Editor;

fn main() {
    Editor::new().unwrap().run();
}
