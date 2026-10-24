use ratatui::DefaultTerminal;

use crate::layout::cursor::Editor;

pub struct App {
    editor: Editor,
}

impl App {
    pub fn new() -> Self {
        Self {
            editor: Editor::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        loop {}

        Ok(())
    }
}
