use crate::editor::cursor::Cursor;

use super::super::super::super::*;

pub(super) fn restore(
    cursor: &mut Cursor,
    parent_id: SequenceId,
    parent_idx: usize,
    num_len: usize,
    before_num_text_pos: Option<usize>,
    before_denom_text_pos: Option<usize>,
    numerator: SequenceId,
    denominator: SequenceId,
    children: &[Child],
) {
    *cursor = Cursor::new_raw(
        parent_id,
        match (
            before_num_text_pos,
            cursor.sequence_id() == numerator,
            before_denom_text_pos,
            cursor.sequence_id() == denominator,
        ) {
            (_, true, _, true) => unreachable!("Both sequences cannot be the same"),
            (_, false, _, false) => unreachable!("At least one of the sequences must be a child"),
            (text_idx_opt, true, _, false) => parent_idx
                .checked_sub(if before_num_text_pos.is_some() { 1 } else { 1 })
                .map(|idx| {
                    (
                        idx,
                        text_idx_opt
                            .and_then(|idx| idx.checked_sub(1))
                            .or(match &children[idx] {
                                Child::Atom(Atom::Text(text)) => text.len().checked_sub(1),
                                Child::Container(_) | Child::Atom(Atom::Operator(_)) => None,
                            }),
                    )
                }),
            (other_text_idx_opt, false, text_idx_opt, true) => {
                let parent_idx = parent_idx + num_len;

                parent_idx
                    .checked_sub(if before_denom_text_pos.is_some() {
                        if before_num_text_pos.is_some() { 2 } else { 1 }
                    } else {
                        0
                    })
                    .map(|idx| {
                        (
                            idx,
                            text_idx_opt
                                .map(|idx| {
                                    if let Some(i) = other_text_idx_opt {
                                        idx + i
                                    } else {
                                        idx
                                    }
                                })
                                .and_then(|idx| idx.checked_sub(1))
                                .or(match &children[idx] {
                                    Child::Atom(Atom::Text(text)) => text.len().checked_sub(1),
                                    Child::Container(_) | Child::Atom(Atom::Operator(_)) => None,
                                }),
                        )
                    })
            }
        },
    );
}
