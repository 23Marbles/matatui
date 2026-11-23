use std::iter::repeat_n;

use matatree::{
    layout::{MeasurementResult, NodeKindRef, PositionedNodeRef},
    render::RenderContext,
};
use ratatui::{layout::Rect, style::Color, text::Text, widgets::Widget};

use crate::layout::RatatuiLayout;

#[cfg(test)]
mod tests;

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
        fn render_positioned_node(
            node_ref: PositionedNodeRef<'_, RatatuiLayout>,
            area: ratatui::prelude::Rect,
            buf: &mut ratatui::prelude::Buffer,
            // TODO add context handling
            _ctx: &RenderContext<'_>,
        ) {
            let (Some(x), Some(y)) = (
                area.x.checked_add_signed(node_ref.x),
                area.y.checked_add_signed(node_ref.y),
            ) else {
                return;
            };

            let rect = Rect {
                x,
                y,
                width: node_ref.measurement.width as u16,
                height: node_ref.measurement.height() as u16,
            };

            if !buf.area.intersects(rect) {
                return;
            }

            RatatuiRender::render_layout(
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

        match layout.kind {
            NodeKindRef::Text(text) => {
                if let Some(line_y) = area
                    .y
                    .checked_add_signed(layout.measurement.ascent.saturating_sub(1))
                {
                    Text::raw(text).centered().render(
                        Rect {
                            y: line_y,
                            height: 1,
                            width: layout.measurement.width as u16,
                            ..area
                        },
                        buf,
                    );
                };
            }
            NodeKindRef::Operator(ch) => {
                if let Some(line_y) = area
                    .y
                    .checked_add_signed(layout.measurement.ascent.saturating_sub(1))
                {
                    Text::raw(ch.to_string()).centered().render(
                        Rect {
                            y: line_y,
                            height: 1,
                            width: layout.measurement.width as u16,
                            ..area
                        },
                        buf,
                    );
                }
            }
            NodeKindRef::Sequence(positioned_node_refs) => {
                if positioned_node_refs.is_empty() {
                    Text::raw("░").render(area, buf);
                }

                for node_ref in positioned_node_refs {
                    render_positioned_node(node_ref, area, buf, _ctx);
                }
            }
            NodeKindRef::Fraction {
                numerator,
                denominator,
            } => {
                render_positioned_node(*numerator, area, buf, _ctx);
                render_positioned_node(*denominator, area, buf, _ctx);

                if let Some(line_y) = area
                    .y
                    .checked_add_signed(layout.measurement.ascent.saturating_sub(1))
                {
                    let text = repeat_n('─', layout.measurement.width as usize).collect::<String>();

                    Text::raw(text).centered().render(
                        Rect {
                            y: line_y,
                            height: 1,
                            width: layout.measurement.width as u16,
                            ..area
                        },
                        buf,
                    );
                };
            }
            NodeKindRef::SingleSequence(node_ref) => {
                render_positioned_node(*node_ref, area, buf, _ctx);
            }
        }

        if let Some((x, y, height)) = layout.cursor_pos {
            for y_offset in 0..height {
                let Some(cell) = area.x.checked_add_signed(x).and_then(|x| {
                    area.y
                        .checked_add_signed(y)?
                        .checked_add_signed(y_offset)
                        .and_then(|y| buf.cell_mut((x, y)))
                }) else {
                    continue;
                };

                (cell.fg, cell.bg) = (cell.bg, Color::LightBlue);
            }
        }
    }
}
