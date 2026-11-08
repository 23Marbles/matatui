use std::iter::repeat_n;

use matatree::{
    layout::{MeasurementResult, NodeKindRef},
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
                    let (Some(x), Some(y)) = (
                        area.x.checked_add_signed(node_ref.x),
                        area.y.checked_add_signed(node_ref.y),
                    ) else {
                        continue;
                    };

                    let rect = Rect {
                        x,
                        y,
                        width: node_ref.measurement.width as u16,
                        height: node_ref.measurement.height() as u16,
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
                    area.x.checked_add_signed(numerator.x),
                    area.y.checked_add_signed(numerator.y),
                ) {
                    let numerator_rect = Rect {
                        x,
                        y,
                        width: numerator.measurement.width as u16,
                        height: numerator.measurement.height() as u16,
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
                    area.x.checked_add_signed(denominator.x),
                    area.y.checked_add_signed(denominator.y),
                ) {
                    let denominator_rect = Rect {
                        x,
                        y,
                        width: denominator.measurement.width as u16,
                        height: denominator.measurement.height() as u16,
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
