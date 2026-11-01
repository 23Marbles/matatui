use std::cmp::max;

use matatree::{editor::cursor::Cursor, layout::*, tree::*};

use crate::layout::atom::layout_atom;

mod atom;
mod fraction;

pub struct RatatuiLayout;

impl FontLayout<u16> for RatatuiLayout {
    fn measure_str(s: &str) -> Measurement<u16> {
        Measurement {
            width: <str as unicode_segmentation::UnicodeSegmentation>::graphemes(s, true).count()
                as u16,
            ascent: 1,
            descent: 0,
        }
    }
}

impl LayoutEngine for RatatuiLayout {
    type Unit = u16;

    type FontLayout = Self;

    fn layout_sequence<'a>(
        sequence: SequenceRef<'a>,
        cursor: Option<&Cursor>,
    ) -> matatree::layout::MeasurementResult<'a, Self> {
        let this_sequence_highlighted = cursor.is_some_and(|c| c.sequence_id() == sequence.id());

        let mut check_cursor = !this_sequence_highlighted;

        let mut measurement = Measurement {
            width: 0,
            ascent: 1,
            descent: 0,
        };

        let mut results = sequence
            .children()
            .into_iter()
            .map(|child| match child {
                ChildRef::Atom(atom) => layout_atom(atom),
                ChildRef::Container(container_ref) => {
                    let res = Self::layout_container(
                        container_ref,
                        check_cursor.then_some(cursor).flatten(),
                    );

                    check_cursor |= res.cursor_pos.is_some();

                    measurement.ascent = max(res.measurement.ascent, measurement.ascent);
                    measurement.descent = max(res.measurement.descent, measurement.descent);

                    res
                }
            })
            .collect::<Vec<_>>()
            .into_iter();

        let mut cursor_pos = None;

        let mut positioned_children = Vec::new();

        let mut current_x_pos = 0;

        if let Some(res) = results.next() {
            let child_y = measurement.ascent - res.measurement.ascent;
            cursor_pos = cursor_pos.or_else(|| {
                res.cursor_pos
                    .and_then(|(x, y, h)| Some((x, y.checked_add(child_y)?, h)))
            });

            positioned_children.push(PositionedNodeRef {
                x: 0,
                y: child_y,
                kind: res.kind,
                measurement: res.measurement,
            });

            current_x_pos += res.measurement.width;
        }

        for res in results {
            current_x_pos += 1;
            let child_y = measurement.ascent - res.measurement.ascent;

            cursor_pos = cursor_pos.or_else(|| {
                res.cursor_pos.and_then(|(x, y, h)| {
                    Some((
                        x.checked_add(current_x_pos)?,
                        y.checked_add(child_y)?,
                        h,
                    ))
                })
            });

            positioned_children.push(PositionedNodeRef {
                x: current_x_pos,
                y: child_y,
                kind: res.kind,
                measurement: res.measurement,
            });

            current_x_pos += res.measurement.width;
        }

        if cursor_pos.is_none() && this_sequence_highlighted {
            cursor_pos = Some((0, measurement.ascent - 1, 1))
        }

        measurement.width = current_x_pos;

        MeasurementResult {
            kind: NodeKindRef::Sequence(positioned_children),
            measurement,
            cursor_pos,
        }
    }

    fn layout_container<'a>(
        container: ContainerRef<'a>,
        cursor: Option<&Cursor>,
    ) -> matatree::layout::MeasurementResult<'a, Self> {
        match *container.kind() {
            ContainerKind::Fraction {
                numerator,
                denominator,
            } => fraction::layout_fraction(container, cursor, numerator, denominator),
        }
    }
}
