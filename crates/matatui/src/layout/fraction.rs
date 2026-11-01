use matatree::{editor::cursor::Cursor, tree::ContainerRef};

use std::cmp::max;

use matatree::{layout::MeasurementResult, tree::SequenceId};

use super::*;

pub fn layout_fraction<'a>(
    container: ContainerRef<'a>,
    cursor: Option<&Cursor>,
    numerator: SequenceId,
    denominator: SequenceId,
) -> MeasurementResult<'a, RatatuiLayout> {
    let MeasurementResult {
        kind: numerator_kind,
        measurement: numerator_measurement,
        cursor_pos: cursor_pos1,
    } = RatatuiLayout::layout_sequence(container.tree().get_sequence(numerator).unwrap(), cursor);

    let MeasurementResult {
        kind: denominator_kind,
        measurement: denominator_measurement,
        cursor_pos: cursor_pos2,
    } = RatatuiLayout::layout_sequence(
        container.tree().get_sequence(denominator).unwrap(),
        if cursor_pos1.is_none() { cursor } else { None },
    );

    let measurement = Measurement {
        width: max(numerator_measurement.width, denominator_measurement.width).saturating_add(2),
        ascent: numerator_measurement.height().saturating_add(2),
        descent: denominator_measurement.height().saturating_add(1),
    };

    let numerator_node: PositionedNodeRef<'a, RatatuiLayout> = {
        PositionedNodeRef {
            // The width is 2 greater than this or bigger so this never panics in release
            x: (measurement.width - numerator_measurement.width) / 2,
            y: 0,
            kind: numerator_kind,
            measurement: numerator_measurement,
        }
    };

    let denominator_node: PositionedNodeRef<'a, RatatuiLayout> = {
        PositionedNodeRef {
            // The width is 2 greater than this or bigger so this never panics in release
            x: (measurement.width - denominator_measurement.width) / 2,
            // The denominator_measurement's height is part of measurement so it must be less
            y: measurement.height() - denominator_measurement.height(),
            kind: denominator_kind,
            measurement: denominator_measurement,
        }
    };

    let cursor_pos = (|| match (cursor_pos1, cursor_pos2) {
        (None, None) => None,
        (Some((x, y, h)), None) => Some((
            x.checked_add(numerator_node.x)?,
            y.checked_add(numerator_node.y)?,
            h,
        )),
        (None, Some((x, y, h))) => Some((
            x.checked_add(denominator_node.x)?,
            y.checked_add(denominator_node.y)?,
            h,
        )),
        (Some(_), Some(_)) => {
            log::error!(
                "Both sequences in a fraction reported having a cursor position, one is lying so no cursor passed up"
            );
            None
        }
    })();

    MeasurementResult {
        measurement,
        kind: NodeKindRef::Fraction {
            numerator: Box::new(numerator_node),
            denominator: Box::new(denominator_node),
        },
        cursor_pos,
    }
}
