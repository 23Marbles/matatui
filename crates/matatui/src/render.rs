use std::iter::repeat_n;

use matatree::{
    layout::{MeasurementResult, NodeKindRef},
    render::RenderContext,
};
use ratatui::{layout::Rect, text::Text, widgets::Widget};

use crate::layout::RatatuiLayout;

pub struct RatatuiRender;

impl matatree::render::Renderer<RatatuiLayout> for RatatuiRender {
    type Rect = ratatui::prelude::Rect;

    type Canvas = ratatui::buffer::Buffer;

    fn render_layout(
        layout: MeasurementResult<RatatuiLayout>,
        area: Self::Rect,
        buf: &mut Self::Canvas,
        // TODO add context handling
        _ctx: &RenderContext<'_>,
    ) {
        match layout.kind {
            NodeKindRef::Text(text) => {
                if let Some(line_y) = area
                    .y
                    .checked_add(layout.measurement.ascent.saturating_sub(1))
                {
                    Text::raw(text).centered().render(
                        Rect {
                            y: line_y,
                            height: 1,
                            width: layout.measurement.width,
                            ..area
                        },
                        buf,
                    );
                };
            }
            NodeKindRef::Operator(ch) => {
                if let Some(line_y) = area
                    .y
                    .checked_add(layout.measurement.ascent.saturating_sub(1))
                {
                    Text::raw(ch.to_string()).centered().render(
                        Rect {
                            y: line_y,
                            height: 1,
                            width: layout.measurement.width,
                            ..area
                        },
                        buf,
                    );
                }
            }
            NodeKindRef::Sequence(positioned_node_refs) => {
                for node_ref in positioned_node_refs {
                    let (Some(x), Some(y)) = (
                        area.x.checked_add(node_ref.x),
                        area.y.checked_add(node_ref.y),
                    ) else {
                        continue;
                    };

                    let rect = Rect {
                        x,
                        y,
                        width: node_ref.measurement.width,
                        height: node_ref.measurement.height(),
                    };

                    if !buf.area.intersects(rect) {
                        continue;
                    }

                    Self::render_layout(
                        MeasurementResult {
                            kind: node_ref.kind,
                            measurement: node_ref.measurement,
                            cursor_pos: None,
                        },
                        rect,
                        buf,
                        _ctx,
                    );
                }
            }
            NodeKindRef::Fraction {
                numerator,
                denominator,
            } => {
                // Numerator
                if let (Some(x), Some(y)) = (
                    area.x.checked_add(numerator.x),
                    area.y.checked_add(numerator.y),
                ) {
                    let numerator_rect = Rect {
                        x,
                        y,
                        width: numerator.measurement.width,
                        height: numerator.measurement.height(),
                    };

                    Self::render_layout(
                        MeasurementResult {
                            kind: numerator.kind,
                            measurement: numerator.measurement,
                            cursor_pos: None,
                        },
                        numerator_rect,
                        buf,
                        _ctx,
                    );
                }

                // Denominator
                if let (Some(x), Some(y)) = (
                    area.x.checked_add(denominator.x),
                    area.y.checked_add(denominator.y),
                ) {
                    let denominator_rect = Rect {
                        x,
                        y,
                        width: denominator.measurement.width,
                        height: denominator.measurement.height(),
                    };

                    Self::render_layout(
                        MeasurementResult {
                            kind: denominator.kind,
                            measurement: denominator.measurement,
                            cursor_pos: None,
                        },
                        denominator_rect,
                        buf,
                        _ctx,
                    );
                }

                if let Some(line_y) = area
                    .y
                    .checked_add(layout.measurement.ascent.saturating_sub(1))
                {
                    let text = repeat_n('─', layout.measurement.width as usize).collect::<String>();

                    Text::raw(text).centered().render(
                        Rect {
                            y: line_y,
                            height: 1,
                            width: layout.measurement.width,
                            ..area
                        },
                        buf,
                    );
                };
            }
        }

        if let Some((x, y, height)) = layout.cursor_pos {
            for y_offset in 0..height {
                let Some(cell) = area.x.checked_add(x).and_then(|x| {
                    area.y
                        .checked_add(y)?
                        .checked_add(y_offset)
                        .and_then(|y| buf.cell_mut((x, y)))
                }) else {
                    continue;
                };

                (cell.fg, cell.bg) = (cell.bg, cell.fg);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use matatree::{
        layout::LayoutEngine,
        render::Renderer,
        tree::{Atom, ContainerKind, Tree},
    };

    use super::*;

    #[test]
    fn renders_fraction_and_surrounding_atoms_on_expected_rows() {
        let mut tree = Tree::new();
        let fraction_id = {
            let mut root = tree.get_mut_sequence(tree.root_id()).unwrap();
            root.add_atom(Atom::Text("2".into()));
            root.add_atom(Atom::Operator('+'));
            let fraction_id = root.add_container(ContainerKind::fraction).unwrap();
            root.add_atom(Atom::Operator('-'));
            root.add_atom(Atom::Text("5".into()));
            fraction_id
        };

        let (numerator, denominator) = match tree.get_container(fraction_id).unwrap().kind() {
            matatree::tree::ContainerKind::Fraction {
                numerator,
                denominator,
            } => (*numerator, *denominator),
        };
        tree.get_mut_sequence(numerator)
            .unwrap()
            .add_atom(Atom::Text("x".into()));
        tree.get_mut_sequence(denominator)
            .unwrap()
            .add_atom(Atom::Text("3".into()));

        let layout = RatatuiLayout::layout_sequence(tree.root(), None);
        let mut buf = ratatui::buffer::Buffer::empty(Rect::new(0, 0, 20, 10));

        RatatuiRender::render_layout(
            layout,
            Rect::new(0, 1, 20, 8),
            &mut buf,
            &RenderContext::none(),
        );

        for (x, y, expected) in [
            (0, 3, "2"),
            (2, 3, "+"),
            (5, 1, "x"),
            (4, 3, "─"),
            (5, 3, "─"),
            (6, 3, "─"),
            (5, 5, "3"),
            (8, 3, "-"),
            (10, 3, "5"),
        ] {
            assert_eq!(buf.cell((x, y)).unwrap().symbol(), expected);
        }
    }
}
