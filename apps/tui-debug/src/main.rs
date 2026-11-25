#[cfg(feature = "gui")]
use std::{sync::Arc, thread};

use std::{fs, io::Read, path::PathBuf};

#[cfg(feature = "gui")]
use clap::Parser;
#[cfg(not(feature = "gui"))]
use clap::Parser;
#[cfg(feature = "gui")]
use eframe::NativeOptions;
#[cfg(feature = "gui")]
use matatree::editor::CursoredTree;
#[cfg(feature = "gui")]
use matatree_tui::RatatuiEditor;
#[cfg(not(feature = "gui"))]
use matatree_tui::RatatuiEditor;

#[cfg(not(feature = "gui"))]
use crate::app::DebugApp;
#[cfg(feature = "gui")]
use crate::app_gui::DebugApp;

#[cfg(feature = "gui")]
mod app_gui;

#[cfg(not(feature = "gui"))]
mod app;

#[derive(Debug, clap::Parser)]
struct Args {
    #[arg(long, short)]
    save_path: Option<PathBuf>,

    #[arg(long, short)]
    load_path: Option<PathBuf>,

    #[arg(long, short)]
    immutable: bool,

    #[cfg(feature = "gui")]
    #[arg(long)]
    show_gui: bool,

    #[cfg(feature = "gui")]
    #[arg(long)]
    gui_mut: bool,
}

#[cfg(not(feature = "gui"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let Args {
        save_path,
        load_path,
        immutable,
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
        immutable,
    };

    ratatui::run(|terminal| app.run(terminal))
}

#[cfg(feature = "gui")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let Args {
        save_path,
        load_path,
        immutable,
        show_gui,
        gui_mut,
    } = Args::parse();

    let show_gui = show_gui || gui_mut;

    let tree: CursoredTree = load_path.map_or_default(|path| {
        let mut file = fs::OpenOptions::new()
            .read(true)
            .open(path)
            .expect("Failed to open file");

        let mut buf = String::with_capacity(
            file.metadata().expect("Failed to get file metadata").len() as usize,
        );

        file.read_to_string(&mut buf).expect("Failed to read file");

        ron::from_str(&buf).expect("Failed to parse file")
    });

    let shared_tree = Arc::new(parking_lot::RwLock::new(tree.clone()));

    let mut tui_app = DebugApp {
        cached_editor: RatatuiEditor::from_tree(tree.clone()),
        shared_tree: shared_tree.clone(),
        immutable,
        save_path,
    };

    if !show_gui {
        if let Err(e) = ratatui::run(|terminal| tui_app.run(terminal)) {
            return Err(e);
        }
        return Ok(());
    }

    let gui_app = debug::app::DebugApp::from_mirror(tree.clone(), shared_tree, gui_mut);

    let tui_thread = thread::spawn(move || ratatui::run(|terminal| tui_app.run(terminal)));

    if let Err(e) = eframe::run_native(
        "Matatree tui debugger",
        NativeOptions::default(),
        Box::new(move |_| Ok(Box::new(gui_app))),
    ) {
        return Err(Box::new(e));
    };

    if let Err(e) = tui_thread.join().expect("Failed to join thread") {
        return Err(e);
    }

    Ok(())
}
