use crate::tree::SequenceId;

/// The cursor which edits a `Tree`
pub struct Cursor {
    pub(super) sequence_id: SequenceId,

    pub(super) child_index: Option<(usize, Option<usize>)>,
}

impl Cursor {
    pub fn sequence_id(&self) -> SequenceId {
        self.sequence_id
    }

    pub fn child_index(&self) -> Option<(usize, Option<usize>)> {
        self.child_index
    }
}