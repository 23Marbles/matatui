use std::{fs, io::Read, path::PathBuf};

use clap::Parser;
use matatree_tui::RatatuiEditor;

use crate::app::DebugApp;

mod app;

#[derive(Debug, clap::Parser)]
struct Args {
    #[arg(long, short)]
    save_path: Option<PathBuf>,

    #[arg(long, short)]
    load_path: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let Args {
        save_path,
        load_path,
    } = Args::parse();

    let mut app = DebugApp {
        editor: RatatuiEditor::from_tree(load_path.map_or_default(|path| {
            let mut file = fs::OpenOptions::new()
                .read(true)
                .open(path)
                .expect("Failed to open file");

            let mut buf = String::with_capacity(
                file.metadata().expect("Failed to get file metadata").len() as usize,
            );

            file.read_to_string(&mut buf).expect("Failed to read file");

            ron::from_str(&buf).expect("Failed to parse file")
        })),
        save_path,
    };

    ratatui::run(|terminal| app.run(terminal))
}
