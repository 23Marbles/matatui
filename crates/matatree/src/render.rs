use crate::{
    editor::cursor::Cursor,
    layout::{self, LayoutEngine, MeasurementResult},
    tree::{ContainerRef, SequenceRef},
};


pub trait Renderer<E: LayoutEngine> {
    type Rect: From<layout::Rect<E::Unit>>;

    // The canvas type which is passed to the rendere to draw on; e.g. &mut ratatui::Buffer
    type Canvas;

    fn render_layout(
        layout: MeasurementResult<E>,
        area: Self::Rect,
        canvas: Self::Canvas,
        ctx: RenderContext<'_>,
    );
    #[inline]
    fn render_sequence(
        sequence: SequenceRef<'_>,
        cursor: &Cursor,
        area: Self::Rect,
        canvas: Self::Canvas,
        ctx: RenderContext<'_>,
    ) {
        Self::render_layout(E::layout_sequence(sequence, cursor), area, canvas, ctx);
    }
    #[inline]
    fn render_container(
        container: ContainerRef<'_>,
        cursor: &Cursor,
        area: Self::Rect,
        canvas: Self::Canvas,
        ctx: RenderContext<'_>,
    ) {
        Self::render_layout(E::layout_container(container, cursor), area, canvas, ctx);
    }
}

pub struct RenderContext<'a> {
    pub defined_functions: &'a [&'a str],
    pub defined_variables: &'a [&'a str],
}
