pub mod cursor;

use crate::{
    editor::cursor::Cursor,
    layout::{LayoutEngine, MeasurementResult},
    render::{RenderContext, Renderer},
    tree::Tree,
};

pub struct Editor {
    tree: Tree,

    cursor: Cursor,
}

impl Editor {
    pub fn layout<'a, E: LayoutEngine>(&'a self) -> MeasurementResult<'a, E> {
        E::layout_sequence(self.tree.root(), &self.cursor)
    }

    pub fn render<'a, R: Renderer<E>, E: LayoutEngine>(&'a self, area: R::Rect, canvas: R::Canvas) {
        R::render_layout(self.layout(), area, canvas, RenderContext::none());
    }

    pub fn render_with_ctx<'a, R: Renderer<E>, E: LayoutEngine>(
        &'a self,
        area: R::Rect,
        canvas: R::Canvas,
        ctx: impl FnOnce(&Self) -> RenderContext<'_>,
    ) {
        R::render_layout(self.layout(), area, canvas, ctx(&self));
    }
}
