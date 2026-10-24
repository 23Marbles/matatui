use unicode_segmentation::UnicodeSegmentation;

use crate::layout::{cursor::Editor, tree::sequence::child::ChildRef};

impl Editor {
    pub fn move_left(&mut self) {
        if self.cursor.text_index != 0
            && let Some((ChildRef::Text(s), _)) = self.get_cursor_node()
        {
            let s = &s[..self.cursor.text_index];

            let mut rev_indices = s.grapheme_indices(true).rev().skip(1);

            let indices = rev_indices.next().unwrap().0;

            self.cursor.text_index = indices
        } else {
            todo!("Move across text boundaries")
        }
    }

    pub fn move_right(&mut self) {
        if let Some((ChildRef::Text(s), start)) = self.get_cursor_node()
            && start != s.len()
        {
            let start = start;

            let trimmed = &s[start..];

            let mut indices = trimmed.grapheme_indices(true);

            let indices = indices.next().map_or(s.len(), |(i, _)| start + i);

            self.cursor.text_index = indices
        } else {
            todo!("Move across text boundaries")
        }
    }

    pub fn move_up(&mut self) {}

    pub fn move_down(&mut self) {}
}
