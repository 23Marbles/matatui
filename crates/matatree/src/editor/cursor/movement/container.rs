use crate::tree::{Atom, ChildRef, ContainerKind, ContainerRef};

use super::super::{Cursor, CursorControl};
use super::text_grapheme_count;

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

fn get_left_of_container(container: &ContainerRef<'_>) -> Cursor {
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
