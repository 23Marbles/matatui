use crate::app::App;

pub mod app;
pub mod event;
pub mod layout;

fn main() -> anyhow::Result<()> {
    let mut app = App::new();

    ratatui::run(|term| app.run(term))
}
