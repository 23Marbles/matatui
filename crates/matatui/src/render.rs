use crate::layout::RatatuiLayout;

pub struct RatatuiRender;

impl matatree::render::Renderer<RatatuiLayout> for RatatuiRender {
    type Rect = ratatui::prelude::Rect;

    type Canvas = ratatui::buffer::Buffer;

    fn render_layout(
        layout: matatree::layout::MeasurementResult<RatatuiLayout>,
        area: Self::Rect,
        canvas: &mut Self::Canvas,
        ctx: matatree::render::RenderContext<'_>,
    ) {
        todo!()
    }
}
