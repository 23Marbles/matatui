mod flatten;

#[cfg(test)]
mod tests;

use crate::editor::{
    cursor::{Cursor, CursorControl},
    event,
};

use super::super::*;

pub(super) fn handle_delete(
    container: ContainerMut<'_>,
    cursor: &mut Cursor,
) -> Option<event::Event> {
    match container.kind() {
        ContainerKind::Fraction {
            numerator,
            denominator,
        } => {
            if *cursor
                == container
                    .tree()
                    .get_sequence(*numerator)
                    .unwrap()
                    .rightmost_cursor()
                || *cursor
                    == container
                        .tree()
                        .get_sequence(*denominator)
                        .unwrap()
                        .rightmost_cursor()
            {
                flatten::flatten_sequences(container, cursor);

                None
            } else {
                Some(event::Event::Delete)
            }
        }
    }
}

pub(super) fn handle_backspace(
    container: ContainerMut<'_>,
    cursor: &mut Cursor,
) -> Option<event::Event> {
    match container.kind() {
        ContainerKind::Fraction {
            numerator,
            denominator,
        } => {
            if cursor.sequence_id() == *numerator && cursor.child_index().is_none()
                || cursor.sequence_id() == *denominator && cursor.child_index().is_none()
            {
                flatten::flatten_sequences(container, cursor);

                None
            } else {
                Some(event::Event::Backspace)
            }
        }
    }
}
