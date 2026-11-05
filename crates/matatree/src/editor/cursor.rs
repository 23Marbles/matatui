mod movement;

#[cfg(test)]
mod tests;

use crate::{
    editor::CursoredTree,
    tree::{SequenceId, Tree},
};

pub trait CursorControl {
    /// Move the input cursor to the left
    fn move_left(&self, cursor: Cursor) -> Option<Cursor>;
    /// Move the input cursor to the right
    fn move_right(&self, cursor: Cursor) -> Option<Cursor>;

    /// Get the leftmost cursor position
    fn leftmost_cursor(&self) -> Cursor;

    /// Get the rightmost cursor position
    fn rightmost_cursor(&self) -> Cursor;
}

#[derive(Debug, Clone, Copy)]
/// The cursor which edits a `Tree`
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cursor {
    sequence_id: SequenceId,

    child_index: Option<(usize, Option<usize>)>,
}

impl Cursor {
    pub fn new(tree: &Tree) -> Self {
        Self {
            sequence_id: tree.root_id(),
            child_index: None,
        }
    }

    pub(crate) fn new_raw(
        sequence_id: SequenceId,
        child_index: Option<(usize, Option<usize>)>,
    ) -> Self {
        Self {
            sequence_id,
            child_index,
        }
    }

    pub fn sequence_id(&self) -> SequenceId {
        self.sequence_id
    }

    pub fn child_index(&self) -> Option<(usize, Option<usize>)> {
        self.child_index
    }
}

impl CursoredTree {
    pub fn move_left(&mut self) {
        if let Some(c) = self
            .tree
            .get_sequence(self.cursor.sequence_id)
            .unwrap()
            .move_left(self.cursor)
        {
            self.cursor = c
        };
    }
    pub fn move_right(&mut self) {
        if let Some(c) = self
            .tree
            .get_sequence(self.cursor.sequence_id)
            .unwrap()
            .move_right(self.cursor)
        {
            self.cursor = c
        };
    }
}
