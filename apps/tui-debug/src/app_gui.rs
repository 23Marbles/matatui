use std::{error::Error, fs, path::PathBuf, sync::Arc};

use matatree::editor::{CursoredTree, Editor};
use matatree_tui::RatatuiEditor;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
};
use ron::ser::PrettyConfig;

#[derive(Clone, Debug)]
pub struct DebugApp {
    pub cached_editor: RatatuiEditor,
    pub shared_tree: Arc<parking_lot::RwLock<CursoredTree>>,

    pub immutable: bool,

    pub save_path: Option<PathBuf>,
}

impl DebugApp {
    pub fn update_shared_tree(&self) {
        *self.shared_tree.write() = self.cached_editor.cursored_tree().clone();
    }

    pub fn update_cache(&mut self) {
        *self.cached_editor.cursored_tree_mut() = self.shared_tree.read().to_owned();
    }

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> Result<(), Box<dyn Error + Send + 'static>> {
        loop {
            self.update_cache();

            if let Err(e) = terminal.draw(|frame| {
                frame.render_widget(
                    &self.cached_editor,
                    frame.area().centered(
                        ratatui::layout::Constraint::Percentage(50),
                        ratatui::layout::Constraint::Percentage(80),
                    ),
                )
            }) {
                return Err(Box::new(e));
            };
            let event = match event::read() {
                Ok(e) => e,
                Err(err) => return Err(Box::new(err)),
            };

            if self.immutable {
                if matches!(
                    event,
                    Event::Key(KeyEvent {
                        code: KeyCode::Esc,
                        ..
                    })
                ) {
                    break Ok(());
                } else {
                    continue;
                }
            }

            let mut changed = false;

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
                            self.cached_editor.cursored_tree(),
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
                    self.cached_editor.move_right();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    modifiers,
                    kind,
                    ..
                }) if modifiers.contains(KeyModifiers::CONTROL) && !kind.is_release() => {
                    self.cached_editor.move_left();
                }
                event => {
                    self.cached_editor.handle_event(event);
                    changed = true
                }
            };

            if changed {
                self.update_shared_tree();
            }
        }
    }
}
