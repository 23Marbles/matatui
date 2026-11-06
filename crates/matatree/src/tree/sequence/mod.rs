mod access;
mod event_handling;
mod mutation;
mod text;
mod tree_mutation;

#[cfg(test)]
mod tests;

use super::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sequence {
    pub(super) parent: Option<ContainerId>,
    pub(super) children: Vec<Child>,
}

impl Sequence {
    pub(super) fn new_unchecked(parent: Option<ContainerId>) -> Self {
        Self {
            parent,
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SequenceRef<'a> {
    id: SequenceId,
    seq: &'a Sequence,
    tree: &'a Tree,
}

pub struct SequenceMut<'a> {
    id: SequenceId,
    tree: &'a mut Tree,
}

#[derive(Debug, Clone)]
pub struct OwnedSequence(pub Vec<OwnedChild>);
