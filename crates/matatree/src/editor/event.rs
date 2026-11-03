pub enum Event {
    MoveCursorLogical { direction: CursorDirection },
}

pub enum CursorDirection {
    Left,
    Right,
}
