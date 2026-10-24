use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEventKind};

#[derive(Clone, Debug)]
pub enum Event {
    Mouse(MouseEvent),
    Char(char),
    Backspace,
}

#[derive(Clone, Debug)]
pub enum MouseEvent {
    Up,
    Down,
    Left,
    Right,
}

impl Event {
    pub fn from_crossterm(event: CrosstermEvent) -> Option<Event> {
        match event {
            CrosstermEvent::Key(key_event) => {
                (key_event.kind != KeyEventKind::Release).then_some(match key_event.code {
                    KeyCode::Backspace => Event::Backspace,
                    KeyCode::Left => Event::Mouse(MouseEvent::Left),
                    KeyCode::Right => Event::Mouse(MouseEvent::Right),
                    KeyCode::Up => Event::Mouse(MouseEvent::Up),
                    KeyCode::Down => Event::Mouse(MouseEvent::Down),
                    KeyCode::Char(ch) => Event::Char(ch),
                    _ => return None,
                })
            }
            _ => None,
        }
    }
}
