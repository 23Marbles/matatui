use unicode_segmentation::UnicodeSegmentation;

use crate::editor::{
    cursor::{Cursor, CursorControl},
    event,
};

use super::*;

impl SequenceMut<'_> {
    pub fn handle_event(&mut self, event: event::Event, cursor: &mut Cursor) {
        match event {
            event::Event::MoveCursorLogical { .. } => {
                unimplemented!("Should be handled by the editor")
            }
            event::Event::Backspace => {
                let Some((child_idx, text_idx)) = cursor.child_index() else {
                    return;
                };

                if let Some(new_cursor) = self.as_ref().move_left(*cursor) {
                    *cursor = new_cursor
                }

                let remove_child = match self.get_child_mut(child_idx).unwrap() {
                    ChildMut::Atom(atom) => match atom {
                        Atom::Text(text) => {
                            let (end_idx, s) = text
                                .grapheme_indices(true)
                                .nth(text_idx.expect("A text child should mean a text_idx"))
                                .unwrap();

                            text.drain(end_idx..end_idx + s.len());
                            text.is_empty()
                        }
                        Atom::Operator(_) => true,
                    },
                    ChildMut::Container(Container {
                        kind: ContainerKind::Fraction { .. },
                        ..
                    }) => false,
                };

                if remove_child {
                    match self.children.remove(child_idx) {
                        Child::Atom(_) => {}
                        Child::Container(container_id) => {
                            let _ = self.tree.delete_container(container_id).unwrap();
                        }
                    }
                }

                let _ = join_text_children(&mut self.children, child_idx);
            }
        }
    }
}
