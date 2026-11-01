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
        enum CursorState<'a> {
            Resolved((u16, u16, u16)),
            Finding(&'a Cursor),
        }

        let results = sequence.children().into_iter().map(|child| match child {
            ChildRef::Atom(atom) => layout_atom(atom),
            ChildRef::Container(container_ref) => todo!(),
        });

        todo!()
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
