pub enum Event {
    MoveCursorLogical { direction: CursorDirection },
    Backspace,
    Delete,
    InsertChar(char),
}

pub enum CursorDirection {
    Left,

    Right,
}
