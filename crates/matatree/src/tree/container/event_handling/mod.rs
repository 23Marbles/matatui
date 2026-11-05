mod backspace;

use crate::editor::{cursor::Cursor, event};

use super::*;

impl ContainerMut<'_> {
    pub fn handle_event(self, event: event::Event, cursor: &mut Cursor) -> Option<event::Event> {
        match event {
            event::Event::MoveCursorLogical { .. } => {
                unimplemented!("Should be handled by the editor")
            }
            event::Event::Backspace => backspace::handle(self, cursor),
        }
    }
}
