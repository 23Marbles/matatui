use crate::{
    editor::{cursor::Cursor, event},
    tree::ContainerMut,
};

pub(super) fn handle(
    container: ContainerMut<'_>,
    ch: char,
    _cursor: &mut Cursor,
) -> Option<event::Event> {
    match container.kind {
        crate::tree::ContainerKind::Fraction { .. } => Some(event::Event::InsertChar(ch)),
    }
}
