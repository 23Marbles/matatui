use std::iter::repeat_n;

use slotmap::SlotMap;

use crate::{
    editor::cursor::{Cursor, CursorControl},
    tree::{
        Atom, Child, Container, ContainerKind, OperatorKind, Sequence, SequenceId, SequenceMut,
        sequence::text::CursorTextMut,
    },
};

type ChildFinder = Box<dyn FnMut(&Child) -> bool>;

struct ContainerBlueprint {
    kind: ContainerKind,
    /// If some if there is a left id expecting everything up to the closest pattern before the insertion point
    /// The function is non-inclusive
    ///
    left_id: Option<(SequenceId, ChildFinder)>,
    /// If some if there is a right id expecting everything up to the closest pattern after the insertion point
    right_id: Option<(SequenceId, ChildFinder)>,
    /// Where to put the cursor after creation
    default_cursor_pos: Cursor,
}

struct ContainerCreator {
    n_ids: usize,
    func: Box<dyn FnOnce(Vec<SequenceId>) -> ContainerBlueprint>,
}

impl ContainerCreator {
    pub fn new<const N: usize>(
        kind_func: impl FnOnce([SequenceId; N]) -> ContainerKind + 'static,
        blueprint_maker: impl FnOnce(ContainerKind, Vec<SequenceId>) -> ContainerBlueprint + 'static,
    ) -> Self {
        Self {
            n_ids: N,
            func: Box::new(|ids| blueprint_maker(kind_func(*ids.as_array().unwrap()), ids)),
        }
    }

    pub fn insert(self, sequence: &mut SequenceMut<'_>, idx: usize, cursor: &mut Cursor) {
        let Self { n_ids, func } = self;

        assert!(
            idx <= sequence.as_ref().child_count(),
            "Insertion index out of bounds"
        );

        let child_seq_ids = repeat_n(SlotMap::insert, n_ids)
            .map(|f| f(&mut sequence.tree.sequences, Sequence::new_unchecked(None)))
            .collect::<Vec<_>>();

        let ContainerBlueprint {
            kind,
            left_id,
            right_id,
            default_cursor_pos,
        } = func(child_seq_ids.clone());

        let container_id = sequence
            .tree
            .containers
            .insert(Container::new_unchecked(sequence.id(), kind));

        for id in child_seq_ids {
            sequence
                .tree
                .sequences
                .get_mut(id)
                .expect("Just inserted this sequence")
                .parent = Some(container_id);
        }

        let removed_before = if let Some((seq_id, mut p)) = left_id {
            let start_idx = sequence.children[..idx]
                .iter()
                .enumerate()
                .rfind(|(_, child)| p(child))
                .map_or(0, |(i, _)| i + 1);

            let moved_children = sequence.children.drain(start_idx..idx).collect::<Vec<_>>();

            for child in &moved_children {
                if let Child::Container(id) = child {
                    sequence
                        .tree
                        .containers
                        .get_mut(*id)
                        .expect("The child just taken has come from the tree so should exist")
                        .parent_id = seq_id
                }
            }

            sequence
                .tree
                .sequences
                .get_mut(seq_id)
                .expect("Self should have a know index")
                .children
                .splice(0..0, moved_children);

            idx - start_idx
        } else {
            0
        };

        if let Some((seq_id, mut p)) = right_id {
            let end_idx = sequence
                .children
                .iter()
                .enumerate()
                .skip(idx)
                .find(|(_, child)| p(child))
                .map_or(0, |(i, _)| i);

            let mut moved_children = sequence.children.drain(idx..end_idx).collect::<Vec<_>>();

            for child in &moved_children {
                if let Child::Container(id) = child {
                    sequence
                        .tree
                        .containers
                        .get_mut(*id)
                        .expect("The child just taken has come from the tree so should exist")
                        .parent_id = seq_id
                }
            }

            sequence
                .tree
                .sequences
                .get_mut(seq_id)
                .expect("Self should have a know index")
                .children
                .append(&mut moved_children);
        }

        sequence
            .children
            .insert(idx - removed_before, Child::Container(container_id));

        *cursor = default_cursor_pos
    }
}

fn container_from_char(ch: char) -> Option<ContainerCreator> {
    match ch {
        '/' => Some(ContainerCreator::new(
            ContainerKind::fraction,
            |kind, ids| ContainerBlueprint {
                kind,
                left_id: Some((
                    ids[0],
                    Box::new(|child| matches!(child, Child::Atom(Atom::Operator(_)))),
                )),
                right_id: None,
                default_cursor_pos: Cursor::new_raw(ids[1], None),
            },
        )),
        _ => None,
    }
}

pub(super) fn handle(sequence: &mut SequenceMut<'_>, ch: char, cursor: &mut Cursor) {
    enum ChildCreator {
        Operator(OperatorKind),
        Container(ContainerCreator),
    }

    let child = if let Some(operator) = OperatorKind::from_char(ch) {
        Some(ChildCreator::Operator(operator))
    } else {
        container_from_char(ch).map(ChildCreator::Container)
    };

    match sequence.text_around_cursor(cursor.child_index()) {
        CursorTextMut::MutText {
            child_idx,
            cursor_idx,
            text,
        } => {
            if let Some(child) = child {
                let offset = if cursor_idx == 0 {
                    0
                } else {
                    let remaining = text.split_off(cursor_idx);

                    if !remaining.is_empty() {
                        sequence.insert_atom(child_idx + 1, Atom::Text(remaining));
                    };

                    1
                };

                match child {
                    ChildCreator::Operator(operator_kind) => {
                        sequence.insert_atom(child_idx + offset, Atom::Operator(operator_kind))
                    }
                    ChildCreator::Container(container_creator) => {
                        container_creator.insert(sequence, child_idx + offset, cursor);
                        return;
                    }
                }
            } else {
                text.insert(cursor_idx, ch);
            }
        }
        CursorTextMut::NoText { child_idx } => {
            if let Some(child) = child {
                match child {
                    ChildCreator::Operator(operator_kind) => sequence
                        .children
                        .insert(child_idx, Child::Atom(Atom::Operator(operator_kind))),
                    ChildCreator::Container(container_creator) => {
                        container_creator.insert(sequence, child_idx, cursor);
                        return;
                    }
                }
            } else {
                sequence.insert_atom(child_idx, Atom::Text(ch.to_string()));
            }
        }
    }

    *cursor = sequence.as_ref().move_right(*cursor).unwrap();
}
