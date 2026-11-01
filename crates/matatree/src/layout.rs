use num_traits::Num;

use crate::{
    editor::cursor::Cursor,
    tree::{ContainerRef, SequenceRef},
};

/// A trait used by `LayoutEngine` implementors in order to measure the dimensions of a peice of text.
/// The measurement measures from the baseline of the text.
pub trait FontLayout<U: Num> {
    /// Measure the size of an &str. This comes with a width, ascent and descent
    /// for accurate positioing around a baseline.
    fn measure_str(s: &str) -> Measurement<U>;
}

/// A trait that recursively lays out the elements of a tree as `Measurement`s
pub trait LayoutEngine: Sized {
    /// The type which all measurements are taken in. E.g. u16 in terminals,
    /// f64 for guis etc.
    type Unit: Num;

    /// The algotithm which will be used in order to measure the text.
    type FontLayout: FontLayout<Self::Unit>;

    fn layout_sequence<'a>(
        sequence: SequenceRef<'a>,
        cursor: &Cursor,
    ) -> MeasurementResult<'a, Self>;
    fn layout_container<'a>(
        container: ContainerRef<'a>,
        cursor: &Cursor,
    ) -> MeasurementResult<'a, Self>;
}

/// A reference to a specific kind of node for rendering purposes
pub enum NodeKindRef<'a, L: LayoutEngine> {
    Text(&'a str),

    Sequence(Vec<PositionedNodeRef<'a, L>>),

    Operator(char),
    // Container node kinds
}

/// A non-positioned measurement with an ascent and descent
pub struct Measurement<U: Num> {
    /// The width of the measurement
    pub width: U,

    /// The height above the baseline
    pub ascent: U,

    /// The height below the baseline
    pub descent: U,
}

impl<U: Num + Clone> Measurement<U> {
    /// The height of the measurement.
    #[inline(always)]
    pub fn height(&self) -> U {
        self.ascent.clone() + self.descent.clone()
    }
}

pub struct MeasurementResult<'a, L: LayoutEngine> {
    pub kind: NodeKindRef<'a, L>,

    pub measurement: Measurement<L::Unit>,

    /// The cursor position (if any). Is stored as (x, y)
    pub cursor_pos: Option<(L::Unit, L::Unit)>,
    pub cursor_height: L::Unit,
}

pub struct PositionedNodeRef<'a, L: LayoutEngine> {
    pub x: L::Unit,

    pub y: L::Unit,

    pub kind: NodeKindRef<'a, L>,

    pub measurement: Measurement<L::Unit>,
}
