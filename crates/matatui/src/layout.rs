mod atom;
mod fraction;

#[cfg(test)]
mod tests;

use std::cmp::max;

use matatree::{editor::cursor::Cursor, layout::*, tree::*};

use crate::layout::atom::layout_atom;

pub struct RatatuiLayout;

impl FontLayout<u16> for RatatuiLayout {
    fn measure_str(&self, s: &str) -> Measurement<u16> {
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

        let results = sequence
            .children()
            .into_iter()
            .map(|child| match child {
                ChildRef::Atom(atom) => layout_atom(atom),
                ChildRef::Container(container_ref) => {
                    let res = Self::layout_container(
                        container_ref,
                        check_cursor.then_some(cursor).flatten(),
                    );

                    check_cursor ^= res.cursor_pos.is_some();

                    measurement.ascent = max(res.measurement.ascent, measurement.ascent);
                    measurement.descent = max(res.measurement.descent, measurement.descent);

                    res
                }
            })
            .collect::<Vec<_>>();

        let cursor_child = this_sequence_highlighted
            .then(|| cursor.and_then(Cursor::child_index))
            .flatten();

        let mut cursor_pos = if this_sequence_highlighted && cursor_child.is_none() {
            Some((0, measurement.ascent - 1, 1))
        } else {
            None
        };

        let mut positioned_children = Vec::with_capacity(results.len());
        let mut current_x_pos: u16 = 0;

        for (idx, res) in results.into_iter().enumerate() {
            if idx > 0 {
                current_x_pos += 1;
            }

            let child_y = measurement.ascent - res.measurement.ascent;

            if let Some((cursor_idx, text_idx)) = cursor_child {
                if cursor_idx == idx {
                    let text_offset = match (&res.kind, text_idx) {
                        (NodeKindRef::Text(_), Some(text_idx)) => text_idx as u16,
                        (NodeKindRef::Text(_), None) => 0,
                        (NodeKindRef::Operator(_), _) => {
                            if let Some(
                                ChildRef::Container(_) | ChildRef::Atom(Atom::Operator(_)),
                            ) = cursor_idx
                                .checked_add(1)
                                .and_then(|idx| sequence.get_child(idx))
                            {
                                0
                            } else {
                                1
                            }
                        }
                        (NodeKindRef::Fraction { .. }, _) => {
                            if let Some(
                                ChildRef::Container(_) | ChildRef::Atom(Atom::Operator(_)),
                            ) = cursor_idx
                                .checked_add(1)
                                .and_then(|idx| sequence.get_child(idx))
                            {
                                res.measurement.width.saturating_sub(1)
                            } else {
                                res.measurement.width
                            }
                        }
                        (NodeKindRef::Sequence(_), _) => {
                            unreachable!("Sequence cannot be a child of a sequence")
                        }
                    };
                    cursor_pos = current_x_pos
                        .checked_add(text_offset)
                        .and_then(|x| Some((x.checked_add(1)?, measurement.ascent - 1, 1)));
                }
            } else if !this_sequence_highlighted {
                cursor_pos = cursor_pos.or_else(|| {
                    res.cursor_pos.and_then(|(x, y, h)| {
                        Some((x.checked_add(current_x_pos)?, y.checked_add(child_y)?, h))
                    })
                });
            }

            positioned_children.push(PositionedNodeRef {
                x: current_x_pos,
                y: child_y,
                kind: res.kind,
                measurement: res.measurement,
            });

            current_x_pos += res.measurement.width;
        }

        measurement.width = max(current_x_pos, 1);

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
