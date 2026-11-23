use std::{error::Error, fs, path::PathBuf};

use matatree::editor::Editor;
use matatree_tui::RatatuiEditor;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
};
use ron::ser::PrettyConfig;

#[derive(Clone, Debug)]
pub struct DebugApp {
    pub editor: RatatuiEditor,

    pub save_path: Option<PathBuf>,
}

impl DebugApp {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn Error + 'static>> {
        loop {
            terminal.draw(|frame| {
                frame.render_widget(
                    &self.editor,
                    frame.area().centered(
                        ratatui::layout::Constraint::Percentage(50),
                        ratatui::layout::Constraint::Percentage(80),
                    ),
                )
            })?;
            let event = event::read()?;

            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                }) => {
                    break Ok(());
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('s'),
                    modifiers,
                    kind,
                    ..
                }) if modifiers.contains(KeyModifiers::CONTROL) && kind.is_press() => {
                    if let Some(ref path) = self.save_path {
                        let contents = match ron::ser::to_string_pretty(
                            self.editor.cursored_tree(),
                            PrettyConfig::new(),
                        ) {
                            Ok(s) => s,
                            Err(e) => {
                                log::error!("Serialize error when saving: {e}");
                                continue;
                            }
                        };

                        if let Err(e) = fs::write(path, contents) {
                            log::error!("File write error when saving: {e}")
                        }
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    modifiers,
                    kind,
                    ..
                }) if modifiers.contains(KeyModifiers::CONTROL) && !kind.is_release() => {
                    self.editor.move_right();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    modifiers,
                    kind,
                    ..
                }) if modifiers.contains(KeyModifiers::CONTROL) && !kind.is_release() => {
                    self.editor.move_left();
                }
                event => self.editor.handle_event(event),
            };
        }
    }
}
