use matatree::editor::{CursoredTree, Editor};
use matatree_tui::RatatuiEditor;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut editor = RatatuiEditor::from_tree(CursoredTree::default());

    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| {
                frame.render_widget(
                    &editor,
                    frame.area().centered(
                        ratatui::layout::Constraint::Percentage(50),
                        ratatui::layout::Constraint::Percentage(80),
                    ),
                )
            })?;
            let event = event::read()?;
            if matches!(
                event,
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    ..
                })
            ) {
                break Ok(());
            };
            editor.handle_event(event);
        }
    })
}
