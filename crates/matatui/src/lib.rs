use matatree::editor::Editor;

use crate::{layout::RatatuiLayout, render::RatatuiRender};

pub(crate) mod layout;
pub(crate) mod render;

pub struct RatatuiEditor(pub matatree::editor::CursoredTree);

impl Editor for RatatuiEditor {
    fn layout<'a, E: matatree::layout::LayoutEngine>(
        &'a self,
    ) -> matatree::layout::MeasurementResult<'a, E> {
        self.0.layout()
    }

    fn handle_event(&mut self, event: matatree::editor::event::Event) {
        self.0.handle_event(event);
    }
}

impl ratatui::widgets::Widget for &'_ RatatuiEditor {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        <RatatuiEditor as Editor>::render::<RatatuiRender, RatatuiLayout>(self, area, buf);
    }
}
