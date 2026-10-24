#[derive(Clone, Debug)]
pub enum Event {
    Mouse(MouseEvent),
    Char(char),
}

#[derive(Clone, Debug)]
pub enum MouseEvent {
    Up,
    Down,
    Left,
    Right,
}
