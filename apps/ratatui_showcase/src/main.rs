use std::{fs, path::PathBuf};

use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use matatree::{editor::Editor, tree::Tree};
use matatui::RatatuiEditor;

#[derive(Debug, clap::Parser)]
struct Args {
    load_path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let tree: Tree = ron::from_str(&fs::read_to_string(args.load_path)?)?;

    let mut editor = RatatuiEditor::from_tree(tree);

    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| frame.render_widget(&editor, frame.area()))?;
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
