use matatree::{layout::*, tree::Atom};

use super::RatatuiLayout;

pub fn layout_atom<'a>(atom: &'a Atom) -> MeasurementResult<'a, RatatuiLayout> {
    match atom {
        Atom::Text(text) => MeasurementResult {
            kind: NodeKindRef::Text(text),
            measurement: RatatuiLayout::measure_str(text),
            cursor_pos: None,
        },
        Atom::Operator(ch) => MeasurementResult {
            kind: NodeKindRef::Operator(*ch),
            measurement: Measurement {
                width: 1,
                ascent: 1,
                descent: 0,
            },
            cursor_pos: None,
        },
    }
}
