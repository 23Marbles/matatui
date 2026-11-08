pub mod cursor;
pub mod event;

#[cfg(test)]
mod tests;

use crate::{
    editor::cursor::Cursor,
    layout::{LayoutEngine, MeasurementResult},
    render::{RenderContext, Renderer},
    tree::{SequenceMut, SequenceRef, Tree},
};

pub trait Editor {
    type Event;

    fn tree(&self) -> &Tree;

    fn layout<'a, E: LayoutEngine>(&'a self) -> MeasurementResult<'a, E>;

    fn render<R: Renderer<E>, E: LayoutEngine>(
        &self,
        area: R::Rect,
        canvas: &mut R::Canvas,
    ) {
        R::render_layout(self.layout(), area, canvas, &RenderContext::none());
    }
    fn render_with_ctx<R: Renderer<E>, E: LayoutEngine>(
        &self,
        area: R::Rect,
        canvas: &mut R::Canvas,
        ctx: impl FnOnce(&Self) -> RenderContext<'_>,
    ) {
        R::render_layout(self.layout(), area, canvas, &ctx(self));
    }

    fn handle_event(&mut self, event: Self::Event);
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CursoredTree {
    tree: Tree,

    cursor: Cursor,
}

impl CursoredTree {
    pub fn cursor(&self) -> Cursor {
        self.cursor
    }

    pub fn sequence_at_cursor<'a>(&'a self) -> SequenceRef<'a> {
        self.tree.get_sequence(self.cursor.sequence_id()).unwrap()
    }

    pub fn mut_sequence_at_cursor<'a>(&'a mut self) -> SequenceMut<'a> {
        self.tree
            .get_mut_sequence(self.cursor.sequence_id())
            .unwrap()
    }

    pub fn tree_mut(&mut self) -> &mut Tree {
        &mut self.tree
    }

    pub fn from_tree(tree: Tree) -> Self {
        Self {
            cursor: Cursor::new(&tree),
            tree,
        }
    }
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

impl Editor for CursoredTree {
    type Event = event::Event;

    fn tree(&self) -> &Tree {
        &self.tree
    }

    fn layout<'a, E: LayoutEngine>(&'a self) -> MeasurementResult<'a, E> {
        E::layout_sequence(self.tree.root(), Some(&self.cursor))
    }

    fn handle_event(&mut self, event: Self::Event) {
        match event {
            event::Event::MoveCursorLogical { direction } => match direction {
                event::CursorDirection::Left => self.move_left(),
                event::CursorDirection::Right => self.move_right(),
            },
            event => {
                let (tree, cursor) = (&mut self.tree, &mut self.cursor);
                let sequence = tree.get_mut_sequence(cursor.sequence_id()).unwrap();

                let event = if let Some(container) = sequence.into_parent_mut() {
                    container.handle_event(event, cursor)
                } else {
                    Some(event)
                };

                if let Some(event) = event {
                    let mut sequence = tree.get_mut_sequence(cursor.sequence_id()).unwrap();

                    sequence.handle_event(event, cursor);
                }
            }
        }
    }
}
