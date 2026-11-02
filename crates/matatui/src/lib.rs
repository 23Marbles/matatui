use matatree::{
    editor::{
        CursoredTree, Editor,
        event::{CursorDirection, Event as MatatreeEvent},
    },
    tree::Tree,
};
use ratatui::crossterm::event::{Event, KeyCode};

use crate::{layout::RatatuiLayout, render::RatatuiRender};

pub(crate) mod layout;
pub(crate) mod render;

pub struct RatatuiEditor {
    tree: matatree::editor::CursoredTree,
    x_offset: u16,
}

impl RatatuiEditor {
    pub fn from_tree(tree: CursoredTree) -> Self {
        Self { tree, x_offset: 0 }
    }
}

impl Editor for RatatuiEditor {
    type Event = Event;

    fn layout<'a, E: matatree::layout::LayoutEngine>(
        &'a self,
    ) -> matatree::layout::MeasurementResult<'a, E> {
        self.tree.layout()
    }

    fn handle_event(&mut self, event: Self::Event) {
        let event = match event {
            Event::Key(key_event) => {
                if !key_event.is_release() {
                    match key_event.code {
                        KeyCode::Left => MatatreeEvent::MoveCursorLogical {
                            directon: CursorDirection::Left,
                        },
                        KeyCode::Right => MatatreeEvent::MoveCursorLogical {
                            directon: CursorDirection::Right,
                        },
                        _ => return,
                    }
                } else {
                    return;
                }
            }
            _ => return,
        };

        self.tree.handle_event(event);
    }

    fn tree(&self) -> &Tree {
        self.tree.tree()
    }
}

impl ratatui::widgets::Widget for &'_ RatatuiEditor {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        <RatatuiEditor as Editor>::render::<RatatuiRender, RatatuiLayout>(self, area, buf);
    }
}
