pub enum Event {
    MoveCursorLogical { direction: CursorDirection },
    Backspace,
}

pub enum CursorDirection {
    Left,

    Right,
}
