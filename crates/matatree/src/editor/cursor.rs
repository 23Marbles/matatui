use unicode_segmentation::UnicodeSegmentation;

use crate::{
    editor::CursoredTree,
    tree::{Atom, ChildRef, ContainerKind, ContainerRef, SequenceId, SequenceRef, Tree},
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

impl CursorControl for ContainerRef<'_> {
    fn move_left(&self, cursor: Cursor) -> Option<Cursor> {
        let child_id = cursor.sequence_id();

        match self.kind() {
            ContainerKind::Fraction {
                numerator,
                denominator,
            } => {
                if numerator == &child_id {
                    Some(get_left_of_container(self))
                } else if denominator == &child_id {
                    let sequence = self.tree().get_sequence(*numerator).unwrap();

                    Some(sequence.rightmost_cursor())
                } else {
                    return None;
                }
            }
        }
    }
    fn move_right(&self, cursor: Cursor) -> Option<Cursor> {
        let child_id = cursor.sequence_id();

        match self.kind() {
            ContainerKind::Fraction {
                numerator,
                denominator,
            } => {
                if numerator == &child_id {
                    let sequence = self.tree().get_sequence(*denominator).unwrap();

                    Some(sequence.leftmost_cursor())
                } else if denominator == &child_id {
                    let parent = self.parent().unwrap();

                    let (parent_index, _) = parent
                        .children()
                        .into_iter()
                        .enumerate()
                        .find(|(_, child)| {
                            matches!(
                                child,
                                &ChildRef::Container(container_ref) if container_ref.id() == self.id()
                            )
                        })
                        .expect("Parent should have this as a child");

                    Some(Cursor {
                        sequence_id: parent.id(),
                        child_index: Some((parent_index, None)),
                    })
                } else {
                    return None;
                }
            }
        }
    }

    fn leftmost_cursor(&self) -> Cursor {
        match self.kind() {
            ContainerKind::Fraction { numerator, .. } => {
                let sequence = self.tree().get_sequence(*numerator).unwrap();

                sequence.leftmost_cursor()
            }
        }
    }

    fn rightmost_cursor(&self) -> Cursor {
        match self.kind() {
            ContainerKind::Fraction { denominator, .. } => {
                let sequence = self.tree().get_sequence(*denominator).unwrap();

                sequence.rightmost_cursor()
            }
        }
    }
}

