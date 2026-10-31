use crate::tree::SequenceId;

/// The cursor which edits a `Tree`
pub struct Cursor {
    pub(super) sequence_id: SequenceId,

    pub(super) child_index: Option<(usize, Option<usize>)>,
}
