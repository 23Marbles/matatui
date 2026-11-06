pub enum Event {
    MoveCursorLogical { direction: CursorDirection },
    Backspace,
    InsertChar(char),
}

pub enum CursorDirection {
    Left,

    Right,
}