impl CursorControl for SequenceRef<'_> {
    fn move_left(&self, mut cursor: Cursor) -> Option<Cursor> {
        assert_eq!(cursor.sequence_id, self.id());

        let mut move_cursor_to_prev_child = false;

        match &mut cursor.child_index {
            Some((0, None | Some(0))) => cursor.child_index = None,
            Some((_, Some(0))) => move_cursor_to_prev_child = true,
            Some((_, Some(text_index))) => *text_index -= 1,
            Some((child_index, None)) => match self.get_child(*child_index).unwrap() {
                ChildRef::Atom(_) => move_cursor_to_prev_child = true,
                ChildRef::Container(container_ref) => cursor = container_ref.rightmost_cursor(),
            },
            None => {
                let parent = self.parent()?;

                cursor = parent.move_left(cursor)?;
            }
        };

        if let Some((idx, text_idx)) = move_cursor_to_prev_child
            .then(|| cursor.child_index.as_mut())
            .flatten()
        {
            *idx -= 1;

            match self.get_child(*idx).unwrap() {
                ChildRef::Atom(Atom::Text(text)) => {
                    *text_idx = text_grapheme_count(text).checked_sub(1)
                }
                ChildRef::Container(_) | ChildRef::Atom(Atom::Operator(_)) => *text_idx = None,
            }
        }

        Some(cursor)
    }
    fn move_right(&self, mut cursor: Cursor) -> Option<Cursor> {
        assert_eq!(cursor.sequence_id, self.id());

        let is_last_idx = match (self.child_count(), cursor.child_index) {
            (0, _) => true,
            (count, Some((idx, _))) if count - 1 == idx => true,
            _ => false,
        };

        let move_onto_next_child = if let Some((ChildRef::Atom(Atom::Text(text)), text_idx)) =
            cursor
                .child_index
                .as_mut()
                .and_then(|(idx, text_idx)| Some((self.get_child(*idx)?, text_idx.as_mut()?)))
        {
            if text_grapheme_count(text)
                .checked_sub(1)
                .map_or(true, |last_idx| last_idx == *text_idx)
            {
                true
            } else {
                *text_idx += 1;

                false
            }
        } else {
            true
        };

        match (move_onto_next_child, is_last_idx) {
            (true, true) => self.parent()?.move_right(cursor),
            (true, false) => {
                let next_idx = cursor
                    .child_index
                    .map_or(0, |(current_idx, _)| current_idx + 1);

                let next_child = self.get_child(next_idx).unwrap();

                match next_child {
                    ChildRef::Atom(atom) => Some(Cursor {
                        child_index: Some((
                            next_idx,
                            match atom {
                                Atom::Text(_) => Some(0),
                                Atom::Operator(_) => None,
                            },
                        )),
                        ..cursor
                    }),
                    ChildRef::Container(container_ref) => Some(container_ref.leftmost_cursor()),
                }
            }
            (false, _) => Some(cursor),
        }
    }

    fn leftmost_cursor(&self) -> Cursor {
        Cursor {
            sequence_id: self.id(),
            child_index: None,
        }
    }

    fn rightmost_cursor(&self) -> Cursor {
        let child = self
            .child_count()
            .checked_sub(1)
            .map(|idx| (self.get_child(idx).unwrap(), idx));

        Cursor {
            sequence_id: self.id(),
            child_index: child.map(|(child, idx)| {
                (
                    idx,
                    match child {
                        ChildRef::Atom(Atom::Text(text)) => {
                            text_grapheme_count(text).checked_sub(1)
                        }
                        ChildRef::Container(_) | ChildRef::Atom(Atom::Operator(_)) => None,
                    },
                )
            }),
        }
    }
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

fn text_grapheme_count(text: &str) -> usize {
    text.graphemes(true).count()
}

fn get_left_of_container(container: &ContainerRef) -> Cursor {
    let parent = container.parent().unwrap();

    let (parent_index, _) = parent
        .children()
        .into_iter()
        .enumerate()
        .find(|(_, child)| {
            matches!(
                child,
                &ChildRef::Container(container_ref) if container_ref.id() == container.id()
            )
        })
        .expect("Parent should have this as a child");

    Cursor {
        sequence_id: parent.id(),
        child_index: parent_index.checked_sub(1).map(|idx| {
            match parent
                .get_child(idx)
                .expect("Is before the current index which does exist")
            {
                ChildRef::Atom(Atom::Text(text)) => (idx, text_grapheme_count(text).checked_sub(1)),
                ChildRef::Container(_) | ChildRef::Atom(Atom::Operator(_)) => (idx, None),
            }
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rightmost_cursor_uses_grapheme_count_for_unicode() {
        let mut tree = Tree::new();
        let mut root = tree.get_mut_sequence(tree.root_id()).unwrap();
        root.add_atom(Atom::Text("é".into()));

        let cursor = tree.root().rightmost_cursor();

        assert_eq!(cursor.child_index, Some((0, Some(0))));
    }

    #[test]
    fn moving_left_from_operator_lands_on_last_text_grapheme() {
        let mut tree = Tree::new();
        {
            let mut root = tree.get_mut_sequence(tree.root_id()).unwrap();
            root.add_atom(Atom::Text("xy".into()));
            root.add_atom(Atom::Operator('+'));
            root.add_atom(Atom::Text("z".into()));
        }

        let root = tree.root();
        let cursor = root.move_right(Cursor::new(&tree)).unwrap();
        let cursor = root.move_right(cursor).unwrap();
        let cursor = root.move_right(cursor).unwrap();
        assert_eq!(cursor.child_index, Some((1, None)));

        let cursor = root.move_left(cursor).unwrap();
        assert_eq!(cursor.child_index, Some((0, Some(1))));

        let cursor = root.move_right(cursor).unwrap();
        assert_eq!(cursor.child_index, Some((1, None)));
        let cursor = root.move_right(cursor).unwrap();
        assert_eq!(cursor.child_index, Some((2, Some(0))));
    }
}
