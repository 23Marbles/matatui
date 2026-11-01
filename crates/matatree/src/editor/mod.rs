pub mod cursor;
pub mod event;

use crate::{
    editor::cursor::Cursor,
    layout::{LayoutEngine, MeasurementResult},
    render::{RenderContext, Renderer},
    tree::Tree,
};

pub struct CursoredTree {
    tree: Tree,

    cursor: Cursor,
}

impl Editor for CursoredTree {
    fn layout<'a, E: LayoutEngine>(&'a self) -> MeasurementResult<'a, E> {
        E::layout_sequence(self.tree.root(), &self.cursor)
    }

    fn handle_event(&mut self, event: event::Event) {
        match event {}
    }
}

pub trait Editor {
    fn layout<'a, E: LayoutEngine>(&'a self) -> MeasurementResult<'a, E>;

    fn handle_event(&mut self, event: event::Event);

    fn render<'a, R: Renderer<E>, E: LayoutEngine>(
        &'a self,
        area: R::Rect,
        canvas: &mut R::Canvas,
    ) {
        R::render_layout(self.layout(), area, canvas, RenderContext::none());
    }

    fn render_with_ctx<'a, R: Renderer<E>, E: LayoutEngine>(
        &'a self,
        area: R::Rect,
        canvas: &mut R::Canvas,
        ctx: impl FnOnce(&Self) -> RenderContext<'_>,
    ) {
        R::render_layout(self.layout(), area, canvas, ctx(&self));
    }
}
