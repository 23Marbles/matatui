pub mod app;
pub mod command;
pub mod tree;

use std::path::PathBuf;

use clap::Parser;
use eframe::NativeOptions;

use crate::app::DebugApp;

#[derive(Debug, clap::Parser)]
struct Args {
    #[arg(long, short)]
    save_path: Option<PathBuf>,

    #[arg(long, short)]
    load_path: Option<PathBuf>,
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let args = Args::parse();

    

    eframe::run_native(
        "Matatree debugger",
        NativeOptions::default(),
        Box::new(|_| {
            let mut app = args
                .load_path
                .map_or(Ok(DebugApp::new()), |path| DebugApp::load_from_file(&path))?;

            app.with_save_path(args.save_path);

            Ok(Box::new(app))
        }),
    )
}
