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
                if let Some(line_y) = area.bottom().checked_add(layout.measurement.descent) {
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
                if let Some(line_y) = area.bottom().checked_add(layout.measurement.descent) {
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
                        area.y.checked_sub(node_ref.y),
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
                    area.y.checked_sub(numerator.y),
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
                    area.y.checked_sub(denominator.y),
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

                if let Some(line_y) = area.bottom().checked_add(layout.measurement.descent) {
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
                let Some(cell) = area
                    .x
                    .checked_add(x)
                    .and_then(|x| buf.cell_mut((x, area.y.checked_sub(y.checked_add(y_offset)?)?)))
                else {
                    continue;
                };

                (cell.fg, cell.bg) = (cell.bg, cell.fg);
            }
        }
    }
}
