use bitflags::bitflags;

use crate::tree::OperatorKind;

bitflags! {
    pub struct CharInsertOptions: u8 {
        /// Allows implicit transformation of `char` to [`OperatorKind`]
        const IMPLICIT_OPERATORS = 1;
        /// Allows implicit transformation of `char` to `ContainerKind`s
        const IMPLICIT_CONTAINERS = 1;
    }
}

pub enum Event {
    MoveCursorLogical { direction: CursorDirection },
    Backspace,
    Delete,
    Insert(InsertItem),
}

pub enum InsertItem {
    Fraction,
    SuperScript,
    SubScript,
    Operator(OperatorKind),
    Char(char),
}

pub enum CursorDirection {
    Left,

    Right,
}
