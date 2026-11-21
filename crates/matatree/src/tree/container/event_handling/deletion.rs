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
    let event = (!match container.kind() {
        ContainerKind::Fraction {
            numerator,
            denominator,
        } => {
            *cursor
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
        }
        ContainerKind::SubScript { sequence } | ContainerKind::SuperScript { sequence } => {
            *cursor
                == container
                    .tree
                    .get_sequence(*sequence)
                    .unwrap()
                    .rightmost_cursor()
        }
    })
    .then_some(event::Event::Delete);

    if event.is_none() {
        flatten::flatten_sequences(container, cursor);
    };

    event
}

pub(super) fn handle_backspace(
    container: ContainerMut<'_>,
    cursor: &mut Cursor,
) -> Option<event::Event> {
    let event = (!match container.kind() {
        ContainerKind::Fraction {
            numerator,
            denominator,
        } => {
            cursor.sequence_id() == *numerator && cursor.child_index().is_none()
                || cursor.sequence_id() == *denominator && cursor.child_index().is_none()
        }
        ContainerKind::SubScript { sequence } | ContainerKind::SuperScript { sequence } => {
            cursor.sequence_id() == *sequence && cursor.child_index().is_none()
        }
    })
    .then_some(event::Event::Backspace);

    if event.is_none() {
        flatten::flatten_sequences(container, cursor);
    };

    event
}
