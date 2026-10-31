use crate::tree::SequenceId;


/// The cursor which edits a `Tree`
pub struct Cursor {
    sequence_id: SequenceId,

    child_index: Option<(usize, Option<usize>)>,
}
