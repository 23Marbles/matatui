pub(crate) mod layout;
pub(crate) mod render;

use matatree::{
    editor::{
        CursoredTree, Editor,
        event::{CharInsertOptions, CursorDirection, Event as MatatreeEvent, InsertItem},
    },
    layout::LayoutEngine,
    render::Renderer,
    tree::Tree,
};
use ratatui::{
    crossterm::event::{Event, KeyCode},
    layout::Rect,
};

use crate::{layout::RatatuiLayout, render::RatatuiRender};

fn visual_width(layout: &matatree::layout::MeasurementResult<'_, RatatuiLayout>) -> u16 {
    let cursor_width = layout
        .cursor_pos
        .map_or(0, |(x, _, _)| x.saturating_add(1).max(0) as u16);
    (layout.measurement.width.unsigned_abs() as u16).max(cursor_width)
}

#[derive(Debug, Clone)]
pub struct EditorWidget<'a> {
    tree: &'a matatree::editor::CursoredTree,
    x_offset: u16,
}

impl<'a> From<&'a RatatuiEditor> for EditorWidget<'a> {
    fn from(value: &'a RatatuiEditor) -> Self {
        Self {
            tree: &value.tree,
            x_offset: value.x_offset,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RatatuiEditor {
    tree: matatree::editor::CursoredTree,

    pub x_offset: u16,
}

impl RatatuiEditor {
    pub fn cursored_tree(&self) -> &matatree::editor::CursoredTree {
        &self.tree
    }

    pub fn cursored_tree_mut(&mut self) -> &mut matatree::editor::CursoredTree {
        &mut self.tree
    }

    pub fn from_tree(tree: CursoredTree) -> Self {
        Self { tree, x_offset: 0 }
    }

    pub fn move_left(&mut self) {
        let layout =
            RatatuiLayout::layout_sequence(self.tree.tree().root(), Some(&self.tree.cursor()));
        let width = visual_width(&layout);

        let new_x_offset = self.x_offset.saturating_add(1);

        if new_x_offset <= width {
            self.x_offset = new_x_offset
        }
    }

    pub fn move_right(&mut self) {
        self.x_offset = self.x_offset.saturating_sub(1);
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
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        EditorWidget::from(self).render(area, buf);
    }
}

impl ratatui::widgets::Widget for EditorWidget<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let clip_area = area.intersection(buf.area);
        let layout = self.tree.layout();
        let width = visual_width(&layout);

        if self.x_offset <= area.x && self.x_offset < width {
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

            RatatuiRender::render_layout(
                layout,
                Rect {
                    x: area.x - self.x_offset,
                    ..area
                },
                &mut clipped_buf,
                &matatree::render::RenderContext::none(),
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
            return;
        }

        let mut render_buf =
            ratatui::buffer::Buffer::empty(Rect::new(0, area.y, width, area.height));

        for x in 0..width {
            let destination_x = i32::from(area.x) + i32::from(x) - i32::from(self.x_offset);
            let Ok(destination_x) = u16::try_from(destination_x) else {
                continue;
            };
            if destination_x < clip_area.x || destination_x >= clip_area.right() {
                continue;
            }

            for y in clip_area.y..clip_area.bottom() {
                if let (Some(source), Some(destination)) =
                    (buf.cell((destination_x, y)), render_buf.cell_mut((x, y)))
                {
                    *destination = source.clone();
                }
            }
        }

        RatatuiRender::render_layout(
            layout,
            Rect { x: 0, ..area },
            &mut render_buf,
            &matatree::render::RenderContext::none(),
        );

        for x in 0..width {
            let destination_x = i32::from(area.x) + i32::from(x) - i32::from(self.x_offset);
            let Ok(destination_x) = u16::try_from(destination_x) else {
                continue;
            };
            if destination_x < clip_area.x || destination_x >= clip_area.right() {
                continue;
            }

            for y in clip_area.y..clip_area.bottom() {
                if let (Some(source), Some(destination)) =
                    (render_buf.cell((x, y)), buf.cell_mut((destination_x, y)))
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
            x_offset: 2,
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

    #[test]
    fn visual_scroll_can_move_content_completely_offscreen() {
        let mut tree = Tree::new();
        tree.get_mut_sequence(tree.root_id())
            .unwrap()
            .add_atom(Atom::Text("abc".into()));

        let mut editor = RatatuiEditor::from_tree(CursoredTree::from_tree(tree));
        for _ in 0..3 {
            editor.cursored_tree_mut().move_right();
        }
        for _ in 0..3 {
            editor.move_left();
        }

        assert_eq!(editor.x_offset, 3);

        let mut buf = ratatui::buffer::Buffer::empty(Rect::new(0, 0, 10, 5));
        <&RatatuiEditor as ratatui::widgets::Widget>::render(
            &editor,
            Rect::new(4, 0, 3, 5),
            &mut buf,
        );
        assert_eq!(
            buf.cell((4, 0)).unwrap().bg,
            ratatui::style::Color::LightBlue
        );

        editor.move_left();
        assert_eq!(editor.x_offset, 4);

        let mut buf = ratatui::buffer::Buffer::empty(Rect::new(0, 0, 10, 5));
        <&RatatuiEditor as ratatui::widgets::Widget>::render(
            &editor,
            Rect::new(4, 0, 3, 5),
            &mut buf,
        );

        for x in 4..7 {
            assert_eq!(buf.cell((x, 0)).unwrap().symbol(), " ");
            assert_ne!(
                buf.cell((x, 0)).unwrap().bg,
                ratatui::style::Color::LightBlue
            );
        }

        editor.move_left();
        assert_eq!(editor.x_offset, 4);
    }
}
