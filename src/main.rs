mod editor;

fn main() {
    let editor = editor::Editor::default();
    editor.run();
}

pub struct KeyEvent {
    pub code: KeyCode,
    // pub modifiers: KeyModifiers,
}

pub enum KeyCode {
    Backspace,
    Enter,
    Char(char),
}
