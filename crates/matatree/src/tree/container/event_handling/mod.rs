mod deletion;

use crate::editor::{cursor::Cursor, event};

use super::*;

impl ContainerMut<'_> {
    pub fn handle_event(self, event: event::Event, cursor: &mut Cursor) -> Option<event::Event> {
        match event {
            event::Event::MoveCursorLogical { .. } => {
                unimplemented!("Should be handled by the editor")
            }
            event::Event::Backspace => deletion::handle_backspace(self, cursor),
            event::Event::Delete => deletion::handle_delete(self, cursor),
            event => Some(event),
        }
    }
}
