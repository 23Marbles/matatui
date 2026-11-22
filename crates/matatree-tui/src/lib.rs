pub(crate) mod layout;
pub(crate) mod render;

use matatree::{
    editor::{
        CursoredTree, Editor,
        event::{CharInsertOptions, CursorDirection, Event as MatatreeEvent, InsertItem},
    },
    layout::LayoutEngine,
    tree::Tree,
};
use ratatui::{
    crossterm::event::{Event, KeyCode},
    layout::Rect,
};

use crate::{layout::RatatuiLayout, render::RatatuiRender};

pub struct RatatuiEditor {
    tree: matatree::editor::CursoredTree,

    pub x_offset: i16,
}

impl RatatuiEditor {
    pub fn move_left(&mut self) {
        let width =
            RatatuiLayout::layout_sequence(self.tree.tree().root(), Some(&self.tree.cursor()))
                .measurement
                .width;

        let new_x_offset = self.x_offset.saturating_sub(1);

        if -new_x_offset <= width {
            self.x_offset = new_x_offset
        }
    }

    pub fn move_right(&mut self) {
        self.x_offset = self.x_offset.saturating_add(1)
    }
}

impl RatatuiEditor {
    pub fn from_tree(tree: CursoredTree) -> Self {
        Self { tree, x_offset: 0 }
    }
}

impl Editor for RatatuiEditor {
    type Event = Event;

    fn tree(&self) -> &Tree {
        self.tree.tree()
    }

    fn layout<'a, E: matatree::layout::LayoutEngine>(
        &'a self,
    ) -> matatree::layout::MeasurementResult<'a, E> {
        self.tree.layout()
    }

    fn handle_event(&mut self, event: Self::Event) {
        let event = match event {
            Event::Key(key_event) if !key_event.is_release() => match key_event.code {
                KeyCode::Left => MatatreeEvent::MoveCursorLogical {
                    direction: CursorDirection::Left,
                },
                KeyCode::Right => MatatreeEvent::MoveCursorLogical {
                    direction: CursorDirection::Right,
                },
                KeyCode::Backspace => MatatreeEvent::Backspace,
                KeyCode::Char('/') => MatatreeEvent::Insert(InsertItem::Fraction),
                KeyCode::Char('_') => MatatreeEvent::Insert(InsertItem::SubScript),
                KeyCode::Char('^') => MatatreeEvent::Insert(InsertItem::SuperScript),
                KeyCode::Char(ch) => MatatreeEvent::Insert(InsertItem::Char(ch)),
                KeyCode::Delete => MatatreeEvent::Delete,
                _ => return,
            },
            _ => return,
        };

        self.tree
            .handle_event_with_char_opt(event, CharInsertOptions::IMPLICIT_OPERATORS);
    }
}

impl ratatui::widgets::Widget for &'_ RatatuiEditor {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let new_area = Rect {
            x: area.x.saturating_add_signed(self.x_offset),
            ..area
        };

        let clip_area = area.intersection(buf.area);
        let mut clipped_buf = ratatui::buffer::Buffer::empty(clip_area);
        for y in clip_area.y..clip_area.bottom() {
            for x in clip_area.x..clip_area.right() {
                if let (Some(source), Some(destination)) =
                    (buf.cell((x, y)), clipped_buf.cell_mut((x, y)))
                {
                    *destination = source.clone();
                }
            }
        }

        <RatatuiEditor as Editor>::render::<RatatuiRender, RatatuiLayout>(
            self,
            new_area,
            &mut clipped_buf,
        );

        for y in clip_area.y..clip_area.bottom() {
            for x in clip_area.x..clip_area.right() {
                if let (Some(source), Some(destination)) =
                    (clipped_buf.cell((x, y)), buf.cell_mut((x, y)))
                {
                    *destination = source.clone();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use matatree::tree::{Atom, Tree};

    use super::*;

    #[test]
    fn negative_x_offset_preserves_viewport_clipping() {
        let mut tree = Tree::new();
        tree.get_mut_sequence(tree.root_id())
            .unwrap()
            .add_atom(Atom::Text("abcdef".into()));
        let editor = RatatuiEditor {
            tree: CursoredTree::from_tree(tree),
            x_offset: -2,
        };
        let mut buf = ratatui::buffer::Buffer::empty(Rect::new(0, 0, 10, 5));

        <&RatatuiEditor as ratatui::widgets::Widget>::render(
            &editor,
            Rect::new(4, 0, 3, 5),
            &mut buf,
        );

        assert_eq!(buf.cell((3, 0)).unwrap().symbol(), " ");
        assert_eq!(buf.cell((4, 0)).unwrap().symbol(), "b");
        assert_eq!(buf.cell((5, 0)).unwrap().symbol(), "c");
        assert_eq!(buf.cell((6, 0)).unwrap().symbol(), "d");
        assert_eq!(buf.cell((7, 0)).unwrap().symbol(), " ");
    }
}
