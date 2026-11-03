use std::{fs, path::PathBuf};

use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use matatree::editor::{CursoredTree, Editor};
use matatui::RatatuiEditor;

#[derive(Debug, clap::Parser)]
struct Args {
    load_path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let tree: CursoredTree = ron::from_str(&fs::read_to_string(args.load_path)?)?;

    let mut editor = RatatuiEditor::from_tree(tree);

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
