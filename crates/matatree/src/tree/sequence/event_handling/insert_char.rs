use crate::{
    editor::cursor::{Cursor, CursorControl},
    tree::{
        Atom, Child, Container, ContainerKind, OperatorKind, Sequence, SequenceId, SequenceMut,
        sequence::text::CursorTextMut,
    },
};

fn container_from_char(ch: char) -> Option<(usize, impl FnOnce(Vec<SequenceId>) -> ContainerKind)> {
    match ch {
        '/' => Some((2, |vec: Vec<SequenceId>| {
            ContainerKind::fraction(*vec.as_array().unwrap())
        })),
        _ => None,
    }
}

pub(super) fn handle(sequence: &mut SequenceMut<'_>, ch: char, cursor: &mut Cursor) {
    let child = if let Some(operator) = OperatorKind::from_char(ch) {
        Some(Child::Atom(Atom::Operator(operator)))
    } else if let Some((seq_ids, func)) = container_from_char(ch) {
        let vec: Vec<_> = (0..seq_ids)
            .map(|_| {
                sequence
                    .tree
                    .sequences
                    .insert(Sequence::new_unchecked(None))
            })
            .collect();

        let kind = func(vec.clone());

        let parent = sequence
            .tree
            .containers
            .insert(Container::new_unchecked(sequence.id, kind));

        for id in vec {
            sequence.tree.sequences.get_mut(id).unwrap().parent = Some(parent);
        }

        Some(Child::Container(parent))
    } else {
        None
    };

    match sequence.text_around_cursor(cursor.child_index()) {
        CursorTextMut::MutText {
            child_idx,
            cursor_idx,
            text,
        } => {
            if let Some(child) = child {
                let remaining = text.split_off(cursor_idx);

                if !remaining.is_empty() {
                    sequence.insert_atom(child_idx + 1, Atom::Text(remaining));
                }
                sequence.children.insert(child_idx + 1, child);
            } else {
                text.insert(cursor_idx, ch);
            }
        }
        CursorTextMut::NoText { child_idx } => {
            if let Some(child) = child {
                sequence.children.insert(child_idx, child);
            } else {
                sequence.insert_atom(child_idx, Atom::Text(ch.to_string()));
            }
        }
    }

    *cursor = sequence.as_ref().move_right(*cursor).unwrap();
}
