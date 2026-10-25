use anyhow::bail;
use ratatui::widgets::Widget;

use crate::{
    event::{Event, MouseEvent},
    layout::tree::{
        SequenceId, Tree,
        sequence::child::{ChildMut, ChildRef},
    },
};

pub mod navigation;

pub struct Cursor {
    focussed: SequenceId,
    index: usize,
    text_index: usize,
    preferred_x: Option<usize>,
}

impl Cursor {
    pub fn focussed(&self) -> SequenceId {
        self.focussed
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn text_index(&self) -> usize {
        self.text_index
    }
}

pub struct Editor {
    tree: Tree,
    cursor: Cursor,
}

impl Widget for &Editor {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        SequenceWidget {
            seq_ref: self
                .tree
                .get_sequence(self.tree.root())
                .expect("Should have a root"),
            cursor: Some(&self.cursor),
        }
        .render(area, buf);
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

impl Editor {
    pub fn new() -> Self {
        let tree = Tree::new();

        Self {
            cursor: Cursor {
                focussed: tree.root(),
                index: 0,
                text_index: 0,
                preferred_x: None,
            },
            tree,
        }
    }

    pub fn get_global_cursor_offset(&self) -> Option<usize> {
        let mut sequence = self.tree.get_sequence(self.cursor.focussed)?;
        let mut offset = sequence.x_size_before_index(&self.tree, self.cursor.index)?;

        match sequence.get_child(self.cursor.index) {
            Some(_) => offset = offset.checked_add(self.cursor.text_index)?,
            None if self.cursor.text_index == 0 => {}
            None => return None,
        }

        while let Some(container_id) = sequence.parent_id() {
            let container = self.tree.get_container(container_id)?;
            let parent_id = container.parent_id();
            let parent = self.tree.get_sequence(parent_id)?;

            offset =
                offset.checked_add(parent.x_size_before_container(&self.tree, container_id)?)?;
            sequence = parent;
        }

        Some(offset)
    }

    pub fn get_cursor_node(&self) -> Option<(ChildRef<'_>, usize)> {
        Some((
            self.tree
                .get_sequence(self.cursor.focussed)?
                .get_child(self.cursor.index)?,
            self.cursor.text_index,
        ))
    }

    pub fn get_mut_cursor_node(&mut self) -> Option<(ChildMut<'_>, usize)> {
        let seq_mut = self.tree.get_mut_sequence(self.cursor.focussed)?;

        let child = seq_mut.into_child_mut(self.cursor.index)?;

        Some((child, self.cursor.text_index))
    }

    #[inline]
    pub fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                MouseEvent::Up => self.move_up(),
                MouseEvent::Down => self.move_down(),
                MouseEvent::Left => self.move_left(),
                MouseEvent::Right => self.move_right(),
            },
            Event::Char(ch) => {
                if let Some((ChildMut::Text(text), idx)) = self.get_mut_cursor_node() {
                    text.insert(idx, ch);
                    self.cursor.text_index += ch.len_utf8();
                } else {
                    bail!("The focussed node was not text so no characters could be added")
                };
            }
            Event::Backspace => todo!(),
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn editor(index: usize, text_index: usize) -> Editor {
        let tree = super::super::tree::Tree::new();
        let focussed = tree.root();

        Editor {
            tree,
            cursor: Cursor {
                focussed,
                index,
                text_index,
                preferred_x: None,
            },
        }
    }

    #[test]
    fn cursor_at_end_of_empty_root_has_zero_offset() {
        let editor = editor(0, 0);

        assert_eq!(editor.get_global_cursor_offset(), Some(0));
        assert!(editor.get_cursor_node().is_none());
    }

    #[test]
    fn cursor_index_beyond_end_of_root_is_invalid() {
        let editor = editor(1, 0);

        assert_eq!(editor.get_global_cursor_offset(), None);
        assert!(editor.get_cursor_node().is_none());
    }

    #[test]
    fn cursor_at_end_of_sequence_cannot_have_a_text_offset() {
        let editor = editor(0, 1);

        assert_eq!(editor.get_global_cursor_offset(), None);
    }
}
