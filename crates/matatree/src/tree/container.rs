use super::*;

mod event_handling;
mod mutable;
mod reference;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ContainerKind {
    Fraction {
        numerator: SequenceId,
        denominator: SequenceId,
    },
}

impl ContainerKind {
    pub fn fraction(seq_ids: [SequenceId; 2]) -> Self {
        Self::Fraction {
            numerator: seq_ids[0],
            denominator: seq_ids[1],
        }
    }
}

#[derive(Debug, Clone)]
pub enum OwnedContainerKind {
    Fraction {
        numerator: OwnedSequence,
        denominator: OwnedSequence,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Container {
    pub(super) kind: ContainerKind,

    pub(super) parent_id: SequenceId,
}

impl Container {
    pub fn kind(&self) -> &ContainerKind {
        &self.kind
    }

    pub fn parent_id(&self) -> SequenceId {
        self.parent_id
    }

    pub(super) fn new_unchecked(parent_id: SequenceId, kind: ContainerKind) -> Self {
        Self { parent_id, kind }
    }
}

#[derive(Debug, Copy)]
pub struct ContainerRef<'a> {
    id: ContainerId,
    tree: &'a Tree,
    container: &'a Container,
}

pub struct ContainerMut<'a> {
    /// On construction this is guaranteed to exist
    pub(super) id: ContainerId,

    /// Mutating tree must not allow `Self::id` to become invalid
    pub(super) tree: &'a mut Tree,
}
#[derive(Debug, Clone)]
pub struct OwnedContainer(pub OwnedContainerKind);
