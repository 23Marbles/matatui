pub enum Event {
    MoveCursorLogical { directon: CursorDirection },
}

pub enum CursorDirection {
    Left,
    Right,
}
