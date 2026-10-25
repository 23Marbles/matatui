use crossterm::event;
use ratatui::{DefaultTerminal, Frame};

use crate::{event::Event, layout::cursor::Editor};

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
        loop {
            terminal.draw(|frame| self.render(frame))?;

            let event = event::read()?;

            if let Some(event) = Event::from_crossterm(event) {
                self.editor.handle_event(event)?;
            };
        }
    }

    pub fn render(&self, frame: &mut Frame<'_>) {
        frame.render_widget(&self.editor, frame.area());
    }
}
