use crate::tree::OperatorKind;

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
