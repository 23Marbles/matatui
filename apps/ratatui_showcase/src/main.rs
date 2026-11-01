use std::{fs, path::PathBuf};

use clap::Parser;
use crossterm::event;
use matatree::tree::Tree;
use matatui::RatatuiEditor;

#[derive(Debug, clap::Parser)]
struct Args {
    load_path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let tree: Tree = ron::from_str(&fs::read_to_string(args.load_path)?)?;

    let editor = RatatuiEditor::from_tree(tree);

    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| frame.render_widget(&editor, frame.area()))?;
            if event::read()?.is_key_press() {
                break Ok(());
            }
        }
    })
}
