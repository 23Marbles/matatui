use matatree::{
    editor::cursor::Cursor,
    layout::{LayoutEngine, Measurement, MeasurementResult, NodeKindRef, PositionedNodeRef},
    tree::SequenceRef,
};

use crate::layout::{RatatuiLayout, RatatuiMeasurementResult};

pub fn super_script<'a>(
    sequence: SequenceRef<'a>,
    cursor: Option<&Cursor>,
    previous_ascent: i16,
) -> RatatuiMeasurementResult<'a> {
    let MeasurementResult {
        kind,
        cursor_pos,
        measurement,
    } = RatatuiLayout::layout_sequence(sequence, cursor);

    MeasurementResult {
        kind: NodeKindRef::SingleSequence(Box::new(PositionedNodeRef {
            x: 0,
            y: 0,
            kind,
            measurement,
        })),
        // No change as the rect is moved up so the cursor just "goes with" it
        cursor_pos,
        measurement: Measurement {
            width: measurement.width,
            ascent: measurement.height() + previous_ascent,
            descent: 0,
        },
    }
}

pub fn sub_script<'a>(
    sequence: SequenceRef<'a>,
    cursor: Option<&Cursor>,
    previous_descent: i16,
) -> RatatuiMeasurementResult<'a> {
    let MeasurementResult {
        kind,
        cursor_pos,
        measurement,
    } = RatatuiLayout::layout_sequence(sequence, cursor);

    MeasurementResult {
        kind: NodeKindRef::SingleSequence(Box::new(PositionedNodeRef {
            x: 0,
            y: 0,
            kind,
            measurement,
        })),
        // Move the y pos down by the descent we are moving down by
        cursor_pos: cursor_pos.map(|(x, y, h)| (x, y + previous_descent, h)),
        measurement: Measurement {
            width: measurement.width,
            ascent: 0,
            descent: measurement.height() + previous_descent,
        },
    }
}
