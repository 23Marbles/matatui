pub(crate) mod layout;
pub(crate) mod render;

pub struct RatatuiEditor(pub matatree::editor::Editor);

impl ratatui::widgets::Widget for RatatuiEditor {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        todo!()
    }
}
