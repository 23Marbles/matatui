use std::collections::VecDeque;

use unicode_segmentation::UnicodeSegmentation;

use crate::{
    editor::cursor::Cursor,
    tree::{Atom, Child, ContainerKind, ContainerMut},
};

pub(super) fn flatten_sequences(container: ContainerMut<'_>, cursor: &mut Cursor) {
    let parent_id = container.parent_id();

    let ContainerMut {
        id: container_id,
        tree,
    } = container;

    let dst_idx = {
        let children = &mut tree.sequences.get_mut(parent_id).unwrap().children;

        let idx = children.iter()
        .enumerate()
        .find(|(_, c)| matches!(c, Child::Container(container_id) if *container_id == container.id))
        .unwrap()
        .0;

        children.remove(idx);

        idx
    };

    let sequences = match tree.containers.remove(container_id).unwrap().kind {
        ContainerKind::Fraction {
            numerator,
            denominator,
        } => vec![numerator, denominator],
    };

    let mut current_idx = dst_idx;

    let mut cursor_updated = false;

    for id in sequences {
        let insertion_idx = current_idx;
        let cursor_at_start = (id == cursor.sequence_id()).then(|| cursor.child_index());
        let previous_child_cursor = insertion_idx.checked_sub(1).map(|i| {
            (
                i,
                match &tree.sequences.get(parent_id).unwrap().children[i] {
                    Child::Atom(Atom::Text(s)) => s.graphemes(true).count().checked_sub(1),
                    _ => None,
                },
            )
        });
        let mut children = tree
            .get_mut_sequence(id)
            .unwrap()
            .children
            .drain(..)
            .collect::<VecDeque<_>>();

        let dst_children = &mut tree.sequences.get_mut(parent_id).unwrap().children;

        let mut merged_left_graphemes = None;
        if let Some(Child::Atom(Atom::Text(text))) = insertion_idx
            .checked_sub(1)
            .and_then(|i| dst_children.get_mut(i))
            && let Some(Child::Atom(Atom::Text(s))) =
                children.pop_front_if(|child| matches!(child, Child::Atom(Atom::Text(_))))
            {
                merged_left_graphemes = Some(text.graphemes(true).count());
                text.push_str(&s);
            }

        for child in &children {
            match child {
                Child::Atom(_) => (),
                Child::Container(container_id) => {
                    tree.containers.get_mut(*container_id).unwrap().parent_id = parent_id
                }
            }
        }

        let cached_idx = insertion_idx;
        current_idx += children.len();

        dst_children.splice(cached_idx..cached_idx, children);

        if let Some(source_cursor) = cursor_at_start {
            let mapped_cursor = match source_cursor {
                None => previous_child_cursor,
                Some((child_idx, text_idx)) => {
                    let merged_leading_text = merged_left_graphemes.is_some();
                    let mapped_idx = if merged_leading_text {
                        if child_idx == 0 {
                            insertion_idx.checked_sub(1)
                        } else {
                            Some(insertion_idx + child_idx - 1)
                        }
                    } else {
                        Some(insertion_idx + child_idx)
                    };
                    mapped_idx.map(|idx| {
                        (
                            idx,
                            if merged_leading_text && child_idx == 0 {
                                text_idx.map(|i| merged_left_graphemes.unwrap() + i)
                            } else {
                                text_idx
                            },
                        )
                    })
                }
            };
            *cursor = Cursor::new_raw(parent_id, mapped_cursor);
            cursor_updated = true;
        }

        tree.sequences.remove(id);
    }

    let dst_children = &mut tree.sequences.get_mut(parent_id).unwrap().children;

    let mut merged_right_text = false;
    if let Some(Child::Atom(Atom::Text(mut text))) = current_idx
        .checked_sub(1)
        .and_then(|i| dst_children.get(i).cloned())
        && let Some(Child::Atom(Atom::Text(s))) = dst_children.get(current_idx) {
            if !cursor_updated && parent_id == cursor.sequence_id()
                && let Some((child_idx, Some(text_idx))) = cursor.child_index()
                    && child_idx == dst_idx + 1
                {
                    *cursor = Cursor::new_raw(
                        parent_id,
                        Some((
                            current_idx - 1,
                            Some(text_idx + text.graphemes(true).count()),
                        )),
                    );
                    cursor_updated = true
                }

            text.push_str(s);

            dst_children.splice(
                current_idx - 1..=current_idx,
                core::iter::once(Child::Atom(Atom::Text(text))),
            );
            merged_right_text = true;
        }

    if !cursor_updated
        && parent_id == cursor.sequence_id()
        && let Some((child_idx, text_idx)) = cursor.child_index()
        && child_idx > dst_idx
    {
        let inserted_count = current_idx - dst_idx;
        let mapped_idx = if merged_right_text && child_idx == dst_idx + 1 {
            current_idx - 1
        } else {
            child_idx - 1 + inserted_count - usize::from(merged_right_text)
        };
        *cursor = Cursor::new_raw(parent_id, Some((mapped_idx, text_idx)));
    }
}
