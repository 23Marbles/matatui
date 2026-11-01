use matatree::layout::Measurement;
use unicode_segmentation::UnicodeSegmentation;

pub struct RatatuiLayout;

impl matatree::layout::FontLayout<u16> for RatatuiLayout {
    fn measure_str(s: &str) -> Measurement<u16> {
        Measurement {
            width: s.graphemes(true).count() as u16,
            ascent: 1,
            descent: 0,
        }
    }
}

impl matatree::layout::LayoutEngine for RatatuiLayout {
    type Unit = u16;

    type FontLayout = Self;

    fn layout_sequence<'a>(
        sequence: matatree::tree::SequenceRef<'a>,
        cursor: &matatree::editor::cursor::Cursor,
    ) -> matatree::layout::MeasurementResult<'a, Self> {
        todo!()
    }

    fn layout_container<'a>(
        container: matatree::tree::ContainerRef<'a>,
        cursor: &matatree::editor::cursor::Cursor,
    ) -> matatree::layout::MeasurementResult<'a, Self> {
        todo!()
    }
}
