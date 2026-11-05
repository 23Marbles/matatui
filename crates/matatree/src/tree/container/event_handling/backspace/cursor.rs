use unicode_segmentation::UnicodeSegmentation;

use crate::editor::cursor::Cursor;

use super::super::super::super::*;

fn cursor_text_index(byte_offset: Option<usize>, child: &Child) -> Option<usize> {
    match child {
        Child::Atom(Atom::Text(text)) => match byte_offset {
            Some(byte_offset) => text
                .grapheme_indices(true)
                .take_while(|(index, _)| *index < byte_offset)
                .count()
                .checked_sub(1),
            None => text.graphemes(true).count().checked_sub(1),
        },
        Child::Container(_) | Child::Atom(Atom::Operator(_)) => None,
    }
}

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
                .checked_sub(1)
                .map(|idx| (idx, cursor_text_index(text_idx_opt, &children[idx]))),
            (other_text_idx_opt, false, text_idx_opt, true) => {
                if num_len != 0 {
                    let idx = parent_idx + num_len - 1 - usize::from(other_text_idx_opt.is_some());
                    let text_offset = text_idx_opt.map(|text_idx| {
                        text_idx
                            + if num_len == 1 {
                                other_text_idx_opt.unwrap_or(0)
                            } else {
                                0
                            }
                    });
                    Some((idx, cursor_text_index(text_offset, &children[idx])))
                } else {
                    parent_idx
                        .checked_sub(1)
                        .map(|idx| (idx, cursor_text_index(other_text_idx_opt, &children[idx])))
                }
            }
        },
    );
}
