use crate::tree::{Atom, ChildRef, SequenceRef};

use super::super::{Cursor, CursorControl};
use super::text_grapheme_count;

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
