use unicode_segmentation::UnicodeSegmentation;

use crate::tree::{Atom, SequenceMut};

pub enum CursorTextMut<'a> {
    MutText {
        child_idx: usize,
        cursor_idx: usize,
        text: &'a mut String,
    },
    NoText {
        child_idx: usize,
    },
}

impl SequenceMut<'_> {
    pub fn text_around_cursor<'a>(
        &'a mut self,
        child_idx: Option<(usize, Option<usize>)>,
    ) -> CursorTextMut<'a> {
        // Is the position after the logical cursor, where the visual cursor is
        let seam = child_idx.map_or((0, 0), |(idx, text_idx)| {
            text_idx.map_or((idx + 1, 0), |text_idx| (idx, text_idx + 1))
        });

        let Some(child) = self.get_child_mut(seam.0) else {
            return CursorTextMut::NoText { child_idx: seam.0 };
        };

        match child {
            crate::tree::ChildMut::Atom(Atom::Text(text)) => {
                let idx = text
                    .grapheme_indices(true)
                    .nth(seam.1)
                    .map_or(text.len(), |(i, _)| i);

                CursorTextMut::MutText {
                    child_idx: seam.0,
                    cursor_idx: idx,
                    text,
                }
            }
            _ => CursorTextMut::NoText { child_idx: seam.0 },
        }
    }
}
