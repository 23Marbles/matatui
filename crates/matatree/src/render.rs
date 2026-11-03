use crate::{
    editor::cursor::Cursor,
    layout::{LayoutEngine, MeasurementResult},
    tree::{ContainerRef, SequenceRef},
};

pub trait Renderer<L: LayoutEngine> {
    type Rect;

    // The canvas type which is passed to the renderer to draw on; e.g. &mut ratatui::Buffer
    type Canvas;

    fn render_layout(
        layout: MeasurementResult<L>,
        area: Self::Rect,
        canvas: &mut Self::Canvas,
        ctx: &RenderContext<'_>,
    );
    #[inline]
    fn render_sequence(
        sequence: SequenceRef<'_>,
        cursor: Option<&Cursor>,
        area: Self::Rect,
        canvas: &mut Self::Canvas,
        ctx: &RenderContext<'_>,
    ) {
        Self::render_layout(L::layout_sequence(sequence, cursor), area, canvas, ctx);
    }
    #[inline]
    fn render_container(
        container: ContainerRef<'_>,
        cursor: Option<&Cursor>,
        area: Self::Rect,
        canvas: &mut Self::Canvas,
        ctx: &RenderContext<'_>,
    ) {
        Self::render_layout(L::layout_container(container, cursor), area, canvas, ctx);
    }
}

#[derive(Debug, Default)]
pub struct RenderContext<'a> {
    pub defined_functions: &'a [&'a str],
    pub defined_variables: &'a [&'a str],
}

impl RenderContext<'_> {
    pub const fn none() -> Self {
        Self {
            defined_functions: &[],
            defined_variables: &[],
        }
    }
}
