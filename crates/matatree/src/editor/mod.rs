pub mod cursor;
pub mod event;

use crate::{
    editor::cursor::Cursor,
    layout::{LayoutEngine, MeasurementResult},
    render::{RenderContext, Renderer},
    tree::Tree,
};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CursoredTree {
    tree: Tree,

    cursor: Cursor,
}

impl Default for CursoredTree {
    fn default() -> Self {
        let tree = Tree::default();

        Self {
            cursor: Cursor::new(&tree),
            tree: Default::default(),
        }
    }
}

impl CursoredTree {
    pub fn from_tree(tree: Tree) -> Self {
        Self {
            cursor: Cursor::new(&tree),
            tree,
        }
    }

    pub fn tree_mut(&mut self) -> &mut Tree {
        &mut self.tree
    }

    pub fn cursor(&self) -> Cursor {
        self.cursor
    }
}

impl Editor for CursoredTree {
    type Event = event::Event;

    fn layout<'a, E: LayoutEngine>(&'a self) -> MeasurementResult<'a, E> {
        E::layout_sequence(self.tree.root(), Some(&self.cursor))
    }

    fn handle_event(&mut self, event: Self::Event) {
        match event {
            event::Event::MoveCursorLogical { direction } => match direction {
                event::CursorDirection::Left => self.move_left(),
                event::CursorDirection::Right => self.move_right(),
            },
        }
    }

    fn tree(&self) -> &Tree {
        &self.tree
    }
}

pub trait Editor {
    type Event;

    fn layout<'a, E: LayoutEngine>(&'a self) -> MeasurementResult<'a, E>;

    fn handle_event(&mut self, event: Self::Event);

    fn tree(&self) -> &Tree;

    fn render<'a, R: Renderer<E>, E: LayoutEngine>(
        &'a self,
        area: R::Rect,
        canvas: &mut R::Canvas,
    ) {
        R::render_layout(self.layout(), area, canvas, &RenderContext::none());
    }

    fn render_with_ctx<'a, R: Renderer<E>, E: LayoutEngine>(
        &'a self,
        area: R::Rect,
        canvas: &mut R::Canvas,
        ctx: impl FnOnce(&Self) -> RenderContext<'_>,
    ) {
        R::render_layout(self.layout(), area, canvas, &ctx(&self));
    }
}
